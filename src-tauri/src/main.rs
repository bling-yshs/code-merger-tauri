// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, io};
use std::io::Read;
use std::path::{Path, PathBuf};

use tauri::Manager;
use walkdir::WalkDir;
use window_vibrancy::apply_mica;

use crate::data_response::DataResponse;
use crate::my_file::MyFile;

mod data_response;
mod my_file;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
                let _ = apply_mica(&window, None);

            window.show().unwrap();
            window.set_focus().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_sub_files, merge_files])
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
                println!("{:?} 是一个二进制文件", path);
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
