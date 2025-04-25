
#[derive(Clone, PartialEq)]
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

    pub fn get_point_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 200,
        }
    }
}
