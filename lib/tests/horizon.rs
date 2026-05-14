//! End-to-end tests for `horizon::ClusterProposal::project` —
//! exercises proposal + node + user + cluster + magnitude in one go.

use std::collections::BTreeMap;

use horizon_lib::address::{YggAddress, YggSubnet};
use horizon_lib::disk::{DevicePath, Disk, FsType, MountPath};
use horizon_lib::error::Error;
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, DomainName, NodeName, UserName};
use horizon_lib::proposal::{
    ClusterProposal, ClusterTrust, Io, Machine, NodeProposal, NodePubKeys, NodeServices,
    TailnetConfig, TailnetControllerRole, UserProposal, UserPubKeyEntry, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies, Style, UserSpecies};
use horizon_lib::Viewpoint;

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn machine_x86() -> Machine {
    Machine {
        species: MachineSpecies::Metal,
        arch: Some(Arch::X86_64),
        cores: 4,
        model: None,
        mother_board: None,
        super_node: None,
        super_user: None,
        chip_gen: None,
        ram_gb: None,
    }
}

fn io() -> Io {
    let mut disks = BTreeMap::new();
    disks.insert(
        MountPath::new("/"),
        Disk {
            device: DevicePath::new("/dev/sda1"),
            fs_type: FsType::Ext4,
            options: Vec::new(),
        },
    );
    Io {
        keyboard: Keyboard::Colemak,
        bootloader: Bootloader::Uefi,
        disks,
        swap_devices: Vec::new(),
    }
}

fn tailnet_controller_server() -> TailnetControllerRole {
    // Step 11 collapse — base_domain lives on Cluster.tailnet, not
    // per-controller.
    TailnetControllerRole::Server { port: 9443 }
}

fn cluster_tailnet() -> TailnetConfig {
    TailnetConfig {
        base_domain: DomainName::try_new("tailnet.goldragon.criome").unwrap(),
        tls: None,
    }
}

fn pub_keys(nix: bool, ygg: bool) -> NodePubKeys {
    NodePubKeys {
        ssh: SshPubKey::try_new("AAA=").unwrap(),
        nix: nix.then(|| NixPubKey::try_new(NIX_KEY).unwrap()),
        yggdrasil: ygg.then(|| YggPubKeyEntry {
            pub_key: YggPubKey::try_new("a".repeat(64)).unwrap(),
            address: YggAddress::try_new("200::1").unwrap(),
            subnet: YggSubnet::try_new("300:ca41:6b12:fba").unwrap(),
        }),
    }
}

fn node_proposal(species: NodeSpecies, size: Magnitude, full_keys: bool) -> NodeProposal {
    NodeProposal {
        species,
        size,
        trust: Magnitude::Max,
        machine: machine_x86(),
        io: io(),
        pub_keys: pub_keys(full_keys, full_keys),
        link_local_ips: Vec::new(),
        node_ip: None,
        wireguard_pub_key: None,
        nordvpn: false,
        wifi_cert: false,
        wireguard_untrusted_proxies: Vec::new(),
        wants_printing: false,
        wants_hw_video_accel: false,
        router_interfaces: None,
        online: None,
        number_of_build_cores: None,
        services: NodeServices::default(),
    }
}

fn user_pubkey_entry() -> UserPubKeyEntry {
    UserPubKeyEntry {
        ssh: SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap(),
        keygrip: horizon_lib::name::Keygrip::try_new(
            "0123456789ABCDEF0123456789ABCDEF01234567",
        )
        .unwrap(),
    }
}

fn user_proposal(species: UserSpecies) -> UserProposal {
    let mut pub_keys = BTreeMap::new();
    pub_keys.insert(NodeName::try_new("ouranos").unwrap(), user_pubkey_entry());
    UserProposal {
        species,
        size: Magnitude::Max,
        keyboard: Keyboard::Colemak,
        style: Style::Emacs,
        github_id: None,
        fast_repeat: None,
        pub_keys,
        editor: None,
        text_size: None,
    }
}

fn cluster_proposal(viewpoint_trust: Magnitude) -> ClusterProposal {
    let mut nodes = BTreeMap::new();
    nodes.insert(
        NodeName::try_new("ouranos").unwrap(),
        node_proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true),
    );
    nodes.insert(
        NodeName::try_new("prometheus").unwrap(),
        node_proposal(NodeSpecies::Center, Magnitude::Min, true),
    );
    nodes.insert(
        NodeName::try_new("zeus").unwrap(),
        node_proposal(NodeSpecies::Edge, Magnitude::Min, false),
    );

    let mut users = BTreeMap::new();
    users.insert(UserName::try_new("li").unwrap(), user_proposal(UserSpecies::Unlimited));

    let mut node_trust = BTreeMap::new();
    node_trust.insert(
        NodeName::try_new("ouranos").unwrap(),
        viewpoint_trust,
    );
    node_trust.insert(
        NodeName::try_new("prometheus").unwrap(),
        Magnitude::Max,
    );
    node_trust.insert(
        NodeName::try_new("zeus").unwrap(),
        Magnitude::Max,
    );

    let mut user_trust = BTreeMap::new();
    user_trust.insert(UserName::try_new("li").unwrap(), Magnitude::Max);

    ClusterProposal {
        nodes,
        users,
        domains: BTreeMap::new(),
        trust: ClusterTrust {
            cluster: Magnitude::Max,
            clusters: BTreeMap::new(),
            nodes: node_trust,
            users: user_trust,
        },
        secret_bindings: Vec::new(),
        lan: None,
        resolver: None,
        // Set so that the tailnet-controller tests below get past the
        // cluster.tailnet-required check and reach the singleton check.
        tailnet: Some(cluster_tailnet()),
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
    }
}

fn viewpoint(node: &str) -> Viewpoint {
    Viewpoint {
        cluster: ClusterName::try_new("goldragon").unwrap(),
        node: NodeName::try_new(node).unwrap(),
    }
}

#[test]
fn project_returns_horizon_with_viewpoint_node_filled_and_others_in_ex_nodes() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();

    assert_eq!(horizon.node.name.as_str(), "ouranos");
    assert!(horizon.ex_nodes.contains_key(&NodeName::try_new("prometheus").unwrap()));
    assert!(horizon.ex_nodes.contains_key(&NodeName::try_new("zeus").unwrap()));
    assert!(!horizon.ex_nodes.contains_key(&NodeName::try_new("ouranos").unwrap()));
}

#[test]
fn project_viewpoint_node_carries_filled_io_and_use_colemak() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert!(horizon.node.io.is_some());
    assert_eq!(horizon.node.use_colemak, Some(true));
}

#[test]
fn project_ex_nodes_have_io_left_unfilled() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    let prometheus = &horizon.ex_nodes[&NodeName::try_new("prometheus").unwrap()];
    assert!(prometheus.io.is_none());
    assert!(prometheus.use_colemak.is_none());
}

#[test]
fn project_cluster_collects_nix_pub_key_lines_from_keyed_nodes() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    // ouranos and prometheus have keys; zeus does not.
    assert_eq!(horizon.cluster.trusted_build_pub_keys.len(), 2);
    let lines: Vec<String> = horizon
        .cluster
        .trusted_build_pub_keys
        .iter()
        .map(|line| line.as_str().to_string())
        .collect();
    assert!(lines.iter().any(|l| l.contains("ouranos.goldragon.criome:")));
    assert!(lines.iter().any(|l| l.contains("prometheus.goldragon.criome:")));
}

#[test]
fn project_node_with_zero_trust_is_excluded_from_horizon() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal.trust.nodes.insert(
        NodeName::try_new("zeus").unwrap(),
        Magnitude::Zero,
    );
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert!(!horizon.ex_nodes.contains_key(&NodeName::try_new("zeus").unwrap()));
    // ouranos and prometheus still present.
    assert!(horizon.ex_nodes.contains_key(&NodeName::try_new("prometheus").unwrap()));
}

#[test]
fn project_user_with_zero_trust_is_excluded_from_horizon() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal.trust.users.insert(
        UserName::try_new("li").unwrap(),
        Magnitude::Zero,
    );
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert!(horizon.users.is_empty());
}

#[test]
fn project_rejects_viewpoint_not_in_cluster() {
    let proposal = cluster_proposal(Magnitude::Max);
    let error = proposal
        .project(&viewpoint("nonexistent"))
        .unwrap_err();
    assert!(matches!(error, Error::NodeNotInCluster(_)));
}

#[test]
fn project_rejects_multiple_active_tailnet_controller_servers() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    for name in ["ouranos", "prometheus"] {
        proposal
            .nodes
            .get_mut(&NodeName::try_new(name).unwrap())
            .unwrap()
            .services
            .tailnet_controller = Some(tailnet_controller_server());
    }

    let error = proposal.project(&viewpoint("ouranos")).unwrap_err();

    assert!(matches!(
        error,
        Error::MultipleTailnetControllers { first, second }
            if first.as_str() == "ouranos" && second.as_str() == "prometheus"
    ));
}

#[test]
fn project_ignores_zero_trust_tailnet_controller_when_validating_singleton() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    for name in ["ouranos", "zeus"] {
        proposal
            .nodes
            .get_mut(&NodeName::try_new(name).unwrap())
            .unwrap()
            .services
            .tailnet_controller = Some(tailnet_controller_server());
    }
    proposal.trust.nodes.insert(
        NodeName::try_new("zeus").unwrap(),
        Magnitude::Zero,
    );

    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();

    assert_eq!(
        horizon.node.services.tailnet_controller,
        Some(tailnet_controller_server())
    );
    assert!(!horizon.ex_nodes.contains_key(&NodeName::try_new("zeus").unwrap()));
}

#[test]
fn project_collects_dispatchers_ssh_pub_keys_from_non_center_trusted_nodes() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    // Dispatchers are non-center, fully-trusted, sized at least Min.
    // ouranos (viewpoint, EdgeTesting Large Max) is itself a dispatcher
    // but doesn't appear in its own ex_nodes_ssh_pub_keys list.
    // zeus (Edge Min Max) is a dispatcher.
    let dispatchers = horizon
        .node
        .dispatchers_ssh_pub_keys
        .as_ref()
        .expect("viewpoint node should have dispatcher list filled");
    assert_eq!(dispatchers.len(), 1);
}

#[test]
fn project_collects_admin_ssh_pub_keys_from_max_trust_users_on_trusted_nodes() {
    let proposal = cluster_proposal(Magnitude::Max);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    let admin_keys = horizon
        .node
        .admin_ssh_pub_keys
        .as_ref()
        .expect("viewpoint node should have admin keys list filled");
    assert_eq!(admin_keys.len(), 1);
}

#[test]
fn project_user_size_floors_at_viewpoint_node_size() {
    // Make ouranos size = Medium and verify Max user gets Medium.
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal
        .nodes
        .get_mut(&NodeName::try_new("ouranos").unwrap())
        .unwrap()
        .size = Magnitude::Medium;
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    let user = &horizon.users[&UserName::try_new("li").unwrap()];
    assert!(user.size.medium);
    assert!(!user.size.large);
}

#[test]
fn project_node_trust_clamps_at_cluster_floor() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal.trust.cluster = Magnitude::Min;
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    // Cluster trust floor of Min clamps every node's trust ladder.
    assert!(horizon.node.trust.min);
    assert!(!horizon.node.trust.max);
}
