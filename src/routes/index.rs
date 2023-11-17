use crate::{error::AppError, templates::GetIndexResponse};

use anyhow::Result;

pub async fn get_index() -> Result<GetIndexResponse, AppError> {
    Ok(GetIndexResponse)
}
