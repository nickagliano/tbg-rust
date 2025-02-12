use std::env;

pub struct GameArgs {
    pub new_game: bool,
    pub thread_demo: bool,
}

pub fn parse_args() -> GameArgs {
    let args: Vec<String> = env::args().collect();
    GameArgs {
        new_game: args.contains(&"--new-game".to_string()),
        thread_demo: args.contains(&"--thread-demo".to_string()),
    }
}
