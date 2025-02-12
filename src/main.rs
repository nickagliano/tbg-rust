mod args;
mod db;
mod game_engine;
pub mod models;
pub mod music;
pub mod terminal_utils;
mod thread_demo;
use args::parse_args;
pub use db::connection::get_connection;
pub use db::save::{delete_save, save_exists};
use game_engine::game_engine::GameEngine;
use thread_demo::run_demo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_args = parse_args();

    if game_args.thread_demo {
        run_demo::run();
        return Ok(()); // Early exit!
    }

    if game_args.new_game {
        if save_exists(None) {
            delete_save(None)?;
            println!("Previous save deleted. Starting a new game...");
        } else {
            println!("No existing save found. Starting a new game...");
        }
    }

    let mut game_engine = GameEngine::new();
    game_engine.start(); // Handles everything

    Ok(())
}
