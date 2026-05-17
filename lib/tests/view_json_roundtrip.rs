//! JSON round-trip witnesses for every `view::*` record kind on the
//! Nix-consumer wire.
//!
//! The view side is the contract surface every Nix module under
//! `inputs.horizon` reads. Audit 83 found zero JSON round-trip tests on
//! these records — the same gap that let the `vpn_profiles`
//! double-wrapping bug ship. This file seeds the discipline:
//!
//! - Each top-level `view::*` record kind gets a per-record round-trip
//!   (construct → `serde_json::to_value` → `from_value` → equal).
//! - Optional fields are exercised in both populated and absent shapes
//!   so `#[serde(skip_serializing_if = "Option::is_none")]` and
//!   `#[serde(default)]` don't drift.
//! - Camel-case wire keys are asserted at least once per record so a
//!   rename-without-renaming silent regression fails loudly.
//! - An end-to-end horizon round-trip projects a small `ClusterProposal`
//!   and round-trips the resulting `view::Horizon` through JSON.
//!
//! These tests do **not** lock the field-set itself — adding a field is
//! a normal evolution. They lock the *codec*: whatever the type has,
//! must survive serialise + parse + serialise unchanged.

use std::collections::BTreeMap;

use horizon_lib::address::{LinkLocalIp, YggAddress, YggSubnet};
use horizon_lib::disk::{DevicePath, Disk, FsType, MountPath};
use horizon_lib::magnitude::{AtLeast, Magnitude};
use horizon_lib::name::{
    ClusterDomain, ClusterName, CriomeDomainName, DomainName, EmailAddress, GithubId, Keygrip,
    MatrixId, ModelName, NodeName, UserName,
};
use horizon_lib::proposal::{
    ClusterProposal, ClusterTrust, ContainedNetwork, ContainedState, Io, Machine, NodePlacement,
    NodeProposal, NodePubKeys, NodeServices, Resources, Substrate,
    TailnetConfig as ProposalTailnetConfig, UserNamespacePolicy, UserProposal, UserPubKeyEntry,
    VirtualIp, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, SshPubKeyLine, YggPubKey};
use horizon_lib::species::{
    Arch, Bootloader, Editor, Keyboard, NodeSpecies, Style, System, TextSize, UserSpecies,
};
use horizon_lib::view::cluster::TailnetConfig as ViewTailnetConfig;
use horizon_lib::view::{
    BehavesAs, BuilderConfig, Cluster, Horizon, NixCache, Node, ProjectedNodeView, ResolverPolicy,
    User,
};
use horizon_lib::{HorizonProposal, Viewpoint};
use serde_json::Value;

// ── small helpers ──────────────────────────────────────────────────────

fn horizon_proposal() -> HorizonProposal {
    HorizonProposal::from_parts(
        "TestOperator",
        "criome",
        "criome.net",
        "10.18.0.0/24",
        "10.18.0.1",
        "10.18.0.100",
        "10.18.0.240",
        "TEMPORARY: single-router IPv4 LAN until IPv6-first networking lands",
    )
    .unwrap()
}

fn assert_camel_key(value: &Value, key: &str) {
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("expected JSON object, got {value}"));
    assert!(
        object.contains_key(key),
        "expected camelCase key `{key}` in JSON object; got keys {:?}",
        object.keys().collect::<Vec<_>>(),
    );
}

// ── per-record fixtures ────────────────────────────────────────────────

fn nix_cache() -> NixCache {
    NixCache {
        domain: CriomeDomainName::for_node(
            &NodeName::try_new("ouranos").unwrap(),
            &ClusterName::try_new("goldragon").unwrap(),
            &ClusterDomain::try_new("criome").unwrap(),
        )
        .nix_subdomain(),
        url: "http://nix.ouranos.goldragon.criome".to_string(),
    }
}

fn machine_view() -> Machine {
    Machine {
        arch: Some(Arch::X86_64),
        cores: 12,
        model: Some(ModelName::try_new("ThinkPadT14Gen5Intel").unwrap()),
        mother_board: None,
        chip_gen: Some(12),
        ram_gb: Some(32),
    }
}

fn io_view() -> Io {
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

fn behaves_as_edge() -> BehavesAs {
    BehavesAs {
        center: false,
        router: false,
        edge: true,
        next_gen: false,
        low_power: true,
        bare_metal: true,
        virtual_machine: false,
        iso: false,
        large_ai: false,
    }
}

fn cluster_view_minimal() -> Cluster {
    Cluster {
        name: ClusterName::try_new("goldragon").unwrap(),
        domain: ClusterDomain::try_new("criome").unwrap(),
        trusted_build_pub_keys: Vec::new(),
        lan: None,
        resolver: ResolverPolicy {
            listens: Vec::new(),
        },
        tailnet: Some(ViewTailnetConfig {
            base_domain: DomainName::try_new("tailnet.goldragon.criome").unwrap(),
            tls: None,
        }),
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        secret_bindings: BTreeMap::new(),
    }
}

fn user_view_minimal() -> User {
    User {
        name: UserName::try_new("li").unwrap(),
        species: UserSpecies::Unlimited,
        size: Magnitude::Max.ladder(),
        trust: Magnitude::Max.ladder(),
        keyboard: Keyboard::Colemak,
        style: Style::Emacs,
        github_id: Some(GithubId::try_new("LiGoldragon").unwrap()),
        pub_keys: BTreeMap::new(),

        has_pub_key: false,
        email_address: EmailAddress::try_new("li@goldragon.criome.net").unwrap(),
        matrix_id: MatrixId::try_new("@li:goldragon.criome.net").unwrap(),
        git_signing_key: None,
        use_colemak: true,
        use_fast_repeat: false,
        is_multimedia_dev: false,
        is_code_dev: true,
        preferred_editor: Editor::Emacs,
        text_size: TextSize::Medium,
        ssh_pub_keys: Vec::new(),
        ssh_pub_key: None,

        extra_groups: vec!["audio".to_string(), "wheel".to_string()],
        enable_linger: false,
    }
}

fn projected_node_view_fixture() -> ProjectedNodeView {
    ProjectedNodeView {
        name: NodeName::try_new("publication-pod").unwrap(),
        user: UserName::try_new("li").unwrap(),
        cores: 2,
        ram_gb: Some(4),
        substrate: Substrate::NixosContainer {},
        resources: Resources {
            cores: 2,
            ram_gb: 4,
        },
        network: ContainedNetwork {
            local_address: VirtualIp::try_new("10.42.0.10").unwrap(),
            host_address: VirtualIp::try_new("10.42.0.1").unwrap(),
        },
        state: ContainedState {
            persistent_paths: Vec::new(),
        },
        trust: Magnitude::Medium,
        user_namespace_policy: UserNamespacePolicy::PrivateUsersPick,
    }
}

fn ssh_pub_key() -> SshPubKey {
    SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap()
}

fn ssh_pub_key_line() -> SshPubKeyLine {
    ssh_pub_key().line()
}

fn nix_pub_key() -> NixPubKey {
    NixPubKey::try_new("A".repeat(44)).unwrap()
}

fn builder_config_fixture() -> BuilderConfig {
    let host = CriomeDomainName::for_node(
        &NodeName::try_new("prometheus").unwrap(),
        &ClusterName::try_new("goldragon").unwrap(),
        &ClusterDomain::try_new("criome").unwrap(),
    );
    BuilderConfig {
        host_name: host,
        ssh_user: "nix-ssh".to_string(),
        ssh_key: "/etc/ssh/ssh_host_ed25519_key".to_string(),
        supported_features: vec!["big-parallel".to_string(), "kvm".to_string()],
        system: System::X86_64Linux,
        systems: Vec::new(),
        max_jobs: 8,
        public_host_key: ssh_pub_key(),
        public_host_key_line: ssh_pub_key_line(),
    }
}

fn node_view_fixture() -> Node {
    let cluster = ClusterName::try_new("goldragon").unwrap();
    let cluster_domain = ClusterDomain::try_new("criome").unwrap();
    let name = NodeName::try_new("ouranos").unwrap();

    let behaves_as = behaves_as_edge();

    Node {
        name: name.clone(),
        species: NodeSpecies::Edge,
        size: Magnitude::Large.ladder(),
        trust: Magnitude::Max.ladder(),
        machine: machine_view(),
        link_local_ips: Vec::new(),
        node_ip: None,
        wireguard_pub_key: None,
        nordvpn: false,
        wifi_cert: false,
        wants_printing: false,
        wants_hw_video_accel: false,
        router_interfaces: None,
        services: NodeServices::default(),
        placement: NodePlacement::Metal {},

        criome_domain_name: CriomeDomainName::for_node(&name, &cluster, &cluster_domain),
        system: System::X86_64Linux,
        max_jobs: 4,

        ssh_pub_key: ssh_pub_key(),
        nix_pub_key: Some(nix_pub_key()),
        yggdrasil: Some(YggPubKeyEntry {
            pub_key: YggPubKey::try_new("a".repeat(64)).unwrap(),
            address: YggAddress::try_new("200::1").unwrap(),
            subnet: YggSubnet::try_new("300:ca41:6b12:fba").unwrap(),
        }),

        is_fully_trusted: true,
        is_remote_nix_builder: false,
        is_dispatcher: true,
        is_large_edge: true,
        enable_network_manager: true,
        chip_is_intel: true,
        model_is_thinkpad: true,

        ssh_pub_key_line: ssh_pub_key_line(),
        nix_pub_key_line: Some(nix_pub_key().line(&CriomeDomainName::for_node(
            &name,
            &cluster,
            &cluster_domain,
        ))),
        nix_cache: Some(nix_cache()),

        behaves_as,

        io: None,
        use_colemak: None,
        builder_configs: None,
        cache_urls: None,
        ex_nodes_ssh_pub_keys: None,
        dispatchers_ssh_pub_keys: None,
        admin_ssh_pub_keys: None,
        wireguard_untrusted_proxies: None,
    }
}

fn node_view_fixture_with_viewpoint_fields() -> Node {
    let mut node = node_view_fixture();
    node.io = Some(io_view());
    node.use_colemak = Some(true);
    node.builder_configs = Some(Vec::new());
    node.cache_urls = Some(Vec::new());
    node.ex_nodes_ssh_pub_keys = Some(Vec::new());
    node.dispatchers_ssh_pub_keys = Some(Vec::new());
    node.admin_ssh_pub_keys = Some(Vec::new());
    node.wireguard_untrusted_proxies = Some(Vec::new());
    node
}

// ── per-record round-trip tests ────────────────────────────────────────

#[test]
fn nix_cache_round_trips_through_json_and_carries_camel_case_keys() {
    let original = nix_cache();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "domain");
    assert_camel_key(&json, "url");
    let recovered: NixCache = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn machine_round_trips_through_json_with_optional_fields_present() {
    let original = machine_view();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "arch");
    assert_camel_key(&json, "cores");
    assert_camel_key(&json, "model");
    assert_camel_key(&json, "motherBoard");
    assert_camel_key(&json, "chipGen");
    assert_camel_key(&json, "ramGb");
    let recovered: Machine = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn io_round_trips_through_json_and_carries_camel_case_keys() {
    let original = io_view();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "keyboard");
    assert_camel_key(&json, "bootloader");
    assert_camel_key(&json, "disks");
    let recovered: Io = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn behaves_as_round_trips_through_json_with_camel_case_keys() {
    let original = behaves_as_edge();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "nextGen");
    assert_camel_key(&json, "lowPower");
    assert_camel_key(&json, "bareMetal");
    assert_camel_key(&json, "virtualMachine");
    assert_camel_key(&json, "largeAi");
    let recovered: BehavesAs = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn node_species_renders_as_pascal_case_string_on_the_wire() {
    // After dropping TypeIs, NodeSpecies is the load-bearing way Nix
    // consumers know what kind of node this is. Gate sites should read
    // `node.species == "Center"` etc., so the wire form must be the
    // PascalCase variant tag, not anything else.
    for (variant, expected) in [
        (NodeSpecies::Center, "Center"),
        (NodeSpecies::Edge, "Edge"),
        (NodeSpecies::EdgeTesting, "EdgeTesting"),
        (NodeSpecies::CloudHost, "CloudHost"),
        (NodeSpecies::Hybrid, "Hybrid"),
        (NodeSpecies::LargeAi, "LargeAi"),
        (NodeSpecies::LargeAiRouter, "LargeAiRouter"),
        (NodeSpecies::MediaBroadcast, "MediaBroadcast"),
        (NodeSpecies::Router, "Router"),
        (NodeSpecies::RouterTesting, "RouterTesting"),
        (NodeSpecies::Publication, "Publication"),
    ] {
        let json = serde_json::to_value(variant).unwrap();
        let recovered: NodeSpecies = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(recovered, variant);
        assert_eq!(
            json.as_str().expect("NodeSpecies renders as string"),
            expected,
        );
    }
}

#[test]
fn builder_config_round_trips_through_json_with_camel_case_keys() {
    let original = builder_config_fixture();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "hostName");
    assert_camel_key(&json, "sshUser");
    assert_camel_key(&json, "sshKey");
    assert_camel_key(&json, "supportedFeatures");
    assert_camel_key(&json, "system");
    assert_camel_key(&json, "maxJobs");
    assert_camel_key(&json, "publicHostKey");
    assert_camel_key(&json, "publicHostKeyLine");
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: BuilderConfig = serde_json::from_slice(&bytes).unwrap();
    let bytes_again = serde_json::to_vec(&recovered).unwrap();
    assert_eq!(
        bytes, bytes_again,
        "BuilderConfig serialisation is not stable across round-trip"
    );
}

#[test]
fn projected_node_view_round_trips_through_json() {
    let original = projected_node_view_fixture();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "name");
    assert_camel_key(&json, "user");
    assert_camel_key(&json, "cores");
    assert_camel_key(&json, "ramGb");
    assert_camel_key(&json, "substrate");
    assert_camel_key(&json, "resources");
    assert_camel_key(&json, "network");
    assert_camel_key(&json, "state");
    assert_camel_key(&json, "userNamespacePolicy");
    let recovered: ProjectedNodeView = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}

#[test]
fn cluster_view_round_trips_through_json_with_camel_case_keys() {
    let original = cluster_view_minimal();
    let json = serde_json::to_value(&original).unwrap();
    assert_camel_key(&json, "trustedBuildPubKeys");
    assert_camel_key(&json, "aiProviders");
    assert_camel_key(&json, "vpnProfiles");
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: Cluster = serde_json::from_slice(&bytes).unwrap();
    let bytes_again = serde_json::to_vec(&recovered).unwrap();
    assert_eq!(
        bytes, bytes_again,
        "Cluster serialisation is not stable across round-trip"
    );
}

#[test]
fn user_view_round_trips_through_json_with_camel_case_keys() {
    let original = user_view_minimal();
    let json = serde_json::to_value(&original).unwrap();
    for key in [
        "hasPubKey",
        "emailAddress",
        "matrixId",
        "useColemak",
        "useFastRepeat",
        "isMultimediaDev",
        "isCodeDev",
        "preferredEditor",
        "textSize",
        "sshPubKeys",
        "extraGroups",
        "enableLinger",
        "githubId",
    ] {
        assert_camel_key(&json, key);
    }
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: User = serde_json::from_slice(&bytes).unwrap();
    let bytes_again = serde_json::to_vec(&recovered).unwrap();
    assert_eq!(
        bytes, bytes_again,
        "User serialisation is not stable across round-trip"
    );
}

#[test]
fn node_view_round_trips_through_json_with_only_always_derived_fields() {
    let original = node_view_fixture();
    let json = serde_json::to_value(&original).unwrap();
    for key in [
        "linkLocalIps",
        "nodeIp",
        "wireguardPubKey",
        "wifiCert",
        "wantsPrinting",
        "wantsHwVideoAccel",
        "routerInterfaces",
        "criomeDomainName",
        "system",
        "maxJobs",
        "sshPubKey",
        "nixPubKey",
        "yggdrasil",
        "isFullyTrusted",
        "isRemoteNixBuilder",
        "isDispatcher",
        "isLargeEdge",
        "enableNetworkManager",
        "chipIsIntel",
        "modelIsThinkpad",
        "sshPubKeyLine",
        "nixPubKeyLine",
        "nixCache",
        "behavesAs",
        "species",
    ] {
        assert_camel_key(&json, key);
    }
    // Viewpoint-only optionals must skip when None — io / useColemak / etc
    // not present in the absent shape.
    let object = json.as_object().unwrap();
    for key in [
        "io",
        "useColemak",
        "builderConfigs",
        "cacheUrls",
        "exNodesSshPubKeys",
        "dispatchersSshPubKeys",
        "adminSshPubKeys",
        "wireguardUntrustedProxies",
    ] {
        assert!(
            !object.contains_key(key),
            "viewpoint-only field `{key}` must be skipped when None",
        );
    }
    // Codec round-trip — byte-stable on second serialisation.
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: Node = serde_json::from_slice(&bytes).unwrap();
    let bytes_again = serde_json::to_vec(&recovered).unwrap();
    assert_eq!(bytes, bytes_again);
}

#[test]
fn node_view_round_trips_through_json_with_viewpoint_fields_populated() {
    let original = node_view_fixture_with_viewpoint_fields();
    let json = serde_json::to_value(&original).unwrap();
    // With the viewpoint fields populated, the camelCase keys must surface.
    for key in [
        "io",
        "useColemak",
        "builderConfigs",
        "cacheUrls",
        "exNodesSshPubKeys",
        "dispatchersSshPubKeys",
        "adminSshPubKeys",
        "wireguardUntrustedProxies",
    ] {
        assert_camel_key(&json, key);
    }
    let bytes = serde_json::to_vec(&original).unwrap();
    let recovered: Node = serde_json::from_slice(&bytes).unwrap();
    let bytes_again = serde_json::to_vec(&recovered).unwrap();
    assert_eq!(bytes, bytes_again);
}

// ── end-to-end horizon round-trip ──────────────────────────────────────

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn proposal_machine() -> Machine {
    Machine {
        arch: Some(Arch::X86_64),
        cores: 4,
        model: Some(ModelName::try_new("ThinkPadT14Gen5Intel").unwrap()),
        mother_board: None,
        chip_gen: Some(12),
        ram_gb: Some(32),
    }
}

fn proposal_io() -> Io {
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

fn node_pub_keys() -> NodePubKeys {
    NodePubKeys {
        ssh: SshPubKey::try_new("AAA=").unwrap(),
        nix: Some(NixPubKey::try_new(NIX_KEY).unwrap()),
        yggdrasil: Some(YggPubKeyEntry {
            pub_key: YggPubKey::try_new("a".repeat(64)).unwrap(),
            address: YggAddress::try_new("200::1").unwrap(),
            subnet: YggSubnet::try_new("300:ca41:6b12:fba").unwrap(),
        }),
    }
}

fn node_proposal(species: NodeSpecies) -> NodeProposal {
    NodeProposal {
        species,
        size: Magnitude::Large,
        trust: Magnitude::Max,
        machine: proposal_machine(),
        io: proposal_io(),
        pub_keys: node_pub_keys(),
        link_local_ips: vec![LinkLocalIp {
            iface: horizon_lib::address::Interface::new("eth0"),
            suffix: "1".to_string(),
        }],
        node_ip: None,
        wireguard_pub_key: None,
        nordvpn: false,
        wifi_cert: false,
        wireguard_untrusted_proxies: Vec::new(),
        wants_printing: false,
        wants_hw_video_accel: false,
        router_interfaces: None,
        online: None,
        number_of_build_cores: Some(4),
        services: NodeServices::default(),
        placement: NodePlacement::Metal {},
    }
}

fn user_proposal() -> UserProposal {
    let mut pub_keys = BTreeMap::new();
    pub_keys.insert(
        NodeName::try_new("ouranos").unwrap(),
        UserPubKeyEntry {
            ssh: SshPubKey::try_new("AAAAC3NzaC1lZDI1NTE5AAAA").unwrap(),
            keygrip: Keygrip::try_new("0123456789ABCDEF0123456789ABCDEF01234567").unwrap(),
        },
    );
    UserProposal {
        species: UserSpecies::Unlimited,
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

fn cluster_proposal() -> ClusterProposal {
    let mut nodes = BTreeMap::new();
    nodes.insert(
        NodeName::try_new("ouranos").unwrap(),
        node_proposal(NodeSpecies::EdgeTesting),
    );
    nodes.insert(
        NodeName::try_new("prometheus").unwrap(),
        node_proposal(NodeSpecies::Center),
    );

    let mut users = BTreeMap::new();
    users.insert(UserName::try_new("li").unwrap(), user_proposal());

    let mut node_trust = BTreeMap::new();
    node_trust.insert(NodeName::try_new("ouranos").unwrap(), Magnitude::Max);
    node_trust.insert(NodeName::try_new("prometheus").unwrap(), Magnitude::Max);

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
        tailnet: Some(ProposalTailnetConfig { tls: None }),
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
    }
}

#[test]
fn horizon_end_to_end_round_trips_through_json_byte_stable() {
    let proposal = cluster_proposal();
    let viewpoint = Viewpoint {
        cluster: ClusterName::try_new("goldragon").unwrap(),
        node: NodeName::try_new("ouranos").unwrap(),
    };
    let horizon: Horizon = proposal
        .project(&horizon_proposal(), &viewpoint)
        .expect("project");

    // First serialisation is the canonical form.
    let bytes = serde_json::to_vec(&horizon).expect("serialise horizon");
    let recovered: Horizon = serde_json::from_slice(&bytes).expect("parse horizon");
    // Second serialisation must match the first byte-for-byte.
    let bytes_again = serde_json::to_vec(&recovered).expect("re-serialise horizon");
    assert_eq!(
        bytes, bytes_again,
        "horizon JSON is not stable across round-trip",
    );

    // Sanity: the top-level keys consumers depend on.
    let value: Value = serde_json::from_slice(&bytes).unwrap();
    for key in ["cluster", "node", "exNodes", "users", "containedNodes"] {
        assert_camel_key(&value, key);
    }
}

#[test]
fn horizon_end_to_end_node_system_renders_as_nix_system_tuple() {
    // Specific witness for the FIX 1 rename surfacing through the full
    // projection pipeline: every node's `system` field must read as the
    // dashed Nix tuple shape, not the PascalCase Rust identifier.
    let proposal = cluster_proposal();
    let viewpoint = Viewpoint {
        cluster: ClusterName::try_new("goldragon").unwrap(),
        node: NodeName::try_new("ouranos").unwrap(),
    };
    let horizon: Horizon = proposal.project(&horizon_proposal(), &viewpoint).unwrap();
    let value = serde_json::to_value(&horizon).unwrap();
    let system = value
        .get("node")
        .and_then(|n| n.get("system"))
        .and_then(|s| s.as_str())
        .expect("horizon.node.system is a string");
    assert_eq!(system, "x86_64-linux");
}

#[test]
fn at_least_round_trips_through_json_with_camel_case_keys() {
    // AtLeast is reused on Node.size, Node.trust, User.size, User.trust.
    // The wire shape is a small but load-bearing record; lock the codec.
    let original = AtLeast {
        min: true,
        medium: true,
        large: true,
        max: false,
    };
    let json = serde_json::to_value(original).unwrap();
    for key in ["min", "medium", "large", "max"] {
        assert_camel_key(&json, key);
    }
    let recovered: AtLeast = serde_json::from_value(json).unwrap();
    assert_eq!(recovered, original);
}
