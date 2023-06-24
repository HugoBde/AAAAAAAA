use axum::extract::Path;
use axum::response::IntoResponse;

pub async fn home_page() -> String {

    String::from("AAAAAAAA")
}

pub async fn article_page(Path(blog_id): Path<i32>) -> impl IntoResponse {

    // Query the db
    let row = crate::db::get_article_by_id(blog_id).await;

    // Extract the path
    let path: &str = row.try_get("path").unwrap();

    let response_body = std::fs::read(path).unwrap();

    ([(axum::http::header::CONTENT_TYPE, "text/html")], response_body)
}
