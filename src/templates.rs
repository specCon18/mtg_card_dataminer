use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate<'a>{
    pub name: &'a str,
}

#[derive(Template)]
#[template(path="base.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path="head.html")]
pub struct  HeadTemplate;