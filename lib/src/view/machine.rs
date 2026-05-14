//! View-side hardware description.
//!
//! Shape-equivalent to `proposal::Machine` today; will diverge as the
//! arc lands data-bearing variants in later steps. `arch` is `Option`
//! on the proposal side (pods defer to their super-node) but is filled
//! in here by the projection.

use serde::{Deserialize, Serialize};

use crate::name::{ModelName, NodeName, UserName};
use crate::proposal;
use crate::species::{Arch, MachineSpecies, MotherBoard};

/// Per-node hardware description in the projected view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub species: MachineSpecies,
    pub arch: Option<Arch>,
    pub cores: u32,
    pub model: Option<ModelName>,
    pub mother_board: Option<MotherBoard>,
    pub super_node: Option<NodeName>,
    pub super_user: Option<UserName>,
    #[serde(default)]
    pub chip_gen: Option<u32>,
    #[serde(default)]
    pub ram_gb: Option<u32>,
}

impl From<proposal::Machine> for Machine {
    fn from(proposal: proposal::Machine) -> Self {
        Self {
            species: proposal.species,
            arch: proposal.arch,
            cores: proposal.cores,
            model: proposal.model,
            mother_board: proposal.mother_board,
            super_node: proposal.super_node,
            super_user: proposal.super_user,
            chip_gen: proposal.chip_gen,
            ram_gb: proposal.ram_gb,
        }
    }
}
