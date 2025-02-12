use crate::db;
use crate::game_engine::game_event::GameEvent;
use crate::models;
use crate::models::game_state::GameState;
use crate::models::player::Gender;
use crate::models::player::Player;
use crate::music::music_player::MusicPlayer;
use crate::terminal_utils;
use std::error::Error;
use std::io::{self};
use std::sync::mpsc;
use std::thread;
pub use terminal_utils::{
    clear_console, get_input, print_menu, prompt_enter_to_continue, simulate_typing,
};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct GameEngine {
    music_player: MusicPlayer,
    event_tx: mpsc::Sender<GameEvent>,
    event_rx: mpsc::Receiver<GameEvent>,
}

impl GameEngine {
    /// Creates a new GameEngine and sets up event handling.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<GameEvent>(); // Channels for event-based / pub-sub style communication
        let music_player = MusicPlayer::new();

        Self {
            music_player,
            event_tx: tx,
            event_rx: rx,
        }
    }

    /// Starts the game, including input listening and event handling.
    pub fn start(&mut self) {
        // Start music
        self.music_player.play();

        // Start input listener in a separate thread
        self.start_input_listener();

        // // Start event loop
        self.event_loop();

        // Start game
        self.start_game().expect("Failed to start game");
    }

    /// Spawns a thread that listens for keyboard input and sends events.
    pub fn start_input_listener(&self) {
        let tx = self.event_tx.clone(); // Clone transmitter for input thread

        thread::spawn(move || {
            let stdin = io::stdin();
            let stdin = stdin.lock();
            let _out = io::stdout()
                .into_raw_mode()
                .expect("Failed to enter raw mode");

            for key in stdin.keys() {
                match key {
                    Ok(Key::Char('q')) => {
                        let _ = tx.send(GameEvent::Quit);
                        break;
                    }
                    Ok(Key::Char('t')) => {
                        let _ = tx.send(GameEvent::ToggleMusic);
                        break;
                    }
                    Ok(Key::Char('r')) => {
                        let _ = tx.send(GameEvent::PlayMusic);
                        break;
                    }
                    Ok(Key::Char('\n')) => {
                        let _ = tx.send(GameEvent::Continue);
                        break;
                    }
                    Ok(other) => {
                        let _ = tx.send(GameEvent::Other(other));
                    }
                    Err(_) => break,
                }
                println!("Sent key event!");
            }
        });
    }

    /// Listens for events and processes them in a loop.
    pub fn event_loop(&mut self) {
        let rx = &mut self.event_rx; // Borrow the receiver instead of moving it

        loop {
            match rx.recv() {
                Ok(GameEvent::Quit) => {
                    println!("Quitting game!!");
                    break;
                }
                Ok(GameEvent::PlayMusic) => {
                    println!("Playing music...");
                }
                Ok(GameEvent::ToggleMusic) => {
                    println!("Toggling music...");
                }
                Ok(GameEvent::Continue) => {
                    println!("Continue");
                }
                Ok(GameEvent::Typing(key)) => {
                    print!("Unhandled key: {:?}", key);
                }
                Ok(GameEvent::Other(key)) => {
                    println!("Unhandled key: {:?}", key);
                }
                _ => {
                    println!("Event channel closed.");
                    break;
                }
            }
        }
    }

    /// Starts the game, loading an existing player or creating a new one.
    ///
    /// This function connects to the database, retrieves or initializes a player,
    /// their game state/savefile, and drops them back in a their current epic and stage.
    ///
    pub fn start_game(&mut self) -> Result<(), Box<dyn Error>> {
        terminal_utils::title_screen();

        prompt_enter_to_continue(&self.event_rx);

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
                prompt_enter_to_continue(&self.event_rx);

                simulate_typing("What is your name, adventurer?");

                let mut name = get_input();

                // Loop until the name is not blank
                while name.trim().is_empty() {
                    simulate_typing(
                        "I'm not sure I caught that... What did you want me to call you?",
                    );
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

        prompt_enter_to_continue(&self.event_rx);

        // Give special message if player is returning, but never completed character creation
        if !is_new_player && game_state.current_stage == "character_creation" {
            simulate_typing("Looks like you're still creating your character.");
            prompt_enter_to_continue(&self.event_rx);
        }

        if game_state.current_stage == "character_creation" {
            // Start gender selection experience
            let gender = self.select_gender();

            // Update player's gender
            player.gender = gender.clone();
            player.update(&conn)?;

            // Update game state, finished with choosing their name and gender
            game_state.current_stage = "book_tutorial".to_string();
            game_state.update(&conn).unwrap();

            // Reload player
            player = Player::load(&conn)?.unwrap();

            simulate_typing(&format!("\nYou selected: {}", player.gender.to_string()));

            prompt_enter_to_continue(&self.event_rx);
        }

        assert!(game_state.current_stage == "book_tutorial".to_string());

        simulate_typing(
            "Amazing.\n\nNow that we have introductions out of the way, let me show you some books.",
        );

        prompt_enter_to_continue(&self.event_rx);

        Ok(())
    }

    /// Programmatically send the `q` key event to stop the input listener
    pub fn stop_input_listener(&self) {
        let _ = self.event_tx.send(GameEvent::Typing(Key::Char('q'))); // Send 'q' to quit
    }

    fn select_gender(&mut self) -> Gender {
        let stdin = io::stdin();
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        let mut selected_index = 0;

        let options = vec![
            Gender::Male.to_string(),
            Gender::Female.to_string(),
            Gender::Unspecified.to_string(),
        ];

        let message = "Please select your gender:";

        print_menu(self, message, &options, selected_index, true);

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
            print_menu(self, message, &options, selected_index, false);
        }

        // Return a default gender if the loop ends unexpectedly
        Gender::Unspecified
    }
}
