use std::env;

pub struct GameArgs {
    pub new_game: bool,
    pub dev: bool,
}

pub fn parse_args() -> GameArgs {
    let args: Vec<String> = env::args().collect();
    GameArgs {
        new_game: args.contains(&"--new-game".to_string()),
        dev: args.contains(&"--dev".to_string()),
    }
}
