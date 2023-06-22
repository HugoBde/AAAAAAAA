#![allow(non_snake_case)]

async fn home_page() -> String {

    String::from("AAAAAAAA")
}

async fn article_page(axum::extract::Path(blog_id) : axum::extract::Path<u32>) -> String {

    String::from(format!("{}", blog_id))
}

#[tokio::main]

async fn main() {

    // build our application with a single route
    let app = axum::Router::new()
        .route("/", axum::routing::get(home_page))
        .route("/:blog_id", axum::routing::get(article_page));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
