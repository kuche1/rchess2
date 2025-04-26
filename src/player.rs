
// Clone - explicitly copy by calling `.clone()`
// Copy - allows implicit copying, say by calling a function `asd(piece.owner)`
// unfortunately, `Copy` seems to have a dependency on `Clone` (except if you implement it yourself)
#[derive(PartialEq, Clone, Copy)]
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
