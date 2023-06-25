use askama::Template;

pub struct BlogArticleInfo {
    pub id:       i32,
    pub title:    String,
    pub pub_date: String,
}

#[derive(Template)]
#[template(path = "homepage.html")]

pub struct BlogHomePageTemplate {
    pub articles: Vec<BlogArticleInfo>,
}
