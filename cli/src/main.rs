//! horizon-cli — read cluster proposal nota on stdin, write
//! enriched horizon JSON (default) or nota on stdout.

use std::io::{Read, Write};
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
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

    /// Output format. JSON is default — Nix consumers read it via
    /// `builtins.fromJSON` (no `builtins.fromNota` exists).
    #[arg(long, value_enum, default_value_t = Format::Json)]
    format: Format,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Format {
    Json,
    Nota,
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

    let proposal: ClusterProposal = {
        let mut decoder = Decoder::nota(&buf);
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

    let out: std::result::Result<String, String> = match cli.format {
        Format::Json => serde_json::to_string_pretty(&horizon).map_err(|e| e.to_string()),
        // Nota emit on output types is currently unwired during the
        // nota-codec migration. Re-enable by deriving NotaRecord on
        // Horizon / Node / User / Cluster / BuilderConfig (and the
        // viewpoint-only fields) once their wire shape is decided.
        Format::Nota => Err("Nota output not implemented in this build (use --format json)".to_string()),
    };
    let out = match out {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: emit horizon: {e}");
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
