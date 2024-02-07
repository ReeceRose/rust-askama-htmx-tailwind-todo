use thiserror::Error;

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
