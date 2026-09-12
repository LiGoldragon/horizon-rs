//! The textual name a projected enumeration carries, and magnitude ordering.

use crate::*;

/// The one name an authored enumeration is known by in a projected Horizon.
pub(crate) trait Named {
    fn name(&self) -> &'static str;
}

impl Named for Architecture {
    fn name(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86_64",
            Architecture::Arm64 => "aarch64",
        }
    }
}

impl Named for Magnitude {
    fn name(&self) -> &'static str {
        match self {
            Magnitude::Zero => "Zero",
            Magnitude::Min => "Min",
            Magnitude::Medium => "Medium",
            Magnitude::Large => "Large",
            Magnitude::Max => "Max",
        }
    }
}

impl Named for Keyboard {
    fn name(&self) -> &'static str {
        match self {
            Keyboard::Qwerty => "Qwerty",
            Keyboard::Colemak => "Colemak",
        }
    }
}

impl Named for Style {
    fn name(&self) -> &'static str {
        match self {
            Style::Vim => "Vim",
            Style::Emacs => "Emacs",
        }
    }
}

impl Named for Editor {
    fn name(&self) -> &'static str {
        match self {
            Editor::Codium => "Codium",
            Editor::Emacs => "Emacs",
        }
    }
}

impl Named for TextSize {
    fn name(&self) -> &'static str {
        match self {
            TextSize::ExtraSmall => "ExtraSmall",
            TextSize::Small => "Small",
            TextSize::Medium => "Medium",
            TextSize::Large => "Large",
            TextSize::ExtraLarge => "ExtraLarge",
        }
    }
}

impl Named for Bootloader {
    fn name(&self) -> &'static str {
        match self {
            Bootloader::Uefi => "Uefi",
            Bootloader::Mbr => "Mbr",
            Bootloader::Uboot => "Uboot",
        }
    }
}

impl Named for FsType {
    fn name(&self) -> &'static str {
        match self {
            FsType::Ext2 => "Ext2",
            FsType::Ext3 => "Ext3",
            FsType::Ext4 => "Ext4",
            FsType::Btrfs => "Btrfs",
            FsType::Xfs => "Xfs",
            FsType::Zfs => "Zfs",
            FsType::F2fs => "F2fs",
            FsType::Bcachefs => "Bcachefs",
            FsType::Vfat => "Vfat",
            FsType::Exfat => "Exfat",
            FsType::Ntfs => "Ntfs",
            FsType::Tmpfs => "Tmpfs",
        }
    }
}

impl Named for MotherBoard {
    fn name(&self) -> &'static str {
        match self {
            MotherBoard::Ondyfaind => "Ondyfaind",
        }
    }
}

impl Named for WlanBand {
    fn name(&self) -> &'static str {
        match self {
            WlanBand::TwoG => "2g",
            WlanBand::FiveG => "5g",
            WlanBand::SixG => "6g",
        }
    }
}

impl Named for WlanStandard {
    fn name(&self) -> &'static str {
        match self {
            WlanStandard::Wifi4 => "Wifi4",
            WlanStandard::Wifi6 => "Wifi6",
            WlanStandard::Wifi7 => "Wifi7",
        }
    }
}

impl Named for KvmAvailability {
    fn name(&self) -> &'static str {
        match self {
            KvmAvailability::Available => "Available",
            KvmAvailability::Absent => "Absent",
        }
    }
}

impl Named for SiteRenderer {
    fn name(&self) -> &'static str {
        match self {
            SiteRenderer::MarkdownStatic => "MarkdownStatic",
        }
    }
}

impl Named for PersonaCapability {
    fn name(&self) -> &'static str {
        match self {
            PersonaCapability::GitoliteServer => "GitoliteServer",
        }
    }
}

impl Named for UserRole {
    fn name(&self) -> &'static str {
        match self {
            UserRole::Code => "Code",
            UserRole::Multimedia => "Multimedia",
            UserRole::Unlimited => "Unlimited",
        }
    }
}

impl Named for DomainProvider {
    fn name(&self) -> &'static str {
        match self {
            DomainProvider::Cloudflare => "Cloudflare",
        }
    }
}

/// Magnitudes are totally ordered, and two of them meet at their lesser.
pub(crate) trait MagnitudeRanking: Clone {
    fn rank(&self) -> u8;

    /// The lesser of the two magnitudes; a tie keeps the receiver.
    fn minimum(&self, other: &Self) -> Self {
        if self.rank() <= other.rank() {
            self.clone()
        } else {
            other.clone()
        }
    }
}

impl MagnitudeRanking for Magnitude {
    fn rank(&self) -> u8 {
        match self {
            Magnitude::Zero => 0,
            Magnitude::Min => 1,
            Magnitude::Medium => 2,
            Magnitude::Large => 3,
            Magnitude::Max => 4,
        }
    }
}

/// A magnitude that has already been projected to its name still ranks.
pub(crate) trait MagnitudeName {
    fn magnitude_rank(&self) -> u8;
}

impl MagnitudeName for str {
    fn magnitude_rank(&self) -> u8 {
        match self {
            "Zero" => 0,
            "Min" => 1,
            "Medium" => 2,
            "Large" => 3,
            "Max" => 4,
            _ => 0,
        }
    }
}
