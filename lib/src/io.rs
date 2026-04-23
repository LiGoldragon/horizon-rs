//! Filesystem and I/O configuration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

/// A filesystem mount point.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MountPath(String);

impl MountPath {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MountPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A device path (e.g. `/dev/disk/by-uuid/abcd-…`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DevicePath(String);

impl DevicePath {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub device: DevicePath,
    pub fs_type: FsType,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDevice {
    pub device: DevicePath,
}

/// Filesystem type. Closed set of NixOS-supported filesystems we
/// realistically use as a root, boot, or data filesystem. Add a
/// variant when a new one shows up in real config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FsType {
    Ext2,
    Ext3,
    Ext4,
    Btrfs,
    Xfs,
    Zfs,
    F2fs,
    Bcachefs,
    Vfat,
    Exfat,
    Ntfs,
    Tmpfs,
}
