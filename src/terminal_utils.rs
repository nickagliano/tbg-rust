use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{self, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

struct Colors;
impl Colors {
    const ACTION_COLOR: style::Color = style::Color::Blue;
    const TEXT_COLOR: style::Color = style::Color::Yellow;
}

pub fn get_input() -> String {
    let mut stdout = io::stdout();
    let mut user_input = String::new();

    // Disable raw mode while we get user input
    terminal::disable_raw_mode().unwrap();

    // Set foreground color
    stdout
        .execute(SetForegroundColor(Colors::ACTION_COLOR))
        .unwrap();
    write!(stdout, "\n> ").unwrap();
    stdout.execute(ResetColor).unwrap();
    stdout.flush().unwrap();

    // Read input
    io::stdin().read_line(&mut user_input).unwrap();

    // Reset terminal colors and clear screen
    stdout.execute(ResetColor).unwrap();
    clear_console(None);

    stdout.flush().unwrap();

    // Renable raw mode after input
    terminal::enable_raw_mode().unwrap();

    user_input.trim().to_string()
}

pub fn clear_console(stdout: Option<&mut dyn Write>) {
    let stdout = match stdout {
        Some(s) => s,              // If a custom writer is passed, use it directly
        None => &mut io::stdout(), // If no custom writer is passed, use io::stdout()
    };

    // Clear the console and reset the cursor to the top left corner
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

pub fn prompt_enter_to_continue() {
    let prompt = "\rPress enter to continue... ";
    action_required(&format!("\n{}", prompt));

    // Block and wait for Enter key press
    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.code == KeyCode::Enter {
                    break; // Exit loop when Enter is pressed
                }
            }
        }
    }

    clear_console(None);
}

pub fn p(message: &str) {
    let mut stdout = io::stdout();
    stdout
        .execute(SetForegroundColor(Colors::TEXT_COLOR))
        .unwrap();
    write!(stdout, "{}", message).unwrap();
    stdout.execute(ResetColor).unwrap();
}

pub fn action_required(message: &str) {
    let mut stdout = io::stdout();
    stdout
        .execute(SetForegroundColor(Colors::ACTION_COLOR))
        .unwrap();
    write!(stdout, "{}", message).unwrap();
    stdout.execute(ResetColor).unwrap();
}

pub fn simulate_typing(message: &str) {
    let typing_speed = 25; // Adjust typing speed here

    let mut stdout = io::stdout();
    stdout
        .execute(SetForegroundColor(Colors::TEXT_COLOR))
        .unwrap();
    stdout.flush().unwrap();

    for c in message.chars() {
        write!(stdout, "{}", c).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(typing_speed));
    }

    stdout.execute(ResetColor).unwrap();
    println!(); // Move to the next line
}

pub fn title_screen() {
    let mut stdout = io::stdout();
    clear_console(None);

    let message = r"
        ___                                               __         __                         __ ,    ___
       -   ---___- _-_-        ,- _~,       _-_ _,,     ,-||-,     ,-||-,   _-_-,             ,-| ~    -   -_,   /\\,/\\,   ,- _~,
          (' ||      /,       (' /| /          -/  )   ('|||  )   ('|||  )    // ,           ('||/__, (  ~/||   /| || ||   (' /| /
         ((  ||      || __   ((  ||/=         ~||_<   (( |||--)) (( |||--))   ||/\\         (( |||  | (  / ||   || || ||  ((  ||/=
        ((   ||     ~||-  -  ((  ||            || \\  (( |||--)) (( |||--))  ~|| <          (( |||==|  \/==||   ||=|= ||  ((  ||
         (( //       ||===||  ( / |            ,/--||  ( / |  )   ( / |  )    ||/\\          ( / |  ,  /_ _||  ~|| || ||   ( / |
           -____-   ( \_, |    -____-         _--_-'    -____-     -____-    _-__,\\,         -____/  (  - \\,  |, \\,\\,   -____-
                                            (                                                                 _-
    ";

    write!(stdout, "{}", message).unwrap();
}

pub fn reset_cursor(stdout: &mut dyn Write) {
    write!(stdout, "{}", cursor::MoveTo(0, 0)).unwrap();
}

pub fn print_menu<T: std::fmt::Display>(
    message: &str,
    options: &Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
) {
    clear_console(None); // Clearing the console at the start

    let mut stdout = io::stdout();

    // Display message with or without simulated typing
    if use_simulate_typing {
        simulate_typing(&format!("{}\n", message));
    } else {
        p(&format!("{}\n\n", message));
    }

    stdout.flush().unwrap();

    // Loop through options and highlight the selected one
    for (i, option) in options.iter().enumerate() {
        if i == selected_index {
            action_required(&format!("\r> {}\n", option)); // Highlight selected option with '>'
        } else {
            action_required(&format!("\r  {}\n", option)); // Regular unhighlighted option
        }
    }

    // Move cursor to the selected index
    let cursor_position = selected_index + 2; // 2 accounts for the message and the extra newline
    write!(stdout, "{}", cursor::MoveTo(0, cursor_position as u16)).unwrap(); // Position cursor at selected index
    stdout.flush().unwrap();
}
