//! Filesystem and I/O configuration.

use std::collections::BTreeMap;

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
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
    /// Operator-authored compressed swap policy for this node. CriomOS
    /// currently renders this through Linux zram.
    #[serde(default)]
    pub compressed_swap: Option<CompressedSwap>,
}

/// A filesystem mount point.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    NotaDecode,
    NotaEncode,
)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
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
    /// Swapfile size in mebibytes. `None` means the path already
    /// names an existing swap partition or pre-sized swap file.
    #[serde(default)]
    pub size_mebibytes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct CompressedSwap {
    /// Percent of physical memory made available as compressed swap.
    pub memory_percent: u32,
}

impl NotaEncode for Io {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([
            self.keyboard.to_nota(),
            self.bootloader.to_nota(),
            self.disks.to_nota(),
            self.swap_devices.to_nota(),
            self.compressed_swap.to_nota(),
        ])
    }
}

impl NotaDecode for Io {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields = NotaBlock::new(block).expect_delimited(Delimiter::Parenthesis, "Io")?;
        if !(4..=5).contains(&fields.len()) {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name: "Io",
                expected: 5,
                found: fields.len(),
            });
        }
        let keyboard = Keyboard::from_nota_block(&fields[0])?;
        let bootloader = Bootloader::from_nota_block(&fields[1])?;
        let disks = BTreeMap::<MountPath, Disk>::from_nota_block(&fields[2])?;
        let swap_devices = Vec::<SwapDevice>::from_nota_block(&fields[3])?;
        let compressed_swap = match fields.get(4) {
            Some(field) => Option::<CompressedSwap>::from_nota_block(field)?,
            None => None,
        };

        Ok(Self {
            keyboard,
            bootloader,
            disks,
            swap_devices,
            compressed_swap,
        })
    }
}

impl NotaEncode for SwapDevice {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([self.device.to_nota(), self.size_mebibytes.to_nota()])
    }
}

impl NotaDecode for SwapDevice {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_delimited(Delimiter::Parenthesis, "SwapDevice")?;
        if !(1..=2).contains(&fields.len()) {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name: "SwapDevice",
                expected: 2,
                found: fields.len(),
            });
        }
        let device = DevicePath::from_nota_block(&fields[0])?;
        let size_mebibytes = match fields.get(1) {
            Some(field) => Option::<u32>::from_nota_block(field)?,
            None => None,
        };

        Ok(Self {
            device,
            size_mebibytes,
        })
    }
}

/// Filesystem type. Closed set of NixOS-supported filesystems we
/// realistically use as a root, boot, or data filesystem. Add a
/// variant when a new one shows up in real config.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
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
