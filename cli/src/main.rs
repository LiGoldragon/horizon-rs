//! horizon-cli — read cluster proposal nota on stdin, write
//! enriched horizon JSON on stdout.

use std::io::{Read, Write};
use std::process::ExitCode;

use clap::Parser;
use horizon_lib::name::{ClusterName, NodeName};
use horizon_lib::{ClusterProposal, Viewpoint};
use nota_codec::{Decoder, NotaDecode};

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

    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
        eprintln!("error: read stdin: {e}");
        return ExitCode::from(2);
    }

    let proposal: ClusterProposal = {
        let mut decoder = Decoder::new(&buf);
        match ClusterProposal::decode(&mut decoder) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: parse cluster proposal: {e}");
                return ExitCode::from(1);
            }
        }
    };

    let horizon = match proposal.project(&viewpoint) {
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
