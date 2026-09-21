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
        ClusterProposal, ClusterTrust, Ipv4Cidr, KvmAvailability, MacAddress, MaximumGuests,
        NodeProposal, NodePubKeys, NodeService, PersonaDevelopmentCapability, YggPubKeyEntry,
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
            NodeService::UsbIpv4Gateway {
                downstream: horizon_lib::address::Interface::new("enp0s20f0u1c2"),
                downstream_mac: MacAddress::try_new("02:0e:c6:33:4f:97").unwrap(),
                gateway: Ipv4Cidr::try_new("10.44.0.1/24").unwrap(),
                uplink: horizon_lib::address::Interface::new("enp0s31f6"),
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
    assert!(text.as_ref().contains("UsbIpv4Gateway."));
}

#[test]
fn usb_gateway_address_types_accept_canonical_unicast_host_values() {
    // `02` is locally administered and unicast. It remains valid because the
    // U/L bit identifies assignment, while the I/G bit controls multicast.
    let mac = MacAddress::try_new("02:0e:c6:33:4f:97").unwrap();
    let gateway = Ipv4Cidr::try_new("10.44.0.1/24").unwrap();
    assert_eq!(mac.to_string(), "02:0e:c6:33:4f:97");
    assert_eq!(gateway.to_string(), "10.44.0.1/24");
}

#[test]
fn usb_gateway_address_types_reject_noncanonical_or_nonhost_values() {
    for mac in [
        "00:00:00:00:00:00",
        "01:0e:c6:33:4f:97",
        "02:0E:c6:33:4f:97",
    ] {
        assert!(MacAddress::try_new(mac).is_err(), "{mac}");
    }
    for gateway in [
        "10.44.0.0/24",
        "10.44.0.255/24",
        "10.44.0.1/31",
        "10.44.0.1/32",
        "fd00::1/64",
    ] {
        assert!(Ipv4Cidr::try_new(gateway).is_err(), "{gateway}");
    }
}

#[test]
fn usb_gateway_serde_json_payload_round_trips_and_validates() {
    let fixture = r#"
        {
          "usbIpv4Gateway": {
            "downstream": "enp0s20f0u1c2",
            "downstreamMac": "02:0e:c6:33:4f:97",
            "gateway": "10.44.0.1/24",
            "uplink": "enp0s31f6"
          }
        }
    "#;
    let service: NodeService = serde_json::from_str(fixture).expect("valid USB gateway payload");
    assert_eq!(
        serde_json::to_string(&service).unwrap(),
        r#"{"usbIpv4Gateway":{"downstream":"enp0s20f0u1c2","downstreamMac":"02:0e:c6:33:4f:97","gateway":"10.44.0.1/24","uplink":"enp0s31f6"}}"#
    );

    let invalid = fixture.replace("10.44.0.1/24", "10.44.0.0/24");
    assert!(serde_json::from_str::<NodeService>(&invalid).is_err());
}

#[test]
fn cluster_proposal_root_rejects_a_short_record() {
    assert!(Text::<ClusterProposal>::from("{«»}").embody().is_err());
}
