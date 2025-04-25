
// TODO0 need some sort of mechanism for evaluating moves that have already happened, and if the same move is repeated 3 times the score is set to 0 (perhaps by clearing all figures?)
// also I just realised that the engine CAN waste lots of time by recalculating already happened moves (say a knight moving back and forth)

mod board;
use board::Board;

mod input;

const ADDITIONAL_THINK_DEPTH: i32 = 1; // 3

fn main() {
    let mut board = Board::standard();

    loop {
        board.draw();
        input::enter();
        board.play_turn(ADDITIONAL_THINK_DEPTH);
        println!();
    }
}
