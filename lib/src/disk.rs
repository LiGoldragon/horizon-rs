//! Filesystem and storage types shared between proposal and view.
//!
//! `MountPath`, `DevicePath`, `Disk`, `FsType`, and `SwapDevice` describe
//! the same physical filesystem on both sides; the input and output
//! shapes do not diverge for these. Kept as one module so both
//! `proposal::Io` and `view::Io` can reference the same canonical types.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use serde::{Deserialize, Serialize};

/// A filesystem mount point.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct MountPath(pub(crate) String);

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct DevicePath(pub(crate) String);

impl DevicePath {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
    pub device: DevicePath,
    pub fs_type: FsType,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct SwapDevice {
    pub device: DevicePath,
}

/// Filesystem type. Closed set of NixOS-supported filesystems we
/// realistically use as a root, boot, or data filesystem. Add a
/// variant when a new one shows up in real config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
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
