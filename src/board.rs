
const BOARD_SIZE_USIZE: usize = 8;
const BOARD_SIZE_ISIZE: isize = BOARD_SIZE_USIZE as isize;

#[derive(PartialEq)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn draw_color_on(&self) {
        match self {
            Player::A => print!("\x1b[38;2;{};{};{}m", 100, 255, 100),
            Player::B => print!("\x1b[38;2;{};{};{}m", 255, 100, 100),
        }
    }

    pub fn draw_color_off(&self) {
        print!("\x1b[0;0m");
    }
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn draw(&self) {
        match self {
            PieceType::Pawn => print!("♟︎"),
            PieceType::Knight => print!("♞"),
            PieceType::Bishop => print!("♝"),
            PieceType::Rook => print!("♜"),
            PieceType::Queen => print!("♛"),
            PieceType::King => print!("♚"),
        }
    }
}

pub struct Piece {
    typee: PieceType,
    owner: Player,
}

impl Piece {
    pub fn draw(&self) {
        self.owner.draw_color_on();
        self.typee.draw();
        self.owner.draw_color_off();
    }
}

pub struct Tile {
    empty: bool, // not sure if this is a good idea, we could add another "empty" piece instead
    piece: Piece,
}

impl Tile {
    pub fn draw(&self) {
        if self.empty {
            print!(" ");
            return
        }
        self.piece.draw();
    }
}

pub struct Board {
    board: [[Tile; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
    players_turn: Player,
}

impl Board {
    pub fn standard() -> Self {
        Board {
            board: [
                [
                    Tile { empty: false, piece: Piece { typee: PieceType::Rook,   owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Knight, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Bishop, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Queen,  owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::King,   owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Bishop, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Knight, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Rook,   owner: Player::B } },
                ],
                [
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                ],
                [
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                ],
                [
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::B } },
                ],
                [
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                ],
                [
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: true, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                ],
                [
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Pawn, owner: Player::A } },
                ],
                [
                    Tile { empty: false, piece: Piece { typee: PieceType::Rook,   owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Knight, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Bishop, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Queen,  owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::King,   owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Bishop, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Knight, owner: Player::A } },
                    Tile { empty: false, piece: Piece { typee: PieceType::Rook,   owner: Player::A } },
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

    pub fn play_turn(&self) {
        for (y_idx, lines) in self.board.iter().enumerate() {
            for (x_idx, tile) in lines.iter().enumerate() {
                if tile.empty {
                    continue;
                }

                let piece = &tile.piece;

                if piece.owner != self.players_turn {
                    continue;
                }

                let available_moves =
                    self.get_available_moves_for(
                        piece,
                        x_idx, y_idx,
                    );

                if available_moves.len() == 0 {
                    continue;
                }

                print!("available moves for ");
                piece.draw();
                print!(":");

                for movee in available_moves {
                    let (x, y) = movee;
                    print!(" x={},y={}", x, y);
                }
                println!();
            }
        }
    }

    fn is_pos_valid(&self, x: usize, y: usize, forr: &Player) -> bool {
        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            return false;
        }

        let tile = &self.board[y][x];

        if tile.empty {
            return true;
        }

        return tile.piece.owner != *forr;
    }

    pub fn get_available_moves_for(&self, piece: &Piece, x_idx: usize, y_idx: usize) -> Vec<(usize, usize)> {

        let mut available_moves: Vec<(usize, usize)> = vec![];

        'matchh: {
            match piece.typee {

                PieceType::Pawn => {

                    let forward_y: isize = if piece.owner == Player::A { -1 } else { 1 };

                    // move forward once

                    let new_y = match y_idx.checked_add_signed(forward_y) {
                        Some(v) => v,
                        None => break 'matchh,
                    };

                    if !self.is_pos_valid(x_idx, new_y, &piece.owner) {
                        break 'matchh;
                    }

                    available_moves.push((x_idx, new_y));

                    // move forward twice (will only trigger if you can also move forward once)
                    // TODO I actually need to fucking check if the position is right (the pawn has not moved)

                    let new_y = match new_y.checked_add_signed(forward_y) {
                        Some(v) => v,
                        None => break 'matchh,
                    };

                    if !self.is_pos_valid(x_idx, new_y, &piece.owner) {
                        break 'matchh;
                    }

                    available_moves.push((x_idx, new_y));

                    // TODO0 en passant
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

                PieceType::Bishop => {},
                PieceType::Rook => {},
                PieceType::Queen => {},
                PieceType::King => {},
            }
        }

        available_moves
    }
}
