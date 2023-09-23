use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::AppResult;
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
}
impl Config {
    pub fn open<T: AsRef<Path>>(path: T) -> AppResult<Self> {
        let data = fs::read(path)?;
        let cfg = serde_json::from_slice(&data)?;
        Ok(cfg)
    }
}