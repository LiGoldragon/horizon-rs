//! Filesystem and I/O configuration.

use std::collections::BTreeMap;

use nota_codec::{NotaDecode, NotaEncode, NotaEnum, NotaMapKey, NotaRecord, NotaTransparent};
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
    NotaMapKey,
    NotaTransparent,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDevice {
    pub device: DevicePath,
    /// Swapfile size in mebibytes. `None` means the path already
    /// names an existing swap partition or pre-sized swap file.
    #[serde(default)]
    pub size_mebibytes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct CompressedSwap {
    /// Percent of physical memory made available as compressed swap.
    pub memory_percent: u32,
}

impl NotaEncode for Io {
    fn encode(&self, encoder: &mut nota_codec::Encoder) -> nota_codec::Result<()> {
        encoder.start_record_untagged()?;
        self.keyboard.encode(encoder)?;
        self.bootloader.encode(encoder)?;
        self.disks.encode(encoder)?;
        self.swap_devices.encode(encoder)?;
        self.compressed_swap.encode(encoder)?;
        encoder.end_record()
    }
}

impl NotaDecode for Io {
    fn decode(decoder: &mut nota_codec::Decoder<'_>) -> nota_codec::Result<Self> {
        decoder.expect_positional_record_start("Io", 5)?;
        let keyboard = Keyboard::decode(decoder)?;
        let bootloader = Bootloader::decode(decoder)?;
        let disks = BTreeMap::<MountPath, Disk>::decode(decoder)?;
        let swap_devices = Vec::<SwapDevice>::decode(decoder)?;
        let compressed_swap = if decoder.peek_is_record_end()? {
            None
        } else {
            Option::<CompressedSwap>::decode(decoder)?
        };
        decoder.expect_record_end()?;

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
    fn encode(&self, encoder: &mut nota_codec::Encoder) -> nota_codec::Result<()> {
        encoder.start_record_untagged()?;
        self.device.encode(encoder)?;
        self.size_mebibytes.encode(encoder)?;
        encoder.end_record()
    }
}

impl NotaDecode for SwapDevice {
    fn decode(decoder: &mut nota_codec::Decoder<'_>) -> nota_codec::Result<Self> {
        decoder.expect_positional_record_start("SwapDevice", 2)?;
        let device = DevicePath::decode(decoder)?;
        let size_mebibytes = if decoder.peek_is_record_end()? {
            None
        } else {
            Option::<u32>::decode(decoder)?
        };
        decoder.expect_record_end()?;

        Ok(Self {
            device,
            size_mebibytes,
        })
    }
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
