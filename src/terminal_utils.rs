use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal::{self, ClearType},
};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    style::ResetColor,
    terminal::Clear,
    ExecutableCommand,
};
use regex::Regex;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

// TODO: Set FrameType in settings
// pub enum FrameType {
//     Normal,
//     Fantasy,
// }

struct Colors;
impl Colors {
    const ACTION_COLOR: Color = Color::DarkCyan;
    const TEXT_COLOR: Color = Color::DarkYellow;

    // A single method to get the ANSI escape code for any color
    fn fg_string(color: Color) -> String {
        match color {
            Color::DarkCyan => "\x1b[36m".to_string(), // ANSI code for DarkCyan
            Color::DarkYellow => "\x1b[33m".to_string(), // ANSI code for DarkYellow
            _ => "\x1b[39m".to_string(),               // Default color if no match
        }
    }

    // Method to reset the color
    fn fg_str_reset() -> String {
        "\x1b[39m".to_string() // ANSI reset code
    }
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
    let mut stdout = io::stdout();
    let prompt = "\rPress enter to continue... ";

    write!(stdout, "{}", action_required(&format!("\n{}", prompt)))
        .expect("Failed to print prompt to continue");
    stdout.flush().unwrap();

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

pub fn action_required(message: &str) -> String {
    // Write the colored message directly without adding a newline
    let formatted_message = format!(
        "{}{}{}",
        Colors::fg_string(Colors::ACTION_COLOR), // Apply color
        message,
        Colors::fg_str_reset() // Reset the color
    );

    return formatted_message;
}

pub fn simulate_typing(message: &str) {
    let mut stdout = io::stdout();

    let typing_speed = 25;
    let mut displayed_message = String::new();

    // Hide the cursor before typing starts
    execute!(stdout, Hide).expect("Failed to hide cursor");

    for c in message.chars() {
        displayed_message.push(c);

        // Apply TEXT_COLOR to the message as it is typed
        let colored_message = format!(
            "{}{}{}",
            Colors::fg_string(Colors::TEXT_COLOR), // Apply the color
            displayed_message,
            Colors::fg_str_reset() // Reset the color after message
        );

        // Draw the window with the colored message
        draw_window(&colored_message).expect("Failed to draw window");

        thread::sleep(Duration::from_millis(typing_speed));
    }

    // Show the cursor again after typing is done
    execute!(stdout, Show).expect("Failed to show cursor");
}

pub fn title_screen() {
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

    // Get the formatted message with gradient applied
    let formatted_message = draw_title_with_gradient(message);

    // Now pass that formatted message to draw_window
    draw_window(&formatted_message).unwrap();
}

pub fn draw_title_with_gradient(message: &str) -> String {
    let lines: Vec<&str> = message.split('\n').collect();
    let mut output = String::new();

    for (i, line) in lines.iter().enumerate() {
        let gradient_color = get_gradient_color(i, lines.len());
        let color_code = format!(
            "\x1b[38;2;{};{};{}m",
            gradient_color.0, gradient_color.1, gradient_color.2
        );
        output.push_str(&format!("{}{}\x1b[0m\n", color_code, line));
    }

    output
}

// Returns an RGB tuple
fn get_gradient_color(index: usize, total_lines: usize) -> (u8, u8, u8) {
    let red = (index as f32 / total_lines as f32 * 255.0).round() as u8;
    let green = ((total_lines as f32 - index as f32) / total_lines as f32 * 255.0).round() as u8;

    (red, green, 128) // Return an RGB tuple
}

pub fn reset_cursor(stdout: &mut dyn Write) {
    write!(stdout, "{}", cursor::MoveTo(0, 0)).unwrap();
}

pub fn print_menu<T: std::fmt::Display>(
    message: &str,
    options: &Vec<T>,
    selected_index: usize,
    use_simulate_typing: bool,
) -> io::Result<()> {
    let mut content = String::new();

    // Pre-fill the content with spaces to prevent jumping
    content.push_str(&format!("{}\n", message));
    for _ in options.iter() {
        content.push_str("\n");
    }

    // Draw initial empty window before typing starts
    draw_window(&content)?;

    if use_simulate_typing {
        let mut typed_message = String::new();
        for c in message.chars() {
            typed_message.push(c);

            // Apply the text color using fg_string
            let colored_message = format!(
                "{}{}{}", // Text color + typed message + reset color
                Colors::fg_string(Colors::TEXT_COLOR),
                typed_message,
                Colors::fg_str_reset()
            );

            // Ensure the rest of the content stays intact, just adding the typed message
            draw_window(&format!(
                "{}\n{}", // typed message + remaining content
                colored_message,
                content.split_once('\n').unwrap().1
            ))?;

            thread::sleep(Duration::from_millis(25));
        }
    } else {
        draw_window(&content)?;
    }

    // Replace placeholder spaces with actual menu options
    let mut final_content = String::new();

    // Apply color to the message and reset it
    let colored_message = format!(
        "{}{}{}",
        Colors::fg_string(Colors::TEXT_COLOR),
        message,
        Colors::fg_str_reset()
    );
    final_content.push_str(&format!("{}\n", colored_message));

    // Build the final content with colored message and options
    for (i, option) in options.iter().enumerate() {
        let colored_option = if i == selected_index {
            format!(
                "{}> {}{}",
                Colors::fg_string(Colors::ACTION_COLOR),
                option,
                Colors::fg_str_reset()
            )
        } else {
            format!(
                "  {}{}{}", // Apply ACTION_COLOR + option + reset color
                Colors::fg_string(Colors::ACTION_COLOR),
                option,
                Colors::fg_str_reset()
            )
        };

        final_content.push_str(&format!("{}\n", colored_option));
    }

    // Draw final stable window with full content
    draw_window(&final_content)?;

    Ok(())
}

// FIXME: add a FrameType setting, use instead of hard-coding "NORMAL" borders
//        - DO NOT want to query this from the GameState every time.
//        - Want to load this at start of game, and only fetch if settings are updated.
pub fn draw_window(content: &str) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Get the terminal size
    let (width, height) = terminal::size()?;
    let width = width.max(10);
    let height = height.max(5);

    // Create borders
    let top_border = format!("┏{}┓", "━".repeat((width - 2) as usize));
    let bottom_border = format!("┗{}┛", "━".repeat((width - 2) as usize));
    let empty_line = format!("┃{}┃", " ".repeat((width - 2) as usize));
    // TODO: Maybe use these "fantasy" style borders
    // let repeat_count = (width - 2) / 3; // Required because the fantasy border is 3 chars long
    // let remainder = (width - 2) % 3; // Required because the fantasy border is 3 chars long
    // let top_border = format!(
    //     "╭{}{}╮",
    //     "╼◈╾".repeat(repeat_count as usize),
    //     "━".repeat(remainder as usize)
    // );
    // let bottom_border = format!(
    //     "╰{}{}╯",
    //     "╼◈╾".repeat(repeat_count as usize),
    //     "━".repeat(remainder as usize)
    // );
    // let empty_line = format!("║{}║", " ".repeat((width - 2) as usize));

    // Regex to remove ANSI escape codes (including color codes and resets)
    let color_code_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    // Move the cursor to the top-left corner and clear the screen
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;
    writeln!(stdout, "{}\r", top_border)?;

    // Split content into lines and calculate padding
    let content_lines: Vec<&str> = content.split('\n').collect();
    let content_height = content_lines.len();
    let padding_top = (height as usize - content_height - 2).max(0) / 2;
    let padding_bottom = (height as usize - content_height - padding_top - 2).max(0);

    // Pad top empty lines
    for _ in 0..padding_top {
        writeln!(stdout, "{}\r", empty_line)?;
    }

    // Pad and print each line of content
    for line in content_lines {
        // Remove color codes to calculate the padding based on the actual length
        let clean_line = color_code_re.replace_all(line, "");
        let line_len = clean_line.len();
        let extra_padding = width as usize - 2 - line_len;

        // Split padding between left and right equally
        let padding_left = extra_padding / 2;
        let padding_right = extra_padding - padding_left;

        // Pad the line and print it, with color codes intact
        let padded_line = format!(
            "┃{}{}{}┃",
            " ".repeat(padding_left),
            line,
            " ".repeat(padding_right)
        );

        writeln!(stdout, "{}\r", padded_line)?;
    }

    // Pad bottom empty lines
    for _ in 0..(padding_bottom.saturating_sub(1)) {
        writeln!(stdout, "{}\r", empty_line)?;
    }

    // Print the bottom border
    write!(stdout, "{}\r", bottom_border)?;
    stdout.flush()
}
