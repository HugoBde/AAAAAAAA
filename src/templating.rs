use askama::Template;
use time::Date;

pub struct BlogArticleInfo<'a> {
    pub id:       i32,
    pub title:    &'a str,
    pub pub_date: Date,
}

#[derive(Template)]
#[template(path = "homepage.html")]

pub struct BlogHomePageTemplate<'a> {
    pub articles: Vec<BlogArticleInfo<'a>>,
}

#[derive(Template)]
#[template(path = "article.html", escape = "none")]

pub struct BlogArticleTemplate<'a> {
    pub title:       &'a str,
    pub update_date: Date,
    pub content:     &'a str,
}
