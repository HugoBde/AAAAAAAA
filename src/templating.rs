use askama::Template;
use time::Date;

pub struct BlogArticleInfo<'a> {
    pub id: i32,
    pub title: &'a str,
    pub pub_date: Date,
}

#[derive(Template)]
#[template(path = "homepage.html")]

pub struct BlogHomePageTemplate<'a> {
    pub articles: Vec<BlogArticleInfo<'a>>,
}
