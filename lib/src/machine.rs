//! Hardware description.

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::name::{ModelName, NodeName, UserName};
use crate::species::{Arch, MachineSpecies, MotherBoard};

/// Per-node hardware description. `arch` is `Option` because pod
/// (virtual) machines defer it to their super-node; resolution into
/// a concrete arch happens at projection time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
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
    /// Intel iGPU graphics generation (e.g. 8 = Broadwell, 11 = Skylake,
    /// 12 = Tiger Lake / Alder Lake / Meteor Lake Xe-LPG, 13 = Lunar
    /// Lake Xe2). Gates the modern Intel media stack: `>= 12` enables
    /// `vpl-gpu-rt` for AV1/HEVC HW decode. None for non-Intel or
    /// unknown — modules fall back to the safe default driver.
    /// MUST stay near the end of the struct so positional nota records
    /// keep parsing with implicit None defaults.
    #[serde(default)]
    pub chip_gen: Option<u32>,
    /// Total system RAM in gibibytes (rounded). Gates downstream
    /// resource policies: `nix.settings.maxJobs` thresholds,
    /// llama.cpp model size, language-server heap. Optional — None
    /// means the operator hasn't filled it in yet.
    #[serde(default)]
    pub ram_gb: Option<u32>,
}
