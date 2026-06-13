//! Tests for `machine::Machine` — the per-node hardware record.

use horizon_lib::machine::Machine;
use horizon_lib::name::ModelName;
use horizon_lib::species::{Arch, MachineSpecies};

fn metal_x86() -> Machine {
    Machine {
        species: MachineSpecies::Metal,
        arch: Some(Arch::X86_64),
        cores: 12,
        model: Some(ModelName::try_new("ThinkPadT14Gen5Intel").unwrap()),
        mother_board: None,
        super_node: None,
        super_user: None,
        chip_gen: Some(13),
        ram_gb: Some(32),
        disk_gb: None,
        location: None,
    }
}

#[test]
fn metal_machine_carries_required_fields() {
    let machine = metal_x86();
    assert!(matches!(machine.species, MachineSpecies::Metal));
    assert_eq!(machine.arch, Some(Arch::X86_64));
    assert_eq!(machine.cores, 12);
    assert_eq!(
        machine.model.as_ref().unwrap().as_str(),
        "ThinkPadT14Gen5Intel"
    );
    assert_eq!(machine.chip_gen, Some(13));
    assert_eq!(machine.ram_gb, Some(32));
}

#[test]
fn pod_machine_can_omit_arch_for_super_node_resolution() {
    let pod = Machine {
        species: MachineSpecies::Pod,
        arch: None,
        cores: 2,
        model: None,
        mother_board: None,
        super_node: Some(horizon_lib::name::NodeName::try_new("ouranos").unwrap()),
        super_user: Some(horizon_lib::name::UserName::try_new("li").unwrap()),
        chip_gen: None,
        ram_gb: None,
        disk_gb: None,
        location: None,
    };
    assert!(matches!(pod.species, MachineSpecies::Pod));
    assert!(pod.arch.is_none());
    assert!(pod.super_node.is_some());
}

#[test]
fn machine_clones_preserve_all_fields() {
    let machine = metal_x86();
    let clone = machine.clone();
    assert_eq!(machine, clone);
}
