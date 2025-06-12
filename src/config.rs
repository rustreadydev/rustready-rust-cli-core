use serde::{Deserialize, Serialize};
use std::fs;

use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub last_run: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn default(path: &str) -> Result<Self> {
        let config = Config {
            last_run: "default".to_string(),
        };
        config.save(path)?;
        Ok(config)
    }
}