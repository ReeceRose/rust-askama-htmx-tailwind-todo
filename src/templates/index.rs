use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct GetIndexResponse;

#[derive(Template)]
#[template(source = "", ext = "")]
pub struct EmptyResponse {}
