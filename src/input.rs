
use std::io::{self, Write};

pub fn enter() {
    // print!("Press Enter to continue...");
    io::stdout().flush().unwrap(); // Make sure the prompt is shown
    let _ = io::stdin().read_line(&mut String::new());
}
