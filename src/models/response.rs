use axum::{
    extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

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

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(BaseResponse))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for BaseResponse {
    fn from(rejection: JsonRejection) -> Self {
        BaseResponse::error(rejection.status().as_u16(), rejection.body_text())
    }
}

impl IntoResponse for BaseResponse {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "status_code": self.status_code,
            "success": self.success,
            "error": self.error
        });
        let code =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (code, axum::Json(payload)).into_response()
    }
}
