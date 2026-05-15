//! Tests for `proposal` — the input shapes goldragon emits as
//! cluster-proposal nota.
//!
//! Round-trips a minimal proposal through `nota-codec` and asserts
//! the typed fields decode at the right positions. Per the
//! all-fields-explicit rule, every Optional position needs a token.

use std::collections::BTreeMap;

use horizon_lib::address::{YggAddress, YggSubnet};
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterDomain, ClusterName, NodeName, UserName};
use horizon_lib::proposal::{
    ClusterProposal, ClusterTrust, Io, Machine, NodePlacement, NodeProposal, NodePubKeys,
    NodeServices, TailnetControllerRole, UserProposal, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{Arch, Bootloader, Keyboard, NodeSpecies, Style, UserSpecies};
use nota_codec::{Decoder, NotaDecode};

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn machine() -> Machine {
    Machine {
        arch: Some(Arch::X86_64),
        cores: 4,
        model: None,
        mother_board: None,
        chip_gen: None,
        ram_gb: None,
    }
}

fn io() -> Io {
    Io {
        keyboard: Keyboard::Qwerty,
        bootloader: Bootloader::Uefi,
        disks: BTreeMap::new(),
        swap_devices: Vec::new(),
    }
}

fn pub_keys() -> NodePubKeys {
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

fn node_proposal(species: NodeSpecies, size: Magnitude) -> NodeProposal {
    NodeProposal {
        species,
        size,
        trust: Magnitude::Max,
        machine: machine(),
        io: io(),
        pub_keys: pub_keys(),
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

fn cluster_proposal() -> ClusterProposal {
    let mut nodes = BTreeMap::new();
    nodes.insert(
        NodeName::try_new("ouranos").unwrap(),
        node_proposal(NodeSpecies::EdgeTesting, Magnitude::Large),
    );
    ClusterProposal {
        nodes,
        users: BTreeMap::new(),
        domains: BTreeMap::new(),
        trust: ClusterTrust {
            cluster: Magnitude::Max,
            clusters: BTreeMap::new(),
            nodes: BTreeMap::new(),
            users: BTreeMap::new(),
        },
        secret_bindings: Vec::new(),
        lan: None,
        resolver: None,
        tailnet: None,
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        domain: ClusterDomain::try_new("criome").unwrap(),
    }
}

#[test]
fn cluster_proposal_constructs_with_minimum_fields() {
    let proposal = cluster_proposal();
    assert_eq!(proposal.nodes.len(), 1);
    assert!(proposal.users.is_empty());
    assert!(matches!(proposal.trust.cluster, Magnitude::Max));
}

#[test]
fn node_proposal_carries_all_input_fields() {
    let proposal = node_proposal(NodeSpecies::Center, Magnitude::Min);
    assert!(matches!(proposal.species, NodeSpecies::Center));
    assert!(matches!(proposal.size, Magnitude::Min));
    assert!(matches!(proposal.trust, Magnitude::Max));
    assert!(proposal.pub_keys.nix.is_some());
}

#[test]
fn user_proposal_decodes_from_minimal_nota_record() {
    let text = "(UserProposal Code Max Colemak Emacs LiGoldragon None [] None None)";
    let mut decoder = Decoder::new(text);
    let user = UserProposal::decode(&mut decoder).unwrap();
    assert!(matches!(user.species, UserSpecies::Code));
    assert!(matches!(user.size, Magnitude::Max));
    assert!(matches!(user.keyboard, Keyboard::Colemak));
    assert!(matches!(user.style, Style::Emacs));
    assert_eq!(user.github_id.as_ref().unwrap().as_str(), "LiGoldragon");
    assert!(user.fast_repeat.is_none());
    assert!(user.pub_keys.is_empty());
    assert!(user.editor.is_none());
    assert!(user.text_size.is_none());
}

#[test]
fn cluster_trust_decodes_per_user_magnitude_with_renamed_variants() {
    let text = "(ClusterTrust Max [] [] [(Entry bird Medium) (Entry li Max)])";
    let mut decoder = Decoder::new(text);
    let trust = ClusterTrust::decode(&mut decoder).unwrap();
    assert!(matches!(trust.cluster, Magnitude::Max));
    let bird = UserName::try_new("bird").unwrap();
    let li = UserName::try_new("li").unwrap();
    assert!(matches!(trust.users.get(&bird), Some(Magnitude::Medium)));
    assert!(matches!(trust.users.get(&li), Some(Magnitude::Max)));
}

#[test]
fn node_proposal_size_zero_decodes_via_renamed_variant() {
    // After the audit Tier 2 rename, balboa's size token in
    // datom.nota is `Zero` (was `None`). Verify the new variant
    // decodes at the size position.
    let text = concat!(
        "(NodeProposal ",
        "Center Zero Min ",
        "(Machine Metal Arm64 4 None None None None None None) ",
        "(Io Qwerty Uboot [] []) ",
        "(NodePubKeys \"AAA=\" None None) ",
        "[] None None false false [] false false None None None (NodeServices None None))",
    );
    let mut decoder = Decoder::new(text);
    let node = NodeProposal::decode(&mut decoder).unwrap();
    assert!(matches!(node.species, NodeSpecies::Center));
    assert!(matches!(node.size, Magnitude::Zero));
    assert!(matches!(node.trust, Magnitude::Min));
    let cluster_name = ClusterName::try_new("c").unwrap();
    assert!(!cluster_name.as_str().is_empty()); // sanity touch
}

#[test]
fn tailnet_controller_server_decodes_with_port_only() {
    // Step 11 collapse — base_domain moved to Cluster.tailnet, so the
    // controller variant carries port alone.
    let text = "(NodeServices Client (Server 9443))";
    let mut decoder = Decoder::new(text);
    let services = NodeServices::decode(&mut decoder).unwrap();

    assert_eq!(
        services.tailnet_controller,
        Some(TailnetControllerRole::Server { port: 9443 })
    );
}
