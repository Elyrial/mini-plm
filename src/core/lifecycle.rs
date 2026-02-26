use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
pub enum Lifecycle {
    Design,
    Prototype,
    Production,
    Obsolete,
}

impl Lifecycle {
    pub fn can_transition(from: &Lifecycle, to: &Lifecycle) -> bool {
        use Lifecycle::*;
        matches!(
            (from, to),
            (Design, Prototype)
                | (Prototype, Production)
                | (Production, Obsolete)
        )
    }
}

