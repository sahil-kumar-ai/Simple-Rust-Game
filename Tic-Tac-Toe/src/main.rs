mod board;
mod moves;

use board::Board;
use moves::input_position;

fn main() {
    let mut prev = 0;
    let mut board = Board::initial_value();
    board.show_board();

    loop {
        let (col, row, value) = input_position(&mut prev);
        let does_user_win : bool = board.update_board(col, row, value);
        board.show_board();

        if does_user_win {
            break;
        }
    }
}
