use crate::db::PLAYER_TABLE;
use rusqlite::Error as RusqliteError;
use rusqlite::{params, Connection, Result};
use std::fmt;
use std::str::FromStr; // Alias for rusqlite::Error

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub gender: Gender,
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
        Player { name, gender }
    }

    // Load a player from the database
    pub fn load(conn: &Connection) -> Result<Option<Player>, RusqliteError> {
        let mut stmt = conn.prepare(&format!(
            "SELECT name, gender FROM {} LIMIT 1",
            PLAYER_TABLE
        ))?;

        let player_iter = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let gender_str: String = row.get(1)?; // Get the gender as a String

            // Use `parse` to convert the gender string to the Gender enum
            let gender: Gender = gender_str.parse().map_err(|e| {
                // Convert the GenderParseError to a RusqliteError
                RusqliteError::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
            })?;

            Ok(Player { name, gender })
        })?;

        // Loop through the result and return the first player found
        for player in player_iter {
            return Ok(Some(player?));
        }

        Ok(None)
    }

    pub fn save(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "INSERT INTO {} (name, gender) VALUES (?1, ?2)",
                PLAYER_TABLE
            ),
            params![self.name, self.gender.to_db_string()],
        )?;
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
