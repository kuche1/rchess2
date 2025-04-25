
use super::piece_type::PieceType;
use super::player::Player;

#[derive(Clone)]
pub struct Piece {
    pub typee: PieceType,
    pub owner: Player,
}

impl Piece {
    pub fn draw(&self) {
        self.owner.draw_color_on();
        self.typee.draw();
        self.owner.draw_color_off();
    }
}
