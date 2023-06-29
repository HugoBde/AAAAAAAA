use crate::config::CONFIG;
use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_CLIENT: AsyncOnce<tokio_postgres::Client> = AsyncOnce::new( async {

        // Create the database config used to connec to our PostgreSQL instance
        let mut db_config = tokio_postgres::Config::new();

        db_config
            .host(&CONFIG.get("database", "host").unwrap())
            .port(CONFIG.getuint("database", "port").unwrap().unwrap() as u16)
            .dbname(&CONFIG.get("database", "name").unwrap())
            .user(&CONFIG.get("database", "user").unwrap())
            .password(&CONFIG.get("database", "password").unwrap());

        // Connect !!!
        let (db_client, db_conn) = db_config.connect(tokio_postgres::NoTls).await.unwrap();

        // Let connection run on its own thread
        tokio::spawn(async move {
            if let Err(e) = db_conn.await {
                println!("connection error {}", e);
            }
        });

        db_client
    });
}

pub async fn get_article_by_id(id: i32) -> tokio_postgres::Row {
    // Get a hold of the DB_CLIENT to make request
    DB_CLIENT
        .get()
        .await
        .query_one(
            "SELECT path FROM blog_articles_testing WHERE id=$1;",
            &[&id],
        )
        .await
        .unwrap()
}

pub async fn get_all_articles() -> Vec<tokio_postgres::Row> {
    DB_CLIENT
        .get()
        .await
        .query(
            "SELECT title, update_date, id FROM blog_articles_testing;",
            &[],
        )
        .await
        .unwrap()
}
