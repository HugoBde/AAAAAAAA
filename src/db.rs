use async_once::AsyncOnce;
use lazy_static::lazy_static;

use crate::config::CONFIG;

lazy_static! {
    pub static ref DB_CLIENT: AsyncOnce<tokio_postgres::Client> = AsyncOnce::new( async {

        // Create the database config used to connec to our PostgreSQL instance
        let mut db_config = tokio_postgres::Config::new();

        db_config
            .host(&CONFIG.get("database", "host").unwrap_or(String::from("localhost")).as_str())
            .port(CONFIG.getuint("database", "port").unwrap().unwrap_or(5432) as u16)
            .user(&CONFIG.get("database", "user").ok_or("Missing \"user\" parameter in \"database\" section").unwrap().as_str())
            .dbname(&CONFIG.get("database", "name").ok_or("Missing \"dbname\" parameter in \"database\" section").unwrap().as_str())
            .password(&CONFIG.get("database", "password").ok_or("Missing \"password\" parameter in \"database\" section").unwrap().as_str());

        // Connect !!!
        let (db_client, db_conn) = db_config.connect(tokio_postgres::NoTls).await.unwrap();

        // Let connection run on its own thread
        tokio::spawn(async move {
            if let Err(e) = db_conn.await {
                println!("connection error {}", e);
            } else {
                println!("Connection established to database");
            }
        });

        db_client
    });
}

pub async fn get_article_by_id(id: i32) -> tokio_postgres::Row {

    // Get a hold of the DB_CLIENT to make request
    DB_CLIENT.get().await.query_one("SELECT * FROM blog_articles WHERE id=$1;", &[&id]).await.unwrap()
}

pub async fn get_all_articles() -> Vec<tokio_postgres::Row> {

    DB_CLIENT.get().await.query("SELECT title, update_date, id FROM blog_articles;", &[]).await.unwrap()
}
