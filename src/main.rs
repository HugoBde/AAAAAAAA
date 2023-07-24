#![allow(non_snake_case)]

use axum::{routing, Router, Server};
use lazy_static;
use AAAAAAAA::config::CONFIG;
use AAAAAAAA::db::DB_CLIENT;
use AAAAAAAA::routes::{article_page, home_page};

#[tokio::main]

async fn main() {

    lazy_static::initialize(&CONFIG);

    lazy_static::initialize(&DB_CLIENT);

    let app = Router::new().route("/blog", routing::get(home_page)).route("/blog/:blog_id", routing::get(article_page));

    Server::bind(&"0.0.0.0:5000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
