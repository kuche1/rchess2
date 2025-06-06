
use std::io;
use std::io::Write;
use std::collections::HashMap;

use super::board::BOARD_SIZE_USIZE;

#[derive(Clone, Copy, Debug)]
pub enum GameChoice {
    PlayTurnBot,
    PlayTurnHuman,
}

// pub fn enter() {
//     // print!("Press Enter to continue...");
//     // io::stdout().flush().unwrap();
//     let _ = io::stdin().read_line(&mut String::new());
// }

pub fn game_choice() -> GameChoice {
    let commands = HashMap::from([
        ("b", GameChoice::PlayTurnBot),
        ("h", GameChoice::PlayTurnHuman),
    ]);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        match commands.get(line) {
            None => {
                println!("invalid choice: {line}");
                println!("commands:");
                for (cmd, act) in &commands {
                    println!("    `{cmd}` => {act:#?}");
                }
            },
            Some(action) => return *action,
        }

    }
}

pub fn position(prompt: &str) -> (usize, usize) { // kinda stupid name
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let pos = match input.trim().parse::<usize>() {
            Err(_e) => {
                println!("invalid position");
                continue;
            },
            Ok(v) => v,
        };

        let x = pos / 10;
        let y = pos % 10;

        if (x >= BOARD_SIZE_USIZE) || (y >= BOARD_SIZE_USIZE) {
            println!("too high");
            continue;
        };

        return (x, y);
    }
}
