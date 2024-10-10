use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MergeFilesRequest {
    pub root_path: String,
    pub exclude_exts: Vec<String>,
    pub exclude_paths: Vec<String>,
}
