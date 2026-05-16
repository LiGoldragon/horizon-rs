//! Tests for `view::Cluster` — the cluster-level roll-up.

use std::collections::BTreeMap;

use horizon_lib::name::{ClusterDomain, ClusterName};
use horizon_lib::proposal::{SecretBackend, SecretName, SopsFilePath, SopsKeyPath};
use horizon_lib::pub_key::{NixPubKey, NixPubKeyLine};
use horizon_lib::view::Cluster;

fn cluster_name() -> ClusterName {
    ClusterName::try_new("goldragon").unwrap()
}

#[test]
fn cluster_round_trips_name_and_keys() {
    let cluster = Cluster {
        name: cluster_name(),
        domain: ClusterDomain::try_new("criome").unwrap(),
        trusted_build_pub_keys: Vec::new(),
        lan: None,
        resolver: None,
        tailnet: None,
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        secret_bindings: BTreeMap::new(),
    };
    assert_eq!(cluster.name.as_str(), "goldragon");
    assert!(cluster.trusted_build_pub_keys.is_empty());
}

#[test]
fn cluster_collects_trusted_build_pub_keys() {
    let cluster_domain = ClusterDomain::try_new("criome").unwrap();
    let domain = horizon_lib::name::CriomeDomainName::for_node(
        &horizon_lib::name::NodeName::try_new("prometheus").unwrap(),
        &cluster_name(),
        &cluster_domain,
    );
    let key = NixPubKey::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let line: NixPubKeyLine = key.line(&domain);
    let cluster = Cluster {
        name: cluster_name(),
        domain: cluster_domain,
        trusted_build_pub_keys: vec![line.clone()],
        lan: None,
        resolver: None,
        tailnet: None,
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        secret_bindings: BTreeMap::new(),
    };
    assert_eq!(cluster.trusted_build_pub_keys.len(), 1);
    assert!(cluster.trusted_build_pub_keys[0].as_str().contains("prometheus.goldragon.criome:"));
}

#[test]
fn cluster_json_round_trip_carries_every_secret_backend_variant() {
    // Construct a view::Cluster carrying one binding per
    // SecretBackend variant — Sops, SystemdCredential, Agenix —
    // serialize it through serde_json, deserialize back, and
    // confirm the lookup table survives the round-trip with the
    // backend variants intact. This exercises the wire contract
    // between horizon-rs (producer) and CriomOS modules (consumer
    // through `inputs.horizon.cluster.secretBindings`).
    let mut secret_bindings = BTreeMap::new();
    secret_bindings.insert(
        SecretName::try_new("router-wifi-pwd").unwrap(),
        SecretBackend::Sops {
            file: SopsFilePath::try_new("secrets/wifi.yaml").unwrap(),
            key: SopsKeyPath::try_new("passwords/router-wifi").unwrap(),
        },
    );
    secret_bindings.insert(
        SecretName::try_new("ghost-mailer-pwd").unwrap(),
        SecretBackend::SystemdCredential {
            credential_name: "ghost-mailer-password".to_string(),
        },
    );
    secret_bindings.insert(
        SecretName::try_new("agenix-canary").unwrap(),
        SecretBackend::Agenix {
            secret_id: "router-wifi-sae".to_string(),
        },
    );

    let cluster = Cluster {
        name: cluster_name(),
        domain: ClusterDomain::try_new("criome").unwrap(),
        trusted_build_pub_keys: Vec::new(),
        lan: None,
        resolver: None,
        tailnet: None,
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        secret_bindings,
    };

    let json = serde_json::to_string(&cluster).expect("serialize cluster");
    // Field is camelCased on the wire (`secretBindings`) — that's
    // what the Nix consumer reads as `horizon.cluster.secretBindings`.
    assert!(json.contains("\"secretBindings\""), "missing secretBindings key in {json}");

    let decoded: Cluster = serde_json::from_str(&json).expect("deserialize cluster");
    assert_eq!(decoded.secret_bindings.len(), 3);

    let sops = decoded
        .secret_bindings
        .get(&SecretName::try_new("router-wifi-pwd").unwrap())
        .expect("router-wifi-pwd binding present");
    match sops {
        SecretBackend::Sops { file, key } => {
            assert_eq!(file.as_str(), "secrets/wifi.yaml");
            assert_eq!(key.as_str(), "passwords/router-wifi");
        }
        other => panic!("expected Sops, got {other:?}"),
    }

    let systemd = decoded
        .secret_bindings
        .get(&SecretName::try_new("ghost-mailer-pwd").unwrap())
        .expect("ghost-mailer-pwd binding present");
    match systemd {
        SecretBackend::SystemdCredential { credential_name } => {
            assert_eq!(credential_name, "ghost-mailer-password");
        }
        other => panic!("expected SystemdCredential, got {other:?}"),
    }

    let agenix = decoded
        .secret_bindings
        .get(&SecretName::try_new("agenix-canary").unwrap())
        .expect("agenix-canary binding present");
    match agenix {
        SecretBackend::Agenix { secret_id } => {
            assert_eq!(secret_id, "router-wifi-sae");
        }
        other => panic!("expected Agenix, got {other:?}"),
    }
}
