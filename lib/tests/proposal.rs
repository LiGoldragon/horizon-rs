//! Tests for `proposal` — the input shapes goldragon emits as
//! cluster-proposal nota.
//!
//! Round-trips a minimal proposal through `nota-codec` and asserts
//! the typed fields decode at the right positions. Per the
//! all-fields-explicit rule, every Optional position needs a token.

use std::collections::BTreeMap;

use horizon_lib::address::{Interface, YggAddress, YggSubnet};
use horizon_lib::io::Io;
use horizon_lib::machine::Machine;
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, DomainName, NodeName, SecretName, UserName};
use horizon_lib::proposal::{
    ClusterProposal, ClusterTrust, NodeProposal, NodePubKeys, NodeServices, RouterInterfaces,
    SecretReference, TailnetControllerRole, UserProposal, WlanBand, WlanStandard, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{
    Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies, Style, UserSpecies,
};
use nota_codec::{Decoder, NotaDecode};

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn machine() -> Machine {
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
        "[] None None false false [] false false None None None (NodeServices None None false))",
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
fn tailnet_controller_server_decodes_with_port_and_base_domain() {
    let text = "(NodeServices Client (Server 9443 \"tailnet.goldragon.criome\") false)";
    let mut decoder = Decoder::new(text);
    let services = NodeServices::decode(&mut decoder).unwrap();

    assert_eq!(
        services.tailnet_controller,
        Some(TailnetControllerRole::Server {
            port: 9443,
            base_domain: DomainName::try_new("tailnet.goldragon.criome").unwrap(),
        })
    );
}

#[test]
fn persona_development_decodes_as_a_single_role_boolean() {
    let text = "(NodeServices Client None true)";
    let mut decoder = Decoder::new(text);
    let services = NodeServices::decode(&mut decoder).unwrap();

    assert_eq!(
        services.tailnet,
        Some(horizon_lib::proposal::TailnetMembership::Client)
    );
    assert!(services.persona_development);
}

#[test]
fn router_interfaces_decode_transitional_wifi_secret_reference() {
    let text =
        "(RouterInterfaces eno1 wlp195s0 TwoG 6 Wifi4 (SecretReference routerWifiSaePasswords))";
    let mut decoder = Decoder::new(text);
    let interfaces = RouterInterfaces::decode(&mut decoder).unwrap();

    assert_eq!(interfaces.wan, Interface::new("eno1"));
    assert_eq!(interfaces.wlan, Interface::new("wlp195s0"));
    assert_eq!(interfaces.wlan_band, WlanBand::TwoG);
    assert_eq!(interfaces.wlan_channel, 6);
    assert_eq!(interfaces.wlan_standard, WlanStandard::Wifi4);
    assert_eq!(
        interfaces.wpa3_sae_password,
        Some(SecretReference {
            name: SecretName::try_new("routerWifiSaePasswords").unwrap(),
        })
    );
}
