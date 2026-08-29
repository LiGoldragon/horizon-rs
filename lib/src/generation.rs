//! Hand-owned regeneration boundary for the committed Horizon Ethos source.
//!
//! The contract is parsed and emitted by Ethos-zero; this module deliberately
//! owns only filesystem placement and rustfmt invocation.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use ethos_zero::{FileLocation, FileReader, Manifest, RustEmitter};
use quote::ToTokens;

const HORIZON_ETHOS: &str = include_str!("../../ethos/horizon.ethos");

struct EmptyManifest;

impl Manifest for EmptyManifest {
    fn resolve(&self, _: &str) -> Option<FileLocation> {
        None
    }
}

/// Emit only the generated Datomic anatomy for Horizon's hand-owned types.
///
/// Ethos-zero's D3 consumer emits public declarations and anatomy together.
/// Horizon owns its public declarations because they carry validation and
/// serde semantics; this structural `syn` projection retains only the emitted
/// `impl` items. No generated text is sliced or reparsed by a second grammar.
pub fn emitted_horizon() -> Result<String, String> {
    let reader = FileReader::new(&EmptyManifest);
    let file = reader
        .read(HORIZON_ETHOS)
        .map_err(|fault| fault.to_string())?;
    let emitted = RustEmitter::new()
        .emit(&file)
        .map_err(|fault| fault.to_string())?;
    let mut syntax = syn::parse_file(&emitted).map_err(|error| error.to_string())?;
    syntax
        .items
        .retain(|item| matches!(item, syn::Item::Impl(_)));
    syntax.items.insert(
        0,
        syn::parse_str(
            "use crate::{io::FsType, magnitude::Magnitude, proposal::{KvmAvailability, PersonaDevelopmentCapability, SiteRenderer, WlanBand, WlanStandard}, species::{Arch, Bootloader, DomainSpecies, Editor, Keyboard, MachineSpecies, MotherBoard, NodeSpecies, Style, System, TextSize, UserSpecies}};",
        )
        .map_err(|error| error.to_string())?,
    );
    Ok(syntax.into_token_stream().to_string())
}

/// Write exactly the rustfmt-normalized generated contract at `destination`.
pub fn regenerate_at(destination: &Path) -> Result<(), String> {
    fs::write(destination, emitted_horizon()?).map_err(|error| error.to_string())?;
    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(destination)
        .status()
        .map_err(|error| error.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("rustfmt failed for {}", destination.display()))
    }
}

/// Regenerate the committed source file from the authored Ethos contract.
pub fn regenerate_committed() -> Result<(), String> {
    regenerate_at(&generated_path())
}

pub fn generated_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generated/d3.rs")
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{generated_path, regenerate_at};

    #[test]
    fn authored_ethos_regenerates_the_committed_contract_byte_for_byte() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let temporary = std::env::temp_dir().join(format!("horizon-regenerate-{unique}.rs"));
        regenerate_at(&temporary).expect("generated contract");
        assert_eq!(
            fs::read(&temporary).expect("temporary generated contract"),
            fs::read(generated_path()).expect("committed generated contract"),
        );
        fs::remove_file(temporary).expect("remove temporary generated contract");
    }
}
