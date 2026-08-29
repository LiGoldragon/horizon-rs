//! Datomic root coverage for Horizon's authored cluster proposal data.

use std::collections::BTreeMap;

use datomic::{Datomic, TextEdge};
use horizon_lib::{
    address::{TapSubnet, YggAddress, YggSubnet},
    domain::DomainConfiguration,
    io::Io,
    machine::Machine,
    magnitude::Magnitude,
    name::{NodeName, UserName},
    proposal::{
        ClusterProposal, ClusterTrust, KvmAvailability, MaximumGuests, NodeProposal, NodePubKeys,
        NodeService, PersonaDevelopmentCapability, YggPubKeyEntry,
    },
    pub_key::{NixPubKey, SshPubKey, YggPubKey},
    species::{Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies},
};
use protos::Text;

const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

fn node() -> NodeProposal {
    NodeProposal {
        species: NodeSpecies::EdgeTesting,
        size: Magnitude::Large,
        trust: Magnitude::Max,
        machine: Machine {
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
            super_nodes: Vec::new(),
        },
        io: Io {
            keyboard: Keyboard::Qwerty,
            bootloader: Bootloader::Uefi,
            disks: BTreeMap::new(),
            swap_devices: Vec::new(),
            compressed_swap: None,
        },
        pub_keys: NodePubKeys {
            ssh: SshPubKey::try_new("AAA=").expect("valid ssh key"),
            nix: Some(NixPubKey::try_new(NIX_KEY).expect("valid nix key")),
            yggdrasil: Some(YggPubKeyEntry {
                pub_key: YggPubKey::try_new("a".repeat(64)).expect("valid ygg key"),
                address: YggAddress::try_new("200::1").expect("valid ygg address"),
                subnet: YggSubnet::try_new("300:ca41:6b12:fba").expect("valid ygg subnet"),
            }),
        },
        link_local_ips: Vec::new(),
        node_ip: None,
        wireguard_pub_key: None,
        nordvpn: false,
        wifi_cert: false,
        wireguard_untrusted_proxies: Vec::new(),
        wants_printing: false,
        wants_hw_video_accel: true,
        router_interfaces: None,
        online: Some(true),
        services: vec![
            NodeService::NixBuilder {
                maximum_jobs: Some(6),
            },
            NodeService::VmHost {
                guest_subnet: TapSubnet::try_new("169.254.100.0/22").expect("valid tap subnet"),
                kvm: KvmAvailability::Available,
                maximum_guests: Some(MaximumGuests::new(4)),
            },
            NodeService::PersonaDevelopment {
                capabilities: vec![PersonaDevelopmentCapability::GitoliteServer],
            },
        ],
    }
}

#[test]
fn cluster_proposal_root_round_trips_its_real_nested_data_through_text() {
    let proposal = ClusterProposal {
        nodes: BTreeMap::from([(
            NodeName::try_new("ouranos").expect("valid node name"),
            node(),
        )]),
        users: BTreeMap::new(),
        domains: BTreeMap::new(),
        trust: ClusterTrust {
            cluster: Magnitude::Max,
            clusters: BTreeMap::new(),
            nodes: BTreeMap::new(),
            users: BTreeMap::from([(
                UserName::try_new("li").expect("valid user name"),
                Magnitude::Max,
            )]),
        },
        domain_configuration: DomainConfiguration::default(),
    };

    let text = proposal.textualize();
    let embodied = Text::<ClusterProposal>::from(text.as_ref())
        .embody()
        .expect("cluster proposal Datomic embodiment");

    assert_eq!(embodied.textualize().as_ref(), text.as_ref());
    assert!(text.as_ref().contains("NixBuilder.Some.6"));
    assert!(
        text.as_ref()
            .contains("VmHost.{169.254.100.0/22 Available Some.4}")
    );
}

#[test]
fn cluster_proposal_root_rejects_a_short_record() {
    assert!(Text::<ClusterProposal>::from("{«»}").embody().is_err());
}
