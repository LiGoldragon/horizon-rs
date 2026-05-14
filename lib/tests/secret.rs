//! Tests for `proposal::secret` — `SecretReference`, `SecretName`,
//! `SecretPurpose`, `ClusterSecretBinding`, and the closed
//! `SecretBackend` enum.

use horizon_lib::error::Error;
use horizon_lib::proposal::{
    ClusterSecretBinding, SecretBackend, SecretName, SecretPurpose, SecretReference, SopsFilePath,
    SopsKeyPath,
};
use nota_codec::{Decoder, NotaDecode};

#[test]
fn secret_name_accepts_letters_digits_and_dashes() {
    let name = SecretName::try_new("router-wifi-pwd").unwrap();
    assert_eq!(name.as_str(), "router-wifi-pwd");
    assert_eq!(format!("{name}"), "router-wifi-pwd");

    let allcaps = SecretName::try_new("WIFI42").unwrap();
    assert_eq!(allcaps.as_str(), "WIFI42");

    let digits_first = SecretName::try_new("42-keys").unwrap();
    assert_eq!(digits_first.as_str(), "42-keys");
}

#[test]
fn secret_name_rejects_empty() {
    let error = SecretName::try_new("").unwrap_err();
    assert!(matches!(error, Error::InvalidSecretName { got } if got.is_empty()));
}

#[test]
fn secret_name_rejects_spaces() {
    let error = SecretName::try_new("has space").unwrap_err();
    assert!(matches!(error, Error::InvalidSecretName { .. }));
}

#[test]
fn secret_name_rejects_slashes() {
    let error = SecretName::try_new("with/slash").unwrap_err();
    assert!(matches!(error, Error::InvalidSecretName { .. }));
}

#[test]
fn secret_name_rejects_dots() {
    let error = SecretName::try_new("with.dot").unwrap_err();
    assert!(matches!(error, Error::InvalidSecretName { .. }));
}

#[test]
fn secret_name_rejects_underscores() {
    // Plan 04: letters-digits-dashes only — underscore is reserved
    // for future expansion; until then it is rejected so authors
    // pick one convention.
    let error = SecretName::try_new("snake_case").unwrap_err();
    assert!(matches!(error, Error::InvalidSecretName { .. }));
}

#[test]
fn secret_name_implements_from_str() {
    let name: SecretName = "key-id".parse().unwrap();
    assert_eq!(name.as_str(), "key-id");
}

#[test]
fn sops_file_path_accepts_non_empty() {
    let path = SopsFilePath::try_new("secrets/wifi.yaml").unwrap();
    assert_eq!(path.as_str(), "secrets/wifi.yaml");
    assert_eq!(format!("{path}"), "secrets/wifi.yaml");
}

#[test]
fn sops_file_path_rejects_empty() {
    let error = SopsFilePath::try_new("").unwrap_err();
    assert!(matches!(error, Error::EmptyName { kind } if kind == "sops file path"));
}

#[test]
fn sops_key_path_accepts_non_empty() {
    let key = SopsKeyPath::try_new("passwords/router-wifi").unwrap();
    assert_eq!(key.as_str(), "passwords/router-wifi");
}

#[test]
fn sops_key_path_rejects_empty() {
    let error = SopsKeyPath::try_new("").unwrap_err();
    assert!(matches!(error, Error::EmptyName { kind } if kind == "sops key path"));
}

#[test]
fn secret_reference_decodes_from_nota_record() {
    let text = "(SecretReference router-wifi-pwd WifiPassword)";
    let mut decoder = Decoder::new(text);
    let reference = SecretReference::decode(&mut decoder).unwrap();
    assert_eq!(reference.name.as_str(), "router-wifi-pwd");
    assert!(matches!(reference.purpose, SecretPurpose::WifiPassword));
}

#[test]
fn secret_backend_sops_decodes_with_file_and_key() {
    let text = r#"(Sops "secrets/wifi.yaml" "passwords/router-wifi")"#;
    let mut decoder = Decoder::new(text);
    let backend = SecretBackend::decode(&mut decoder).unwrap();
    match backend {
        SecretBackend::Sops { file, key } => {
            assert_eq!(file.as_str(), "secrets/wifi.yaml");
            assert_eq!(key.as_str(), "passwords/router-wifi");
        }
        other => panic!("expected Sops, got {other:?}"),
    }
}

#[test]
fn secret_backend_systemd_credential_decodes_with_name() {
    let text = r#"(SystemdCredential "ghost-mailer-password")"#;
    let mut decoder = Decoder::new(text);
    let backend = SecretBackend::decode(&mut decoder).unwrap();
    match backend {
        SecretBackend::SystemdCredential { credential_name } => {
            assert_eq!(credential_name, "ghost-mailer-password");
        }
        other => panic!("expected SystemdCredential, got {other:?}"),
    }
}

#[test]
fn secret_backend_agenix_decodes_with_secret_id() {
    let text = r#"(Agenix "router-wifi-sae")"#;
    let mut decoder = Decoder::new(text);
    let backend = SecretBackend::decode(&mut decoder).unwrap();
    match backend {
        SecretBackend::Agenix { secret_id } => {
            assert_eq!(secret_id, "router-wifi-sae");
        }
        other => panic!("expected Agenix, got {other:?}"),
    }
}

#[test]
fn cluster_secret_binding_decodes_with_name_and_backend() {
    let text = r#"(ClusterSecretBinding router-wifi-pwd (Sops "secrets/wifi.yaml" "passwords/router-wifi"))"#;
    let mut decoder = Decoder::new(text);
    let binding = ClusterSecretBinding::decode(&mut decoder).unwrap();
    assert_eq!(binding.name.as_str(), "router-wifi-pwd");
    match binding.backend {
        SecretBackend::Sops { file, key } => {
            assert_eq!(file.as_str(), "secrets/wifi.yaml");
            assert_eq!(key.as_str(), "passwords/router-wifi");
        }
        other => panic!("expected Sops, got {other:?}"),
    }
}

#[test]
fn secret_purpose_variants_round_trip_through_nota_enum() {
    use horizon_lib::proposal::SecretPurpose::*;
    let cases = [
        ("BinaryCacheSigning", BinaryCacheSigning),
        ("WireguardPrivateKey", WireguardPrivateKey),
        ("NordvpnCredentials", NordvpnCredentials),
        ("WifiPassword", WifiPassword),
        ("EapTlsClientKey", EapTlsClientKey),
        ("GhostMailerPassword", GhostMailerPassword),
        ("GhostStripeKey", GhostStripeKey),
        ("AiProviderApiKey", AiProviderApiKey),
    ];
    for (text, expected) in cases {
        let mut decoder = Decoder::new(text);
        let decoded = SecretPurpose::decode(&mut decoder).unwrap();
        assert_eq!(decoded, expected, "round-trip failed for {text}");
    }
}
