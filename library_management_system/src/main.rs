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
                let book = Books::create_book(
                    book_details.name,
                    book_details.author,
                    book_details.available,
                );
                let id = Books::add_book(&pool, &book).await;
                println!("Book added with ID: {:?}", id);
            }
            Choose::UpdateBook => {
                let book_details: Books = booksOperation::user_input();
                let book = Books::create_book(
                    book_details.name,
                    book_details.author,
                    book_details.available,
                );
                let id = Books::add_book(&pool, &book).await;
                println!("Book added with ID: {:?}", id);
            }
            Choose::SeeAllBooks => {
                let books = Books::get_all_books(&pool).await;
                match books {
                    Ok(books) => {
                        for book in books {
                            println!("{:?}", book);
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Choose::SeeBookByID => {
                let book_id: i32 = booksOperation::user_id_input();
                let book = Books::get_book_by_id(&pool, book_id).await.unwrap();

                println!("Book with the ID is : {:?}", book);
            }
            Choose::DeleteBook => {
                let book_id: i32 = booksOperation::user_id_input();
                let book = Books::delete_book_by_id(&pool, book_id).await.unwrap();

                println!("Book Deleted With ID : {:?}", book);
            }
            Choose::Exit => break,
        }
    }
}
