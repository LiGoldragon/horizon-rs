//! Tests for `proposal::services` tailnet types — `TailnetConfig`,
//! `TlsTrustPolicy`, `PublicCertificate`, the collapsed
//! `TailnetControllerRole::Server { port }`, and the
//! `TailnetControllerWithoutClusterConfig` validation.

use std::collections::BTreeMap;

use horizon_lib::address::{YggAddress, YggSubnet};
use horizon_lib::error::Error;
use horizon_lib::magnitude::Magnitude;
use horizon_lib::name::{ClusterDomain, ClusterName, DomainName, NodeName, UserName};
use horizon_lib::proposal::{
    ClusterProposal, ClusterTrust, Io, Machine, NodeProposal, NodePubKeys, NodeServices,
    PublicCertificate, TailnetConfig, TailnetControllerRole, TailnetMembership, TlsTrustPolicy,
    UserProposal,
};
use horizon_lib::pub_key::{NixPubKey, SshPubKey, YggPubKey};
use horizon_lib::species::{Arch, Bootloader, Keyboard, MachineSpecies, NodeSpecies};
use horizon_lib::Viewpoint;
use nota_codec::{Decoder, NotaDecode};

const VALID_PEM: &str = "-----BEGIN CERTIFICATE-----\nMIIBxxxxxx...\n-----END CERTIFICATE-----";
const NIX_KEY: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

#[test]
fn public_certificate_accepts_pem() {
    let cert = PublicCertificate::try_new(VALID_PEM).unwrap();
    assert_eq!(cert.as_str(), VALID_PEM);
}

#[test]
fn public_certificate_rejects_missing_begin_marker() {
    let error = PublicCertificate::try_new("not a cert").unwrap_err();
    assert!(matches!(error, Error::InvalidPublicCertificate { .. }));
}

#[test]
fn public_certificate_rejects_empty() {
    let error = PublicCertificate::try_new("").unwrap_err();
    assert!(matches!(error, Error::InvalidPublicCertificate { .. }));
}

#[test]
fn tailnet_controller_server_decodes_with_port_only() {
    // After step 11 collapse: Server { port } only.
    let text = "(Server 8443)";
    let mut decoder = Decoder::new(text);
    let role = TailnetControllerRole::decode(&mut decoder).unwrap();
    assert_eq!(role, TailnetControllerRole::Server { port: 8443 });
}

#[test]
fn tailnet_config_decodes_with_base_domain_and_no_tls() {
    let text = r#"(TailnetConfig "tailnet.goldragon.criome" None)"#;
    let mut decoder = Decoder::new(text);
    let config = TailnetConfig::decode(&mut decoder).unwrap();
    assert_eq!(config.base_domain.as_str(), "tailnet.goldragon.criome");
    assert!(config.tls.is_none());
}

#[test]
fn tls_trust_policy_decodes_with_ca_certificate() {
    let text = format!(
        r#"(TlsTrustPolicy "{}")"#,
        VALID_PEM.replace('\n', "\\n")
    );
    let mut decoder = Decoder::new(&text);
    let policy = TlsTrustPolicy::decode(&mut decoder).unwrap();
    // PEM round-trips via the nota string codec (escape sequences in
    // text → embedded newlines in the struct).
    assert!(policy.ca_certificate.as_str().starts_with("-----BEGIN CERTIFICATE-----"));
}

fn minimal_node(species: NodeSpecies, full_keys: bool) -> NodeProposal {
    NodeProposal {
        species,
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
        },
        io: Io {
            keyboard: Keyboard::Colemak,
            bootloader: Bootloader::Uefi,
            disks: BTreeMap::new(),
            swap_devices: Vec::new(),
        },
        pub_keys: NodePubKeys {
            ssh: SshPubKey::try_new("AAA=").unwrap(),
            nix: full_keys.then(|| NixPubKey::try_new(NIX_KEY).unwrap()),
            yggdrasil: full_keys.then(|| horizon_lib::proposal::YggPubKeyEntry {
                pub_key: YggPubKey::try_new("a".repeat(64)).unwrap(),
                address: YggAddress::try_new("200::1").unwrap(),
                subnet: YggSubnet::try_new("300::").unwrap(),
            }),
        },
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

fn cluster_with_one_controller(tailnet: Option<TailnetConfig>) -> ClusterProposal {
    let mut nodes = BTreeMap::new();
    let mut ouranos = minimal_node(NodeSpecies::EdgeTesting, true);
    ouranos.services.tailnet = Some(TailnetMembership::Client);
    ouranos.services.tailnet_controller = Some(TailnetControllerRole::Server { port: 8443 });
    nodes.insert(NodeName::try_new("ouranos").unwrap(), ouranos);

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
        tailnet,
        ai_providers: Vec::new(),
        vpn_profiles: Vec::new(),
        domain: ClusterDomain::try_new("criome").unwrap(),
    }
}

fn viewpoint(node: &str) -> Viewpoint {
    Viewpoint {
        cluster: ClusterName::try_new("goldragon").unwrap(),
        node: NodeName::try_new(node).unwrap(),
    }
}

#[test]
fn project_rejects_controller_without_cluster_tailnet_config() {
    let proposal = cluster_with_one_controller(None);
    let error = proposal.project(&viewpoint("ouranos")).unwrap_err();
    assert!(matches!(
        error,
        Error::TailnetControllerWithoutClusterConfig { node }
            if node.as_str() == "ouranos"
    ));
}

#[test]
fn project_accepts_controller_when_cluster_tailnet_is_some() {
    let proposal = cluster_with_one_controller(Some(TailnetConfig {
        base_domain: DomainName::try_new("tailnet.goldragon.criome").unwrap(),
        tls: None,
    }));
    let horizon = proposal.project(&viewpoint("ouranos")).unwrap();
    assert_eq!(
        horizon.node.services.tailnet_controller,
        Some(TailnetControllerRole::Server { port: 8443 })
    );
    let cluster_tailnet = horizon.cluster.tailnet.as_ref().expect("cluster.tailnet projected");
    assert_eq!(cluster_tailnet.base_domain.as_str(), "tailnet.goldragon.criome");
    assert!(cluster_tailnet.tls.is_none());
}
