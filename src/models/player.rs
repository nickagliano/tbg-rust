use crate::db::PLAYER_TABLE;
use chrono::NaiveDateTime;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection, Result};
use std::fmt;
use std::str::FromStr; // Alias for rusqlite::Error
use uuid::Uuid; // For handling timestamps

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
    Unspecified,
}

impl Gender {
    // Convert Gender enum to string for database
    pub fn to_db_string(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Unspecified => "unspecified",
        }
    }

    // Convert from string to Gender enum
    pub fn from_string(s: &str) -> Self {
        match s {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            "Choose not to specify" => Gender::Unspecified,
            _ => Gender::Unspecified, // FIXME: I'd rather panic here.
        }
    }

    // Convert from string to Gender enum
    pub fn from_db_string(s: &str) -> Self {
        match s {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "unspecified" => Gender::Unspecified,
            _ => Gender::Unspecified, // FIXME: I'd rather panic here.
        }
    }
}

impl FromStr for Gender {
    type Err = GenderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "unspecified" => Ok(Gender::Unspecified),
            e => Err(GenderParseError::InvalidGender(e.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum GenderParseError {
    InvalidGender(String),
}

impl std::error::Error for GenderParseError {}

impl fmt::Display for GenderParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid gender: {:?}", self)
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let gender_str = match *self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Unspecified => "Choose not to specify",
        };
        write!(f, "{}", gender_str)
    }
}

impl Player {
    // Create a new player instance
    pub fn new(name: String, gender: Gender) -> Self {
        Player {
            id: 0, // DB will auto-increment this
            uuid: Uuid::new_v4(),
            name,
            gender,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn load(conn: &Connection) -> Result<Option<Self>> {
        Self::load_most_recent(conn)
    }

    // Load the most recent player by the updated_at field
    pub fn load_most_recent(conn: &Connection) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, uuid, name, gender, created_at, updated_at FROM players ORDER BY updated_at DESC LIMIT 1",
        )?;
        let mut player_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let uuid: String = row.get(1)?;
            let name: String = row.get(2)?;
            let gender: String = row.get(3)?;
            let created_at: NaiveDateTime = row.get(4)?;
            let updated_at: NaiveDateTime = row.get(5)?;
            Ok(Player {
                id,
                uuid: Uuid::parse_str(&uuid).unwrap(),
                name,
                gender: Gender::from_db_string(&gender),
                created_at,
                updated_at,
            })
        })?;

        if let Some(player) = player_iter.next() {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    // Load a player by UUID
    pub fn load_by_uuid(conn: &Connection, player_uuid: &Uuid) -> Result<Option<Self>> {
        let mut stmt = conn.prepare(
            "SELECT id, uuid, name, gender, created_at, updated_at FROM players WHERE uuid = ?1",
        )?;
        let player_iter = stmt.query_map([player_uuid.to_string()], |row| {
            let id: i32 = row.get(0)?;
            let uuid: String = row.get(1)?;
            let name: String = row.get(2)?;
            let gender: String = row.get(3)?;
            let created_at: NaiveDateTime = row.get(4)?;
            let updated_at: NaiveDateTime = row.get(5)?;
            Ok(Player {
                id,
                uuid: Uuid::parse_str(&uuid).unwrap(),
                name,
                gender: Gender::from_db_string(&gender), // Assuming you have a method to convert from DB string to Gender
                created_at,
                updated_at,
            })
        })?;

        for player in player_iter {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    // Save the player to the database
    pub fn create(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO players (uuid, name, gender, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                self.uuid.to_string(),
                self.name,
                self.gender.to_db_string(),
                self.created_at,
                self.created_at // Default to created at
            ],
        )?;
        Ok(())
    }

    pub fn update(&self, conn: &Connection) -> Result<()> {
        let rows_updated = conn.execute(
            &format!(
                "UPDATE {} SET gender = ?1, updated_at = ?2 WHERE name = ?3",
                PLAYER_TABLE
            ),
            params![
                self.gender.to_db_string(),
                chrono::Local::now().naive_local(),
                self.name
            ],
        )?;

        if rows_updated == 0 {
            // Handle the case where no rows were updated, i.e., no player was found
            return Err(RusqliteError::QueryReturnedNoRows);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player() {
        let player = Player::new("Test Player".to_string(), Gender::Male);
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.gender.to_db_string(), "male");
    }
}
