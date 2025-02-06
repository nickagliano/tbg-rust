use crate::db::save::{ensure_save_directory, get_save_path};
use crate::db::{GAME_STATE_TABLE, PLAYER_TABLE};
use rusqlite::{Connection, Result};

pub fn get_connection(db_path: Option<&str>) -> Result<Connection> {
    let save_path = get_save_path(db_path);

    // Ensure the save directory exists
    ensure_save_directory().map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;

    let conn = Connection::open(save_path)?;

    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                gender TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
        )",
            PLAYER_TABLE
        ),
        [],
    )?;

    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER NOT NULL,
                current_epic TEXT NOT NULL,
                current_stage TEXT NOT NULL,
                x INTEGER NOT NULL DEFAULT 0, -- X coordinate of the player
                y INTEGER NOT NULL DEFAULT 0, -- Y coordinate of the player
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
        );",
            GAME_STATE_TABLE
        ),
        [],
    )?;

    Ok(conn)
}
