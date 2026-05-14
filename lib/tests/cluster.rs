//! Tests for `view::Cluster` — the cluster-level roll-up.

use horizon_lib::view::Cluster;
use horizon_lib::name::ClusterName;
use horizon_lib::pub_key::{NixPubKey, NixPubKeyLine};

fn cluster_name() -> ClusterName {
    ClusterName::try_new("goldragon").unwrap()
}

#[test]
fn cluster_round_trips_name_and_keys() {
    let cluster = Cluster {
        name: cluster_name(),
        trusted_build_pub_keys: Vec::new(),
        lan: None,
        resolver: None,
        tailnet: None,
    };
    assert_eq!(cluster.name.as_str(), "goldragon");
    assert!(cluster.trusted_build_pub_keys.is_empty());
}

#[test]
fn cluster_collects_trusted_build_pub_keys() {
    let domain = horizon_lib::name::CriomeDomainName::for_node(
        &horizon_lib::name::NodeName::try_new("prometheus").unwrap(),
        &cluster_name(),
    );
    let key = NixPubKey::try_new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let line: NixPubKeyLine = key.line(&domain);
    let cluster = Cluster {
        name: cluster_name(),
        trusted_build_pub_keys: vec![line.clone()],
        lan: None,
        resolver: None,
        tailnet: None,
    };
    assert_eq!(cluster.trusted_build_pub_keys.len(), 1);
    assert!(cluster.trusted_build_pub_keys[0].as_str().contains("prometheus.goldragon.criome:"));
}
