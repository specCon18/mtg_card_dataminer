use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate;