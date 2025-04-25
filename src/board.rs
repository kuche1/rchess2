
use super::player::Player;
use super::piece_type::PieceType;
use super::piece::Piece;
use super::tile::Tile;

const BOARD_SIZE_USIZE: usize = 8;
// const BOARD_SIZE_ISIZE: isize = BOARD_SIZE_USIZE as isize;

#[derive(Clone)]
pub struct Board {
    board: [[Tile; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
    players_turn: Player,
    // TODO0 maybe make a `game` struct, put the board there, and have a vector "already played board", then clean that vector on every pawn move
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

    fn evaluate_score(&self, forr: &Player) -> i32 {
        let mut score:i32 = 0;

        for line in self.board.iter() {
            for tile in line {

                let piece = match &tile.piece {
                    None => continue,
                    Some(v) => v,
                };

                if piece.owner == *forr {
                    score += piece.typee.get_point_value();
                }else{
                    score -= piece.typee.get_point_value();
                }
            }
        }

        score
    }

    fn commit_turn(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) { // kinda stupid name
        self.board[to_y][to_x] = self.board[from_y][from_x].clone();
        self.board[from_y][from_x].piece = None;

        let piece = self.board[to_y][to_x].piece.as_mut().unwrap();

        if piece.typee == PieceType::Pawn {
            if (to_y == 0) || (to_y == BOARD_SIZE_USIZE-1) { // no need to actually check the owner
                piece.typee = PieceType::Queen;
            }
        }
    }

    fn switch_to_next_players_turn(&mut self) {
        self.players_turn = match self.players_turn {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub fn play_turn(&mut self, additional_think_depth: i32) {
        let mut best_move_score: Option<i32> = None;
        let mut best_move: (usize, usize, usize, usize) = (0, 0, 0, 0);

        for (y_idx, lines) in self.board.iter().enumerate() {
            for (x_idx, tile) in lines.iter().enumerate() {

                let piece = match &tile.piece {
                    None => continue,
                    Some(v) => v,
                };

                if piece.owner != self.players_turn {
                    continue;
                }

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
                    virtual_board.commit_turn(x_idx, y_idx, x, y);
                    if additional_think_depth > 0 {
                        virtual_board.switch_to_next_players_turn();
                        virtual_board.play_turn(additional_think_depth - 1);
                    }
                    let score = virtual_board.evaluate_score(&piece.owner); // TODO1 maybe keep a variable `score`, then only updates it, but then again this will break multiplayer chess

                    // println!("{}move {},{}->{},{} evaluates to {}", "    ".repeat(additional_think_depth as usize), x_idx, y_idx, x, y, score);

                    match best_move_score {
                        None => {
                            best_move_score = Some(score);
                            best_move = (x_idx, y_idx, x, y);
                            // println!("{}this is my new fav move: first available", "    ".repeat(additional_think_depth as usize));
                        },
                        Some(val) => {
                            if val < score {
                                // println!("{}this is my new fav move: old({}) < new({})", "    ".repeat(additional_think_depth as usize), val, score);
                                best_move_score = Some(score);
                                best_move = (x_idx, y_idx, x, y);
                            }
                        },
                    }

                }
                // println!();
            }
        }

        if best_move_score == None {
            // fucking what to do ? draw ? yeah, seems reasonable TODO1(don't just fucking crash)
            panic!("draw");
        }

        {
            self.players_turn.draw_color_on();
            // print!("player ");
            self.players_turn.draw_color_off();
            // print!("plays ");

            let (fx, fy, tx, ty) = best_move;
            // println!("{},{} -> {},{}", fx, fy, tx, ty);
            self.commit_turn(fx, fy, tx, ty);
        }

        self.switch_to_next_players_turn();
    }

    fn is_pos_valid(&self, x: usize, y: usize, forr: &Player) -> bool {
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return false;
        }

        let tile = &self.board[y][x];

        let piece = match &tile.piece {
            None => return true,
            Some(v) => v,
        };

        return piece.owner != *forr;
    }

    fn is_pos_valid_for_regular_pawn_move(&self, x: usize, y: usize) -> bool {
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return false;
        }
        return self.board[y][x].piece.is_none();
    }

    fn is_pos_valid_for_attack_pawn_move(&self, x: usize, y: usize, owner: &Player) -> bool {
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
                    // TODO1 I actually need to fucking check if the position is right (the pawn has not moved)

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

                    if !self.is_pos_valid(dest_x, dest_y, &piece.owner) {
                        continue;
                    }

                    available_moves.push((dest_x, dest_y));
                }

            },

            PieceType::Bishop => {}, // TODO0

            PieceType::Rook => {}, // TODO0

            PieceType::Queen => {}, // TODO0

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

                    if !self.is_pos_valid(pos_x, pos_y, &piece.owner) {
                        continue;
                    }

                    available_moves.push((pos_x, pos_y));
                }
            },
        }

        available_moves
    }
}
