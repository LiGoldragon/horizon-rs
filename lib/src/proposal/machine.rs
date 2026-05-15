//! Proposal-side hardware description.
//!
//! `arch` is `Option` because contained nodes defer it to their host
//! (resolved at projection time via `NodePlacement::Contained.host`).
//! The view-side `Machine` carries the resolved arch.

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::name::ModelName;
use crate::species::{Arch, MotherBoard};

/// Per-node hardware description as authored in the proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub arch: Option<Arch>,
    pub cores: u32,
    pub model: Option<ModelName>,
    pub mother_board: Option<MotherBoard>,
    /// Intel iGPU graphics generation. `>= 12` enables `vpl-gpu-rt`
    /// for AV1/HEVC HW decode. None for non-Intel or unknown.
    #[serde(default)]
    pub chip_gen: Option<u32>,
    /// Total system RAM in gibibytes. None when not yet filled in.
    #[serde(default)]
    pub ram_gb: Option<u32>,
}
