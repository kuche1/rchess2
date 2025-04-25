
use super::piece::Piece;

#[derive(Clone)]
pub struct Tile {
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn draw(&self) {
        if let Some(piece) = &self.piece {
            piece.draw();
        }else{
            print!(" ");
        }
    }
}
