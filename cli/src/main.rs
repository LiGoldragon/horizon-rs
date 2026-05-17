//! horizon-cli — read pan-horizon and cluster proposal nota, write
//! enriched horizon JSON on stdout.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use horizon_lib::name::{ClusterName, NodeName};
use horizon_lib::{ClusterProposal, HorizonProposal, Viewpoint};
use nota_codec::{Decoder, NotaDecode};

#[derive(Parser)]
#[command(
    name = "horizon-cli",
    about = "Project a cluster proposal into the enriched horizon for one viewpoint node"
)]
struct Cli {
    /// Pan-horizon configuration nota file.
    #[arg(long)]
    horizon: PathBuf,

    /// Cluster proposal nota file.
    #[arg(long)]
    proposal: PathBuf,

    /// Cluster name (matches the proposal's cluster identity).
    #[arg(long)]
    cluster: String,

    /// Viewpoint node name (must exist in the proposal).
    #[arg(long)]
    node: String,
}

impl Cli {
    fn viewpoint(&self) -> Result<Viewpoint, String> {
        let cluster = ClusterName::try_new(&self.cluster).map_err(|e| e.to_string())?;
        let node = NodeName::try_new(&self.node).map_err(|e| e.to_string())?;
        Ok(Viewpoint { cluster, node })
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let viewpoint = match cli.viewpoint() {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("error: {msg}");
            return ExitCode::from(2);
        }
    };

    let horizon_text = match std::fs::read_to_string(&cli.horizon) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("error: read horizon config {}: {e}", cli.horizon.display());
            return ExitCode::from(2);
        }
    };
    let proposal_text = match std::fs::read_to_string(&cli.proposal) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("error: read cluster proposal {}: {e}", cli.proposal.display());
            return ExitCode::from(2);
        }
    };

    let horizon_proposal: HorizonProposal = {
        let mut decoder = Decoder::new(&horizon_text);
        match HorizonProposal::decode(&mut decoder) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: parse horizon config: {e}");
                return ExitCode::from(1);
            }
        }
    };

    let proposal: ClusterProposal = {
        let mut decoder = Decoder::new(&proposal_text);
        match ClusterProposal::decode(&mut decoder) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: parse cluster proposal: {e}");
                return ExitCode::from(1);
            }
        }
    };

    let horizon = match proposal.project(&horizon_proposal, &viewpoint) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: project: {e}");
            return ExitCode::from(1);
        }
    };

    let json = match serde_json::to_string_pretty(&horizon) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: emit horizon: {e}");
            return ExitCode::from(1);
        }
    };

    if let Err(e) = std::io::stdout().write_all(json.as_bytes()) {
        eprintln!("error: write stdout: {e}");
        return ExitCode::from(2);
    }
    let _ = std::io::stdout().write_all(b"\n");

    ExitCode::SUCCESS
}
