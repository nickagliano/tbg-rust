pub mod db;
pub mod models;
pub use models::player::Gender;
pub use models::player::Player;
pub mod terminal_utils;
pub use terminal_utils::{clear_console, get_input, prompt_enter_to_continue, simulate_typing};
pub mod test_utils;
use std::error::Error;

use std::io::{self, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn select_gender() -> Gender {
    let stdin = io::stdin();
    let mut _stdout = io::stdout().into_raw_mode().unwrap();

    let mut selected_index = 0;

    let options = vec![
        Gender::Male.to_string(),
        Gender::Female.to_string(),
        Gender::Unspecified.to_string(),
    ];

    let message = "Please select your gender:";

    print_menu(message, &options, selected_index, true);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                }
            }
            Key::Down => {
                if selected_index < options.len() - 1 {
                    selected_index += 1;
                }
            }
            Key::Char('\n') => {
                // Return the selected gender
                return Gender::from_string(&options[selected_index]);
            }
            _ => {}
        }

        // Pass use_simulate_typing to false so it doesn't re-type when user updates selection
        print_menu(message, &options, selected_index, false);
    }

    // Return a default gender if the loop ends unexpectedly
    Gender::Unspecified
}

pub fn print_menu<T: std::fmt::Display>(
    message: &str,
    options: &Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
) {
    clear_console();

    let mut stdout = io::stdout();

    if use_simulate_typing {
        simulate_typing(&message);
    } else {
        println!("{}", message)
    }

    // Move cursor back to the beginning of the line after printing
    write!(stdout, "{}\n", cursor::Goto(1, 1)).unwrap();

    // Loop through options and highlight the selected one
    for (i, option) in options.iter().enumerate() {
        if i == selected_index {
            // Highlight the selected option
            write!(stdout, "> {}", option).unwrap();
        } else {
            // Print unselected options without extra indentation
            write!(stdout, "  {}", option).unwrap();
        }

        // Move cursor back to the beginning of the line after printing
        write!(stdout, "{}\n", cursor::Goto(1, i as u16 + 2)).unwrap();
        stdout.flush().unwrap(); // Flush after each line to update the display immediately
    }
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

    clear_console();

    let _player = match Player::load(&conn)? {
        Some(player) => {
            simulate_typing(&format!(
                "Welcome back, {}! I hope you're ready to continue your adventure.",
                player.name
            ));
            player
        }
        None => {
            simulate_typing("Welcome to the wonderful world of The Book Game!");
            prompt_enter_to_continue();

            simulate_typing("What is your name, adventurer?");

            let name = get_input();

            let new_player = Player::new(name.clone(), models::player::Gender::Male);
            new_player.save(&conn)?;

            clear_console();

            simulate_typing(&format!(
                "Hello, {}. Welcome to the adventure!\n",
                new_player.name
            ));
            new_player
        }
    };

    prompt_enter_to_continue();

    // Start gender selection experience
    let gender = select_gender();

    simulate_typing(&format!("\nYou selected: {}", gender.to_string()));
    prompt_enter_to_continue();

    Ok(())
}
