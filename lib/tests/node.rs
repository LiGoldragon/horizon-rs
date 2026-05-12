//! Tests for `node::NodeProposal::project` — derived booleans,
//! pubkey shadows, lid-switch policy, and arch resolution.

use std::collections::BTreeMap;

use horizon_lib::address::{YggAddress, YggSubnet};
use horizon_lib::io::Io;
use horizon_lib::machine::Machine;
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, ClusterTld, ModelName, NodeName, UserName};
use horizon_lib::node::{LidSwitchAction, NodeProjection};
use horizon_lib::proposal::{
    NodeProposal, NodePubKeys, NodeServices, TailnetControllerRole, TailnetMembership,
    YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies};

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

fn io_with_root_disk() -> Io {
    use std::collections::BTreeMap;
    use horizon_lib::io::{DevicePath, Disk, FsType, MountPath};
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
        placement: None,
    }
}

fn ctx_for(name: &str, trust: Magnitude) -> NodeProjection<'static> {
    static CLUSTER: std::sync::OnceLock<ClusterName> = std::sync::OnceLock::new();
    static TLD: std::sync::OnceLock<ClusterTld> = std::sync::OnceLock::new();
    let cluster = CLUSTER.get_or_init(|| ClusterName::try_new("goldragon").unwrap());
    let tld = TLD.get_or_init(ClusterTld::default_criome);
    NodeProjection {
        name: NodeName::try_new(name).unwrap(),
        cluster,
        tld,
        trust,
        resolved_arch: Arch::X86_64,
    }
}

#[test]
fn center_node_with_full_keys_is_nix_cache_and_dispatcher() {
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    assert!(node.is_nix_cache);
    assert!(!node.is_dispatcher); // center → not a dispatcher (dispatcher is non-center)
    assert!(node.is_fully_trusted);
    assert!(node.has_base_pub_keys);
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
    assert!(!node.has_base_pub_keys);
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
fn nix_url_present_when_nix_cache() {
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    let url = node.nix_url.as_ref().unwrap();
    assert_eq!(url, "http://nix.prometheus.goldragon.criome");
    assert_eq!(
        node.nix_cache_domain.as_ref().unwrap().as_str(),
        "nix.prometheus.goldragon.criome",
    );
}

#[test]
fn nix_url_absent_for_non_cache() {
    let node = proposal(NodeSpecies::Edge, Magnitude::Large, true)
        .project(ctx_for("zeus", Magnitude::Max));
    assert!(node.nix_url.is_none());
    assert!(node.nix_cache_domain.is_none());
}

#[test]
fn tailnet_roles_project_from_proposal_not_node_name() {
    let mut prop = proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true);
    prop.services.tailnet = Some(TailnetMembership::Client);
    prop.services.tailnet_controller = Some(TailnetControllerRole::Server);

    let node = prop.project(ctx_for("arbitrary-node", Magnitude::Max));

    assert_eq!(node.services.tailnet, Some(TailnetMembership::Client));
    assert_eq!(
        node.services.tailnet_controller,
        Some(TailnetControllerRole::Server)
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
fn pod_arch_resolved_via_super_node() {
    let mut proposals = BTreeMap::new();
    let host = NodeName::try_new("ouranos").unwrap();
    proposals.insert(host.clone(), proposal(NodeSpecies::EdgeTesting, Magnitude::Large, true));

    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.species = MachineSpecies::Pod;
    pod_proposal.machine.arch = None;
    pod_proposal.machine.super_node = Some(host.clone());
    pod_proposal.machine.super_user = Some(UserName::try_new("li").unwrap());
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let resolved = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap();
    assert_eq!(resolved, Arch::X86_64);
}

#[test]
fn pod_arch_unresolvable_when_super_node_missing() {
    let mut proposals = BTreeMap::new();
    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.species = MachineSpecies::Pod;
    pod_proposal.machine.arch = None;
    pod_proposal.machine.super_node = Some(NodeName::try_new("missing-host").unwrap());
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let error = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap_err();
    assert!(error.to_string().contains("missing super-node"));
}

#[test]
fn pod_arch_unresolvable_when_super_node_pointer_absent() {
    let mut proposals = BTreeMap::new();
    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.species = MachineSpecies::Pod;
    pod_proposal.machine.arch = None;
    pod_proposal.machine.super_node = None;
    let pod_name = NodeName::try_new("pod-1").unwrap();
    proposals.insert(pod_name.clone(), pod_proposal);

    let error = proposals[&pod_name]
        .resolve_arch(&pod_name, &proposals)
        .unwrap_err();
    assert!(error.to_string().contains("no super-node"));
}

#[test]
fn remote_nix_builder_projects_to_build_host_capability() {
    // A center node with full keys is_remote_nix_builder; capabilities
    // .build_host should be populated with max_jobs, cores_per_job, and
    // trust matching the projected node.
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    assert!(node.is_remote_nix_builder);
    let build_host = node
        .capabilities
        .build_host
        .as_ref()
        .expect("remote nix builder should have build_host capability");
    assert_eq!(build_host.max_jobs, node.max_jobs);
    assert_eq!(build_host.cores_per_job, node.build_cores);
    assert_eq!(build_host.trust, node.trust);
}

#[test]
fn non_builder_node_has_no_build_host_capability() {
    // An edge node is_not_remote_nix_builder; capabilities.build_host
    // should be None.
    let node = proposal(NodeSpecies::Edge, Magnitude::Min, true)
        .project(ctx_for("edge1", Magnitude::Max));
    assert!(!node.is_remote_nix_builder);
    assert!(node.capabilities.build_host.is_none());
}

#[test]
fn metal_machine_projects_to_metal_placement() {
    use horizon_lib::placement::NodePlacement;
    let node = proposal(NodeSpecies::Center, Magnitude::Min, true)
        .project(ctx_for("prometheus", Magnitude::Max));
    match &node.placement {
        NodePlacement::Metal(metal) => {
            assert_eq!(metal.arch, Arch::X86_64);
            assert!(metal.model.is_none());
        }
        other => panic!("expected NodePlacement::Metal, got {:?}", other),
    }
}

#[test]
fn proposal_authored_placement_overrides_legacy_machine_species_derivation() {
    use horizon_lib::placement::{
        Contained, ContainerResources, ContainmentSubstrate, NodePlacement, UserNamespacePolicy,
    };
    // A Metal-shaped legacy machine still gets a `Contained` projection
    // when the proposal authors `placement` directly. Authoring wins.
    let mut p = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    p.machine.species = MachineSpecies::Metal; // legacy says metal
    p.placement = Some(NodePlacement::Contained(Contained {
        host: NodeName::try_new("ouranos").unwrap(),
        substrate: ContainmentSubstrate::MicroVm,
        resources: ContainerResources { cores: 4, ram_gb: 16 },
        network: None,
        state: None,
        trust: Magnitude::Max.ladder(),
        user_namespace_policy: UserNamespacePolicy::PrivateUsersPick {},
        super_user: None,
    }));
    let node = p.project(ctx_for("ghost", Magnitude::Medium));
    match &node.placement {
        NodePlacement::Contained(c) => {
            assert_eq!(c.substrate, ContainmentSubstrate::MicroVm);
            assert_eq!(c.resources.cores, 4);
            assert_eq!(c.resources.ram_gb, 16);
            assert_eq!(c.host.as_str(), "ouranos");
        }
        other => panic!(
            "authored placement should override legacy derivation, got {:?}",
            other
        ),
    }
}

#[test]
fn pod_machine_projects_to_contained_placement_with_nixos_container() {
    use horizon_lib::placement::{ContainmentSubstrate, NodePlacement, UserNamespacePolicy};
    let mut pod_proposal = proposal(NodeSpecies::Edge, Magnitude::Min, true);
    pod_proposal.machine.species = MachineSpecies::Pod;
    pod_proposal.machine.super_node = Some(NodeName::try_new("ouranos").unwrap());
    pod_proposal.machine.super_user = Some(UserName::try_new("li").unwrap());
    pod_proposal.machine.cores = 2;
    pod_proposal.machine.ram_gb = Some(8);

    let node = pod_proposal.project(ctx_for("pod-1", Magnitude::Min));
    match &node.placement {
        NodePlacement::Contained(contained) => {
            assert_eq!(contained.host.as_str(), "ouranos");
            assert_eq!(contained.substrate, ContainmentSubstrate::NixosContainer);
            assert_eq!(contained.resources.cores, 2);
            assert_eq!(contained.resources.ram_gb, 8);
            assert!(matches!(
                contained.user_namespace_policy,
                UserNamespacePolicy::PrivateUsersPick {}
            ));
            assert_eq!(
                contained.super_user.as_ref().map(|u| u.as_str()),
                Some("li")
            );
            // Legacy Pod proposals don't author network/state — both
            // are None during the compat-shim cycle.
            assert!(contained.network.is_none());
            assert!(contained.state.is_none());
        }
        other => panic!("expected NodePlacement::Contained, got {:?}", other),
    }
}
