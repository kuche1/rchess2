
use rand::seq::IndexedRandom; // cargo add rand
use rayon::prelude::*; // cargo add rayon

use super::player::Player;
use super::piece_type::PieceType;
use super::piece::Piece;
use super::tile::Tile;

const BOARD_SIZE_USIZE: usize = 8;
// const BOARD_SIZE_ISIZE: isize = BOARD_SIZE_USIZE as isize;

const SCORE_IF_DRAW: i32 = -1;
// it's logical that the score should be 0 if a draw occurs
// however this makes the bots too cowardly and willing to throw
// a game over a lost pawn

const SCORE_IF_WIN: i32 = 1_000_000;
const SCORE_IF_LOOSE: i32 = -1_000_000;

const PICK_RANDOM_MOVE_IF_MANY_AVAILABLE: bool = true;
// `true` makes for better games
// `false` makes for easier debugging

type BoardPosition = [[Tile; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE];
type CompleteMove = (usize, usize, usize, usize);

#[derive(Clone)]
pub struct Board {
    board: BoardPosition,
    players_turn: Player,

    non_virtual_board: bool,
    already_played_positions: Vec<BoardPosition>, // assuming that `players_turn` doesnt relly matter
}

impl Board {
    pub fn standard() -> Self {
        Board {
            board: [
                [
                    Tile { piece: Some(Piece { typee: PieceType::Rook,   owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Knight, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Bishop, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Queen,  owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::King,   owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Bishop, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Knight, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Rook,   owner: Player::B }) },
                ],
                [
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::B }) },
                ],
                [
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                ],
                [
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                ],
                [
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                ],
                [
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                    Tile { piece: None },
                ],
                [
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Pawn, owner: Player::A }) },
                ],
                [
                    Tile { piece: Some(Piece { typee: PieceType::Rook,   owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Knight, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Bishop, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Queen,  owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::King,   owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Bishop, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Knight, owner: Player::A }) },
                    Tile { piece: Some(Piece { typee: PieceType::Rook,   owner: Player::A }) },
                ],
            ],
            players_turn: Player::A,
            non_virtual_board: true,
            already_played_positions: vec![],
        }
    }

    pub fn draw(&self) {
        for lines in &self.board {
            for tile in lines {
                print!("|");
                tile.draw();
            }
            println!("|");
        }
    }

    fn evaluate_score(&self, forr: Player) -> i32 {
        let mut score:i32 = 0;

        for line in self.board.iter() {
            for tile in line {

                let piece = match &tile.piece {
                    None => continue,
                    Some(v) => v,
                };

                if piece.owner == forr {
                    score += piece.typee.get_point_value();
                }else{
                    score -= piece.typee.get_point_value();
                }
            }
        }

        score
    }

    // returns the winner of the game, or `None` if it's a draw
    // None - nothing interesting
    // Some(None) - draw
    // Some(Player) - winner
    fn commit_turn(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Option<Option<Player>> { // kinda stupid name
        let mut taken_piece_was_king = false;

        if let Some(piece) = &self.board[to_y][to_x].piece {
            if self.non_virtual_board {
                print!("captured piece: ");
                piece.draw();
                println!();
            }

            if piece.typee == PieceType::King {
                taken_piece_was_king = true;
            }
        }

        self.board[to_y][to_x] = self.board[from_y][from_x].clone();
        self.board[from_y][from_x].piece = None;

        if taken_piece_was_king {
            return Some(Some(self.board[to_y][to_x].piece.as_ref().unwrap().owner));
        }

        if self.already_played_positions.contains(&self.board) {
            //// vvv code to delete all pieces, but that's not really needed
            // for line in &mut self.board {
            //     for tile in line {
            //         tile.piece = None;
            //     }
            // }
            return Some(None);
        }

        let piece = self.board[to_y][to_x].piece.as_mut().unwrap();

        if piece.typee == PieceType::Pawn {
            self.already_played_positions.clear(); // if a pawn has been played we know for sure that all previously recorded board cannot be repeated

            if (to_y == 0) || (to_y == BOARD_SIZE_USIZE-1) { // no need to actually check the owner
                piece.typee = PieceType::Queen;
            }
        }

        self.already_played_positions.push(self.board.clone());

        self.switch_to_next_players_turn();

        return None;
    }

    fn switch_to_next_players_turn(&mut self) {
        self.players_turn = match self.players_turn {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    fn calc_move_score(&self, piece: &Option<Piece>, x_idx: usize, y_idx: usize, additional_think_breadth: i32, additional_think_depth: i32) -> Option<(i32, Vec<CompleteMove>)> {

        let piece: &Piece = match piece {
            None => return None,
            Some(v) => v,
        };

        if piece.owner != self.players_turn {
            return None;
        }

        let mut best_move_score: Option<i32> = None;
        let mut best_moves: Vec<CompleteMove> = vec![];

        let available_moves =
            self.get_available_moves_for(
                piece,
                x_idx, y_idx,
            );

        // print!("available moves for ");
        // piece.draw();
        // print!(":");

        for movee in available_moves {
            let (x, y) = movee;
            // print!(" x={},y={}", x, y);

            // println!("{}evaluating {},{}->{},{}", "    ".repeat(additional_think_depth as usize), x_idx, y_idx, x, y);
            
            let mut virtual_board = self.clone();
            virtual_board.non_virtual_board = false;
        
            let draw = virtual_board.commit_turn(x_idx, y_idx, x, y);

            let score = 'score: {

                if let Some(winner) = draw {
                    break 'score
                        match winner {
                            None => SCORE_IF_DRAW,
                            Some(player) => {
                                if player == piece.owner {
                                    SCORE_IF_WIN
                                }else{
                                    SCORE_IF_LOOSE
                                }
                            },
                        }
                }

                if additional_think_breadth > 0 {
                    if let Some(winner) = virtual_board.play_turn(additional_think_breadth - 1, 0) {
                        break 'score
                            match winner {
                                None => SCORE_IF_DRAW,
                                Some(player) => {
                                    if player == piece.owner {
                                        SCORE_IF_WIN
                                    }else{
                                        SCORE_IF_LOOSE
                                    }
                                },
                            }
                    }
                }

                let score = virtual_board.evaluate_score(piece.owner); // TODO2 maybe keep a variable `score`, then only updates it, but then again this will break multiplayer chess

                if additional_think_depth > 0 {
                    if score > self.evaluate_score(piece.owner) { // TODO2 yeah, I'm not sure I like having to recalc this every time, I think the comment above is right
                        if let Some(winner) = virtual_board.play_turn(0, additional_think_depth - 1) {
                            break 'score
                            match winner {
                                None => SCORE_IF_DRAW,
                                Some(player) => {
                                    if player == piece.owner {
                                        SCORE_IF_WIN
                                    }else{
                                        SCORE_IF_LOOSE
                                    }
                                },
                            }
                        }
                    }
                }

                score
            };

            // println!("{}move {},{}->{},{} evaluates to {}", "    ".repeat(additional_think_depth as usize), x_idx, y_idx, x, y, score);

            // match best_move_score {
            //     None => {
            //         best_move_score = Some(score);
            //         best_moves.push((x_idx, y_idx, x, y));
            //         // println!("{}this is my new fav move: first available", "    ".repeat(additional_think_depth as usize));
            //     },
            //     Some(val) => {
            //         if val < score {
            //             // println!("{}this is my new fav move: old({}) < new({})", "    ".repeat(additional_think_depth as usize), val, score);
            //             best_move_score = Some(score);
            //             // best_move = (x_idx, y_idx, x, y);
            //             best_moves.clear();
            //             best_moves.push((x_idx, y_idx, x, y));
            //         }else if val == score {
            //             best_moves.push((x_idx, y_idx, x, y));
            //         }
            //     },
            // }

            if best_move_score < Some(score) {
                best_move_score = Some(score);

                best_moves.clear();
                best_moves.push((x_idx, y_idx, x, y));

            } else if best_move_score == Some(score) {
                best_moves.push((x_idx, y_idx, x, y));
            }

        }

        match best_move_score {
            None => None,
            Some(score) => Some((score, best_moves))

        }
    }

    pub fn play_turn(&mut self, additional_think_breadth: i32, additional_think_depth: i32) -> Option<Option<Player>> {

        let mut overall_best_score: Option<i32> = None;
        let mut overall_best_moves: Vec<CompleteMove> = vec![];

        for (y_idx, lines) in self.board.iter().enumerate() {
        // self.board.iter().enumerate().for_each(|(y_idx, lines)| {

            let tiles_best_score_and_moves: Vec<Option<(i32, Vec<CompleteMove>)>> = {

                if self.non_virtual_board {

                    lines
                    .par_iter()
                    .enumerate()
                    .map( |(x_idx, tile)| self.calc_move_score(&tile.piece, x_idx, y_idx, additional_think_breadth, additional_think_depth))
                    .collect()
                
                } else { // the difference is that this one is not multithreaded

                    lines
                    .iter() // TODO9 I really hate the fact that we have the exact same code in 2 places, with this being the only difference, but I don't think there is anything I can do // note: I did try to assign the appropriate iter function to a variable then call it, but rust didn't let me
                    .enumerate()
                    .map( |(x_idx, tile)| self.calc_move_score(&tile.piece, x_idx, y_idx, additional_think_breadth, additional_think_depth))
                    .collect()

                }
            };

            for best_score_and_moves in tiles_best_score_and_moves {
                let best_score_and_moves = match best_score_and_moves {
                    None => continue,
                    Some(v) => v,
                };

                let (score, moves) = best_score_and_moves;

                if overall_best_score < Some(score) { // `None < Some(number)` is always true
                    overall_best_score = Some(score);
                    overall_best_moves = moves;
                } else if overall_best_score == Some(score) {
                    overall_best_moves.extend(moves);
                }
            }
        }

        let overall_best_score = match overall_best_score {
            None => return Some(None),
            Some(v) => v,
        };

        {
            let (fx, fy, tx, ty) = {
                if PICK_RANDOM_MOVE_IF_MANY_AVAILABLE {
                    let (fx, fy, tx, ty) = overall_best_moves.choose(&mut rand::rng()).unwrap();
                    (*fx, *fy, *tx, *ty)
                } else {
                    overall_best_moves[0]
                }
            };

            if self.non_virtual_board {
                self.players_turn.draw_color_on();
                print!("player ");
                self.players_turn.draw_color_off();
                print!("plays ");
                self.board[fy][fx].draw();
                println!(" => {},{}->{},{}", fx, fy, tx, ty);

                for (fx, fy, tx, ty) in overall_best_moves {
                    println!("best available moves score: {} => {},{}->{},{}", overall_best_score, fx, fy, tx, ty);
                }
            }

            if let Some(winner) = self.commit_turn(fx, fy, tx, ty) {
                return Some(winner);
            }
        }

        None
    }

    // first ret bool - true if the position is valid
    // second ret bool - true if an enemy piece is to be taken
    fn is_pos_valid(&self, x: usize, y: usize, forr: Player) -> (bool, bool) {
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return (false, false);
        }

        let tile = &self.board[y][x];

        let piece = match &tile.piece {
            None => return (true, false),
            Some(v) => v,
        };

        if piece.owner == forr {
            return (false, false);
        }

        return (true, true);
    }

    fn is_pos_valid_for_regular_pawn_move(&self, x: usize, y: usize) -> bool { // TODO2 perhaps we could now use the updated `is_pos_valid` instead of this
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return false;
        }
        return self.board[y][x].piece.is_none();
    }

    fn is_pos_valid_for_attack_pawn_move(&self, x: usize, y: usize, owner: &Player) -> bool { // TODO2 perhaps we could now use the updated `is_pos_valid` instead of this
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return false;
        }

        let tile = &self.board[y][x];

        let piece = match &tile.piece {
            None => return false,
            Some(v) => v,
        };

        return piece.owner != *owner;
    }

    pub fn get_available_moves_for(&self, piece: &Piece, x_idx: usize, y_idx: usize) -> Vec<(usize, usize)> {

        let mut available_moves: Vec<(usize, usize)> = vec![];

        match piece.typee {

            PieceType::Pawn => {

                let forward_y: isize = match piece.owner {
                    Player::A => -1,
                    Player::B => 1,
                };

                // check if we can capture any enemy piece

                'pawn_attack: {
                    // we're kinda duplicating code, but I hope that the compiler is going to take care of that
                    let atk_y = match y_idx.checked_add_signed(forward_y) {
                        Some(v) => v,
                        None => break 'pawn_attack,
                    };

                    if self.is_pos_valid_for_attack_pawn_move(x_idx+1, atk_y, &piece.owner) {
                        available_moves.push((x_idx+1, atk_y));
                    }

                    let atk_x: usize = match x_idx.checked_add_signed(-1) {
                        Some(v) => v,
                        None => break 'pawn_attack,
                    };

                    if self.is_pos_valid_for_attack_pawn_move(atk_x, atk_y, &piece.owner) {
                        available_moves.push((atk_x, atk_y));
                    }
                }

                // just move

                'pawn_move: {

                    // move forward once

                    let new_y = match y_idx.checked_add_signed(forward_y) {
                        Some(v) => v,
                        None => break 'pawn_move,
                    };

                    if !self.is_pos_valid_for_regular_pawn_move(x_idx, new_y) {
                        break 'pawn_move;
                    }

                    available_moves.push((x_idx, new_y));

                    // move forward twice (will only trigger if you can also move forward once)

                    { // check if pawn is at start pos

                        let pos_y = match piece.owner {
                            Player::A => BOARD_SIZE_USIZE - 1 - 1,
                            Player::B => 0                    + 1,
                        };

                        if y_idx != pos_y {
                            break 'pawn_move;
                        }
                    }

                    let new_y = match new_y.checked_add_signed(forward_y) {
                        Some(v) => v,
                        None => break 'pawn_move,
                    };

                    if !self.is_pos_valid_for_regular_pawn_move(x_idx, new_y) {
                        break 'pawn_move;
                    }

                    available_moves.push((x_idx, new_y));

                }

                // TODO9 en passant
            },

            PieceType::Knight => {

                for (ofs_x, ofs_y) in
                    [
                        (0-1, 0-2), // top left
                        (0+1, 0-2), // top right
                        (0-1, 0+2), // bot left
                        (0+1, 0+2), // bot right
                        (0-2, 0-1), // left top
                        (0-2, 0+1), // left bot
                        (0+2, 0-1), // right top
                        (0+2, 0+1), // right bot
                ] {
                    let dest_x = match x_idx.checked_add_signed(ofs_x) {
                        Some(v) => v,
                        None => continue,
                    };

                    let dest_y = match y_idx.checked_add_signed(ofs_y) {
                        Some(v) => v,
                        None => continue,
                    };

                    let (pos_is_valid, _take_piece) = self.is_pos_valid(dest_x, dest_y, piece.owner);
                    if !pos_is_valid {
                        continue;
                    }

                    available_moves.push((dest_x, dest_y));
                }

            },

            PieceType::Bishop => {
                for (ofs_x, ofs_y) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                    let mut pos_x = x_idx;
                    let mut pos_y = y_idx;

                    loop {
                        pos_x = match pos_x.checked_add_signed(ofs_x) {
                            Some(v) => v,
                            None => break,
                        };
                        pos_y = match pos_y.checked_add_signed(ofs_y) {
                            Some(v) => v,
                            None => break,
                        };

                        let (pos_is_valid, piece_to_be_taken) = self.is_pos_valid(pos_x, pos_y, piece.owner);
                        if !pos_is_valid {
                            break;
                        }

                        available_moves.push((pos_x, pos_y));

                        if piece_to_be_taken {
                            break;
                        }
                    }
                }
            },

            PieceType::Rook => {
                for (ofs_x, ofs_y) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                    let mut pos_x = x_idx;
                    let mut pos_y = y_idx;

                    loop {
                        pos_x = match pos_x.checked_add_signed(ofs_x) {
                            Some(v) => v,
                            None => break,
                        };
                        pos_y = match pos_y.checked_add_signed(ofs_y) {
                            Some(v) => v,
                            None => break,
                        };

                        let (pos_is_valid, piece_to_be_taken) = self.is_pos_valid(pos_x, pos_y, piece.owner);
                        if !pos_is_valid {
                            break;
                        }

                        available_moves.push((pos_x, pos_y));

                        if piece_to_be_taken {
                            break;
                        }
                    }
                }
            },

            PieceType::Queen => {
                for (ofs_x, ofs_y) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                    let mut pos_x = x_idx;
                    let mut pos_y = y_idx;

                    loop {
                        pos_x = match pos_x.checked_add_signed(ofs_x) {
                            Some(v) => v,
                            None => break,
                        };
                        pos_y = match pos_y.checked_add_signed(ofs_y) {
                            Some(v) => v,
                            None => break,
                        };

                        let (pos_is_valid, piece_to_be_taken) = self.is_pos_valid(pos_x, pos_y, piece.owner);
                        if !pos_is_valid {
                            break;
                        }

                        available_moves.push((pos_x, pos_y));

                        if piece_to_be_taken {
                            break;
                        }
                    }
                }
            },

            PieceType::King => {
                for (ofs_x, ofs_y) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                    let pos_x = match x_idx.checked_add_signed(ofs_x) {
                        Some(v) => v,
                        None => continue,
                    };
                    let pos_y = match y_idx.checked_add_signed(ofs_y) {
                        Some(v) => v,
                        None => continue,
                    };

                    let (pos_is_valid, _piece_to_be_taken) = self.is_pos_valid(pos_x, pos_y, piece.owner);
                    if !pos_is_valid {
                        continue;
                    }

                    available_moves.push((pos_x, pos_y));
                }
            },
        }

        available_moves
    }
}
