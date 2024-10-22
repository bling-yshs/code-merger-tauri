// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ignore::WalkBuilder;
use request::are_files_less_than_request::AreFilesLessThanRequest;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tauri::Manager;
use tauri_plugin_os::Version::Semantic;
use tauri_plugin_store::StoreExt;
use tiktoken_rs::o200k_base;
use walkdir::{DirEntry, WalkDir};
use window_vibrancy::apply_mica;

use data_response::DataResponse;
use my_file::MyFile;
use request::get_sub_files_request::GetSubFilesRequest;
use request::merge_files_request::MergeFilesRequest;

mod data_response;
mod my_file;
mod request;

fn main() {
    // 判断是不是 windows 11，如果是则设置窗口透明
    let is_win11 = matches!(
        tauri_plugin_os::version(),
        Semantic(_, _, c) if cfg!(target_os = "windows") && c >= 22000
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(move |app| {
            // 得到软件数据存储路径
            let data_path = app.path().app_data_dir().unwrap();
            // 创建一个bin文件，用于存储本地数据
            let bin_path = data_path.join("code-merger-tauri.bin");
            // 如果bin文件不存在，则创建一个，否则会报错
            if !(bin_path.exists()) {
                File::create(&bin_path).unwrap();
            }
            // 创建一个store管理器
            let store = app.handle().store_builder(bin_path).build();
            // 旧版本兼容
            let is_dark = store
                .get("isDark")
                .unwrap_or(Value::Null)
                .as_bool()
                .unwrap_or(false);
            store.delete("isDark");
            if is_dark {
                store.set("theme", "dark");
            }
            // 从磁盘得到isDark的值
            let theme = store
                .get("theme")
                .unwrap_or(Value::Null)
                .as_str()
                .unwrap_or("light")
                .to_string();
            let _ = store.save();
            // 创建主窗口
            let main_window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
                .title("code-merger-tauri")
                .transparent(is_win11)
                .center()
                .visible(false)
                .build()?;
            // 如果是windows 11，则设置窗口透明，并且应用mica效果
            if is_win11 {
                let is_dark = theme == "dark";
                let _ = apply_mica(&main_window, Some(is_dark));
            }
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_sub_files,
            merge_files,
            is_existing_directory,
            count_tokens,
            are_files_less_than
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 输入一个文件夹路径，得到当前文件夹下的一级文件和文件夹的相对路径，完整路径，以及是否为文件夹，是否为空文件夹
#[tauri::command(async)]
fn get_sub_files(request: GetSubFilesRequest) -> DataResponse<Vec<MyFile>> {
    if !Path::new(request.current_path.as_str()).exists() {
        return DataResponse::failure("文件夹不存在".to_string());
    }

    let mut files = Vec::new();

    match fs::read_dir(request.current_path.as_str()) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let path_str = path.to_string_lossy().to_string();
                let is_folder = path.is_dir();
                let is_folder_empty = if is_folder {
                    match fs::read_dir(&path) {
                        Ok(mut dir_entries) => dir_entries.next().is_none(),
                        Err(_) => false,
                    }
                } else {
                    false
                };
                // 计算相对路径
                let relative_path = path
                    .strip_prefix(&PathBuf::from(&request.root_path))
                    .unwrap()
                    .to_str()
                    .unwrap();
                files.push(MyFile::new(
                    &path_str,
                    relative_path,
                    is_folder,
                    is_folder_empty,
                ));
            }
        }
        Err(_) => {
            return DataResponse::failure("读取文件夹内容失败".to_string());
        }
    }

    DataResponse::success(files)
}

// 合并文件夹下的所有文件内容
#[tauri::command(async)]
fn merge_files(request: MergeFilesRequest) -> DataResponse<String> {
    let root_path = &request.root_path;

    if !Path::new(root_path).exists() {
        return DataResponse::failure("文件夹不存在".to_string());
    }

    let walker = WalkDir::new(root_path).into_iter();

    let mut res = String::new();

    if request.enable_gitignore {
        // 使用 ignore::Walk 进行遍历
        let walker = ignore::WalkBuilder::new(root_path).hidden(true).build();
        for entry in walker
            .filter_map(Result::ok)
            .filter(|each| {
                !is_path_excluded(each.path(), &request.no_selected_paths)
            })
            .filter(|each| {
                !is_dir_excluded(each.path(), Path::new(root_path), &request.exclude_dirs)
            })
            .filter(|each| each.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .filter(|each| {
                !is_ext_excluded(each.path(), &request.exclude_exts)
            })
        {
            let relative_path = entry
                .path()
                .strip_prefix(root_path)
                .unwrap()
                .to_str()
                .unwrap();
            let content = read_file_to_string(entry.path()).unwrap_or_else(|_| {
                res.push_str(
                    format!(
                        "> {}\n```\n该文件是二进制文件，具体内容已忽略\n```\n",
                        entry.path().to_string_lossy()
                    )
                        .as_str(),
                );
                String::new()
            });

            if !content.is_empty() {
                res.push_str(format!("> {}\n```\n{}\n```\n", relative_path, content).as_str());
            }
        }
    } else {
        for entry in walker
            // 过滤掉在排除列表中的文件夹
            .filter_entry(|each| {
                !is_path_excluded(each.path(), &request.no_selected_paths)
            })
            .filter_map(Result::ok)
            .filter(|each| {
                !is_dir_excluded(each.path(), Path::new(root_path), &request.exclude_dirs)
            })
            .filter(|each| each.path().is_file())
            .filter(|each| {
                !is_ext_excluded(each.path(), &request.exclude_exts)
            })
        {
            let relative_path = entry
                .path()
                .strip_prefix(root_path)
                .unwrap()
                .to_str()
                .unwrap();
            let content = read_file_to_string(entry.path()).unwrap_or_else(|_| {
                res.push_str(
                    format!(
                        "> {}\n```\n该文件是二进制文件，具体内容已忽略\n```\n",
                        entry.path().to_string_lossy()
                    )
                        .as_str(),
                );
                String::new()
            });

            if !content.is_empty() {
                res.push_str(format!("> {}\n```\n{}\n```\n", relative_path, content).as_str());
            }
        }
    }

    if res.is_empty() {
        return DataResponse::failure("该文件夹下没有任何可读文件".to_string());
    }

    DataResponse::success(res)
}

// 读取文件内容到字符串
fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    match String::from_utf8(buffer) {
        Ok(content) => Ok(content),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid UTF-8 sequence",
        )),
    }
}

// 判断传入的路径下的总文件数量，是否小于某个数
#[tauri::command(async)]
fn are_files_less_than(request: AreFilesLessThanRequest) -> DataResponse<bool> {
    let root_path = Path::new(&request.root_path);

    if !root_path.exists() {
        return DataResponse::failure("文件夹不存在".to_string());
    }

    let mut file_count = 0;

    if request.enable_gitignore {
        let walker = WalkBuilder::new(&request.root_path).hidden(true).build();
        for entry in walker.filter_map(Result::ok)
            .filter(|each| {
                !is_path_excluded(each.path(), &request.exclude_dirs)
            })
            .filter(|each| {
                !is_dir_excluded(each.path(), Path::new(root_path), &request.exclude_dirs)
            }) {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                file_count += 1;
                if file_count >= request.num {
                    return DataResponse::success(false);
                }
            }
        }
    } else {
        for entry in WalkDir::new(&request.root_path)
            .into_iter()
            .filter_entry(|each| {
                !is_path_excluded(each.path(), &request.exclude_dirs)
            })
            .filter_map(Result::ok)
            .filter(|each| {
                !is_dir_excluded(each.path(), Path::new(root_path), &request.exclude_dirs)
            })
        {
            if entry.file_type().is_file() {
                file_count += 1;
                if file_count >= request.num {
                    return DataResponse::success(false);
                }
            }
        }
    }

    DataResponse::success(file_count < request.num)
}

// 判断传入的路径是否存在，并且是否是文件夹
#[tauri::command(async)]
fn is_existing_directory(path: &str) -> DataResponse<bool> {
    DataResponse::success(fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false))
}

// 通过 tiktoken-rs 计算传入的字符串有多少 token
#[tauri::command(async)]
fn count_tokens(content: &str) -> DataResponse<usize> {
    let bpe = o200k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(content);
    let length = tokens.len();
    DataResponse::success(length)
}

// 判断路径是否在排除列表中，如果在则返回true，否则返回false
fn is_path_excluded(path: &Path, exclude_paths: &[String]) -> bool {
    exclude_paths.iter().any(|each| {
        let exclude_path = Path::new(each);
        // 检查是否以某个 exclude_path 为前缀
        path.strip_prefix(exclude_path).is_ok()
    })
}

// 判断路径的扩展名是否在排除列表中，如果在则返回true，否则返回false
fn is_ext_excluded(path: &Path, exclude_exts: &[String]) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map_or(false, |ext| exclude_exts.contains(&ext.to_string()))
}

// 判断路径是否在排除列表中，如果在则返回true，否则返回false
fn is_dir_excluded(path: &Path, root_path: &Path, exclude_dirs: &[String]) -> bool {
    exclude_dirs.iter().any(|each| {
        // 去掉前缀，然后看看是否路径中包含 exclude_dir
        path.strip_prefix(root_path)
            .ok()
            .and_then(|p| p.to_str())
            .map_or(false, |p| p.contains(each))
    })
}