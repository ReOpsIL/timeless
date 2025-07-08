use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct BackupManager {
    data_dir: String,
    backup_dir: String,
    max_backups: u32,
}

impl BackupManager {
    pub fn new(data_dir: &str, max_backups: u32) -> Self {
        let backup_dir = format!("{}/backups", data_dir);
        BackupManager {
            data_dir: data_dir.to_string(),
            backup_dir,
            max_backups,
        }
    }
    
    pub fn create_backup(&self) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("backup_{}", timestamp);
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);
        
        // Ensure backup directory exists
        fs::create_dir_all(&self.backup_dir)?;
        
        // Copy data directory to backup location
        self.copy_dir(&self.data_dir, &backup_path)?;
        
        // Clean up old backups
        self.cleanup_old_backups()?;
        
        Ok(backup_name)
    }
    
    pub fn restore_backup(&self, backup_name: &str) -> Result<()> {
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);
        
        if !Path::new(&backup_path).exists() {
            return Err(anyhow::anyhow!("Backup not found: {}", backup_name));
        }
        
        // Remove current data directory
        if Path::new(&self.data_dir).exists() {
            fs::remove_dir_all(&self.data_dir)?;
        }
        
        // Restore from backup
        self.copy_dir(&backup_path, &self.data_dir)?;
        
        Ok(())
    }
    
    pub fn list_backups(&self) -> Result<Vec<String>> {
        let mut backups = Vec::new();
        
        if !Path::new(&self.backup_dir).exists() {
            return Ok(backups);
        }
        
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    backups.push(name.to_string());
                }
            }
        }
        
        backups.sort();
        Ok(backups)
    }
    
    fn copy_dir(&self, src: &str, dst: &str) -> Result<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = Path::new(dst).join(entry.file_name());
            
            if src_path.is_dir() {
                self.copy_dir(src_path.to_str().unwrap(), dst_path.to_str().unwrap())?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }
    
    fn cleanup_old_backups(&self) -> Result<()> {
        let mut backups = self.list_backups()?;
        
        if backups.len() <= self.max_backups as usize {
            return Ok(());
        }
        
        backups.sort();
        let to_remove = backups.len() - self.max_backups as usize;
        
        for backup in backups.iter().take(to_remove) {
            let backup_path = format!("{}/{}", self.backup_dir, backup);
            fs::remove_dir_all(backup_path)?;
        }
        
        Ok(())
    }
}