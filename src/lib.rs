pub mod db;
pub mod models;
pub use models::player::Player; // Re-exports Player so it's accessible as `tbg::Player`
pub mod test_utils;

use std::error::Error;

use std::io::{self, Write};
use termion::clear;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn clear_console() {
    let mut stdout = io::stdout();
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();
}

fn prompt_enter_to_continue() {
    let prompt = "Press any key to continue... ";
    println!("{}", prompt);

    // Ensure prompt is printed before waiting for input
    io::stdout().flush().unwrap();

    // Set terminal to raw mode
    let stdin = io::stdin();
    let mut _stdout = io::stdout().into_raw_mode().unwrap();

    // Wait for any key press
    stdin.keys().next().unwrap().unwrap();
    clear_console();
}

pub fn start_game() -> Result<(), Box<dyn Error>> {
    // Open the SQLite connection using the get_connection function
    // This will create the players table if it doesn't already exist
    // NOTE: by specifying None, this is going to use the default, 'game.db' database,
    //       and I realized... Rust has no "optional" parameters
    let conn = match db::connection::get_connection(None) {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error opening database connection: {}", e);
            return Err(Box::new(e));
        }
    };

    let _player = match Player::load(&conn)? {
        Some(player) => {
            println!(
                "Welcome back, {}! I hope you're ready to continue your adventure.",
                player.name
            );
            player
        }
        None => {
            println!("Welcome to The Book Game universe!");
            prompt_enter_to_continue();

            println!("What is your name, adventurer?");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            let new_player = Player::new(name.clone());
            new_player.save(&conn)?; // Saving to the file-based database

            clear_console();

            println!(
                "\n\nHello, {}. Welcome to the adventure!\n",
                new_player.name
            );
            new_player
        }
    };

    prompt_enter_to_continue();

    Ok(())
}
