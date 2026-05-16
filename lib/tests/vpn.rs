//! Tests for the VPN-profile schema (`proposal::vpn`).
//!
//! Round-trips a NordvpnProfile through nota-codec end-to-end.

use horizon_lib::proposal::{
    NordvpnProfile, NordvpnServer, NordvpnServerName, SecretName, SecretPurpose, SecretReference,
    VpnClient, VpnClientAddress, VpnCountryCode, VpnDns, VpnIpAddress, VpnProfile,
};
use horizon_lib::pub_key::WireguardPubKey;
use nota_codec::{Decoder, NotaDecode};

#[test]
fn nordvpn_server_name_accepts_letters_digits_dashes() {
    assert!(NordvpnServerName::try_new("es-madrid").is_ok());
    assert!(NordvpnServerName::try_new("us9799").is_ok());
}

#[test]
fn nordvpn_server_name_rejects_dots() {
    assert!(NordvpnServerName::try_new("us.east").is_err());
}

#[test]
fn nordvpn_server_name_rejects_empty() {
    assert!(NordvpnServerName::try_new("").is_err());
}

#[test]
fn vpn_country_code_accepts_two_uppercase_letters() {
    assert!(VpnCountryCode::try_new("ES").is_ok());
    assert!(VpnCountryCode::try_new("US").is_ok());
}

#[test]
fn vpn_country_code_rejects_lowercase() {
    assert!(VpnCountryCode::try_new("es").is_err());
}

#[test]
fn vpn_country_code_rejects_wrong_length() {
    assert!(VpnCountryCode::try_new("ESP").is_err());
    assert!(VpnCountryCode::try_new("E").is_err());
}

#[test]
fn nordvpn_profile_decodes_from_nota_record() {
    let text = r#"(NordvpnProfile
        (VpnDns "103.86.96.100" "103.86.99.100")
        (VpnClient "10.5.0.2/32" 51820)
        [(NordvpnServer "es-madrid"
                        "es150.nordvpn.com"
                        "185.183.106.19:51820"
                        "IF1FGVSzrUznFVZ+dymIz+6bdlCgsuiT/d6cyapN8lw="
                        "ES"
                        "Madrid")]
        (SecretReference "nordvpn-credentials" NordvpnCredentials))"#;
    let mut decoder = Decoder::new(text);
    let profile = VpnProfile::decode(&mut decoder).unwrap();
    let nordvpn = match &profile {
        VpnProfile::NordvpnProfile(n) => n,
    };
    assert_eq!(nordvpn.dns.primary.as_str(), "103.86.96.100");
    assert_eq!(nordvpn.client.port, 51820);
    assert_eq!(nordvpn.servers.len(), 1);
    let s = &nordvpn.servers[0];
    assert_eq!(s.name.as_str(), "es-madrid");
    assert_eq!(s.country.as_str(), "ES");
    assert_eq!(s.city, "Madrid");
    assert_eq!(
        nordvpn.credentials.purpose,
        SecretPurpose::NordvpnCredentials
    );
    let _: &SecretReference = &nordvpn.credentials;
}

#[test]
fn nordvpn_profile_constructs_via_rust_literal() {
    let profile = NordvpnProfile {
        dns: VpnDns {
            primary: VpnIpAddress::try_new("1.1.1.1").unwrap(),
            secondary: VpnIpAddress::try_new("1.0.0.1").unwrap(),
        },
        client: VpnClient {
            address: VpnClientAddress::try_new("10.5.0.2/32").unwrap(),
            port: 51820,
        },
        servers: vec![NordvpnServer {
            name: NordvpnServerName::try_new("test-server").unwrap(),
            hostname: "test.example.com".into(),
            endpoint: "1.2.3.4:51820".into(),
            public_key: WireguardPubKey::try_new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa=")
                .unwrap(),
            country: VpnCountryCode::try_new("US").unwrap(),
            city: "Test".into(),
        }],
        credentials: SecretReference {
            name: SecretName::try_new("nordvpn-credentials").unwrap(),
            purpose: SecretPurpose::NordvpnCredentials,
        },
    };
    assert_eq!(profile.servers.len(), 1);
    assert_eq!(profile.client.port, 51820);
}
