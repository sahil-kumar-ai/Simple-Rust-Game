use crate::booksOperation::create_book_table;

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub async fn connect_db() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://library.db?mode=rwc")
        .await
        .unwrap();

    create_book_table(&pool).await;

    println!("DB Connected");

    pool
}