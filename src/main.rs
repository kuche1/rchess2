
mod board;
use board::Board;

fn main() {
    let mut board = Board::standard();
    board.draw();
    board.play_turn();
    board.draw();
    board.play_turn();
    board.draw();
}
