//! Tests for contained-node placement records. These records are the
//! proposal-side contract that cloud-host and container-host modules
//! consume through the projected view.

use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{NodeName, UserName};
use horizon_lib::proposal::{
    ContainedNetwork, ContainedState, NodePlacement, PersistentPath, Resources, Substrate,
    UserNamespacePolicy, VirtualIp,
};
use nota_codec::{Decoder, NotaDecode};

fn resources() -> Resources {
    Resources {
        cores: 2,
        ram_gb: 4,
    }
}

fn network() -> ContainedNetwork {
    ContainedNetwork {
        local_address: VirtualIp::try_new("10.42.0.10").unwrap(),
        host_address: VirtualIp::try_new("10.42.0.1").unwrap(),
    }
}

fn state() -> ContainedState {
    ContainedState {
        persistent_paths: vec![PersistentPath::new("/var/lib/ghost")],
    }
}

fn placement() -> NodePlacement {
    NodePlacement::Contained {
        host: NodeName::try_new("atlas").unwrap(),
        user: UserName::try_new("aria").unwrap(),
        substrate: Substrate::NixosContainer {},
        resources: resources(),
        network: network(),
        state: state(),
        trust: Magnitude::Medium,
        user_namespace_policy: UserNamespacePolicy::PrivateUsersPick,
    }
}

#[test]
fn contained_placement_decodes_from_nota_record() {
    let text = concat!(
        "(Contained atlas aria ",
        "(NixosContainer) ",
        "(Resources 2 4) ",
        r#"(ContainedNetwork "10.42.0.10" "10.42.0.1") "#,
        r#"(ContainedState ["/var/lib/ghost"]) "#,
        "Medium ",
        "PrivateUsersPick)"
    );
    let mut decoder = Decoder::new(text);
    let decoded = NodePlacement::decode(&mut decoder).unwrap();
    assert_eq!(decoded, placement());
}

#[test]
fn contained_placement_round_trips_through_json_with_camel_case_fields() {
    let original = placement();
    let json = serde_json::to_value(&original).unwrap();
    let contained = json
        .as_object()
        .and_then(|object| object.get("Contained"))
        .and_then(serde_json::Value::as_object)
        .expect("contained placement renders as externally tagged object");

    for key in [
        "host",
        "user",
        "substrate",
        "resources",
        "network",
        "state",
        "trust",
        "userNamespacePolicy",
    ] {
        assert!(
            contained.contains_key(key),
            "expected contained placement JSON to carry `{key}`, got keys {:?}",
            contained.keys().collect::<Vec<_>>()
        );
    }

    let recovered: NodePlacement = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn microvm_cloud_hypervisor_is_closed_substrate_variant() {
    let text = "(MicrovmCloudHypervisor)";
    let mut decoder = Decoder::new(text);
    let decoded = Substrate::decode(&mut decoder).unwrap();
    assert_eq!(decoded, Substrate::MicrovmCloudHypervisor {});
}
