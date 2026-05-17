//! Tests for projected network records — `LanCidr`, `LanNetwork`,
//! `DhcpPool`, and `ResolverPolicy`.

use horizon_lib::address::IpAddress;
use horizon_lib::error::Error;
use horizon_lib::proposal::{DhcpPool, LanCidr, LanNetwork, ResolverPolicy};
use nota_codec::{Decoder, NotaDecode};

#[test]
fn lan_cidr_accepts_ipv4_cidr() {
    let cidr = LanCidr::try_new("10.18.0.0/24").unwrap();
    assert_eq!(cidr.to_string(), "10.18.0.0/24");
}

#[test]
fn lan_cidr_accepts_ipv6_cidr() {
    let cidr = LanCidr::try_new("2001:db8::/32").unwrap();
    assert_eq!(cidr.to_string(), "2001:db8::/32");
}

#[test]
fn lan_cidr_rejects_bare_ip() {
    let error = LanCidr::try_new("10.18.0.0").unwrap_err();
    assert!(matches!(error, Error::InvalidLanCidr { .. }));
}

#[test]
fn lan_cidr_rejects_garbage() {
    let error = LanCidr::try_new("not a cidr").unwrap_err();
    assert!(matches!(error, Error::InvalidLanCidr { .. }));
}

#[test]
fn lan_network_decodes_from_nota_record() {
    let text = r#"(LanNetwork "10.18.0.0/24" "10.18.0.1" (DhcpPool "10.18.0.100" "10.18.0.240"))"#;
    let mut decoder = Decoder::new(text);
    let lan = LanNetwork::decode(&mut decoder).unwrap();
    assert_eq!(lan.cidr.to_string(), "10.18.0.0/24");
    assert_eq!(lan.gateway.to_string(), "10.18.0.1");
    assert_eq!(lan.dhcp_pool.start.to_string(), "10.18.0.100");
    assert_eq!(lan.dhcp_pool.end.to_string(), "10.18.0.240");
}

#[test]
fn dhcp_pool_decodes_from_nota_record() {
    let text = r#"(DhcpPool "192.168.1.10" "192.168.1.250")"#;
    let mut decoder = Decoder::new(text);
    let pool = DhcpPool::decode(&mut decoder).unwrap();
    assert_eq!(pool.start.to_string(), "192.168.1.10");
    assert_eq!(pool.end.to_string(), "192.168.1.250");
}

#[test]
fn resolver_policy_decodes_with_listens() {
    let text = r#"(ResolverPolicy ["127.0.0.1" "::1"])"#;
    let mut decoder = Decoder::new(text);
    let resolver = ResolverPolicy::decode(&mut decoder).unwrap();
    assert_eq!(resolver.listens.len(), 2);
    assert_eq!(resolver.listens[0].to_string(), "127.0.0.1");
    assert_eq!(resolver.listens[1].to_string(), "::1");
}

#[test]
fn resolver_policy_decodes_with_empty_lists() {
    let text = "(ResolverPolicy [])";
    let mut decoder = Decoder::new(text);
    let resolver = ResolverPolicy::decode(&mut decoder).unwrap();
    assert!(resolver.listens.is_empty());
}

#[test]
fn lan_network_constructs_via_rust_literal() {
    let lan = LanNetwork {
        cidr: LanCidr::try_new("10.0.0.0/8").unwrap(),
        gateway: IpAddress::try_new("10.0.0.1").unwrap(),
        dhcp_pool: DhcpPool {
            start: IpAddress::try_new("10.0.0.100").unwrap(),
            end: IpAddress::try_new("10.0.0.200").unwrap(),
        },
    };
    assert_eq!(lan.cidr.to_string(), "10.0.0.0/8");
    assert_eq!(lan.dhcp_pool.end.to_string(), "10.0.0.200");
}
