use std::io::{self, Write};
use std::{thread, time::Duration};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub fn get_input() -> String {
    let mut stdout = io::stdout();
    let mut user_input = String::new();

    // Show the input prompt
    write!(stdout, "\n> ").unwrap();
    stdout.flush().unwrap();

    std::io::stdin().read_line(&mut user_input).unwrap();

    clear_console();
    return user_input.trim().to_string();
}

pub fn clear_console() {
    let mut stdout = io::stdout();
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();

    // Use termion's cursor to reset the cursor to the top
    write!(stdout, "{}{}", cursor::Goto(1, 1), cursor::Hide).unwrap();
}

pub fn prompt_enter_to_continue() {
    let prompt = "Press any key to continue... ";
    println!("\n{}", prompt);

    // Ensure prompt is printed before waiting for input
    io::stdout().flush().unwrap();

    // Set terminal to raw mode
    let stdin = io::stdin();
    let mut _stdout = io::stdout().into_raw_mode().unwrap();

    // Wait for any key press
    stdin.keys().next().unwrap().unwrap();
    clear_console();
}

pub fn simulate_typing(message: &str) {
    let mut stdout = io::stdout();
    for c in message.chars() {
        write!(stdout, "{}", c).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(25)); // Adjust typing speed here
    }
    println!(); // Move to the next line after typing is complete
}
