use askama::Template;

use crate::cards::CardForTemplate;


#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate {
    pub name: String,
    pub cards: Vec<CardForTemplate>,
}



#[derive(Template)]
#[template(path="base.html")]
pub struct BaseTemplate;

#[derive(Template)]
#[template(path="head.html")]
pub struct  HeadTemplate;