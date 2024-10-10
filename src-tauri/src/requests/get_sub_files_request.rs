use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubFilesRequest {
    pub root_path: String,
    pub current_path: String,
}
