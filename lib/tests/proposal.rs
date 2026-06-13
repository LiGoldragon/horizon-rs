//! Tests for `proposal` — the input shapes goldragon emits as
//! cluster-proposal nota.
//!
//! Round-trips a minimal proposal through `nota-next` and asserts
//! the typed fields decode at the right positions. Per the
//! all-fields-explicit rule, every Optional position needs a token.

use std::collections::BTreeMap;

use horizon_lib::address::{Interface, YggAddress, YggSubnet};
use horizon_lib::io::Io;
use horizon_lib::machine::Machine;
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterName, NodeName, SecretName, UserName, WirelessNetworkName};
use horizon_lib::proposal::{
    BackupWireless, ClusterProposal, ClusterTrust, NodeProposal, NodePubKeys, NodeService,
    PersonaDevelopmentCapability, RouterInterfaces, SecretReference, UserProposal, WlanBand,
    WlanStandard, YggPubKeyEntry,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{
    Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies, Style, UserSpecies,
};
use nota_next::{NotaDecode, NotaSource};

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
        disk_gb: None,
        location: None,
    }
}

fn io() -> Io {
    Io {
        keyboard: Keyboard::Qwerty,
        bootloader: Bootloader::Uefi,
        disks: BTreeMap::new(),
        swap_devices: Vec::new(),
        compressed_swap: None,
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
        services: Vec::new(),
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

fn decode<Value>(text: &str) -> Result<Value, nota_next::NotaDecodeError>
where
    Value: NotaDecode,
{
    NotaSource::new(text).parse()
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
    let text = "(Code Max Colemak Emacs (Some [LiGoldragon]) None {} None None)";
    let user = decode::<UserProposal>(text).unwrap();
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
fn user_proposal_rejects_quote_delimited_string() {
    let text = "(Code Max Colemak Emacs (Some \"LiGoldragon\") None {} None None)";
    let error = decode::<UserProposal>(text).unwrap_err();

    assert!(
        error.to_string().contains("quotation mark"),
        "unexpected error: {error}",
    );
}

#[test]
fn cluster_trust_decodes_per_user_magnitude_with_renamed_variants() {
    let text = "(Max {} {} {bird Medium li Max})";
    let trust = decode::<ClusterTrust>(text).unwrap();
    assert!(matches!(trust.cluster, Magnitude::Max));
    let bird = UserName::try_new("bird").unwrap();
    let li = UserName::try_new("li").unwrap();
    assert!(matches!(trust.users.get(&bird), Some(Magnitude::Medium)));
    assert!(matches!(trust.users.get(&li), Some(Magnitude::Max)));
}

#[test]
fn io_decodes_legacy_shape_with_swap_defaults() {
    let text = "(Qwerty Uefi {} [([/dev/disk/by-uuid/swap])])";
    let io = decode::<Io>(text).unwrap();

    assert!(matches!(io.keyboard, Keyboard::Qwerty));
    assert_eq!(io.swap_devices.len(), 1);
    assert_eq!(io.swap_devices[0].device.as_str(), "/dev/disk/by-uuid/swap");
    assert_eq!(io.swap_devices[0].size_mebibytes, None);
    assert!(io.compressed_swap.is_none());
}

#[test]
fn io_decodes_swapfile_size_and_compressed_swap() {
    let text = "(Colemak Uefi {} [([/swapfile] (Some 32768))] (Some (25)))";
    let io = decode::<Io>(text).unwrap();

    assert!(matches!(io.keyboard, Keyboard::Colemak));
    assert_eq!(io.swap_devices.len(), 1);
    assert_eq!(io.swap_devices[0].device.as_str(), "/swapfile");
    assert_eq!(io.swap_devices[0].size_mebibytes, Some(32768));
    assert_eq!(io.compressed_swap.unwrap().memory_percent, 25);
}

#[test]
fn node_proposal_size_zero_decodes_via_renamed_variant() {
    // After the audit Tier 2 rename, balboa's size token in
    // datom.nota is `Zero` (was `None`). Verify the new variant
    // decodes at the size position.
    let text = concat!(
        "(",
        "Center Zero Min ",
        "(Metal (Some Arm64) 4 None None None None None None None None) ",
        "(Qwerty Uboot {} []) ",
        "([AAA=] None None) ",
        "[] None None False False [] False False None None [])",
    );
    let node = decode::<NodeProposal>(text).unwrap();
    assert!(matches!(node.species, NodeSpecies::Center));
    assert!(matches!(node.size, Magnitude::Zero));
    assert!(matches!(node.trust, Magnitude::Min));
    let cluster_name = ClusterName::try_new("c").unwrap();
    assert!(!cluster_name.as_str().is_empty()); // sanity touch
}

#[test]
fn service_vector_decodes_tailnet_controller_without_parameters() {
    let text = "[(TailnetClient) (TailnetController)]";
    let services = decode::<Vec<NodeService>>(text).unwrap();

    assert_eq!(
        services,
        vec![
            NodeService::TailnetClient {},
            NodeService::TailnetController {},
        ]
    );
}

#[test]
fn persona_development_decodes_as_nested_capability_vector() {
    let text = "[(PersonaDevelopment [(GitoliteServer)])]";
    let services = decode::<Vec<NodeService>>(text).unwrap();

    assert_eq!(
        services,
        vec![NodeService::PersonaDevelopment {
            capabilities: vec![PersonaDevelopmentCapability::GitoliteServer {}],
        }]
    );
}

#[test]
fn nix_builder_decodes_capacity_policy_inside_role_variant() {
    let text = "[(NixBuilder (Some 6)) (NixCache)]";
    let services = decode::<Vec<NodeService>>(text).unwrap();

    assert_eq!(
        services,
        vec![
            NodeService::NixBuilder {
                maximum_jobs: Some(6),
            },
            NodeService::NixCache {},
        ]
    );
}

#[test]
fn router_interfaces_decode_transitional_wifi_secret_reference() {
    let text = "(eno1 wlp195s0 TwoG 6 Wifi4 (Some (routerWifiSaePasswords)) None)";
    let interfaces = decode::<RouterInterfaces>(text).unwrap();

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
    assert_eq!(interfaces.backup_wireless, None);
}

#[test]
fn router_interfaces_decode_backup_wireless_access_point() {
    let text = "(eno1 wlp195s0 TwoG 6 Wifi4 (Some (routerWifiSaePasswords)) (Some (wlp199s0f0u4 [CRIOM Backup] TwoG 11 Wifi4 (routerBackupWifiPassword))))";
    let interfaces = decode::<RouterInterfaces>(text).unwrap();

    assert_eq!(
        interfaces.backup_wireless,
        Some(BackupWireless {
            interface: Interface::new("wlp199s0f0u4"),
            network_name: WirelessNetworkName::try_new("CRIOM Backup").unwrap(),
            band: WlanBand::TwoG,
            channel: 11,
            standard: WlanStandard::Wifi4,
            password: SecretReference {
                name: SecretName::try_new("routerBackupWifiPassword").unwrap(),
            },
        })
    );
}
