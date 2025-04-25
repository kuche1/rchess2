
mod board;
use board::Board;

fn main() {
    let board = Board::standard();
    board.draw();
    board.play_turn();
    board.draw();
}
