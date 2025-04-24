
mod board;
use board::Board;

fn main() {
    println!("Hello, world!");

    let board = Board::standard();
    board.draw();
    board.play_turn();
    board.draw();
}
