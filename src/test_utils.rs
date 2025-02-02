use crate::db;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct TestDb {
    pub conn: Connection,
    pub test_db_path: String,
}

pub fn setup_test_db() -> TestDb {
    let test_db_path = generate_test_db_path();
    println!("Creating test database at: {}", test_db_path);
    let conn = db::connection::get_connection(Some(&test_db_path)).unwrap();
    println!("Opened new test database connection");

    TestDb { conn, test_db_path }
}

fn generate_test_db_path() -> String {
    let uuid = Uuid::new_v4();
    format!("test_{}.db", uuid)
}

impl Drop for TestDb {
    fn drop(&mut self) {
        println!("Tearing down test database at: {}", self.test_db_path);
        if Path::new(&self.test_db_path).exists() {
            fs::remove_file(&self.test_db_path).expect("Failed to delete test database");
            println!(
                "Successfully deleted test database at: {}",
                self.test_db_path
            );
        }
    }
}
