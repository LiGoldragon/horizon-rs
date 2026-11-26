//! A generated value survives the rkyv archive, with or without `datom`.
use horizon_lib::*;

#[test]
fn hardware_round_trips_through_the_rkyv_archive() {
    let hardware = Hardware {
        integer: 16,
        model_name_option: Some("ThinkPadT14Gen5Intel".to_owned()),
        mother_board_option: Some(MotherBoard::Ondyfaind),
        first_integer_option: Some(12),
        second_integer_option: None,
        location_option: Some("home-lab".to_owned()),
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&hardware).expect("archive");
    let restored = rkyv::from_bytes::<Hardware, rkyv::rancor::Error>(&bytes).expect("read back");
    assert_eq!(restored, hardware);
}

#[test]
fn user_pub_key_capabilities_round_trip_through_the_rkyv_archive() {
    let key = UserPubKey {
        node_name: "zeus".to_owned(),
        ssh_pub_key: "AAAAuser".to_owned(),
        keygrip: "keygrip".to_owned(),
        user_capability_vector: vec![
            UserCapability::AgentStack,
            UserCapability::CodexCandidate,
            UserCapability::StableCodex,
        ],
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&key).expect("archive");
    let restored = rkyv::from_bytes::<UserPubKey, rkyv::rancor::Error>(&bytes).expect("read back");
    assert_eq!(restored, key);
}

#[test]
fn open_code_testing_owner_round_trips_through_the_rkyv_archive() {
    let capability = NodeCapability::OpenCodeTesting("li".to_owned());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&capability).expect("archive");
    let restored =
        rkyv::from_bytes::<NodeCapability, rkyv::rancor::Error>(&bytes).expect("read back");
    assert_eq!(restored, capability);
}

#[test]
fn git_server_capability_round_trips_through_the_rkyv_archive() {
    let capability = NodeCapability::GitServer(NoSettings {});
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&capability).expect("archive");
    let restored =
        rkyv::from_bytes::<NodeCapability, rkyv::rancor::Error>(&bytes).expect("read back");
    assert_eq!(restored, capability);
}

#[test]
fn router_usb_parent_power_identity_round_trips_through_the_rkyv_archive() {
    let capability = NodeCapability::RouterUsbParentPowerOn(vec![RouterUsbParentPowerIdentity {
        pci_controller_address: "0000:c7:00.0".to_owned(),
        integer: 2,
        usb_vendor_id: "0b95".to_owned(),
        usb_product_id: "1790".to_owned(),
        usb_device_revision: "0200".to_owned(),
        usb_serial_number: "00000000009525".to_owned(),
    }]);
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&capability).expect("archive");
    let restored =
        rkyv::from_bytes::<NodeCapability, rkyv::rancor::Error>(&bytes).expect("read back");
    assert_eq!(restored, capability);
}

#[test]
fn append_only_capability_keeps_deployed_archive_layout_and_reads_legacy_bytes() {
    assert_eq!(std::mem::size_of::<NodeCapability>(), 72);
    assert_eq!(std::mem::align_of::<NodeCapability>(), 8);
    assert_eq!(std::mem::size_of::<ArchivedNodeCapability>(), 26);
    assert_eq!(std::mem::align_of::<ArchivedNodeCapability>(), 1);

    // Produced by horizon-lib 0.17.2 at deployed revision 4d206ff from a
    // TailnetController, UsbDownlink, GitServer vector. This is an immutable
    // old-writer witness, rather than bytes re-encoded by the new type.
    let legacy = [
        49, 48, 46, 52, 52, 46, 48, 46, 48, 47, 50, 52, 16, 1, 99, 97, 255, 255, 255, 255, 255,
        255, 99, 101, 114, 116, 255, 255, 255, 255, 107, 101, 121, 255, 255, 255, 255, 255, 23,
        140, 0, 0, 0, 217, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 178, 255, 255, 255,
        3, 0, 0, 0,
    ];
    let mut aligned: rkyv::util::AlignedVec<16> = rkyv::util::AlignedVec::new();
    aligned.extend_from_slice(&legacy);
    let restored = rkyv::from_bytes::<Vec<NodeCapability>, rkyv::rancor::Error>(&aligned)
        .expect("new reader accepts deployed Horizon archive");
    assert!(matches!(restored[0], NodeCapability::TailnetController(_)));
    assert!(matches!(restored[1], NodeCapability::UsbDownlink(_)));
    assert!(matches!(restored[2], NodeCapability::GitServer(_)));
}
