use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

// TODO: refactor to remove error?
// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Todo not found.")]
    NotFound,
    #[error("Failed to get todos from database.")]
    FailedToGetLock,
    #[error("Failed to get todos.")]
    FailedToGet,
    #[error("Failed to create todo. Please try again.")]
    FailedToCreate,
    #[error("Failed to update todo. Please try again.")]
    FailedToUpdate,
    #[error("Failed to delete todo. Please try again.")]
    FailedToDelete,
}

// impl IntoResponse for TodoError {
//     fn into_response(self) -> Response {
//         let mut target = "this";
//         match self {
//             Self::FailedToCreate => {}
//             Self::FailedToDelete => {}
//             Self::FailedToUpdate => {}
//             Self::FailedToGet => target = "#load-error",
//             Self::FailedToGetLock => {}
//             Self::NotFound => {}
//         }
//         (
//             StatusCode::OK,
//             [
//                 ("HX-Retarget", target),
//                 ("HX-Reswap", "innerHTML"),
//                 ("content-type", "text/html"),
//             ],
//             format!("Something went wrong: {}", self),
//         )
//             .into_response()
//     }
// }
