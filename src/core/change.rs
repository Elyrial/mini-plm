use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::core::Lifecycle;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    pub part_number: String,
    pub eco: String,
    pub from: Lifecycle,
    pub to: Lifecycle,
    pub reason: String,
    pub at_utc: DateTime<Utc>,
}

