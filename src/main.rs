
// TODO2? I need some sort of mechanism that gives the bots tunnel vision so that they don't just get depressed and give up as to not loose a piece

mod board;
mod input;
mod player;
mod piece_type;
mod piece;
mod tile;

use board::Board;

const ADDITIONAL_THINK_DEPTH: i32 = 1; // 3

fn main() {
    let mut board = Board::standard();

    loop {
        board.draw();

        input::enter();

        let draw = board.play_turn(ADDITIONAL_THINK_DEPTH);

        println!();

        if draw {
            board.draw();
            println!("draw");
            break;
        }
    }
}
