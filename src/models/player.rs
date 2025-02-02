use crate::db::PLAYER_TABLE;
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
}

impl Player {
    // Create a new player instance
    pub fn new(name: String) -> Player {
        Player { name }
    }

    // Setup the database table
    pub fn setup(conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL
                )",
                PLAYER_TABLE
            ),
            [],
        )?;
        Ok(())
    }

    // Load a player from the database
    pub fn load(conn: &Connection) -> Result<Option<Player>> {
        let mut stmt = conn.prepare(&format!("SELECT name FROM {} LIMIT 1", PLAYER_TABLE))?;
        let player_iter = stmt.query_map([], |row| Ok(Player { name: row.get(0)? }))?;

        for player in player_iter {
            return Ok(Some(player?));
        }
        Ok(None)
    }

    // Save the player to the database
    pub fn save(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!("INSERT INTO {} (name) VALUES (?1)", PLAYER_TABLE),
            params![self.name],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player() {
        let player = Player::new("Test Player".to_string());
        assert_eq!(player.name, "Test Player");
    }
}
