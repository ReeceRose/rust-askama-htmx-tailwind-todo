use askama::Template;
use serde::Deserialize;

use crate::models::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct GetIndexResponse;

#[derive(Template, Deserialize)]
#[template(path = "todo/list.html")]
pub struct ListTodoResponse {
    pub todos: Vec<Todo>,
}
#[derive(Template)]
#[template(source = "", ext = "")]
pub struct EmptyResponse {}
