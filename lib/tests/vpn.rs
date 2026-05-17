//! Tests for cluster-level VPN provider selections.

use horizon_lib::error::Error;
use horizon_lib::proposal::{
    NordvpnLocationPreference, NordvpnProfile, SecretPurpose, SecretReference, VpnCountryCode,
    VpnProfile,
};
use nota_codec::{Decoder, NotaDecode};

#[test]
fn vpn_country_code_accepts_iso_alpha2() {
    let code = VpnCountryCode::try_new("ES").unwrap();
    assert_eq!(code.as_str(), "ES");
}

#[test]
fn vpn_country_code_rejects_lowercase() {
    let error = VpnCountryCode::try_new("es").unwrap_err();
    assert!(matches!(error, Error::InvalidVpnCountryCode { .. }));
}

#[test]
fn nordvpn_profile_decodes_credentials_and_empty_preferences() {
    let text = r#"(NordvpnProfile (SecretReference "nordvpn-credentials" NordvpnCredentials) [])"#;
    let mut decoder = Decoder::new(text);
    let profile = VpnProfile::decode(&mut decoder).unwrap();
    let VpnProfile::NordvpnProfile(nordvpn) = profile;

    assert_eq!(nordvpn.credentials.name.as_str(), "nordvpn-credentials");
    assert_eq!(
        nordvpn.credentials.purpose,
        SecretPurpose::NordvpnCredentials
    );
    assert!(nordvpn.preferred_locations.is_empty());
}

#[test]
fn nordvpn_profile_can_carry_location_preferences() {
    let profile = NordvpnProfile {
        credentials: SecretReference {
            name: "nordvpn-credentials".parse().unwrap(),
            purpose: SecretPurpose::NordvpnCredentials,
        },
        preferred_locations: vec![NordvpnLocationPreference {
            country: VpnCountryCode::try_new("ES").unwrap(),
            city: Some("Madrid".to_string()),
        }],
    };

    assert_eq!(profile.preferred_locations.len(), 1);
    assert_eq!(profile.preferred_locations[0].country.as_str(), "ES");
    assert_eq!(
        profile.preferred_locations[0].city.as_deref(),
        Some("Madrid")
    );
}
