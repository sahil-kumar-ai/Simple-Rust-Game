use crate::moves::Move;

pub struct Board {
    arr: [[Move; 3]; 3],
    score: [i32; 8],
    is_win: bool,
}

impl Board {
    pub fn initial_value() -> Self {
        Self {
            arr: [[Move::Empty; 3]; 3],
            score: [0; 8],
            is_win: false,
        }
    }

    pub fn show_board(&self) {
        println!("|-----------|");

        for row in self.arr {
            print!("|");

            for col in row {
                match col {
                    Move::Empty => print!(" - |"),
                    Move::X => print!(" X |"),
                    Move::O => print!(" O |"),
                }
            }

            println!();
            println!("|-----------|");
        }
    }

    pub fn update_board(&mut self, row: usize, col: usize, user_input: char) -> bool {
        if self.is_win {
            return true;
        }

        if !matches!(self.arr[row][col], Move::Empty) {
            println!("Position already occupied!");
            return false;
        }

        let value: i32 = match user_input.to_ascii_lowercase() {
            'x' => {
                self.arr[row][col] = Move::X;
                1
            }
            'o' => {
                self.arr[row][col] = Move::O;
                -1
            }
            _ => return false,
        };

        self.score[row] += value;
        self.score[col + 3] += value;

        if row == col {
            self.score[6] += value;
        }

        if row + col == 2 {
            self.score[7] += value;
        }

        if self.score.contains(&3) {
            println!("X Wins!");
            self.is_win = true;
        } else if self.score.contains(&-3) {
            println!("O Wins!");
            self.is_win = true;
        }

        self.is_win
    }
}
