//! Tests for `pub_key` — public-key newtypes and pre-rendered line
//! forms.

use horizon_lib::error::Error;
use horizon_lib::name::{ClusterName, CriomeDomainName, NodeName};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, WireguardPubKey, YggPubKey};

const VALID_NIX: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const VALID_WG: &str = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";

fn cache_domain() -> CriomeDomainName {
    let cluster = ClusterName::try_new("goldragon").unwrap();
    let node = NodeName::try_new("prometheus").unwrap();
    CriomeDomainName::for_node(&node, &cluster)
}

#[test]
fn ssh_pub_key_accepts_base64_body() {
    let key = SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap();
    assert_eq!(key.as_str(), "AAAAC3NzaC1lZDI1NTE5AAAA");
}

#[test]
fn ssh_pub_key_rejects_empty_string() {
    let error = SshPubKey::try_new("").unwrap_err();
    assert!(matches!(error, Error::InvalidBase64Key { .. }));
}

#[test]
fn ssh_pub_key_rejects_chars_outside_base64() {
    let error = SshPubKey::try_new("not-base64!@#").unwrap_err();
    assert!(matches!(error, Error::InvalidBase64Key { .. }));
}

#[test]
fn ssh_pub_key_line_carries_ssh_ed25519_prefix() {
    let key = SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap();
    let line = key.line();
    assert_eq!(line.as_str(), "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA");
    assert_eq!(format!("{line}"), "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA");
}

#[test]
fn ygg_pub_key_accepts_64_hex_chars() {
    let hex = "a".repeat(64);
    let key = YggPubKey::try_new(hex.clone()).unwrap();
    assert_eq!(key.as_str(), hex);
}

#[test]
fn ygg_pub_key_normalises_to_lowercase() {
    let key = YggPubKey::try_new("A".repeat(64)).unwrap();
    assert_eq!(key.as_str(), "a".repeat(64));
}

#[test]
fn ygg_pub_key_rejects_wrong_length() {
    assert!(YggPubKey::try_new("a".repeat(63)).is_err());
    assert!(YggPubKey::try_new("a".repeat(65)).is_err());
}

#[test]
fn ygg_pub_key_rejects_non_hex_chars() {
    let mut hex = "a".repeat(63);
    hex.push('z');
    let error = YggPubKey::try_new(hex).unwrap_err();
    assert!(matches!(error, Error::InvalidHexKey { .. }));
}

#[test]
fn nix_pub_key_accepts_44_base64_chars() {
    let key = NixPubKey::try_new(VALID_NIX).unwrap();
    assert_eq!(key.as_str(), VALID_NIX);
}

#[test]
fn nix_pub_key_rejects_wrong_length() {
    assert!(NixPubKey::try_new(&VALID_NIX[..43]).is_err());
}

#[test]
fn nix_pub_key_line_renders_domain_colon_key() {
    let key = NixPubKey::try_new(VALID_NIX).unwrap();
    let line = key.line(&cache_domain());
    assert_eq!(line.as_str(), format!("prometheus.goldragon.criome:{VALID_NIX}"));
}

#[test]
fn wireguard_pub_key_accepts_44_base64_chars() {
    let key = WireguardPubKey::try_new(VALID_WG).unwrap();
    assert_eq!(key.as_str(), VALID_WG);
}

#[test]
fn wireguard_pub_key_rejects_wrong_length() {
    assert!(WireguardPubKey::try_new("too-short").is_err());
}
