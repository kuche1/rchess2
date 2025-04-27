
// TODO1? maybe add something that makes the bot consider a play even if it doesn't change the score
// TODO0 allow the end user to play moves
// TODO1? maybe give checks 0.5 points, or at least make them extend the thinking

// run with:
// cargo run --release

mod board;
mod input;
mod player;
mod piece_type;
mod piece;
mod tile;

use board::Board;
use input::GameChoice;
use player::Player;

// the machine thinks about every of it's possible moves
// then picks the onces that result in the most points
// then picks a random one out of them

const ADDITIONAL_THINK_BREADTH: i32 = 5;
// the machine will consider all available positions for it's next turn
// if this is 1, then it will consider all of the opponent's responses to each of the available turns
// if this is 2, then it will consoder all avaiable ...
//
// therefore a value that is `% 2 == 0` should make the AI more aggressive

const ADDITIONAL_THINK_DEPTH: i32 = 4;
// after considering all moves (`ADDITIONAL_THINK_BREADTH`), the machine will be permitted continue to think about the moves that have generated a positive change in score
// if this is 1, it will think "how can my opponent respond in a way that increases the score for him"
// if this is 2, it will think "how can I respond to my opponent's response, in a way that increases the score for me"
//
// therefor a value that is `% 2 == 0` should keep `ADDITIONAL_THINK_BREADTH`'s agressiveness value, otherwise it should inverse it

fn main() {
    let mut board = Board::standard();

    loop {
        println!();
        board.draw();

        // input::enter();

        match input::game_choice() {

            GameChoice::NextTurnAi => {

                let winner = board.play_turn(ADDITIONAL_THINK_BREADTH, ADDITIONAL_THINK_DEPTH);
        
                if let Some(winner) = winner {
                    board.draw();
                    // println!("winner: {}", winner);
                    match winner {
                        None => println!("draw"),
                        Some(Player::A) => println!("winner: player a"),
                        Some(Player::B) => println!("winner: player b"),
                    }
                    break;
                }

            },

            GameChoice::MovePiece => {
                println!("todo, implement piece move");
            },
        }

    }
}
