
mod board;
use board::Board;

mod input;

fn main() {
    let mut board = Board::standard();

    loop {
        board.draw();
        input::enter();
        board.play_turn();
        println!();
    }
}
