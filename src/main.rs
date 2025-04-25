
// TODO2? I need some sort of mechanism that gives the bots tunnel vision so that they don't just get depressed and give up as to not loose a piece

mod board;
mod input;
mod player;
mod piece_type;
mod piece;
mod tile;

use board::Board;

const ADDITIONAL_THINK_DEPTH: i32 = 3; // TODO0 rename this to `BREADTH_FIRST_THINK_DEPTH` or something like that and give it value `x%2==1` so that it thinks about "how can my opponent respond to my move"

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
