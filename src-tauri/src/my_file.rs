use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyFile {
    path: String,
    relative_path: String,
    is_folder: bool,
    is_folder_empty: bool,
}

impl MyFile {
    pub fn new(path: &str, relative_path: &str, is_folder: bool, is_folder_empty: bool) -> Self {
        MyFile {
            path: path.to_string(),
            relative_path: relative_path.to_string(),
            is_folder,
            is_folder_empty,
        }
    }
}
