//! `System` enum serialises to Nix's system-tuple shape on the JSON
//! wire — `"x86_64-linux"` / `"aarch64-linux"` — and round-trips.
//!
//! Witness for the `#[serde(rename = ...)]` on every `System` variant.
//! Without those renames the variants emit as their PascalCase Rust
//! spelling (`"X86_64Linux"`), which is not what `inputs.horizon`
//! consumers in CriomOS / CriomOS-home read. This test fails loudly if
//! the rename ever drifts.

use horizon_lib::species::System;
use serde_json::{json, Value};

#[test]
fn x86_64_linux_serialises_to_nix_system_tuple_shape() {
    let value = serde_json::to_value(System::X86_64Linux).unwrap();
    assert_eq!(value, Value::String("x86_64-linux".to_string()));
}

#[test]
fn aarch64_linux_serialises_to_nix_system_tuple_shape() {
    let value = serde_json::to_value(System::Aarch64Linux).unwrap();
    assert_eq!(value, Value::String("aarch64-linux".to_string()));
}

#[test]
fn x86_64_linux_deserialises_from_nix_system_tuple_shape() {
    let parsed: System = serde_json::from_value(json!("x86_64-linux")).unwrap();
    assert_eq!(parsed, System::X86_64Linux);
}

#[test]
fn aarch64_linux_deserialises_from_nix_system_tuple_shape() {
    let parsed: System = serde_json::from_value(json!("aarch64-linux")).unwrap();
    assert_eq!(parsed, System::Aarch64Linux);
}

#[test]
fn x86_64_linux_round_trips_through_json() {
    let original = System::X86_64Linux;
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: System = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn aarch64_linux_round_trips_through_json() {
    let original = System::Aarch64Linux;
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: System = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(recovered, original);
}
