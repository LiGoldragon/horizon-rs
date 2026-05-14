//! Proposal-side filesystem and I/O configuration.
//!
//! The sub-shapes (`MountPath`, `DevicePath`, `Disk`, `FsType`,
//! `SwapDevice`) live in `crate::disk` because they describe the
//! same physical filesystem on both sides.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::disk::{Disk, MountPath, SwapDevice};
use crate::species::{Bootloader, Keyboard};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Io {
    pub keyboard: Keyboard,
    pub bootloader: Bootloader,
    pub disks: BTreeMap<MountPath, Disk>,
    #[serde(default)]
    pub swap_devices: Vec<SwapDevice>,
}
