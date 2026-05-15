//! Tests for `node::NodeProposal::project` — derived booleans,
//! pubkey shadows, lid-switch policy, and arch resolution.

use std::collections::BTreeMap;

use horizon_lib::address::{YggAddress, YggSubnet};
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterDomain, ClusterName, ModelName, NodeName, UserName};
use horizon_lib::proposal::{
    Io, Machine, NodePlacement, NodeProjection, NodeProposal, NodePubKeys, NodeServices,
    TailnetControllerRole,
    TailnetMembership, YggPubKeyEntry,
};
use horizon_lib::view::LidSwitchAction;
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{Arch, Bootloader, Keyboard, NodeSpecies};

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn machine_x86() -> Machine {
    Machine {
        arch: Some(Arch::X86_64),
        cores: 4,
        model: None,
        mother_board: None,
        chip_gen: None,
        ram_gb: None,
    }
}

fn io_with_root_disk() -> Io {
    use std::collections::BTreeMap;
    use horizon_lib::disk::{DevicePath, Disk, FsType, MountPath};
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

fn proposal(species: NodeSpecies, size: Magnitude, with_keys: bool) -> NodeProposal {
    NodeProposal {
        species,
        size,
        trust: Magnitude::Max,
        machine: machine_x86(),
        io: io_with_root_disk(),
        pub_keys: pub_keys(with_keys, with_keys),
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
        placement: NodePlacement::Metal {},
    }
}

fn ctx_for(name: &str, trust: Magnitude) -> NodeProjection<'static> {
    static CLUSTER: std::sync::OnceLock<ClusterName> = std::sync::OnceLock::new();
    static DOMAIN: std::sync::OnceLock<ClusterDomain> = std::sync::OnceLock::new();
    let cluster = CLUSTER.get_or_init(|| ClusterName::try_new("goldragon").unwrap());
    let cluster_domain = DOMAIN.get_or_init(|| ClusterDomain::try_new("criome").unwrap());
    NodeProjection {
        name: NodeName::try_new(name).unwrap(),
        cluster,
        cluster_domain,
        trust,
        resolved_arch: Arch::X86_64,
    }
}

fn tailnet_controller_server() -> TailnetControllerRole {
    // Step 11 collapse — base_domain lives on Cluster.tailnet, not
    // per-controller.
    TailnetControllerRole::Server { port: 9443 }
}

#[test]
fn center_node_with_full_keys_is_nix_cache_and_dispatcher() {
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    assert!(node.nix_cache.is_some());
    assert!(!node.is_dispatcher); // center → not a dispatcher (dispatcher is non-center)
    assert!(node.is_fully_trusted);
    assert!(node.nix_pub_key.is_some() && node.yggdrasil.is_some());
    assert!(node.behaves_as.center);
    assert!(node.type_is.center);
}

#[test]
fn edge_node_at_least_medium_with_keys_is_remote_nix_builder() {
    let node = proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true)
        .project(ctx_for("ouranos", Magnitude::Max));
    assert!(node.is_remote_nix_builder);
    assert!(node.is_dispatcher);
    assert!(node.has_video_output);
    assert!(node.behaves_as.edge);
    assert!(node.behaves_as.next_gen);
}

#[test]
fn edge_node_below_medium_is_not_remote_nix_builder() {
    let node = proposal(NodeSpecies::Edge, Magnitude::Min, true)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(!node.is_remote_nix_builder);
}

#[test]
fn node_without_full_pub_keys_is_not_remote_nix_builder() {
    let node = proposal(NodeSpecies::EdgeTesting, Magnitude::Large, false)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(!node.is_remote_nix_builder);
    assert!(node.nix_pub_key.is_none() || node.yggdrasil.is_none());
}

#[test]
fn ladder_booleans_match_input_size() {
    let node = proposal(NodeSpecies::Edge, Magnitude::Large, true)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(node.size.min);
    assert!(node.size.medium);
    assert!(node.size.large);
    assert!(!node.size.max);
}

#[test]
fn lid_switch_policy_for_center_ignores_all_states() {
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    assert!(matches!(node.handle_lid_switch, LidSwitchAction::Ignore));
    assert!(matches!(
        node.handle_lid_switch_external_power,
        LidSwitchAction::Ignore
    ));
}

#[test]
fn lid_switch_policy_for_edge_locks_when_docked_and_on_external_power() {
    let node = proposal(NodeSpecies::Edge, Magnitude::Min, true)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(matches!(
        node.handle_lid_switch_docked,
        LidSwitchAction::Lock
    ));
    // Edge is low_power → on external power, suspend rather than lock.
    assert!(matches!(
        node.handle_lid_switch_external_power,
        LidSwitchAction::Suspend
    ));
    // Default lid action (battery) for non-center is Suspend.
    assert!(matches!(node.handle_lid_switch, LidSwitchAction::Suspend));
}

#[test]
fn nix_cache_present_for_center_node() {
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    let cache = node.nix_cache.as_ref().unwrap();
    assert_eq!(cache.url, "http://nix.prometheus.goldragon.criome");
    assert_eq!(cache.domain.as_str(), "nix.prometheus.goldragon.criome");
}

#[test]
fn nix_cache_absent_for_non_cache_node() {
    let node = proposal(NodeSpecies::Edge, Magnitude::Large, true)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(node.nix_cache.is_none());
}

#[test]
fn tailnet_roles_project_from_proposal_not_node_name() {
    let mut prop = proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true);
    prop.services.tailnet = Some(TailnetMembership::Client);
    prop.services.tailnet_controller = Some(tailnet_controller_server());

    let node = prop.project(ctx_for("arbitrary-node", Magnitude::Max));

    assert_eq!(node.services.tailnet, Some(TailnetMembership::Client));
    assert_eq!(
        node.services.tailnet_controller,
        Some(tailnet_controller_server())
    );
}

#[test]
fn model_is_thinkpad_recognises_known_thinkpads() {
    let mut prop = proposal(NodeSpecies::Edge, Magnitude::Large, true);
    prop.machine.model = Some(ModelName::try_new("ThinkPadT14Gen5Intel").unwrap());
    let node = prop.project(ctx_for("zeus", Magnitude::Max));
    assert!(node.model_is_thinkpad);
}

#[test]
fn model_is_thinkpad_false_for_unknown_models() {
    let mut prop = proposal(NodeSpecies::Edge, Magnitude::Large, true);
    prop.machine.model = Some(ModelName::try_new("RandomLaptopX").unwrap());
    let node = prop.project(ctx_for("zeus", Magnitude::Max));
    assert!(!node.model_is_thinkpad);
}

#[test]
fn contained_arch_resolved_via_placement_host() {
    let mut proposals = BTreeMap::new();
    let host = NodeName::try_new("ouranos").unwrap();
    proposals.insert(host.clone(), proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true));

    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.arch = None;
    pod_proposal.placement = NodePlacement::Contained {
        host: host.clone(),
        user: UserName::try_new("li").unwrap(),
    };
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let resolved = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap();
    assert_eq!(resolved, Arch::X86_64);
}

#[test]
fn contained_arch_unresolvable_when_host_missing() {
    let mut proposals = BTreeMap::new();
    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.arch = None;
    pod_proposal.placement = NodePlacement::Contained {
        host: NodeName::try_new("missing-host").unwrap(),
        user: UserName::try_new("li").unwrap(),
    };
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let error = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap_err();
    assert!(error.to_string().contains("missing super-node"));
}

#[test]
fn metal_arch_unresolvable_when_no_arch_set() {
    let mut proposals = BTreeMap::new();
    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.arch = None;
    // placement defaults to Metal — no host to inherit arch from
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let error = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap_err();
    assert!(error.to_string().contains("no architecture"));
}
