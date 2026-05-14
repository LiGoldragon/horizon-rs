//! View-side filesystem and I/O configuration.
//!
//! Shape-equivalent to `proposal::Io` today; the sub-shapes (`MountPath`,
//! `Disk`, `FsType`, `SwapDevice`) live in `crate::disk` and pass through
//! unchanged.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::disk::{Disk, MountPath, SwapDevice};
use crate::proposal;
use crate::species::{Bootloader, Keyboard};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Io {
    pub keyboard: Keyboard,
    pub bootloader: Bootloader,
    pub disks: BTreeMap<MountPath, Disk>,
    #[serde(default)]
    pub swap_devices: Vec<SwapDevice>,
}

impl From<proposal::Io> for Io {
    fn from(proposal: proposal::Io) -> Self {
        Self {
            keyboard: proposal.keyboard,
            bootloader: proposal.bootloader,
            disks: proposal.disks,
            swap_devices: proposal.swap_devices,
        }
    }
}
