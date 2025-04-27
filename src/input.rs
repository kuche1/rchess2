
use std::io;
use std::io::Write;

pub enum GameChoice {
    NextTurnAi,
    MovePiece,
}

// pub fn enter() {
//     // print!("Press Enter to continue...");
//     // io::stdout().flush().unwrap();
//     let _ = io::stdin().read_line(&mut String::new());
// }

pub fn game_choice() -> GameChoice {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match line {
            "ai" | "" => return GameChoice::NextTurnAi,
            "move" => return GameChoice::MovePiece,
            _ => println!("invalid choice"),
        }
    }
}
