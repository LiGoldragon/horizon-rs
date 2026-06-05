//! Tests for `io` — filesystem and I/O configuration types.

use std::collections::BTreeMap;

use horizon_lib::io::{CompressedSwap, DevicePath, Disk, FsType, Io, MountPath, SwapDevice};
use horizon_lib::species::{Bootloader, Keyboard};

#[test]
fn mount_path_displays_as_string() {
    let path = MountPath::new("/boot");
    assert_eq!(path.as_str(), "/boot");
    assert_eq!(format!("{path}"), "/boot");
}

#[test]
fn device_path_carries_uuid_form() {
    let path = DevicePath::new("/dev/disk/by-uuid/abcd-1234");
    assert_eq!(path.as_str(), "/dev/disk/by-uuid/abcd-1234");
}

#[test]
fn disk_carries_device_filesystem_and_options() {
    let disk = Disk {
        device: DevicePath::new("/dev/disk/by-label/NIXOS_SD"),
        fs_type: FsType::Ext4,
        options: vec!["noatime".to_string()],
    };
    assert_eq!(disk.device.as_str(), "/dev/disk/by-label/NIXOS_SD");
    assert!(matches!(disk.fs_type, FsType::Ext4));
    assert_eq!(disk.options, vec!["noatime".to_string()]);
}

#[test]
fn io_struct_holds_keyboard_bootloader_disks_and_swap() {
    let mut disks = BTreeMap::new();
    disks.insert(
        MountPath::new("/"),
        Disk {
            device: DevicePath::new("/dev/disk/by-uuid/0000"),
            fs_type: FsType::Btrfs,
            options: vec!["subvol=root".to_string()],
        },
    );
    let io = Io {
        keyboard: Keyboard::Colemak,
        bootloader: Bootloader::Uefi,
        disks,
        swap_devices: vec![SwapDevice {
            device: DevicePath::new("/swapfile"),
            size_mebibytes: Some(32768),
        }],
        compressed_swap: Some(CompressedSwap { memory_percent: 25 }),
    };
    assert!(matches!(io.keyboard, Keyboard::Colemak));
    assert!(matches!(io.bootloader, Bootloader::Uefi));
    assert_eq!(io.disks.len(), 1);
    assert_eq!(io.swap_devices.len(), 1);
    assert_eq!(io.swap_devices[0].size_mebibytes, Some(32768));
    assert_eq!(io.compressed_swap.unwrap().memory_percent, 25);
}

#[test]
fn fs_type_variants_are_distinguishable() {
    assert!(matches!(FsType::Ext4, FsType::Ext4));
    assert!(matches!(FsType::Btrfs, FsType::Btrfs));
    assert!(matches!(FsType::Vfat, FsType::Vfat));
    assert_ne!(FsType::Ext4, FsType::Btrfs);
}
