//! horizon-cli — read cluster proposal nota on stdin, write
//! enriched horizon nota on stdout.

use std::io::{Read, Write};
use std::process::ExitCode;

use clap::Parser;
use horizon_lib::{ClusterProposal, Viewpoint};
use horizon_lib::name::{ClusterName, NodeName};

#[derive(Parser)]
#[command(
    name = "horizon-cli",
    about = "Project a cluster proposal into the enriched horizon for one viewpoint node"
)]
struct Cli {
    /// Cluster name (matches the proposal's cluster identity).
    #[arg(long)]
    cluster: String,

    /// Viewpoint node name (must exist in the proposal).
    #[arg(long)]
    node: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let viewpoint = match build_viewpoint(&cli) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("error: {msg}");
            return ExitCode::from(2);
        }
    };

    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
        eprintln!("error: read stdin: {e}");
        return ExitCode::from(2);
    }

    let proposal: ClusterProposal = match nota_serde::from_str(&buf) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: parse cluster proposal: {e}");
            return ExitCode::from(1);
        }
    };

    let horizon = match proposal.project(&viewpoint) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: project: {e}");
            return ExitCode::from(1);
        }
    };

    let out = match nota_serde::to_string(&horizon) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: emit horizon nota: {e}");
            return ExitCode::from(1);
        }
    };

    if let Err(e) = std::io::stdout().write_all(out.as_bytes()) {
        eprintln!("error: write stdout: {e}");
        return ExitCode::from(2);
    }
    let _ = std::io::stdout().write_all(b"\n");

    ExitCode::SUCCESS
}

fn build_viewpoint(cli: &Cli) -> Result<Viewpoint, String> {
    let cluster = ClusterName::try_new(&cli.cluster).map_err(|e| e.to_string())?;
    let node = NodeName::try_new(&cli.node).map_err(|e| e.to_string())?;
    Ok(Viewpoint { cluster, node })
}
