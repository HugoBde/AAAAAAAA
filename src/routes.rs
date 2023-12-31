use askama::Template;
use axum::extract::Path;
use axum::http::header;
use axum::response::IntoResponse;

use crate::config::CONFIG;
use crate::db;
use crate::templating::{BlogArticleInfo, BlogArticleTemplate, BlogHomePageTemplate};

pub async fn home_page() -> impl IntoResponse {

    let articles = db::get_all_articles().await;

    let ctx = BlogHomePageTemplate {
        articles: articles
            .iter()
            .map(|article| BlogArticleInfo {
                id:       article.get("id"),
                title:    article.get("title"),
                pub_date: article.get("update_date"),
            })
            .collect(),
    };

    ([(header::CONTENT_TYPE, "text/html")], ctx.render().unwrap())
}

pub async fn article_page(Path(blog_id): Path<i32>) -> impl IntoResponse {

    // Query the db
    let row = crate::db::get_article_by_id(blog_id).await;

    // Extract the path and prepend /blog to it
    let mut path = CONFIG.get("general", "articles_dir").unwrap();

    path.push('/');

    path.push_str(row.try_get("path").unwrap());

    let article_content = std::fs::read_to_string(path).unwrap();

    let ctx = BlogArticleTemplate {
        title:       row.get("title"),
        update_date: row.get("update_date"),
        content:     article_content.as_str(),
    };

    ([(axum::http::header::CONTENT_TYPE, "text/html")], ctx.render().unwrap())
}
