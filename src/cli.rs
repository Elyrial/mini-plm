use clap::{Parser, Subcommand};
use crate::core::Lifecycle;

#[derive(Parser, Debug)]
#[command(name = "mini-plm")]
#[command(version, about = "Tiny PLM: parts, lifecycle, ECO-based audit trail")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new part in Design lifecycle
    Create {
        /// Part number, e.g. P-1001
        part: String,
    },

    /// Promote lifecycle with an ECO and reason (audit trail)
    Promote {
        /// Part number, e.g. P-1001
        part: String,

        /// Target lifecycle
        #[arg(value_enum)]
        to: Lifecycle,

        /// ECO identifier, e.g. ECO-001
        #[arg(long)]
        eco: String,

        /// Reason for change
        #[arg(long)]
        reason: String,
    },

    /// Record a change order (ECO) without lifecycle promotion
    ChangeOrder {
        /// Part number, e.g. P-1001
        part: String,

        /// ECO identifier, e.g. ECO-002
        #[arg(long)]
        eco: String,

        /// Reason for change
        #[arg(long)]
        reason: String,
    },

    /// Show a part as JSON
    Show {
        /// Part number
        part: String,
    },

    /// List all parts
    List,

    /// Show change history for a part
    History {
        /// Part number
        part: String,
    },

    /// Start the web UI server
    Serve {
        /// Port to listen on
        #[arg(long, short, default_value_t = 3000)]
        port: u16,
    },
}

