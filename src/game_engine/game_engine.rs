use crate::db;
use crate::models::game_state::GameState;
use crate::models::player::Gender;
use crate::models::player::Player;
use crate::music::music_player::MusicPlayer;
use crate::terminal_utils;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::error::Error;
use std::io;
use std::time::Duration;

pub struct GameEngine {
    music_player: MusicPlayer,
}

impl GameEngine {
    pub fn new() -> Self {
        let music_player = MusicPlayer::new();

        Self { music_player }
    }

    pub fn start(&mut self) {
        self.music_player.play();
        self.event_loop();
        self.start_game().expect("Failed to start game");
    }

    pub fn event_loop(&mut self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        while let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('q') => {
                        println!("Quitting game!!");
                        break;
                    }
                    KeyCode::Char('t') => println!("Toggling music..."),
                    KeyCode::Char('r') => println!("Playing music..."),
                    KeyCode::Enter => println!("Continue"),
                    _ => println!("Unhandled key: {:?}", key_event.code),
                }
            }
        }
        disable_raw_mode().expect("Failed to disable raw mode");
    }

    pub fn start_game(&mut self) -> Result<(), Box<dyn Error>> {
        terminal_utils::title_screen();
        terminal_utils::prompt_enter_to_continue();

        let conn = db::connection::get_connection(None)?;
        let mut is_new_player = false;

        // Start game by either welcoming back player, or
        // guiding them through the intro
        // FIXME: Return a player from this, instead of having to reload
        match Player::load(&conn)? {
            Some(player) => {
                terminal_utils::simulate_typing(&format!(
                    "Welcome back, {}! Ready to continue?",
                    player.name
                ));
                player
            }
            None => {
                // New player is being created
                is_new_player = true;

                terminal_utils::simulate_typing("Welcome to the wonderful world of The Book Game!");

                terminal_utils::prompt_enter_to_continue();

                terminal_utils::simulate_typing("What is your name?");
                let mut name = terminal_utils::get_input();

                // Loop until the name is not blank
                while name.trim().is_empty() {
                    terminal_utils::simulate_typing("Please enter a valid name.");
                    name = terminal_utils::get_input();
                }

                // We save with a default Gender. This gets overwritten in the next step.
                let new_player = Player::new(name.clone(), Gender::Male);
                new_player.create(&conn)?;

                // Grab the newly created player's id from the database
                // and create the player's game state
                GameState::new(Player::load(&conn).unwrap().unwrap().id)
                    .create(&conn)
                    .unwrap();

                terminal_utils::simulate_typing(&format!("Hello, {}!", new_player.name));
                new_player
            }
        };

        // Reload player
        let mut player = Player::load(&conn)?.unwrap();

        // Reload game state
        let mut game_state = GameState::load_for_player(&conn, player.id)?.unwrap();

        terminal_utils::prompt_enter_to_continue();

        // Give special message if player is returning, but never completed character creation
        if !is_new_player && game_state.current_stage == "character_creation" {
            terminal_utils::simulate_typing("Looks like you're still creating your character.");
            terminal_utils::prompt_enter_to_continue();
        }

        if game_state.current_stage == "character_creation" {
            // Start gender selection experience
            let gender = self.select_gender();

            // Update player's gender
            player.gender = gender.clone();
            player.update(&conn)?;

            // Update game state, finished with choosing their name and gender
            game_state.current_stage = "book_tutorial".to_string();
            game_state.update(&conn)?;

            // Reload player
            player = Player::load(&conn)?.unwrap();

            terminal_utils::simulate_typing(&format!(
                "You selected: {}",
                player.gender.to_string()
            ));
            terminal_utils::prompt_enter_to_continue();
        }

        terminal_utils::simulate_typing("Now, let's start the adventure!");
        terminal_utils::prompt_enter_to_continue();

        // TODO: Implement map piece here.

        Ok(())
    }

    // FIXME: Abstract out this select gender logic to be reusable as a "menu_select" or something.
    //        - like buying things from a shop, other character configuration, etc.
    //        - selecting a move in a battle will be similar
    pub fn select_gender(&self) -> Gender {
        let mut stdout = io::stdout();
        enable_raw_mode().expect("Failed to enable raw mode");

        let options = vec![
            Gender::Male.to_string(),
            Gender::Female.to_string(),
            Gender::Unspecified.to_string(),
        ];
        let message = "Please select your gender:";
        let mut selected_index = 0;

        // Hide the cursor before selection starts
        execute!(stdout, Hide).expect("Cursor failed to hide");

        terminal_utils::print_menu(message, &options, selected_index, true)
            .expect("Printing gender menu failed");

        loop {
            // Block and wait for a key event
            if let Ok(Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < options.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        disable_raw_mode().expect("Failed to disable raw mode");
                        execute!(stdout, Show).expect("Cursor failed to show");
                        terminal_utils::clear_console(None);
                        return Gender::from_string(&options[selected_index]);
                    }
                    _ => {
                        // FIXME: Handle this better? Re-pick gender?
                        terminal_utils::clear_console(None);
                        execute!(stdout, Show).expect("Cursor failed to show");
                        return Gender::Unspecified;
                    }
                }

                // Redraw the menu after every key press to update the selection
                // Set use_simulate_typing to false so it doesn't re-type when user updates selection
                terminal_utils::print_menu(message, &options, selected_index, false)
                    .expect("Printing menu failed");
            }
        }
    }
}
