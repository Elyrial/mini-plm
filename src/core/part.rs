use serde::{Deserialize, Serialize};
use crate::core::{Lifecycle, PlmError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Part {
    pub number: String,
    pub lifecycle: Lifecycle,

    // Internal helper for audit trail
    #[serde(skip)]
    pub(crate) last_from: Option<Lifecycle>,
}

impl Part {
    pub fn new(number: String) -> Self {
        Self {
            number,
            lifecycle: Lifecycle::Design,
            last_from: None,
        }
    }

    pub fn promote(&mut self, to: Lifecycle) -> Result<(), PlmError> {
        let from = self.lifecycle.clone();

        if from == to {
            return Err(PlmError::InvalidTransition {
                from,
                to,
                msg: "already in that lifecycle".to_string(),
            });
        }

        if !Lifecycle::can_transition(&from, &to) {
            return Err(PlmError::InvalidTransition {
                from,
                to,
                msg: "transition not allowed".to_string(),
            });
        }

        self.last_from = Some(from);
        self.lifecycle = to;
        Ok(())
    }
}

