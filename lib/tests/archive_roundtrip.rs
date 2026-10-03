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
