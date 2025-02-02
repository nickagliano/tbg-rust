#[cfg(test)]
use tbg::test_utils;
use tbg::Player;

// Test saving a new player
#[test]
fn test_save_player() {
    let conn = &test_utils::setup_test_db().conn;

    let player = Player::new("Test Player".to_string());

    // Save the player to the in-memory database
    player.save(&conn).unwrap();

    // Verify that the player is saved correctly
    let loaded_player = Player::load(&conn).unwrap();
    assert!(loaded_player.is_some());
    assert_eq!(loaded_player.unwrap().name, "Test Player");
}

// Test loading a player when there is no player in the DB
#[test]
fn test_load_no_player() {
    let conn = &test_utils::setup_test_db().conn;

    let loaded_player = Player::load(&conn).unwrap();

    assert!(loaded_player.is_none());
}

// Test loading a player when one is saved
#[test]
fn test_load_player() {
    let conn = &test_utils::setup_test_db().conn;

    let player = Player::new("Test Player".to_string());

    player.save(&conn).unwrap();

    // Load the player back from the database
    let loaded_player = Player::load(&conn).unwrap();
    assert!(loaded_player.is_some());
    assert_eq!(loaded_player.unwrap().name, "Test Player");
}
