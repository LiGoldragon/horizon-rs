//! Direct Datomic anatomy for machine records and their capacity fields.

use datomic::{Datomic, TextEdge};
use horizon_lib::{
    machine::{Location, Machine},
    name::{ModelName, NodeName, UserName},
    species::{Arch, MachineSpecies, MotherBoard},
};
use protos::Text;

fn fully_specified_machine() -> Machine {
    Machine {
        species: MachineSpecies::Metal,
        arch: Some(Arch::X86_64),
        cores: u32::MAX,
        model: Some(ModelName::try_new("ThinkPadT14Gen5Intel").expect("valid model")),
        mother_board: Some(MotherBoard::Ondyfaind),
        super_node: Some(NodeName::try_new("ouranos").expect("valid node")),
        super_user: Some(UserName::try_new("li").expect("valid user")),
        chip_gen: Some(13),
        ram_gb: Some(32),
        disk_gb: Some(u32::MAX),
        location: Some(Location::new("home-lab")),
        super_nodes: vec![
            NodeName::try_new("ouranos").expect("valid node"),
            NodeName::try_new("gaia").expect("valid node"),
        ],
    }
}

#[test]
fn generated_species_and_direct_machine_round_trip_through_text_edge() {
    let machine = fully_specified_machine();
    let text = machine.textualize();

    assert_eq!(
        Text::<Machine>::from(text.as_ref())
            .embody()
            .expect("machine Datomic embodiment"),
        machine
    );
    assert!(text.as_ref().starts_with("{Metal "));
    assert!(text.as_ref().contains("4294967295"));
}

#[test]
fn machine_datomic_record_requires_all_twelve_fields() {
    assert!(Text::<Machine>::from("{Metal}").embody().is_err());
}

#[test]
fn machine_datomic_rejects_capacity_outside_u32_range() {
    let invalid = fully_specified_machine()
        .textualize()
        .as_ref()
        .replacen("4294967295", "-1", 1);

    assert!(Text::<Machine>::from(invalid.as_str()).embody().is_err());
}

#[test]
fn location_round_trips_as_a_distinct_string_value() {
    let location = Location::new("hetzner-fsn1");
    let text = location.textualize();

    assert_eq!(text.as_ref(), "hetzner-fsn1");
    assert_eq!(
        Text::<Location>::from(text.as_ref())
            .embody()
            .expect("location Datomic embodiment"),
        location
    );
}
