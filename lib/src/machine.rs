//! Hardware description.

use serde::{Deserialize, Serialize};

use crate::name::{ModelName, NodeName, UserName};
use crate::species::{Arch, MachineSpecies, MotherBoard};

/// Per-node hardware description. `arch` is `Option` because pod
/// (virtual) machines defer it to their super-node; resolution into
/// a concrete arch happens at projection time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Machine {
    pub species: MachineSpecies,
    pub arch: Option<Arch>,
    pub cores: u32,
    pub model: Option<ModelName>,
    pub mother_board: Option<MotherBoard>,
    /// Pod-only: which node hosts this pod.
    pub super_node: Option<NodeName>,
    /// Pod-only: which user runs this pod.
    pub super_user: Option<UserName>,
}

/// Number of build cores (`maxJobs` for nix builders). Always ≥ 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildCores(u32);

impl BuildCores {
    pub fn new(n: u32) -> Self {
        Self(n.max(1))
    }

    pub fn get(self) -> u32 {
        self.0
    }
}
