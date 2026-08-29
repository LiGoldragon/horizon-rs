//! Datomic text-edge coverage for the hand-owned I/O data records.

use std::collections::BTreeMap;

use datomic::{Datomic, TextEdge};
use horizon_lib::{
    io::{CompressedSwap, DevicePath, Disk, FsType, Io, MountPath, SwapDevice},
    species::{Bootloader, Keyboard},
};
use protos::Text;

fn configured_io() -> Io {
    Io {
        keyboard: Keyboard::Colemak,
        bootloader: Bootloader::Uefi,
        disks: BTreeMap::from([(
            MountPath::new("/"),
            Disk {
                device: DevicePath::new("/dev/disk/by-uuid/root"),
                fs_type: FsType::Btrfs,
                options: vec!["subvol=root".to_owned(), "noatime".to_owned()],
            },
        )]),
        swap_devices: vec![SwapDevice {
            device: DevicePath::new("/swapfile"),
            size_mebibytes: Some(u32::MAX),
        }],
        compressed_swap: Some(CompressedSwap { memory_percent: 25 }),
    }
}

#[test]
fn io_and_nested_records_round_trip_through_the_datomic_text_edge() {
    let io = configured_io();
    let text = io.textualize();

    assert_eq!(
        Text::<Io>::from(text.as_ref())
            .embody()
            .expect("I/O Datomic embodiment"),
        io
    );
    assert!(text.as_ref().starts_with("{Colemak Uefi «"));
    assert!(text.as_ref().contains("4294967295"));
}

#[test]
fn io_rejects_missing_record_fields() {
    assert!(Text::<Io>::from("{Colemak Uefi}").embody().is_err());
}

#[test]
fn swap_size_rejects_values_outside_the_horizon_u32_bound() {
    let invalid = configured_io()
        .textualize()
        .as_ref()
        .replacen("4294967295", "-1", 1);
    assert!(Text::<Io>::from(invalid.as_str()).embody().is_err());
}
