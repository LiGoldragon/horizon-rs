//! Tests for `proposal::Machine` — the per-node hardware record on
//! the input side.

use horizon_lib::name::ModelName;
use horizon_lib::proposal::Machine;
use horizon_lib::species::Arch;

fn metal_x86() -> Machine {
    Machine {
        arch: Some(Arch::X86_64),
        cores: 12,
        model: Some(ModelName::try_new("ThinkPadT14Gen5Intel").unwrap()),
        mother_board: None,
        chip_gen: Some(13),
        ram_gb: Some(32),
    }
}

#[test]
fn metal_machine_carries_required_fields() {
    let machine = metal_x86();
    assert_eq!(machine.arch, Some(Arch::X86_64));
    assert_eq!(machine.cores, 12);
    assert_eq!(machine.model.as_ref().unwrap().as_str(), "ThinkPadT14Gen5Intel");
    assert_eq!(machine.chip_gen, Some(13));
    assert_eq!(machine.ram_gb, Some(32));
}

#[test]
fn contained_node_can_omit_arch_for_host_resolution() {
    // Pod-style machines (no architecture of their own) inherit arch
    // from their host through `NodePlacement::Contained`. Machine no
    // longer carries the placement information.
    let machine_without_arch = Machine {
        arch: None,
        cores: 2,
        model: None,
        mother_board: None,
        chip_gen: None,
        ram_gb: None,
    };
    assert!(machine_without_arch.arch.is_none());
}

#[test]
fn machine_clones_preserve_all_fields() {
    let machine = metal_x86();
    let clone = machine.clone();
    assert_eq!(machine, clone);
}
