use askama::Template;
use axum::extract::Path;
use axum::http::header;
use axum::response::IntoResponse;

use crate::db;
use crate::templating::{BlogArticleInfo, BlogHomePageTemplate};

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

    // let ctx = BlogHomePageTemplate {
    //     articles: vec![
    //         BlogArticleInfo {
    //             id:       1,
    //             title:    String::from("Powering a contact form with Go and Docker"),
    //             pub_date: String::from("June 22nd 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       2,
    //             title:    String::from("How to over-engineer your portfolio website"),
    //             pub_date: String::from("July 5th 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       3,
    //             title:    String::from("Boosting Website Interactivity: Implementing Real-Time Chat with Go and Docker"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       4,
    //             title:    String::from("Mastering the Art of Complexity: Over-Engineering Your E-Commerce Website"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       5,
    //             title:    String::from("Building Scalable Microservices Architecture with Go and Docker"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       6,
    //             title:    String::from("The Magic of Go: Unleashing Concurrent Processing in Your Web Applications"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       7,
    //             title:    String::from("Elevating User Experience: Crafting Responsive Web Designs with Over-Engineering Techniques"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       8,
    //             title:    String::from("Unleashing the Power of Go and Docker: Developing a High-Performance API Backend"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //         BlogArticleInfo {
    //             id:       9,
    //             title:    String::from("Exploring Cutting-Edge Web Technologies: Over-Engineered Frontend Frameworks for Your Portfolio Website"),
    //             pub_date: String::from("August 8th, 2023"),
    //         },
    //     ],
    // };

    ([(header::CONTENT_TYPE, "text/html")], ctx.render().unwrap())
}

pub async fn article_page(Path(blog_id): Path<i32>) -> impl IntoResponse {

    // Query the db
    let row = crate::db::get_article_by_id(blog_id).await;

    // Extract the path
    let path: &str = row.try_get("path").unwrap();

    let response_body = std::fs::read(path).unwrap();

    ([(axum::http::header::CONTENT_TYPE, "text/html")], response_body)
}
