use termion::event::Key;

#[derive(Debug)]
pub enum GameEvent {
    Quit,
    ToggleMusic,
    PlayMusic,
    Continue,
    Typing(Key),
    Other(Key),
}
