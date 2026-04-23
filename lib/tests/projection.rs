//! End-to-end projection test: parse the maisiliym TOML fixture,
//! project from each viewpoint, assert the load-bearing derivations.

use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, NodeName, UserName};
use horizon_lib::{ClusterProposal, Viewpoint};

const FIXTURE: &str = include_str!("fixtures/maisiliym.toml");

fn fixture() -> ClusterProposal {
    toml::from_str(FIXTURE).expect("parse fixture toml")
}

fn project(viewpoint_node: &str) -> horizon_lib::Horizon {
    let proposal = fixture();
    let viewpoint = Viewpoint {
        cluster: ClusterName::try_new("maisiliym").unwrap(),
        node: NodeName::try_new(viewpoint_node).unwrap(),
    };
    proposal.project(&viewpoint).expect("project succeeds")
}

#[test]
fn parses_proposal() {
    let p = fixture();
    assert_eq!(p.nodes.len(), 3);
    assert_eq!(p.users.len(), 1);
    assert_eq!(p.trust.cluster, Magnitude::Max);
}

#[test]
fn ouranos_viewpoint_basics() {
    let h = project("ouranos");
    assert_eq!(h.cluster.name.as_str(), "maisiliym");
    assert_eq!(h.node.name.as_str(), "ouranos");
    assert_eq!(h.ex_nodes.len(), 2);
    assert!(h.ex_nodes.contains_key(&NodeName::try_new("tiger").unwrap()));
    assert!(h.ex_nodes.contains_key(&NodeName::try_new("balboa").unwrap()));
}

#[test]
fn tiger_is_builder_dispatcher_from_ouranos() {
    let h = project("ouranos");
    let tiger = h
        .ex_nodes
        .get(&NodeName::try_new("tiger").unwrap())
        .expect("tiger present");
    assert!(tiger.is_fully_trusted);
    assert!(tiger.has_base_pub_keys);
    assert!(tiger.is_builder, "tiger should be a builder");
    assert!(tiger.is_dispatcher, "tiger should be a dispatcher");
    assert!(!tiger.is_nix_cache, "tiger is edge-testing, not center");
}

#[test]
fn balboa_is_low_trust_no_keys() {
    let h = project("ouranos");
    let balboa = h
        .ex_nodes
        .get(&NodeName::try_new("balboa").unwrap())
        .expect("balboa present");
    assert_eq!(balboa.trust, Magnitude::Min);
    assert!(!balboa.is_fully_trusted);
    assert!(!balboa.has_nix_pub_key);
    assert!(!balboa.has_ygg_pub_key);
    assert!(!balboa.is_builder);
    assert_eq!(balboa.system, horizon_lib::species::System::Aarch64Linux);
}

#[test]
fn ouranos_viewpoint_only_fields_populated() {
    let h = project("ouranos");
    assert!(h.node.io.is_some(), "horizon.node.io must be Some");
    assert!(h.node.computer_is.is_some());
    assert!(h.node.builder_configs.is_some());
    assert!(h.node.admin_ssh_pub_keys.is_some());

    let computer_is = h.node.computer_is.as_ref().unwrap();
    assert!(computer_is.thinkpad_t14_gen5_intel);
    assert!(!computer_is.thinkpad_x230);

    let builders = h.node.builder_configs.as_ref().unwrap();
    assert_eq!(builders.len(), 1, "only tiger qualifies as builder from ouranos");
    assert_eq!(
        builders[0].host_name.as_str(),
        "tiger.maisiliym.criome"
    );
}

#[test]
fn ex_node_has_no_viewpoint_fields() {
    let h = project("ouranos");
    let tiger = h.ex_nodes.get(&NodeName::try_new("tiger").unwrap()).unwrap();
    assert!(tiger.io.is_none());
    assert!(tiger.builder_configs.is_none());
    assert!(tiger.admin_ssh_pub_keys.is_none());
    assert!(tiger.computer_is.is_none());
}

#[test]
fn user_li_from_ouranos_has_pub_key() {
    let h = project("ouranos");
    let li = h.users.get(&UserName::try_new("li").unwrap()).expect("li present");
    assert!(li.has_pub_key, "li has a pubKey for ouranos");
    assert_eq!(li.email_address, "li@maisiliym.criome.net");
    assert_eq!(li.matrix_id, "@li:maisiliym.criome.net");
    assert_eq!(
        li.git_signing_key.as_deref(),
        Some("&7FAFE190D2C749B222B249E54E5A7AD71C1BDDBD")
    );
    assert!(li.is_code_dev);
    assert!(li.is_multimedia_dev);
    assert!(li.use_colemak);
    assert!(li.use_fast_repeat);
}

#[test]
fn user_li_from_balboa_has_no_pub_key() {
    let h = project("balboa");
    let li = h.users.get(&UserName::try_new("li").unwrap()).expect("li present");
    assert!(!li.has_pub_key);
    assert!(li.git_signing_key.is_none());
    assert!(li.ssh_pub_key.is_none());
    // sshPubKeys still lists every node's line:
    assert_eq!(li.ssh_pub_keys.len(), 2);
}

#[test]
fn cluster_trusted_build_keys_only_from_keyed_nodes() {
    let h = project("ouranos");
    // tiger and ouranos have nix pub keys; balboa does not.
    assert_eq!(h.cluster.trusted_build_pub_keys.len(), 2);
    let lines: Vec<&str> = h
        .cluster
        .trusted_build_pub_keys
        .iter()
        .map(|l| l.as_str())
        .collect();
    assert!(lines.iter().any(|l| l.starts_with("ouranos.maisiliym.criome:")));
    assert!(lines.iter().any(|l| l.starts_with("tiger.maisiliym.criome:")));
}

#[test]
fn admin_ssh_keys_come_from_fully_trusted_users_on_trusted_nodes() {
    let h = project("ouranos");
    let admins = h.node.admin_ssh_pub_keys.as_ref().unwrap();
    // li (trust=max) has pub_keys on tiger & ouranos, both fully-trusted.
    assert_eq!(admins.len(), 2);
}

#[test]
fn build_concurrency_role_aware() {
    let h = project("ouranos");
    // ouranos: 12 cores, edge-testing (not center), size=max → cores/2 = 6.
    assert_eq!(h.node.max_jobs, 6);
    assert_eq!(h.node.build_cores, 0);

    let tiger = h.ex_nodes.get(&NodeName::try_new("tiger").unwrap()).unwrap();
    // tiger: 4 cores, edge-testing → cores/2 = 2.
    assert_eq!(tiger.max_jobs, 2);

    let balboa = h.ex_nodes.get(&NodeName::try_new("balboa").unwrap()).unwrap();
    // balboa: 4 cores, center, but size=None → 1.
    assert_eq!(balboa.max_jobs, 1);
}

#[test]
fn round_trip_horizon_toml() {
    let h = project("ouranos");
    let s = toml::to_string(&h).expect("serialize horizon to toml");
    let _: horizon_lib::Horizon = toml::from_str(&s).expect("re-parse horizon toml");
}
