
const BOARD_SIZE_USIZE: usize = 8;

pub enum Player {
    A,
    B,
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Tile {
    empty: bool,
    piece: Piece,
    owner: Player,
}

pub struct Board {
    tiles: [[Tile; BOARD_SIZE_USIZE]; BOARD_SIZE_USIZE],
}

impl Board {
    pub fn standard() -> Self {
        Board {
            tiles: [
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
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::B },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::B },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::B },
                    Tile { empty: false, piece: Piece::Queen,  owner: Player::B },
                    Tile { empty: false, piece: Piece::King,   owner: Player::B },
                    Tile { empty: false, piece: Piece::Bishop, owner: Player::B },
                    Tile { empty: false, piece: Piece::Knight, owner: Player::B },
                    Tile { empty: false, piece: Piece::Rook,   owner: Player::B },
                ],
            ]
        }
    }
}