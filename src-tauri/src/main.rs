// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, io};

use serde_json::{json, Value};
use tauri::Manager;
use tauri_plugin_os::Version::Semantic;
use tauri_plugin_store::StoreExt;
use tiktoken_rs::o200k_base;
use walkdir::WalkDir;
use window_vibrancy::apply_mica;

use crate::data_response::DataResponse;
use crate::my_file::MyFile;

mod data_response;
mod my_file;

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
            let mut is_dark = false;
            // 得到软件数据存储路径
            let data_path = app.path().app_data_dir().unwrap();
            // 创建一个bin文件，用于存储isDark的值
            let bin_path = data_path.join("code-merger-tauri.bin");
            // 如果bin文件不存在，则创建一个，否则会报错
            if !(bin_path.exists()) {
                File::create(&bin_path).unwrap();
            }
            // 创建一个store管理器
            let store = app.handle().store_builder(bin_path).build();
            // 从磁盘得到isDark的值
            if store.has("isDark") {
                // 如果有值，则取出来，赋值给is_dark
                is_dark = store
                    .get("isDark")
                    .unwrap_or(Value::Null)
                    .as_bool()
                    .unwrap_or(false);
            } else {
                // 如果没有值，则设置一个默认值，然后保存到磁盘
                store.set("isDark", json!(is_dark));
                store.save().expect("Failed to save store to disk");
            }

            // 创建主窗口
            let main_window = tauri::WebviewWindowBuilder::new(app, "main", Default::default())
                .title("code-merger-tauri")
                .transparent(is_win11)
                .center()
                .visible(false)
                .build()?;
            // 如果是windows 11，则设置窗口透明，并且应用mica效果
            if is_win11 {
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

// 输入一个文件夹路径，得到该路径下的所有文件和文件夹的路径，并判断子文件夹是否为空
#[tauri::command(async)]
fn get_sub_files(path: &str) -> DataResponse<Vec<MyFile>> {
    if !Path::new(path).exists() {
        return DataResponse::failure("文件夹不存在".to_string());
    }

    let mut files = Vec::new();

    match fs::read_dir(path) {
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
                files.push(MyFile::new(&path_str, is_folder, is_folder_empty));
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
fn merge_files(path: &str, exclude: Option<Vec<String>>) -> DataResponse<String> {
    if !Path::new(path).exists() {
        return DataResponse::failure("文件夹不存在".to_string());
    }

    let mut res = String::new();
    let exclude: Vec<String> = exclude
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect();

    let paths = if Path::new(path).is_file() {
        vec![PathBuf::from(path)]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.into_path())
            .collect()
    };

    for path in paths {
        if let Some(ext) = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
        {
            if exclude.contains(&ext) {
                continue;
            }
        }

        match read_file_to_string(&path) {
            Ok(content) => {
                res.push_str(format!("> {}\n", path.to_string_lossy()).as_str());
                res.push_str("```\n");
                res.push_str(&content);
                res.push_str("\n```\n");
            }
            Err(_) => {
                res.push_str(format!("> {}\n", path.to_string_lossy()).as_str());
                res.push_str("```\n");
                res.push_str("该文件是二进制文件，具体内容已忽略");
                res.push_str("\n```\n");
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
fn are_files_less_than(paths: Vec<&str>, num: usize) -> DataResponse<bool> {
    let mut file_count = 0;
    for path in paths {
        if !Path::new(path).exists() {
            continue;
        }
        if Path::new(path).is_file() {
            file_count += 1;
        } else {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                if entry.path().is_file() {
                    file_count += 1;
                    if file_count >= num {
                        return DataResponse::success(false);
                    }
                }
            }
        }
    }
    if file_count < num {
        DataResponse::success(true)
    } else {
        DataResponse::success(false)
    }
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
