//! Tests for `address` — yggdrasil identifiers, node IPs, link-local
//! per-interface addresses.

use horizon_lib::address::{Interface, LinkLocalIp, NodeIp, TapSubnet, YggAddress, YggSubnet};
use horizon_lib::error::Error;

#[test]
fn ygg_address_accepts_canonical_ipv6() {
    let address = YggAddress::try_new("200:1234::1").unwrap();
    assert_eq!(address.to_string(), "200:1234::1");
}

#[test]
fn ygg_address_rejects_garbage() {
    let error = YggAddress::try_new("not-an-ip").unwrap_err();
    assert!(matches!(error, Error::InvalidYggAddress { .. }));
}

#[test]
fn ygg_subnet_accepts_non_empty_prefix() {
    let subnet = YggSubnet::try_new("300:ca41:6b12:fba").unwrap();
    assert_eq!(subnet.as_str(), "300:ca41:6b12:fba");
}

#[test]
fn ygg_subnet_rejects_empty() {
    let error = YggSubnet::try_new("").unwrap_err();
    assert!(matches!(error, Error::EmptyYggSubnet));
}

#[test]
fn node_ip_accepts_cidr() {
    let ip = NodeIp::try_new("10.0.0.1/32").unwrap();
    assert_eq!(ip.ipnet().to_string(), "10.0.0.1/32");
}

#[test]
fn node_ip_rejects_non_cidr_string() {
    let error = NodeIp::try_new("definitely not a cidr").unwrap_err();
    assert!(matches!(error, Error::InvalidNodeIp { .. }));
}

#[test]
fn tap_subnet_accepts_ipv4_cidr() {
    let subnet = TapSubnet::try_new("169.254.100.0/22").unwrap();
    assert_eq!(subnet.ipv4_net().to_string(), "169.254.100.0/22");
}

#[test]
fn tap_subnet_rejects_ipv6_cidr() {
    // PATTERN: TapSubnet is IPv4-only because the Nix generator slices
    // it on `.` as dotted-decimal; an IPv6 net must fail at the typed
    // boundary, not silently misbehave later in Nix.
    let error = TapSubnet::try_new("fd00::/64").unwrap_err();
    assert!(matches!(error, Error::InvalidTapSubnet { .. }));
}

#[test]
fn tap_subnet_rejects_non_cidr_string() {
    let error = TapSubnet::try_new("definitely not a cidr").unwrap_err();
    assert!(matches!(error, Error::InvalidTapSubnet { .. }));
}

#[test]
fn tap_subnet_usable_host_count_excludes_network_and_broadcast() {
    // /22 = 1024 addresses, minus network + broadcast = 1022 usable.
    let large = TapSubnet::try_new("169.254.100.0/22").unwrap();
    assert_eq!(large.usable_host_count(), 1022);
    // /30 = 4 addresses, minus network + broadcast = 2 usable.
    let small = TapSubnet::try_new("10.0.0.0/30").unwrap();
    assert_eq!(small.usable_host_count(), 2);
}

#[test]
fn tap_subnet_can_host_gates_on_usable_capacity() {
    // PATTERN: the generator asserts the hosted guest set fits the
    // subnet so over-subscription fails loudly instead of slicing
    // outside the declared network.
    let subnet = TapSubnet::try_new("10.0.0.0/30").unwrap();
    assert!(subnet.can_host(2));
    assert!(!subnet.can_host(3));
}

#[test]
fn interface_displays_as_carried_string() {
    let interface = Interface::new("enp0s25");
    assert_eq!(interface.as_str(), "enp0s25");
    assert_eq!(format!("{interface}"), "enp0s25");
}

#[test]
fn link_local_ip_render_concatenates_fe80_prefix_suffix_and_iface() {
    let link = LinkLocalIp {
        iface: Interface::new("enp0s25"),
        suffix: "1234:abcd".to_string(),
    };
    let rendered = link.render();
    assert_eq!(rendered.as_str(), "fe80::1234:abcd%enp0s25");
}
