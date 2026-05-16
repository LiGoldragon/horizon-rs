//! Tests for `species` — closed enums and `KnownModel` typed dispatch.

use horizon_lib::species::{Arch, KnownModel, NodeSpecies, System};

#[test]
fn arch_x86_maps_to_x86_64_linux() {
    assert!(matches!(Arch::X86_64.system(), System::X86_64Linux));
}

#[test]
fn arch_arm_maps_to_aarch64_linux() {
    assert!(matches!(Arch::Arm64.system(), System::Aarch64Linux));
}

#[test]
fn arch_x86_is_intel() {
    assert!(Arch::X86_64.is_intel());
}

#[test]
fn arch_arm_is_not_intel() {
    assert!(!Arch::Arm64.is_intel());
}

#[test]
fn known_model_thinkpads_pass_is_thinkpad() {
    assert!(KnownModel::ThinkPadX230.is_thinkpad());
    assert!(KnownModel::ThinkPadX240.is_thinkpad());
    assert!(KnownModel::ThinkPadT14Gen2Intel.is_thinkpad());
    assert!(KnownModel::ThinkPadT14Gen5Intel.is_thinkpad());
}

#[test]
fn known_model_rpi_is_not_thinkpad() {
    assert!(!KnownModel::Rpi3B.is_thinkpad());
}

#[test]
fn cloud_host_species_is_closed_variant() {
    assert!(matches!(NodeSpecies::CloudHost, NodeSpecies::CloudHost));
}
