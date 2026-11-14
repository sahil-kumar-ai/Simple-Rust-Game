mod board;
mod intractive;
mod moves;

use board::Board;
use intractive::get_option;
use moves::input_position;

fn main() {
    let mut board = Board::initial_value();
    let mut curr: bool = true;

    loop {
        board.show_board();

        let position = input_position(&mut curr);

        let won = board.update_board(position.row, position.col, position.value, &mut curr);

        if won {
            board.show_board();
            curr = true;
            println!();
            let choice: &str = get_option();

            if choice == "Continue Game" {
                board.reset("prev".to_string());
            } else if choice == "Load New Game" {
                board.reset("new".to_string());
            } else {
                break;
            }
        }
    }
}
