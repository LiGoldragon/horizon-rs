#![allow(dead_code)]
use crate::{
    io::FsType,
    magnitude::Magnitude,
    proposal::{
        KvmAvailability, PersonaDevelopmentCapability, SiteRenderer, WlanBand, WlanStandard,
    },
    species::{
        Arch, Bootloader, DomainSpecies, Editor, Keyboard, MachineSpecies, MotherBoard,
        NodeSpecies, Style, System, TextSize, UserSpecies,
    },
};
impl datomic::Datomic for Magnitude {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Zero)) {
            return Ok(Self::Zero);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Min)) {
            return Ok(Self::Min);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Medium)) {
            return Ok(Self::Medium);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Large)) {
            return Ok(Self::Large);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Max)) {
            return Ok(Self::Max);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Zero => datomic::PortionBuilding::bare(stringify!(Zero)),
            Self::Min => datomic::PortionBuilding::bare(stringify!(Min)),
            Self::Medium => datomic::PortionBuilding::bare(stringify!(Medium)),
            Self::Large => datomic::PortionBuilding::bare(stringify!(Large)),
            Self::Max => datomic::PortionBuilding::bare(stringify!(Max)),
        }
    }
}
impl datomic::Datomic for NodeSpecies {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Center)) {
            return Ok(Self::Center);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(LargeAi)) {
            return Ok(Self::LargeAi);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(LargeAiRouter)) {
            return Ok(Self::LargeAiRouter);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Hybrid)) {
            return Ok(Self::Hybrid);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Edge)) {
            return Ok(Self::Edge);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(EdgeTesting)) {
            return Ok(Self::EdgeTesting);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(MediaBroadcast)) {
            return Ok(Self::MediaBroadcast);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Router)) {
            return Ok(Self::Router);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(RouterTesting)) {
            return Ok(Self::RouterTesting);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(TestVm)) {
            return Ok(Self::TestVm);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(CloudNode)) {
            return Ok(Self::CloudNode);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Center => datomic::PortionBuilding::bare(stringify!(Center)),
            Self::LargeAi => datomic::PortionBuilding::bare(stringify!(LargeAi)),
            Self::LargeAiRouter => datomic::PortionBuilding::bare(stringify!(LargeAiRouter)),
            Self::Hybrid => datomic::PortionBuilding::bare(stringify!(Hybrid)),
            Self::Edge => datomic::PortionBuilding::bare(stringify!(Edge)),
            Self::EdgeTesting => datomic::PortionBuilding::bare(stringify!(EdgeTesting)),
            Self::MediaBroadcast => datomic::PortionBuilding::bare(stringify!(MediaBroadcast)),
            Self::Router => datomic::PortionBuilding::bare(stringify!(Router)),
            Self::RouterTesting => datomic::PortionBuilding::bare(stringify!(RouterTesting)),
            Self::TestVm => datomic::PortionBuilding::bare(stringify!(TestVm)),
            Self::CloudNode => datomic::PortionBuilding::bare(stringify!(CloudNode)),
        }
    }
}
impl datomic::Datomic for UserSpecies {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Code)) {
            return Ok(Self::Code);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Multimedia)) {
            return Ok(Self::Multimedia);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Unlimited)) {
            return Ok(Self::Unlimited);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Code => datomic::PortionBuilding::bare(stringify!(Code)),
            Self::Multimedia => datomic::PortionBuilding::bare(stringify!(Multimedia)),
            Self::Unlimited => datomic::PortionBuilding::bare(stringify!(Unlimited)),
        }
    }
}
impl datomic::Datomic for MachineSpecies {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Metal)) {
            return Ok(Self::Metal);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Pod)) {
            return Ok(Self::Pod);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Metal => datomic::PortionBuilding::bare(stringify!(Metal)),
            Self::Pod => datomic::PortionBuilding::bare(stringify!(Pod)),
        }
    }
}
impl datomic::Datomic for Keyboard {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Qwerty)) {
            return Ok(Self::Qwerty);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Colemak)) {
            return Ok(Self::Colemak);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Qwerty => datomic::PortionBuilding::bare(stringify!(Qwerty)),
            Self::Colemak => datomic::PortionBuilding::bare(stringify!(Colemak)),
        }
    }
}
impl datomic::Datomic for Style {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Vim)) {
            return Ok(Self::Vim);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Emacs)) {
            return Ok(Self::Emacs);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Vim => datomic::PortionBuilding::bare(stringify!(Vim)),
            Self::Emacs => datomic::PortionBuilding::bare(stringify!(Emacs)),
        }
    }
}
impl datomic::Datomic for Editor {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Codium)) {
            return Ok(Self::Codium);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Emacs)) {
            return Ok(Self::Emacs);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Codium => datomic::PortionBuilding::bare(stringify!(Codium)),
            Self::Emacs => datomic::PortionBuilding::bare(stringify!(Emacs)),
        }
    }
}
impl datomic::Datomic for TextSize {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(ExtraSmall)) {
            return Ok(Self::ExtraSmall);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Small)) {
            return Ok(Self::Small);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Medium)) {
            return Ok(Self::Medium);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Large)) {
            return Ok(Self::Large);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(ExtraLarge)) {
            return Ok(Self::ExtraLarge);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::ExtraSmall => datomic::PortionBuilding::bare(stringify!(ExtraSmall)),
            Self::Small => datomic::PortionBuilding::bare(stringify!(Small)),
            Self::Medium => datomic::PortionBuilding::bare(stringify!(Medium)),
            Self::Large => datomic::PortionBuilding::bare(stringify!(Large)),
            Self::ExtraLarge => datomic::PortionBuilding::bare(stringify!(ExtraLarge)),
        }
    }
}
impl datomic::Datomic for Bootloader {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Uefi)) {
            return Ok(Self::Uefi);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Mbr)) {
            return Ok(Self::Mbr);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Uboot)) {
            return Ok(Self::Uboot);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Uefi => datomic::PortionBuilding::bare(stringify!(Uefi)),
            Self::Mbr => datomic::PortionBuilding::bare(stringify!(Mbr)),
            Self::Uboot => datomic::PortionBuilding::bare(stringify!(Uboot)),
        }
    }
}
impl datomic::Datomic for Arch {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(X86_64)) {
            return Ok(Self::X86_64);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Arm64)) {
            return Ok(Self::Arm64);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::X86_64 => datomic::PortionBuilding::bare(stringify!(X86_64)),
            Self::Arm64 => datomic::PortionBuilding::bare(stringify!(Arm64)),
        }
    }
}
impl datomic::Datomic for System {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(X86_64Linux)) {
            return Ok(Self::X86_64Linux);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Aarch64Linux)) {
            return Ok(Self::Aarch64Linux);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::X86_64Linux => datomic::PortionBuilding::bare(stringify!(X86_64Linux)),
            Self::Aarch64Linux => datomic::PortionBuilding::bare(stringify!(Aarch64Linux)),
        }
    }
}
impl datomic::Datomic for MotherBoard {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Ondyfaind)) {
            return Ok(Self::Ondyfaind);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Ondyfaind => datomic::PortionBuilding::bare(stringify!(Ondyfaind)),
        }
    }
}
impl datomic::Datomic for DomainSpecies {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Cloudflare)) {
            return Ok(Self::Cloudflare);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Cloudflare => datomic::PortionBuilding::bare(stringify!(Cloudflare)),
        }
    }
}
impl datomic::Datomic for FsType {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Ext2)) {
            return Ok(Self::Ext2);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Ext3)) {
            return Ok(Self::Ext3);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Ext4)) {
            return Ok(Self::Ext4);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Btrfs)) {
            return Ok(Self::Btrfs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Xfs)) {
            return Ok(Self::Xfs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Zfs)) {
            return Ok(Self::Zfs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(F2fs)) {
            return Ok(Self::F2fs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Bcachefs)) {
            return Ok(Self::Bcachefs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Vfat)) {
            return Ok(Self::Vfat);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Exfat)) {
            return Ok(Self::Exfat);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Ntfs)) {
            return Ok(Self::Ntfs);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Tmpfs)) {
            return Ok(Self::Tmpfs);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Ext2 => datomic::PortionBuilding::bare(stringify!(Ext2)),
            Self::Ext3 => datomic::PortionBuilding::bare(stringify!(Ext3)),
            Self::Ext4 => datomic::PortionBuilding::bare(stringify!(Ext4)),
            Self::Btrfs => datomic::PortionBuilding::bare(stringify!(Btrfs)),
            Self::Xfs => datomic::PortionBuilding::bare(stringify!(Xfs)),
            Self::Zfs => datomic::PortionBuilding::bare(stringify!(Zfs)),
            Self::F2fs => datomic::PortionBuilding::bare(stringify!(F2fs)),
            Self::Bcachefs => datomic::PortionBuilding::bare(stringify!(Bcachefs)),
            Self::Vfat => datomic::PortionBuilding::bare(stringify!(Vfat)),
            Self::Exfat => datomic::PortionBuilding::bare(stringify!(Exfat)),
            Self::Ntfs => datomic::PortionBuilding::bare(stringify!(Ntfs)),
            Self::Tmpfs => datomic::PortionBuilding::bare(stringify!(Tmpfs)),
        }
    }
}
impl datomic::Datomic for KvmAvailability {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Available)) {
            return Ok(Self::Available);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Absent)) {
            return Ok(Self::Absent);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Available => datomic::PortionBuilding::bare(stringify!(Available)),
            Self::Absent => datomic::PortionBuilding::bare(stringify!(Absent)),
        }
    }
}
impl datomic::Datomic for SiteRenderer {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(MarkdownStatic)) {
            return Ok(Self::MarkdownStatic);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::MarkdownStatic => datomic::PortionBuilding::bare(stringify!(MarkdownStatic)),
        }
    }
}
impl datomic::Datomic for PersonaDevelopmentCapability {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(GitoliteServer)) {
            return Ok(Self::GitoliteServer);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::GitoliteServer => datomic::PortionBuilding::bare(stringify!(GitoliteServer)),
        }
    }
}
impl datomic::Datomic for WlanBand {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(TwoG)) {
            return Ok(Self::TwoG);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(FiveG)) {
            return Ok(Self::FiveG);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(SixG)) {
            return Ok(Self::SixG);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::TwoG => datomic::PortionBuilding::bare(stringify!(TwoG)),
            Self::FiveG => datomic::PortionBuilding::bare(stringify!(FiveG)),
            Self::SixG => datomic::PortionBuilding::bare(stringify!(SixG)),
        }
    }
}
impl datomic::Datomic for WlanStandard {
    fn embody(portion: &protos::Portion) -> std::result::Result<Self, datomic::Fault> {
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Wifi4)) {
            return Ok(Self::Wifi4);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Wifi6)) {
            return Ok(Self::Wifi6);
        }
        if datomic::PortionViewing::bare_symbol(portion) == Some(stringify!(Wifi7)) {
            return Ok(Self::Wifi7);
        }
        Err(datomic::PortionViewing::fault(
            portion,
            datomic::FaultProblem::Shape,
        ))
    }
    fn portion(&self) -> protos::Portion {
        match self {
            Self::Wifi4 => datomic::PortionBuilding::bare(stringify!(Wifi4)),
            Self::Wifi6 => datomic::PortionBuilding::bare(stringify!(Wifi6)),
            Self::Wifi7 => datomic::PortionBuilding::bare(stringify!(Wifi7)),
        }
    }
}
