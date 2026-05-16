//! Display + variant tests for `error::Error`.

use horizon_lib::error::Error;
use horizon_lib::name::NodeName;

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
fn unknown_variant_names_kind_and_value() {
    let error = Error::UnknownVariant {
        kind: "Magnitude",
        got: "Galaxy".to_string(),
    };
    let text = error.to_string();
    assert!(text.contains("Magnitude"));
    assert!(text.contains("Galaxy"));
}
