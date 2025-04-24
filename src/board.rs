
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
                        &piece.typee,
                        x_idx, y_idx,
                        if piece.owner == Player::A { -1 } else { 1 }
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

    pub fn get_available_moves_for(&self, piece_type: &PieceType, x_idx_usize: usize, y_idx_usize: usize, forward_y: isize) -> Vec<(usize, usize)> { // TODO actually, we also need to pass the owner so that we can check if the pawn can move 2 moves forward
        // let x_idx: isize = x_idx_usize.try_into().unwrap();
        let y_idx: isize = y_idx_usize.try_into().unwrap();

        let mut available_moves: Vec<(usize, usize)> = vec![];

        match piece_type {
            PieceType::Pawn => {
                let new_y = y_idx + forward_y;
                if (new_y >= 0) && (new_y < BOARD_SIZE_ISIZE) {
                    available_moves.push((x_idx_usize, new_y.try_into().unwrap()));
                }
            },

            PieceType::Knight => {},
            PieceType::Bishop => {},
            PieceType::Rook => {},
            PieceType::Queen => {},
            PieceType::King => {},
        }

        available_moves
    }
}
