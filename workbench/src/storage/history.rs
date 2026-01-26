use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::models::BenchmarkRun;

/// Manages storage and retrieval of benchmark history
pub struct HistoryStorage {
    storage_dir: PathBuf,
}

impl HistoryStorage {
    /// Create a new history storage instance
    pub fn new() -> Self {
        let storage_dir = Self::get_storage_dir();
        Self { storage_dir }
    }

    /// Get the storage directory path
    fn get_storage_dir() -> PathBuf {
        // Use platform-appropriate app data directory
        let base = if cfg!(target_os = "windows") {
            std::env::var("LOCALAPPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| std::env::temp_dir())
        } else if cfg!(target_os = "macos") {
            dirs::home_dir()
                .map(|h| h.join("Library/Application Support"))
                .unwrap_or_else(std::env::temp_dir)
        } else {
            dirs::home_dir()
                .map(|h| h.join(".local/share"))
                .unwrap_or_else(std::env::temp_dir)
        };

        base.join("WorkBench-Pro").join("history")
    }

    /// Ensure storage directory exists
    fn ensure_dir(&self) -> Result<()> {
        if !self.storage_dir.exists() {
            fs::create_dir_all(&self.storage_dir)?;
        }
        Ok(())
    }

    /// Save a benchmark run to history
    pub fn save(&self, run: &BenchmarkRun) -> Result<PathBuf> {
        self.ensure_dir()?;

        let filename = format!(
            "run_{}.json",
            run.timestamp.format("%Y%m%d_%H%M%S")
        );
        let path = self.storage_dir.join(&filename);

        let json = serde_json::to_string_pretty(run)?;
        fs::write(&path, json)?;

        Ok(path)
    }

    /// Load all saved benchmark runs
    pub fn load_all(&self) -> Result<Vec<BenchmarkRun>> {
        self.ensure_dir()?;

        let mut runs = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "json") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(run) = serde_json::from_str::<BenchmarkRun>(&contents) {
                            runs.push(run);
                        }
                    }
                }
            }
        }

        // Sort by timestamp, newest first
        runs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(runs)
    }

    /// Load the most recent N runs
    pub fn load_recent(&self, count: usize) -> Result<Vec<BenchmarkRun>> {
        let all = self.load_all()?;
        Ok(all.into_iter().take(count).collect())
    }

    /// Delete a specific run by timestamp
    pub fn delete(&self, run: &BenchmarkRun) -> Result<()> {
        let filename = format!(
            "run_{}.json",
            run.timestamp.format("%Y%m%d_%H%M%S")
        );
        let path = self.storage_dir.join(&filename);

        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// Clear all history
    pub fn clear_all(&self) -> Result<()> {
        if self.storage_dir.exists() {
            fs::remove_dir_all(&self.storage_dir)?;
        }
        Ok(())
    }

    /// Get number of saved runs
    pub fn count(&self) -> usize {
        self.load_all().map(|r| r.len()).unwrap_or(0)
    }
}

impl Default for HistoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

// Add dirs dependency fallback
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .map(PathBuf::from)
    }
}
