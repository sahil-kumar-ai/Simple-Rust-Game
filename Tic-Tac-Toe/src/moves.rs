use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    X,
    O,
    Empty,
}

struct Position {
    col: usize,
    row: usize,
    value: char,
}

impl Position {
    pub fn update_position(col: usize, row: usize, value: char) -> Self {
        Self { col, row, value }
    }
}

pub fn input_position(prev: &mut i32) -> (usize, usize, char) {
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

    let value: char = loop {
        let mut input = String::new();

        println!("Enter a value (X or O) -> ");

        io::stdin().read_line(&mut input).expect("Error");

        if input.trim().to_ascii_lowercase() == "x" {
            if *prev == -1 {
                println!("Please enter a value except X, X is used for last move");
                *prev = -1;
                continue;
            }
        }

        if input.trim().to_ascii_lowercase() == "o" {
            if *prev == 1 {
                println!("Please enter a value except O, O is used for last move");
                *prev = 1;
                continue;
            }
        }

        match input.trim().to_uppercase().as_str() {
            "X" => {
                *prev = -1;
                break 'X';
            }
            "O" => {
                *prev = -1;
                break 'O';
            }
            _ => println!("Invalid value! Enter X or O."),
        }

        println!("{input}")
    };

    Position::update_position(col, row, value);

    (row, col, value)
}
