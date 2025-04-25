
mod board;
use board::Board;

mod input;

const ADDITIONAL_THINK_DEPTH: i32 = 3;

fn main() {
    let mut board = Board::standard();

    loop {
        board.draw();
        input::enter();
        board.play_turn(ADDITIONAL_THINK_DEPTH);
        println!();
    }
}
