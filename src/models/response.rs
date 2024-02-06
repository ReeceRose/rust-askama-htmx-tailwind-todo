use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse {
    pub success: bool,
    pub error: String,
    pub status_code: u16,
}

impl BaseResponse {
    pub fn success() -> Self {
        Self {
            status_code: 200,
            success: true,
            error: "".to_string(),
        }
    }

    pub fn error(status_code: u16, error: String) -> Self {
        Self {
            status_code,
            success: false,
            error,
        }
    }
}
