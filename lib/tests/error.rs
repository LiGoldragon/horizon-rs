//! Display + variant tests for `error::Error`.

use horizon_lib::error::Error;
use horizon_lib::name::NodeName;
use horizon_lib::species::Arch;

fn node(name: &str) -> NodeName {
    NodeName::try_new(name).unwrap()
}

#[test]
fn empty_name_displays_kind() {
    let error = Error::EmptyName { kind: "node name" };
    assert_eq!(error.to_string(), "invalid name: node name cannot be empty");
}

#[test]
fn invalid_keygrip_includes_got_value() {
    let error = Error::InvalidKeygrip {
        got: "TOOSHORT".to_string(),
    };
    assert!(error.to_string().contains("TOOSHORT"));
    assert!(error.to_string().contains("40 hex chars"));
}

#[test]
fn empty_ygg_subnet_is_self_describing() {
    let error = Error::EmptyYggSubnet;
    assert_eq!(error.to_string(), "yggdrasil subnet must not be empty");
}

#[test]
fn node_not_in_cluster_names_the_node() {
    let error = Error::NodeNotInCluster(node("missing"));
    assert!(error.to_string().contains("missing"));
}

#[test]
fn missing_super_node_names_both_nodes() {
    let error = Error::MissingSuperNode(node("pod"), node("host"));
    let text = error.to_string();
    assert!(text.contains("pod"));
    assert!(text.contains("host"));
}

#[test]
fn unresolvable_arch_names_the_node() {
    let error = Error::UnresolvableArch(node("orphan"));
    assert!(error.to_string().contains("orphan"));
}

#[test]
fn host_set_arch_mismatch_names_the_node_and_both_diverging_hosts() {
    let error = Error::HostSetArchMismatch {
        node: node("guest"),
        first_host: node("host-x86"),
        first_arch: Arch::X86_64,
        second_host: node("host-arm"),
        second_arch: Arch::Arm64,
    };
    let text = error.to_string();
    assert!(text.contains("guest"));
    assert!(text.contains("host-x86"));
    assert!(text.contains("host-arm"));
    assert!(text.contains("X86_64"));
    assert!(text.contains("Arm64"));
}

#[test]
fn unknown_variant_names_kind_and_value() {
    let error = Error::UnknownVariant {
        kind: "Magnitude",
        got: "Galaxy".to_string(),
    };
    let text = error.to_string();
    assert!(text.contains("Magnitude"));
    assert!(text.contains("Galaxy"));
}
