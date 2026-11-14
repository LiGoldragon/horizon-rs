//! A generated value survives the rkyv archive, with or without `datom`.
use horizon_lib::*;

#[test]
fn hardware_round_trips_through_the_rkyv_archive() {
    let hardware = Hardware {
        integer: 16,
        model_name_option: Some("ThinkPadT14Gen5Intel".to_owned()),
        mother_board_option: Some(MotherBoard::Undefined),
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
