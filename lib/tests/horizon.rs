//! End-to-end tests for `horizon::ClusterProposal::project` —
//! exercises proposal + node + user + cluster + magnitude in one go.

use std::collections::BTreeMap;

use horizon_lib::Viewpoint;
use horizon_lib::address::{Interface, NodeIp, TapSubnet, YggAddress, YggSubnet};
use horizon_lib::error::Error;
use horizon_lib::io::{DevicePath, Disk, FsType, Io, MountPath};
use horizon_lib::machine::{Location, Machine};
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, NodeName, SecretName, UserName, WirelessNetworkName};
use horizon_lib::proposal::{
    BackupWireless, ClusterProposal, ClusterTrust, KvmAvailability, MaximumGuests, NodeProposal,
    NodePubKeys, NodeService, RouterInterfaces, SecretReference, UserProposal, UserPubKeyEntry,
    WlanBand, WlanStandard, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{
    Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies, Style, UserSpecies,
};

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
        disk_gb: None,
        location: None,
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
        compressed_swap: None,
    }
}

fn tailnet_controller_service() -> NodeService {
    NodeService::TailnetController {}
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
        services: Vec::new(),
    }
}

fn user_pubkey_entry() -> UserPubKeyEntry {
    UserPubKeyEntry {
        ssh: SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap(),
        keygrip: horizon_lib::name::Keygrip::try_new("0123456789ABCDEF0123456789ABCDEF01234567")
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
    users.insert(
        UserName::try_new("li").unwrap(),
        user_proposal(UserSpecies::Unlimited),
    );

    let mut node_trust = BTreeMap::new();
    node_trust.insert(NodeName::try_new("ouranos").unwrap(), viewpoint_trust);
    node_trust.insert(NodeName::try_new("prometheus").unwrap(), Magnitude::Max);
    node_trust.insert(NodeName::try_new("zeus").unwrap(), Magnitude::Max);

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
    assert!(
        horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("prometheus").unwrap())
    );
    assert!(
        horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("zeus").unwrap())
    );
    assert!(
        !horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("ouranos").unwrap())
    );
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
    assert!(
        lines
            .iter()
            .any(|l| l.contains("ouranos.goldragon.criome:"))
    );
    assert!(
        lines
            .iter()
            .any(|l| l.contains("prometheus.goldragon.criome:"))
    );
}

#[test]
fn project_node_with_zero_trust_is_excluded_from_horizon() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal
        .trust
        .nodes
        .insert(NodeName::try_new("zeus").unwrap(), Magnitude::Zero);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert!(
        !horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("zeus").unwrap())
    );
    // ouranos and prometheus still present.
    assert!(
        horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("prometheus").unwrap())
    );
}

#[test]
fn project_user_with_zero_trust_is_excluded_from_horizon() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal
        .trust
        .users
        .insert(UserName::try_new("li").unwrap(), Magnitude::Zero);
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert!(horizon.users.is_empty());
}

#[test]
fn project_rejects_viewpoint_not_in_cluster() {
    let proposal = cluster_proposal(Magnitude::Max);
    let error = proposal.project(&viewpoint("nonexistent")).unwrap_err();
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
            .push(tailnet_controller_service());
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
            .push(tailnet_controller_service());
    }
    proposal
        .trust
        .nodes
        .insert(NodeName::try_new("zeus").unwrap(), Magnitude::Zero);

    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();

    assert_eq!(horizon.node.services, vec![tailnet_controller_service()]);
    assert!(
        !horizon
            .ex_nodes
            .contains_key(&NodeName::try_new("zeus").unwrap())
    );
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

#[test]
fn project_preserves_router_wifi_secret_reference() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    let router_interfaces = RouterInterfaces {
        wan: Interface::new("eno1"),
        wlan: Interface::new("wlp195s0"),
        wlan_band: WlanBand::TwoG,
        wlan_channel: 6,
        wlan_standard: WlanStandard::Wifi4,
        wpa3_sae_password: Some(SecretReference {
            name: SecretName::try_new("routerWifiSaePasswords").unwrap(),
        }),
        backup_wireless: Some(BackupWireless {
            interface: Interface::new("wlp199s0f0u4"),
            network_name: WirelessNetworkName::try_new("CRIOM Backup").unwrap(),
            band: WlanBand::TwoG,
            channel: 11,
            standard: WlanStandard::Wifi4,
            password: SecretReference {
                name: SecretName::try_new("routerBackupWifiPassword").unwrap(),
            },
        }),
    };
    proposal
        .nodes
        .get_mut(&NodeName::try_new("prometheus").unwrap())
        .unwrap()
        .router_interfaces = Some(router_interfaces.clone());

    let horizon = proposal.project(&viewpoint("prometheus")).unwrap();

    assert_eq!(horizon.node.router_interfaces, Some(router_interfaces));
}

/// A test-VM Pod hosted on `prometheus` with a real root disk (NOT
/// tmpfs), declaring its own disk size and physical location. This is
/// horizon-rs's own test fixture (host `prometheus`, cluster
/// `goldragon`); it does NOT mirror the `mercury` declaration in
/// `CriomOS-test-cluster/clusters/fieldlab.nota`.
fn test_vm_pod() -> NodeProposal {
    let mut disks = BTreeMap::new();
    disks.insert(
        MountPath::new("/"),
        Disk {
            device: DevicePath::new("/dev/vda"),
            fs_type: FsType::Ext4,
            options: Vec::new(),
        },
    );
    let real_disk_io = Io {
        keyboard: Keyboard::Qwerty,
        bootloader: Bootloader::Uefi,
        disks,
        swap_devices: Vec::new(),
        compressed_swap: None,
    };

    NodeProposal {
        species: NodeSpecies::TestVm,
        size: Magnitude::Min,
        trust: Magnitude::Max,
        machine: Machine {
            species: MachineSpecies::Pod,
            arch: Some(Arch::X86_64),
            cores: 4,
            model: None,
            mother_board: None,
            super_node: Some(NodeName::try_new("prometheus").unwrap()),
            super_user: Some(UserName::try_new("li").unwrap()),
            chip_gen: None,
            ram_gb: Some(8),
            disk_gb: Some(40),
            location: Some(Location::new("home-lab")),
        },
        io: real_disk_io,
        pub_keys: pub_keys(true, true),
        link_local_ips: Vec::new(),
        node_ip: Some(NodeIp::try_new("10.77.0.7/24").unwrap()),
        wireguard_pub_key: None,
        nordvpn: false,
        wifi_cert: false,
        wireguard_untrusted_proxies: Vec::new(),
        wants_printing: false,
        wants_hw_video_accel: false,
        router_interfaces: None,
        online: None,
        services: vec![NodeService::TailnetClient {}],
    }
}

/// A horizon-rs unit test of `TestVm` projection, driven entirely by
/// horizon-rs's own test `cluster_proposal` (host `prometheus`, cluster
/// `goldragon`, derived domain `mercury.goldragon.criome`). It asserts
/// the lean derived profile, the host/location/disk machine facts
/// surviving projection, and the derived Criome domain. It does NOT
/// mirror the `mercury` declaration in
/// `CriomOS-test-cluster/clusters/fieldlab.nota`.
#[test]
fn project_test_vm_pod_derives_lean_profile_and_carries_host_location_disk() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), test_vm_pod());
    proposal
        .trust
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), Magnitude::Max);

    let horizon = proposal.project(&viewpoint("mercury")).unwrap();
    let mercury = &horizon.node;

    // Species + the lean derived behaves-as profile: test_vm and
    // virtual_machine (from the Pod substrate) are the ONLY role facets
    // set. It is NOT an edge/center/router node, so the heavy desktop /
    // server stacks never derive onto the guest.
    assert!(matches!(mercury.species, NodeSpecies::TestVm));
    assert!(mercury.behaves_as.test_vm);
    assert!(mercury.behaves_as.virtual_machine);
    assert!(!mercury.behaves_as.edge);
    assert!(!mercury.behaves_as.center);
    assert!(!mercury.behaves_as.router);
    assert!(!mercury.behaves_as.large_ai);
    assert!(!mercury.behaves_as.next_gen);
    assert!(!mercury.behaves_as.low_power);
    assert!(!mercury.behaves_as.bare_metal);
    // A Pod with a real root disk is not an installer image.
    assert!(!mercury.behaves_as.iso);

    // type_is reflects only the TestVm role.
    assert!(mercury.type_is.test_vm);
    assert!(!mercury.type_is.edge);
    assert!(!mercury.type_is.edge_testing);
    assert!(!mercury.type_is.center);
    assert!(!mercury.type_is.router);

    // Machine facts survive projection unchanged: the host
    // (`super_node`), the declared virtual-disk size, and the location.
    assert_eq!(
        mercury.machine.super_node.as_ref().unwrap().as_str(),
        "prometheus"
    );
    assert_eq!(mercury.machine.disk_gb, Some(40));
    assert_eq!(
        mercury.machine.location.as_ref().unwrap().as_str(),
        "home-lab"
    );
    // Pod arch is inherited from the resolved host arch.
    assert_eq!(mercury.machine.arch, Some(Arch::X86_64));

    // Its own routed address and the derived Criome domain — the
    // address lojix deploys to with no special path.
    assert_eq!(
        mercury
            .node_ip
            .as_ref()
            .unwrap()
            .clone()
            .ipnet()
            .to_string(),
        "10.77.0.7/24"
    );
    assert_eq!(
        mercury.criome_domain_name.as_str(),
        "mercury.goldragon.criome"
    );
}

/// The cluster-authored VM-host capability a host declares: one sliced
/// tap subnet, hardware-acceleration availability, and a concurrent-guest
/// ceiling. Built in the test (Spirit [dqg3]) — no shared fixture carries
/// a `VmHost` service yet.
fn vm_host_service() -> NodeService {
    NodeService::VmHost {
        guest_subnet: TapSubnet::try_new("169.254.100.0/22").unwrap(),
        kvm: KvmAvailability::Available,
        maximum_guests: Some(MaximumGuests::new(4)),
    }
}

/// PATTERN — the host-viewpoint interface invariant: projecting from the
/// VM HOST's viewpoint exposes exactly the data `mkVmTest` reads —
/// (a) the host's own `VmHost` capability (tap subnet / KVM / capacity)
/// on `horizon.node.services`, and (b) the host→guest exNode relation
/// (`super_node == host && behaves_as.test_vm`) on `horizon.ex_nodes`.
/// Driven entirely by horizon-rs's own `cluster_proposal` (host
/// `prometheus`, guest `mercury`, cluster `goldragon`); it does NOT
/// mirror `CriomOS-test-cluster/clusters/fieldlab.nota`.
#[test]
fn project_host_viewpoint_exposes_vm_host_capability_and_guest_relation() {
    // A host (`prometheus`) declaring its VmHost capability, and a
    // TestVm Pod guest (`mercury`) hosted on it. Both built here.
    let mut proposal = cluster_proposal(Magnitude::Max);
    proposal
        .nodes
        .get_mut(&NodeName::try_new("prometheus").unwrap())
        .unwrap()
        .services
        .push(vm_host_service());
    proposal
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), test_vm_pod());
    proposal
        .trust
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), Magnitude::Max);

    // Project from the HOST's viewpoint.
    let horizon = proposal.project(&viewpoint("prometheus")).unwrap();

    // (a) The host's own projected VmHost service carries the
    // cluster-authored guest_subnet / kvm / maximum_guests. Read it
    // through the typed projection accessor, not by re-matching the raw
    // vector — the data the generator reads off `horizon.node.services`.
    let capability = horizon
        .node
        .vm_host_capability()
        .expect("host projection should expose its VmHost capability");
    assert_eq!(
        capability.guest_subnet.ipnet().to_string(),
        "169.254.100.0/22"
    );
    assert_eq!(capability.kvm, KvmAvailability::Available);
    assert_eq!(capability.maximum_guests, Some(MaximumGuests::new(4)));

    // (b) The host→guest exNode relation: the guest appears in the
    // host's ex_nodes, names this host as its super_node, and carries
    // the test_vm facet — exactly the fold `mkVmTest` runs to discover
    // its guests.
    let mercury = &horizon.ex_nodes[&NodeName::try_new("mercury").unwrap()];
    assert_eq!(
        mercury.machine.super_node.as_ref().unwrap().as_str(),
        "prometheus"
    );
    assert!(mercury.behaves_as.test_vm);
}

/// PATTERN — the Pod-super-node-exists invariant: a Pod (test-VM guest)
/// whose `super_node` names a host ABSENT from the cluster must fail
/// projection with `Error::MissingSuperNode`, even when the Pod's arch
/// is explicit (which short-circuits `resolve_arch` before its own
/// existence check). The host→guest graph must be total. Fixture built
/// in the test (Spirit [dqg3]): a Pod pointing at a non-existent host.
#[test]
fn project_rejects_pod_with_explicit_arch_and_absent_super_node() {
    let mut proposal = cluster_proposal(Magnitude::Max);
    // A test-VM Pod whose super_node names a host that is NOT in the
    // cluster. `test_vm_pod()` carries an explicit arch (Some(X86_64)),
    // so the failure must come from the dedicated invariant, not arch
    // resolution.
    let mut orphan_guest = test_vm_pod();
    orphan_guest.machine.super_node = Some(NodeName::try_new("nonexistent-host").unwrap());
    proposal
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), orphan_guest);
    proposal
        .trust
        .nodes
        .insert(NodeName::try_new("mercury").unwrap(), Magnitude::Max);

    let error = proposal.project(&viewpoint("prometheus")).unwrap_err();

    assert!(matches!(
        error,
        Error::MissingSuperNode(guest, host)
            if guest.as_str() == "mercury" && host.as_str() == "nonexistent-host"
    ));
}
