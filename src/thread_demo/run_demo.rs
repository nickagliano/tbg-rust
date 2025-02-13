use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

struct App {
    counter: i32,
    exit: bool,
    input_mode: bool,
    player_name: String,
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            exit: false,
            input_mode: false,
            player_name: String::new(),
        }
    }

    fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All), cursor::Hide)?;

        while !self.exit {
            self.draw(&mut stdout)?;
            self.handle_events()?;
        }

        execute!(stdout, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (width, height) = terminal::size()?; // Get terminal dimensions

        let width = width.max(10); // Prevents too narrow display
        let height = height.max(5); // Prevents too short display

        // Define top and bottom borders
        let top_border = format!("┏{}┓", "━".repeat((width - 2) as usize));
        let bottom_border = format!("┗{}┛", "━".repeat((width - 2) as usize));

        // Define an empty middle line (sides)
        let empty_line = format!("┃{}┃", " ".repeat((width - 2) as usize));

        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;

        // Print top border
        writeln!(stdout, "{}\r", top_border)?;

        // Prepare the content lines
        let content = vec![
            format!("Counter App"),
            format!("Value: {}", self.counter),
            format!("Use Left/Right to change, Q to quit, I for input mode"),
            if self.input_mode {
                format!("Enter Name: {}", self.player_name)
            } else {
                format!("Player: {}", self.player_name)
            },
        ];

        // Calculate available space inside the frame
        let content_height = content.len();
        let padding_top = (height as usize - content_height - 2).max(0) / 2;
        let padding_bottom = (height as usize - content_height - padding_top - 2).max(0);

        // Print top padding
        for _ in 0..padding_top {
            writeln!(stdout, "{}\r", empty_line)?;
        }

        // Print content inside the frame
        for line in &content {
            writeln!(stdout, "┃{:^width$}┃\r", line, width = (width - 2) as usize)?;
        }

        // Print bottom padding (subtract 1 to prevent extra line)
        for _ in 0..(padding_bottom.saturating_sub(1)) {
            writeln!(stdout, "{}\r", empty_line)?;
        }

        // Print bottom border **without an extra new line**
        write!(stdout, "{}\r", bottom_border)?;

        stdout.flush()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if self.input_mode {
                    match key_event.code {
                        KeyCode::Enter => self.input_mode = false,
                        KeyCode::Backspace => {
                            self.player_name.pop();
                        }
                        KeyCode::Char(c) => self.player_name.push(c),
                        _ => {}
                    }
                } else {
                    match key_event.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Left => self.counter -= 1,
                        KeyCode::Right => self.counter += 1,
                        KeyCode::Char('i') => self.input_mode = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn run() -> io::Result<()> {
    App::new().run()
}
