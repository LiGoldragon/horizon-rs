//! Closed-set enums for cluster species.
//!
//! Mirrors `mkCrioSphere/speciesModule.nix` from the legacy CriomOS
//! archive, modernized to kebab-case throughout.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeSpecies {
    Center,
    LargeAi,
    LargeAiRouter,
    Hybrid,
    Edge,
    EdgeTesting,
    MediaBroadcast,
    Router,
    RouterTesting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserSpecies {
    Code,
    Multimedia,
    Unlimited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MachineSpecies {
    Metal,
    Pod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Keyboard {
    Qwerty,
    Colemak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    Vim,
    Emacs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Bootloader {
    Uefi,
    Mbr,
    Uboot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Arch {
    #[serde(rename = "x86-64")]
    X86_64,
    #[serde(rename = "arm64")]
    Arm64,
}

/// The Nix system tuple. Derived from `Arch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum System {
    #[serde(rename = "x86_64-linux")]
    X86_64Linux,
    #[serde(rename = "aarch64-linux")]
    Aarch64Linux,
}

impl Arch {
    pub fn system(self) -> System {
        match self {
            Arch::X86_64 => System::X86_64Linux,
            Arch::Arm64 => System::Aarch64Linux,
        }
    }

    pub fn is_intel(self) -> bool {
        matches!(self, Arch::X86_64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MotherBoard {
    Ondyfaind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DomainSpecies {
    Cloudflare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkLocalSpecies {
    Ethernet,
    Wifi,
}
