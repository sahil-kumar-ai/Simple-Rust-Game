mod board;
mod moves;

use board::Board;
use moves::input_position;

fn main() {
    let mut board = Board::initial_value();
    let mut prev = 0;

    loop {
        board.show_board();

        let position = input_position(&mut prev);

        let won = board.update_board(position.row, position.col, position.value);

        if won {
            board.show_board();
            break;
        }
    }
}
