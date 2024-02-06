use askama::Template;
use serde::Deserialize;

use crate::models::Todo;

#[derive(Template, Deserialize)]
#[template(path = "todo/list.html")]
pub struct ListTodoResponse {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo/list-error.html")]
pub struct ListTodoErrorResponse {
    pub error: String,
}
