
// run with:
// cargo run --release

mod board;
mod input;
mod player;
mod piece_type;
mod piece;
mod tile;

use board::Board;

const ADDITIONAL_THINK_BREADTH: i32 = 2;
// if the value is `x % 2 == 1`, the machine will think "how can my opponent respond to my move"
// if it is `x % 2 == 0`, it will be more aggressive, as it will not consider how the opponent might respond to it's last move
// putting at least 1 is recommended so that the bot can expect "draw scumming" from the opponent

const ADDITIONAL_THINK_DEPTH: i32 = 0;
// if a given virtual board has more score than the current, extend the breath by this much (but only once)
// setting this to `x % 2 == 0` should keep ADDITIONAL_THINK_BREADTH's aggressiveness property
// setting this to `x % 2 == 1` should invert it
// TODO1 we need to make this so that after it has been triggered it ignores moves that do nothing

fn main() {
    let mut board = Board::standard();

    loop {
        board.draw();

        input::enter();

        let draw = board.play_turn(ADDITIONAL_THINK_BREADTH, ADDITIONAL_THINK_DEPTH);

        println!();

        if draw {
            board.draw();
            println!("draw");
            break;
        }
    }
}
