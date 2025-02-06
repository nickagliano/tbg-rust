use crate::db::{DEFAULT_DB, SAVE_DIR};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_save_path(db_name: Option<&str>) -> PathBuf {
    let db_file = db_name.unwrap_or(DEFAULT_DB);
    Path::new(SAVE_DIR).join(db_file)
}

pub fn ensure_save_directory() -> std::io::Result<()> {
    fs::create_dir_all(SAVE_DIR)
}

pub fn delete_save(db_name: Option<&str>) -> std::io::Result<()> {
    let save_path = &get_save_path(db_name);
    if save_path.exists() {
        fs::remove_file(save_path)?;
        println!("Save file deleted: {:?}", save_path);
    } else {
        println!("No existing save file found.");
    }
    Ok(())
}

pub fn save_exists(db_name: Option<&str>) -> bool {
    get_save_path(db_name).exists()
}
