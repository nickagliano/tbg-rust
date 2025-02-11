use tbg::start_game;
mod args;
mod db;
use args::parse_args;
pub use db::connection::get_connection;
pub use db::save::{delete_save, save_exists};
mod music;

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

    // music::manager::play_sound()?; // FIXME: This blocks the thread

    start_game()?;

    Ok(())
}
