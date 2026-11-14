use crate::moves::Move;

pub struct Board {
    arr: [[Move; 3]; 3],
    score: [i32; 8],
    moves: u8,
    is_win: bool,
    who_wins: char,
    winning_positions: [(usize, usize); 3],
    x_player_wins: i32,
    o_player_wins: i32,
    draws : i32
}

impl Board {
    pub fn initial_value() -> Self {
        Self {
            arr: [[Move::Empty; 3]; 3],
            score: [0; 8],
            moves: 0,
            is_win: false,
            who_wins: ' ',
            winning_positions: [(usize::MAX, usize::MAX); 3],
            x_player_wins: 0,
            o_player_wins: 0,
            draws : 0,
        }
    }

    pub fn reset(&mut self, how_reset: String) {
        self.arr = [[Move::Empty; 3]; 3];
        self.score = [0; 8];
        self.moves = 0;
        self.is_win = false;
        self.who_wins = ' ';
        self.winning_positions = [(usize::MAX, usize::MAX); 3];

        if how_reset == "new" {
            self.x_player_wins = 0;
            self.o_player_wins = 0;
            self.draws = 0;
        }
    }

    pub fn show_board(&self) {
        println!("|-----------|");

        for (row_index, row) in self.arr.iter().enumerate() {
            print!("|");

            for (col_index, cell) in row.iter().enumerate() {
                let is_winning_cell = self.winning_positions.contains(&(row_index, col_index));

                match cell {
                    Move::Empty => print!(" - |"),

                    Move::X if is_winning_cell => {
                        print!("\x1b[1;31m X \x1b[0m|");
                    }

                    Move::O if is_winning_cell => {
                        print!("\x1b[1;31m O \x1b[0m|");
                    }

                    Move::X => print!(" X |"),
                    Move::O => print!(" O |"),
                }
            }

            println!();
            println!("|-----------|");
        }
    }

    pub fn update_board(
        &mut self,
        row: usize,
        col: usize,
        user_input: char,
        curr: &mut bool,
    ) -> bool {
        if self.is_win {
            return true;
        }

        if self.arr[row][col] != Move::Empty {
            println!("There is already a value, please choose another position");
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

        self.moves += 1;

        if let Some(index) = self.score.iter().position(|&x| x == 3 || x == -3) {
            let winner = if self.score[index] == 3 {
                self.x_player_wins += 1;
                'X'
            } else {
                self.o_player_wins += 1;
                'O'
            };

            println!("\x1b[1;31mPlayer {} Wins!\x1b[0m", winner);

            println!("Player X Wins : {}", self.x_player_wins);
            println!("Player O Wins : {}", self.o_player_wins);
            println!("Draws : {}", self.draws);

            self.who_wins = winner;
            self.is_win = true;

            self.winning_positions = match index {
                0..=2 => [(index, 0), (index, 1), (index, 2)],

                3..=5 => {
                    let winning_col = index - 3;
                    [(0, winning_col), (1, winning_col), (2, winning_col)]
                }

                6 => [(0, 0), (1, 1), (2, 2)],

                7 => [(0, 2), (1, 1), (2, 0)],

                _ => unreachable!(),
            };
        } else if self.moves == 9 {
            self.draws += 1;
            println!("\x1b[33mMatch Draw!\x1b[0m");
            self.who_wins = 'Z';
            self.is_win = true;
        }

        *curr = !*curr;

        self.is_win
    }
}
