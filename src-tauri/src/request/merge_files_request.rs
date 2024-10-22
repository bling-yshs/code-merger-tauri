use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeFilesRequest {
    pub root_path: String,
    pub no_selected_paths: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub exclude_exts: Vec<String>,
    pub enable_gitignore: bool,
}
