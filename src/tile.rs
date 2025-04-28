
use super::piece::Piece;

const WHITE_BRIGHTNESS: u8 = 100;
const BLACK_BRIGHTNESS: u8 = 0;

#[derive(Clone, PartialEq)]
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

    pub fn draw_black_background_on(&self) {
        print!("\x1b[48;2;{};{};{}m", BLACK_BRIGHTNESS, BLACK_BRIGHTNESS, BLACK_BRIGHTNESS);
    }

    pub fn draw_black_background_off(&self) {
        print!("\x1b[0;0m");
    }

    pub fn draw_white_background_on(&self) {
        print!("\x1b[48;2;{};{};{}m", WHITE_BRIGHTNESS, WHITE_BRIGHTNESS, WHITE_BRIGHTNESS);
    }

    pub fn draw_white_background_off(&self) {
        print!("\x1b[0;0m");
    }
}
