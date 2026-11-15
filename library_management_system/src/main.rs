mod booksOperation;
mod connectDB;
mod intractive;

use booksOperation::Books;
use connectDB::connect_db;
use intractive::{Choose, get_option};
use sqlx::{Pool, Sqlite};

#[tokio::main]
async fn main() {
    println!("This is an library management system");
    let pool: Pool<Sqlite> = connect_db().await;

    loop {
        let choice = get_option();

        match choice {
            Choose::AddBook => {
                let book_details: Books = booksOperation::user_input();
                let book = Books::create_book(book_details.name, book_details.author, book_details.available);
                let id = Books::add_book(&pool, &book).await;
                println!("Book added with ID: {:?}", id);
            }
            Choose::UpdateBook => {
                let book_details: Books = booksOperation::user_input();
                let book = Books::create_book(book_details.name, book_details.author, book_details.available);
                let id = Books::add_book(&pool, &book).await;
                println!("Book added with ID: {:?}", id);
            }
            Choose::SeeAllBooks => {
                let books = Books::get_all_books(&pool).await;
                for book in books {
                    println!("{:#?}", book);
                }
            }
            Choose::SeeBookByID => {
                let book_details: Books = booksOperation::user_input();
                let book = Books::create_book(book_details.name, book_details.author, book_details.available);
                let id = Books::add_book(&pool, &book).await;
                println!("Book added with ID: {:?}", id);
            }
            Choose::DeleteBook => {
                let book_details: Books = booksOperation::user_input();
                let book = Books::create_book(book_details.name, book_details.author, book_details.available);
                Books::add_book(&pool, &book);
            }
            Choose::Exit => break,
        }
    }
}
