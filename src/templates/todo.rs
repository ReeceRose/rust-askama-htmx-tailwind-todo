use askama::Template;
use serde::Deserialize;

use crate::models::Todo;

#[derive(Template, Deserialize)]
#[template(path = "todo/list.html")]
pub struct ListTodoResponse {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo/error.html")]
pub struct ListTodoErrorResponse {
    pub error: String,
}

#[derive(Template)]
#[template(path = "todo/error.html")]
pub struct CreateTodoErrorResponse {
    pub error: String,
}

#[derive(Template)]
#[template(path = "todo/error.html")]
pub struct DeleteTodoErrorResponse {
    pub error: String,
}

#[derive(Template)]
#[template(path = "todo/error.html")]
pub struct ToggleTodoErrorResponse {
    pub error: String,
}
