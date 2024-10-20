use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreFilesLessThanRequest {
    pub root_path: String,
    pub num: i32,
    pub exclude_exts: Vec<String>,
    pub exclude_paths: Vec<String>,
    pub enable_gitignore: bool,
}
