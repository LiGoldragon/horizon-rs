//! Closed-set enums for cluster species.
//!
//! Mirrors `mkCrioSphere/speciesModule.nix` from the legacy archive.
//! Variants serialize as their natural Rust spelling (PascalCase) per
//! the nota identifier convention.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
pub enum UserSpecies {
    Code,
    Multimedia,
    Unlimited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MachineSpecies {
    Metal,
    Pod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Keyboard {
    Qwerty,
    Colemak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Style {
    Vim,
    Emacs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bootloader {
    Uefi,
    Mbr,
    Uboot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Arch {
    X86_64,
    Arm64,
}

/// The Nix system tuple. Derived from `Arch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum System {
    X86_64Linux,
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
pub enum MotherBoard {
    Ondyfaind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DomainSpecies {
    Cloudflare,
}
