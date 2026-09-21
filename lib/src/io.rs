//! Filesystem and I/O configuration.

use std::collections::BTreeMap;

use datomic::{Datomic, DatomicString, Fault, FaultProblem, PortionBuilding, PortionViewing};
use protos::{Portion, StructuralEnclosure};
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Swapfile size in mebibytes. `None` means the path already
    /// names an existing swap partition or pre-sized swap file.
    #[serde(default)]
    pub size_mebibytes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressedSwap {
    /// Percent of physical memory made available as compressed swap.
    pub memory_percent: u32,
}

/// Filesystem type. Closed set of NixOS-supported filesystems we
/// realistically use as a root, boot, or data filesystem. Add a
/// variant when a new one shows up in real config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Datomic for MountPath {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        datomic_string_portion(&self.0, "mount path")
    }
}

impl Datomic for DevicePath {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        datomic_string_portion(&self.0, "device path")
    }
}

impl Datomic for Disk {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [device, fs_type, options] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            device: DevicePath::embody(device)?,
            fs_type: FsType::embody(fs_type)?,
            options: strings_from_portion(options)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.device.portion(),
                self.fs_type.portion(),
                strings_portion(&self.options, "disk option"),
            ],
        )
    }
}

impl Datomic for SwapDevice {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [device, size_mebibytes] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            device: DevicePath::embody(device)?,
            size_mebibytes: optional_u32_from_portion(size_mebibytes)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.device.portion(),
                optional_u32_portion(self.size_mebibytes),
            ],
        )
    }
}

impl Datomic for CompressedSwap {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [memory_percent] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            memory_percent: u32_from_portion(memory_percent)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![i64::from(self.memory_percent).portion()],
        )
    }
}

impl Datomic for Io {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [keyboard, bootloader, disks, swap_devices, compressed_swap] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            keyboard: Keyboard::embody(keyboard)?,
            bootloader: Bootloader::embody(bootloader)?,
            disks: BTreeMap::<MountPath, Disk>::embody(disks)?,
            swap_devices: Vec::<SwapDevice>::embody(swap_devices)?,
            compressed_swap: Option::<CompressedSwap>::embody(compressed_swap)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.keyboard.portion(),
                self.bootloader.portion(),
                self.disks.portion(),
                self.swap_devices.portion(),
                self.compressed_swap.portion(),
            ],
        )
    }
}

fn u32_from_portion(portion: &Portion) -> std::result::Result<u32, Fault> {
    u32::try_from(i64::embody(portion)?).map_err(|_| portion.fault(FaultProblem::Value))
}

fn optional_u32_from_portion(portion: &Portion) -> std::result::Result<Option<u32>, Fault> {
    Option::<i64>::embody(portion)?
        .map(|value| u32::try_from(value).map_err(|_| portion.fault(FaultProblem::Value)))
        .transpose()
}

fn optional_u32_portion(value: Option<u32>) -> Portion {
    value.map(i64::from).portion()
}

fn strings_from_portion(portion: &Portion) -> std::result::Result<Vec<String>, Fault> {
    Ok(Vec::<DatomicString>::embody(portion)?
        .into_iter()
        .map(|value| value.as_ref().to_owned())
        .collect())
}

fn strings_portion(values: &[String], kind: &str) -> Portion {
    PortionBuilding::structural(
        "",
        StructuralEnclosure::Bracketed,
        values
            .iter()
            .map(|value| datomic_string_portion(value, kind))
            .collect(),
    )
}

fn datomic_string_portion(value: &str, kind: &str) -> Portion {
    let value = DatomicString::try_from(value.to_owned())
        .unwrap_or_else(|_| panic!("{kind} must be Datomic-representable"));
    Datomic::portion(&value)
}
