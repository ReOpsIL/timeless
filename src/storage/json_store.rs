use crate::storage::Storage;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct JsonStore {
    data_dir: PathBuf,
}

impl JsonStore {
    pub fn new(data_dir: &str) -> Result<Self> {
        let data_dir = PathBuf::from(data_dir);
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)?;
        }
        Ok(JsonStore { data_dir })
    }
    
    fn get_file_path(&self, key: &str) -> PathBuf {
        self.data_dir.join(format!("{}.json", key))
    }
}

impl Storage for JsonStore {
    fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<()> {
        let path = self.get_file_path(key);
        let json = serde_json::to_string_pretty(data)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    fn load<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let path = self.get_file_path(key);
        if !path.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(path)?;
        let data: T = serde_json::from_str(&content)?;
        Ok(Some(data))
    }
    
    fn delete(&self, key: &str) -> Result<()> {
        let path = self.get_file_path(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
    
    fn list_keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        for entry in fs::read_dir(&self.data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(stem) = path.file_stem() {
                    if let Some(key) = stem.to_str() {
                        keys.push(key.to_string());
                    }
                }
            }
        }
        Ok(keys)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollection<T> {
    pub items: HashMap<String, T>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl<T> Default for DataCollection<T> {
    fn default() -> Self {
        DataCollection {
            items: HashMap::new(),
            last_updated: chrono::Utc::now(),
        }
    }
}

impl<T> DataCollection<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn insert(&mut self, key: String, value: T) {
        self.items.insert(key, value);
        self.last_updated = chrono::Utc::now();
    }
    
    pub fn get(&self, key: &str) -> Option<&T> {
        self.items.get(key)
    }
    
    pub fn remove(&mut self, key: &str) -> Option<T> {
        let result = self.items.remove(key);
        if result.is_some() {
            self.last_updated = chrono::Utc::now();
        }
        result
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}