use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataResponse<T> {
    code: u32,
    success: bool,
    data: Option<T>,
    message: String,
}

#[allow(dead_code)]
impl<T> DataResponse<T> {
    pub fn success(data: T) -> Self {
        DataResponse {
            code: 200,
            success: true,
            data: Some(data),
            message: String::from("请求成功"),
        }
    }
    pub fn failure(message: String) -> Self {
        DataResponse {
            code: 400,
            success: false,
            data: None,
            message,
        }
    }

    pub fn fast_failure() -> Self {
        DataResponse {
            code: 400,
            success: false,
            data: None,
            message: String::from("请求失败"),
        }
    }

    pub fn fast_success() -> Self {
        DataResponse {
            code: 200,
            success: true,
            data: None,
            message: String::from("请求成功"),
        }
    }
}
