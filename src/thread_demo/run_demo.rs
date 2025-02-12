use std::{
    io::{self, stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};
#[derive(Debug)]
pub enum GameEvent {
    Quit,
    Direction(Key),
    Other(Key),
}
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, terminal_size};

pub fn run() {
    let (tx, rx) = mpsc::channel();

    // Spawn input listener thread
    let tx_clone = tx.clone();
    thread::spawn(move || start_input_listener(tx_clone));

    // Start event loop in main thread
    thread::spawn(move || event_loop(rx));

    // Start game
    start_game();
}

/// Listens for keyboard input and sends GameEvents via the channel
pub fn start_input_listener(tx: mpsc::Sender<GameEvent>) {
    let stdin = io::stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}Entering raw mode...\r\n", cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for key in stdin.keys() {
        clear_console();

        match key {
            Ok(Key::Char('q')) => {
                tx.send(GameEvent::Quit).unwrap();
                break;
            }
            Ok(Key::Up) | Ok(Key::Down) | Ok(Key::Left) | Ok(Key::Right) => {
                tx.send(GameEvent::Direction(key.unwrap())).unwrap();
            }
            Ok(k) => {
                tx.send(GameEvent::Other(k)).unwrap();
            }
            Err(_) => break,
        }
    }

    write!(stdout, "{}Exiting raw mode...\r\n", cursor::Goto(1, 10)).unwrap();
    stdout.flush().unwrap();
}

/// Processes events from the channel
pub fn event_loop(rx: mpsc::Receiver<GameEvent>) {
    loop {
        if let Ok(event) = rx.try_recv() {
            match event {
                GameEvent::Quit => {
                    println!("Quitting game...");
                    break;
                }
                GameEvent::Direction(key) => {
                    println!("\rDirection: {:?}", key);
                }
                GameEvent::Other(key) => {
                    println!("\rOther key pressed: {:?}", key);
                }
            }
        }
        thread::sleep(Duration::from_millis(10)); // Reduce CPU usage
    }
}

pub fn start_game() {
    let mut line = 1;
    let (_, height) = terminal_size().unwrap(); // Get terminal size

    loop {
        // If we reach the bottom, just print normally to let it scroll
        if line >= height {
            line = 0;
        }

        print!("\r{}Hello, World!        ", cursor::Goto(1, line));

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        line += 1;
    }
}

pub fn clear_console() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Write the clear sequence
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();

    // Reset cursor to the top and hide it
    write!(stdout, "{}{}", cursor::Goto(1, 1), cursor::Hide).unwrap();
}
