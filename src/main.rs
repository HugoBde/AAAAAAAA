#![allow(non_snake_case)]

use AAAAAAAA::routes::{article_page, home_page};

#[tokio::main]

async fn main() {

    // build our application with a single route
    let app = axum::Router::new()
        .route("/blog", axum::routing::get(home_page))
        .route("/blog/:blog_id", axum::routing::get(article_page));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
