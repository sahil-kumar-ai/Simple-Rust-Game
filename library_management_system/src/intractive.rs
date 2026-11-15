use dialoguer::Select;

#[derive(Debug, Clone, Copy)]
pub enum Choose {
    AddBook,
    SeeAllBooks,
    SeeBookByID,
    UpdateBook,
    DeleteBook,
    Exit,
}

impl std::fmt::Display for Choose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Choose::AddBook => "Add Book",
            Choose::SeeAllBooks => "See All Books",
            Choose::SeeBookByID => "See Book By It's ID",
            Choose::UpdateBook => "Update Book",
            Choose::DeleteBook => "Delete Book",
            Choose::Exit => "Exit",
        };

        write!(f, "{}", text)
    }
}

pub fn get_option() -> Choose {
    let options = vec![
        Choose::AddBook,
        Choose::SeeAllBooks,
        Choose::SeeBookByID,
        Choose::UpdateBook,
        Choose::DeleteBook,
        Choose::Exit,
    ];

    let selection = Select::new()
        .with_prompt("What do you want to do?")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    options[selection]
}