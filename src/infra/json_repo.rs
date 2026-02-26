use std::fs;
use std::path::{Path, PathBuf};

use crate::core::{Repo, RepoSnapshot, PlmError};

pub struct JsonRepo {
    path: PathBuf,
}

impl JsonRepo {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, PlmError> {
        let path = path.into();

        if let Some(parent) = path.parent() {
            ensure_dir(parent)?;
        }

        Ok(Self { path })
    }
}

impl Repo for JsonRepo {
    fn load(&self) -> Result<RepoSnapshot, PlmError> {
        if !self.path.exists() {
            return Ok(RepoSnapshot::default());
        }
        let bytes = fs::read(&self.path).map_err(|e| PlmError::Storage(e.to_string()))?;
        let snap: RepoSnapshot =
            serde_json::from_slice(&bytes).map_err(|e| PlmError::Storage(e.to_string()))?;
        Ok(snap)
    }

    fn save(&self, snap: &RepoSnapshot) -> Result<(), PlmError> {
        let data =
            serde_json::to_vec_pretty(snap).map_err(|e| PlmError::Storage(e.to_string()))?;
        fs::write(&self.path, data).map_err(|e| PlmError::Storage(e.to_string()))?;
        Ok(())
    }
}

fn ensure_dir(p: &Path) -> Result<(), PlmError> {
    fs::create_dir_all(p).map_err(|e| PlmError::Storage(e.to_string()))?;
    Ok(())
}

