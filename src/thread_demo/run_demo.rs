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
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            exit: false,
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
        execute!(stdout, cursor::MoveTo(0, 0))?;
        writeln!(stdout, "Counter App\r")?;
        writeln!(stdout, "Value: {}\r", self.counter)?;
        writeln!(stdout, "Use Left/Right to change, Q to quit\r")?;
        stdout.flush()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Left => self.counter -= 1,
                    KeyCode::Right => self.counter += 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

pub fn run() -> io::Result<()> {
    App::new().run()
}
