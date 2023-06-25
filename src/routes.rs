use axum::extract::Path;
use axum::http::header;
use axum::response::IntoResponse;
use sailfish::TemplateOnce;

use crate::templating::{BlogArticleTemplate, BlogHomePageTemplate};

pub async fn home_page() -> impl IntoResponse {

    let ctx = BlogHomePageTemplate {
        articles: vec![
            BlogArticleTemplate {
                title:    String::from("Article 1"),
                pub_date: String::from("Jun 25th 2023"),
            },
            BlogArticleTemplate {
                title:    String::from("Article 2"),
                pub_date: String::from("Jun 26th 2023"),
            },
        ],
    };

    ([(header::CONTENT_TYPE, "text/html")], ctx.render_once().unwrap())
}

pub async fn article_page(Path(blog_id): Path<i32>) -> impl IntoResponse {

    // Query the db
    let row = crate::db::get_article_by_id(blog_id).await;

    // Extract the path
    let path: &str = row.try_get("path").unwrap();

    let response_body = std::fs::read(path).unwrap();

    ([(axum::http::header::CONTENT_TYPE, "text/html")], response_body)
}
