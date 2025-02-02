use rusqlite::{Connection, Result};

use crate::db::PLAYER_TABLE;

pub fn get_connection(db_path: Option<&str>) -> Result<Connection> {
    let path = db_path.unwrap_or("game.db");

    let conn = Connection::open(path)?;

    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            gender TEXT NOT NULL
        )",
            PLAYER_TABLE
        ),
        [],
    )?;

    Ok(conn)
}
