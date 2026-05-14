//! Domain-record proposal — the per-domain shape goldragon emits.

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::species::DomainSpecies;

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct DomainProposal {
    pub species: DomainSpecies,
}
