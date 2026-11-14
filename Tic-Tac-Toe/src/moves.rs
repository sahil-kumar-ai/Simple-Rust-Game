use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    X,
    O,
    Empty,
}

pub struct Position {
    pub col: usize,
    pub row: usize,
    pub value: char,
}

impl Position {
    pub fn update_position(col: usize, row: usize, value: char) -> Self {
        Self { col, row, value }
    }
}

pub fn input_position(curr: &mut bool) -> Position {
    let row: usize = loop {
        let mut input = String::new();

        println!("Enter row from 1 to 3 -> ");
        io::stdin().read_line(&mut input).expect("Error");

        match input.trim().parse::<usize>() {
            Ok(num) if (1..=3).contains(&num) => break num - 1,
            _ => println!("Invalid row! Enter a number between 1 and 3."),
        }
    };

    let col: usize = loop {
        let mut input = String::new();

        println!("Enter col from 1 to 3 -> ");
        io::stdin().read_line(&mut input).expect("Error");

        match input.trim().parse::<usize>() {
            Ok(num) if (1..=3).contains(&num) => break num - 1,
            _ => println!("Invalid column! Enter a number between 1 and 3."),
        }
    };

    let value: char = if *curr { 'X' } else { 'O' };

    Position::update_position(col, row, value)
}
