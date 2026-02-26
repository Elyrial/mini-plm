mod cli;
mod core;
mod infra;

use clap::Parser;
use cli::{Cli, Commands};
use core::{Lifecycle, PlmError};
use infra::JsonRepo;

fn main() -> Result<(), PlmError> {
    let cli = Cli::parse();

    // All data is stored in ./data/plm.json
    let repo = JsonRepo::new("data/plm.json")?;

    match cli.command {
        Commands::Create { part } => {
            core::app_create_part(&repo, part)?;
        }
        Commands::Promote {
            part,
            to,
            eco,
            reason,
        } => {
            core::app_promote_part(&repo, part, to, eco, reason)?;
        }
        Commands::Show { part } => {
            let p = core::app_get_part(&repo, part)?;
            println!("{}", serde_json::to_string_pretty(&p).unwrap());
        }
        Commands::List => {
            let parts = core::app_list_parts(&repo)?;
            for p in parts {
                println!("{}  {:?}", p.number, p.lifecycle);
            }
        }
        Commands::History { part } => {
            let hist = core::app_history(&repo, part)?;
            println!("{}", serde_json::to_string_pretty(&hist).unwrap());
        }
    }

    Ok(())
}

