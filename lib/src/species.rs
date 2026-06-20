//! Closed-set enums for cluster species.
//!
//! Mirrors `mkCrioSphere/speciesModule.nix` from the legacy archive.
//! Variants serialize as their natural Rust spelling (PascalCase) per
//! the nota identifier convention.

use nota_next::{NotaDecode, NotaEncode};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
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
    /// On-demand test virtual machine — a first-class cluster role,
    /// distinct from `EdgeTesting` (a next-gen edge desktop). A
    /// `TestVm` node derives a deliberately minimal profile: it is a
    /// virtual machine (its substrate is `MachineSpecies::Pod`) but is
    /// NOT an edge, center, or router node. The host the VM runs on is
    /// `Machine::super_node`; the guest is launched to run a test and
    /// stopped after.
    TestVm,

    /// A cloud provider node (DigitalOcean, etc.) — a first-class cluster
    /// role whose CriomOS config renders a NEW minimal, content-sized cloud
    /// image built declaratively from the projection, NOT a snapshot of a
    /// converted droplet. Like `TestVm` it derives a deliberately lean
    /// profile: it derives the `behaves_as.cloud_node` facet and none of the
    /// role facets, so edge/center/router/large_ai all stay false because
    /// none of those facets' species-unions include `CloudNode`. Unlike `TestVm`
    /// it is NOT a `Pod` guest — it is the bare machine it boots on
    /// (`MachineSpecies::Metal`), has no `super_node`, and so derives
    /// `virtual_machine` false. The cloud image's bootloader follows
    /// `io.bootloader` (`Bootloader::Mbr` for DigitalOcean BIOS/GRUB);
    /// cloud-init network/ssh injection and growpart are emitted by the
    /// CriomOS cloud-image module gated on `behaves_as.cloud_node`.
    CloudNode,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum UserSpecies {
    Code,
    Multimedia,
    Unlimited,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum MachineSpecies {
    Metal,
    Pod,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum Keyboard {
    Qwerty,
    Colemak,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum Style {
    Vim,
    Emacs,
}

/// User's preferred top-level editor application. Distinct from
/// [`Style`] (modal-keystroke style — Vim or Emacs bindings can be
/// selected on top of either editor). When absent on a `UserProposal`,
/// the projection picks `Emacs` for code developers and `Codium`
/// otherwise.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum Editor {
    Codium,
    Emacs,
}

/// User's preferred relative text and UI size — covers terminal font
/// size, editor font size, and editor UI zoom. A user setting; later
/// composed with hardware DPI to compute actual pixel values.
/// Default is `Medium`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum TextSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum Bootloader {
    Uefi,
    Mbr,
    Uboot,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum Arch {
    X86_64,
    Arm64,
}

/// The Nix system tuple. Derived from `Arch`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum MotherBoard {
    Ondyfaind,
}

/// Closed set of computer models the projection recognises by
/// `ModelName` string. Drives `ComputerIs` flags and
/// `model_is_thinkpad` on `Node`. Add a variant when a new
/// model needs a config branch.
///
/// Not on the wire — `ModelName` stays an open string in the
/// proposal. This is the parsed form used internally by
/// `ModelName::known()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownModel {
    ThinkPadX230,
    ThinkPadX240,
    ThinkPadT14Gen2Intel,
    ThinkPadT14Gen5Intel,
    Rpi3B,
}

impl KnownModel {
    pub fn is_thinkpad(self) -> bool {
        matches!(
            self,
            Self::ThinkPadX230
                | Self::ThinkPadX240
                | Self::ThinkPadT14Gen2Intel
                | Self::ThinkPadT14Gen5Intel
        )
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode,
)]
pub enum DomainSpecies {
    Cloudflare,
}
