use tbg::start_game;
mod args;
mod db;
use args::parse_args;
pub use db::connection::get_connection;
pub use db::save::{delete_save, save_exists};
use tbg::music::music_player::MusicPlayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_args = parse_args();

    if game_args.new_game {
        if save_exists(None) {
            delete_save(None)?;
            println!("Previous save deleted. Starting a new game...");
        } else {
            println!("No existing save found. Starting a new game...");
        }
    }

    // TODO: Handle this better!
    // TODO: Need a new construct, GameEngine
    // Start game should probably be called through GameEngine.
    // GameEngine should receive the dj
    let dj = MusicPlayer::new();
    dj.start_music_thread();

    start_game(dj)?;

    Ok(())
}
