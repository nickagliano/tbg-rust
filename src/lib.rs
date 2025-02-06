pub mod db;
pub mod models;
pub use models::game_state::GameState;
pub use models::player::{Gender, Player};
pub mod terminal_utils;
pub use terminal_utils::{
    action_required, clear_console, get_input, p, prompt_enter_to_continue, reset_cursor,
    simulate_typing,
};
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
    clear_console(None);

    let mut stdout = io::stdout();

    if use_simulate_typing {
        simulate_typing(&message);
    } else {
        p(&format!("{}", message))
    }

    stdout = reset_cursor(stdout);

    // Loop through options and highlight the selected one
    for (i, option) in options.iter().enumerate() {
        if i == selected_index {
            // Highlight the selected option
            action_required(&format!("> {}", option))
        } else {
            // Print unselected options without extra indentation
            action_required(&format!("  {}", option))
        }

        // Move cursor back to the beginning of the line after printing
        // FIXME: need to abstract this into reset_cursor. target + padding or something...
        write!(stdout, "{}\n", cursor::Goto(1, i as u16 + 2)).unwrap();
        stdout.flush().unwrap(); // Flush after each line to update the display immediately
    }
}

pub fn start_game() -> Result<(), Box<dyn Error>> {
    terminal_utils::title_screen();

    prompt_enter_to_continue();

    // Open the SQLite connection using the get_connection function
    // This will create the players table if it doesn't already exist
    // NOTE: by specifying None, this is going to use the default, 'save_file.db' database,
    //       and I realized... Rust has no "optional" parameters
    let conn = match db::connection::get_connection(None) {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error opening database connection: {}", e);
            return Err(Box::new(e));
        }
    };

    clear_console(None);

    let mut is_new_player = false; // Track whethe ra player was created in current session

    // Start game by either welcoming back player, or
    // guiding them through the intro
    match Player::load(&conn)? {
        Some(player) => {
            simulate_typing(&format!(
                "Welcome back, {}! I hope you're ready to continue your adventure.",
                player.name
            ));
            player
        }
        None => {
            is_new_player = true; // New player is being created

            simulate_typing("Welcome to the wonderful world of The Book Game!");
            prompt_enter_to_continue();

            simulate_typing("What is your name, adventurer?");

            let mut name = get_input();

            // Loop until the name is not blank
            while name.trim().is_empty() {
                simulate_typing("I'm not sure I caught that... What did you want me to call you?");
                name = get_input();
            }

            // We save with a default Gender. This gets overwritten in the next step.
            let new_player = Player::new(name.clone(), models::player::Gender::Male);
            new_player.create(&conn)?;

            // Grab the newly created player's id from the database
            // and create the player's game state
            GameState::new(Player::load(&conn).unwrap().unwrap().id)
                .create(&conn)
                .unwrap();

            clear_console(None);

            simulate_typing(&format!(
                "Hello, {}. Welcome to the adventure!\n",
                new_player.name
            ));
            new_player
        }
    };

    // Reload player
    let mut player = Player::load(&conn)?.unwrap();

    let mut game_state = GameState::load_for_player(&conn, player.id)
        .unwrap()
        .unwrap();

    prompt_enter_to_continue();

    // Give special message if player is returning, but never completed character creation
    if !is_new_player && game_state.current_stage == "character_creation" {
        simulate_typing("Looks like you're still creating your character.");
        prompt_enter_to_continue();
    }

    if game_state.current_stage == "character_creation" {
        // Start gender selection experience
        let gender = select_gender();

        // Update player's gender
        player.gender = gender.clone();
        player.update(&conn)?;

        // Update game state, finished with choosing their name and gender
        game_state.current_stage = "book_tutorial".to_string();
        game_state.update(&conn).unwrap();

        // Reload player
        player = Player::load(&conn)?.unwrap();

        simulate_typing(&format!("\nYou selected: {}", player.gender.to_string()));

        prompt_enter_to_continue();
    }

    assert!(game_state.current_stage == "book_tutorial".to_string());

    simulate_typing(
        "Amazing.\n\nNow that we have introductions out of the way, let me show you some books.",
    );

    prompt_enter_to_continue();

    Ok(())
}
