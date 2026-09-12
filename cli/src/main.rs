//! Horizon's one-document projection CLI.

use std::io::{Read, Write};
use std::process::ExitCode;

use clap::Parser;
use horizon_lib::{DatomDecoding, HorizonDefinition, Projecting};

#[derive(Parser)]
#[command(
    name = "horizon-cli",
    about = "Resolve and project a Horizon definition"
)]
struct Cli {
    #[arg(long)]
    node: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut text = String::new();
    if let Err(error) = std::io::stdin().read_to_string(&mut text) {
        eprintln!("error: read stdin: {error}");
        return ExitCode::from(2);
    }
    let definition = match HorizonDefinition::decode(&text) {
        Ok(definition) => definition,
        Err(error) => {
            eprintln!("error: parse horizon definition: {error}");
            return ExitCode::from(1);
        }
    };
    let horizon = match definition.project(&cli.node) {
        Ok(horizon) => horizon,
        Err(error) => {
            eprintln!("error: resolve horizon definition: {error}");
            return ExitCode::from(1);
        }
    };
    match serde_json::to_string_pretty(&horizon) {
        Ok(json) => {
            if let Err(error) = std::io::stdout().write_all(format!("{json}\n").as_bytes()) {
                eprintln!("error: write stdout: {error}");
                return ExitCode::from(2);
            }
        }
        Err(error) => {
            eprintln!("error: serialize horizon: {error}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
