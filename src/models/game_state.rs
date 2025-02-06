use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};

use crate::db::GAME_STATE_TABLE;

// TODO: Model epics, and stages within epics.

#[derive(Debug, Clone)]
pub struct GameState {
    pub player_id: i32,            // Foreign key to the player
    pub current_epic: String,      // Represents a larger story arc of the game
    pub current_stage: String,     // Represents the current stage of the epic
    pub x: i32,                    // Player's X coordinate
    pub y: i32,                    // Player's Y coordinate
    pub created_at: NaiveDateTime, // Timestamp when the game state was created
    pub updated_at: NaiveDateTime, // Timestamp when the game state was last updated
}

impl GameState {
    pub fn new(player_id: i32) -> Self {
        GameState {
            current_epic: "intro".to_string(),
            current_stage: "character_creation".to_string(),
            player_id,
            x: 0,
            y: 0,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "current_epic:{}, current_stage:{} -- x:{}, y:{}",
            self.current_epic, self.current_stage, self.x, self.y
        )
    }

    pub fn create(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "INSERT INTO {} (current_epic, current_stage, player_id, x, y, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                GAME_STATE_TABLE
            ),
            rusqlite::params![self.current_epic, self.current_stage, self.player_id, self.x, self.y, self.created_at, self.created_at],
        )?;
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "UPDATE {}
                    SET current_epic = ?1, current_stage = ?2, x = ?3, y = ?4, updated_at = ?5
                    WHERE player_id = ?6",
                GAME_STATE_TABLE
            ),
            rusqlite::params![
                self.current_epic,
                self.current_stage,
                self.x,
                self.y,
                chrono::Local::now().naive_local(),
                self.player_id
            ],
        )?;
        Ok(())
    }

    pub fn load_for_player(conn: &Connection, player_id: i32) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(&format!(
            "SELECT current_epic, current_stage, x, y, created_at, updated_at FROM {} WHERE player_id = ?1",
            GAME_STATE_TABLE
        ))?;
        let mut game_state_iter = stmt.query_map([player_id], |row| {
            Ok(GameState {
                current_epic: row.get(0)?,
                current_stage: row.get(1)?,
                player_id,
                x: row.get(2)?,
                y: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        if let Some(game_state) = game_state_iter.next() {
            return Ok(Some(game_state?));
        }

        Ok(None)
    }
}
