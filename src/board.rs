
const BOARD_SIZE_USIZE: usize = 8;

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

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn draw(&self) {
        match self {
            Piece::Pawn => print!("♟︎"),
            Piece::Knight => print!("♞"),
            Piece::Bishop => print!("♝"),
            Piece::Rook => print!("♜"),
            Piece::Queen => print!("♛"),
            Piece::King => print!("♚"),
        }
    }
}

pub struct Tile {
    empty: bool, // not sure if this is a good idea, we could add another "empty" piece instead
    piece: Piece,
    owner: Player,
}

impl Tile {
    pub fn draw(&self) {
        if self.empty {
            print!(" ");
            return
        }

        self.owner.draw_color_on();
        self.piece.draw();
        self.owner.draw_color_off();
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
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::B },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::B },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::B },
                    Tile { empty: false, piece: Piece::Queen,  owner: Player::B },
                    Tile { empty: false, piece: Piece::King,   owner: Player::B },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::B },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::B },
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::B },
                ],
                [
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::B },
                ],
                [
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                ],
                [
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::B },
                ],
                [
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                ],
                [
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: true, piece: Piece::Pawn, owner: Player::A },
                ],
                [
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                    Tile { empty: false, piece: Piece::Pawn, owner: Player::A },
                ],
                [
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::A },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::A },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::A },
                    Tile { empty: false, piece: Piece::Queen,  owner: Player::A },
                    Tile { empty: false, piece: Piece::King,   owner: Player::A },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::A },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::A },
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::A },
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
        for lines in &self.board {
            for tile in lines {
                if tile.empty {
                    continue;
                }
                if tile.owner != self.players_turn {
                    continue;
                }
                print!("available move: ");
                tile.draw();
                println!();
            }
        }
    }
}
