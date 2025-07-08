use anyhow::Result;
use std::path::Path;

pub mod json_store;
pub mod backup;
pub mod repository;

pub use json_store::JsonStore;
pub use backup::BackupManager;
pub use repository::TeamRepository;

pub trait Storage {
    fn save<T: serde::Serialize>(&self, key: &str, data: &T) -> Result<()>;
    fn load<T: for<'de> serde::Deserialize<'de>>(&self, key: &str) -> Result<Option<T>>;
    fn delete(&self, key: &str) -> Result<()>;
    fn list_keys(&self) -> Result<Vec<String>>;
}

pub fn ensure_data_directory(path: &str) -> Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}