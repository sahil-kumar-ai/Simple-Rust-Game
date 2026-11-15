use std::io;

use sqlx::{Pool, Sqlite, pool, query, query_as};

#[derive(Debug, sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub available: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Books {
    pub name: String,
    pub author: String,
    pub available: i32,
}

pub async fn create_book_table(pool: &Pool<Sqlite>) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            author TEXT NOT NULL,
            available BOOLEAN NOT NULL
        )",
    )
    .execute(pool)
    .await
    .unwrap();
}

impl Books {
    pub fn create_book(name: String, author: String, available: i32) -> Self {
        Self {
            name: name,
            author: author,
            available: available,
        }
    }

    pub async fn add_book(pool: &Pool<Sqlite>, book: &Books) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO books (name, author, available)
         VALUES (?, ?, ?)",
        )
        .bind(&book.name)
        .bind(&book.author)
        .bind(book.available)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_all_books(pool: &Pool<Sqlite>) -> Result<Vec<Book>, sqlx::Error> {
        let rows = sqlx::query_as::<_, Book>("SELECT id, name, author, available FROM books")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }
}

pub fn user_input() -> Books {
    let mut name: String = String::new();

    println!("Enter The Name of The Book :");

    io::stdin().read_line(&mut name).expect("Error");
    let name: String = name.trim().parse().unwrap();

    let mut author: String = String::new();

    println!("Enter The Author of The Book :");

    io::stdin().read_line(&mut author).expect("Error");
    let author: String = author.trim().parse().unwrap();

    let mut available: String = String::new();

    println!("Enter The Amount of Book Present In Library :");

    io::stdin().read_line(&mut available).expect("Error");

    let available: i32 = available.trim().parse().expect("Error");

    Books {
        name,
        author,
        available,
    }
}
