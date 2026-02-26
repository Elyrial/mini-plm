use serde::{Deserialize, Serialize};
use crate::core::{Part, Change, PlmError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepoSnapshot {
    pub parts: Vec<Part>,
    pub changes: Vec<Change>,
}

impl Default for RepoSnapshot {
    fn default() -> Self {
        Self {
            parts: vec![],
            changes: vec![],
        }
    }
}

pub trait Repo {
    fn load(&self) -> Result<RepoSnapshot, PlmError>;
    fn save(&self, snap: &RepoSnapshot) -> Result<(), PlmError>;
}

