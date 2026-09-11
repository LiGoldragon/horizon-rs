use datom_codec::Datomizable;
use horizon_lib::*;
use protos::{Protosizable, Textualizable};

fn text(value: &str) -> String {
    value.to_owned()
}
fn encode(value: &HorizonDefinition) -> String {
    value.datomize(vec![]).protosize().textualize()
}
fn hardware() -> Hardware {
    Hardware {
        integer: 16,
        model_name_option: Some(text("ThinkPadT14Gen5Intel")),
        mother_board_option: Some(MotherBoard::Ondyfaind),
        first_integer_option: Some(12),
        second_integer_option: Some(32),
        location_option: Some(text("home-lab")),
    }
}
fn metal() -> MachineDefinition {
    MachineDefinition::Metal(Metal_Data {
        architecture: Architecture::X86_64,
        hardware: hardware(),
    })
}
fn keys() -> NodeKeys {
    NodeKeys {
        ssh_pub_key: text("ssh-ed25519 AAAAfixture"),
        nix_pub_key_option: Some(text("cache-key:fixture")),
        yggdrasil_key_option: Some(YggdrasilKey {
            ygg_pub_key: text("ygg-key"),
            ygg_address: text("200:1::1"),
            ygg_subnet: text("200:1::/64"),
        }),
    }
}
fn node(
    name: &str,
    variant: NodeVariant,
    machine: MachineDefinition,
    capabilities: Vec<NodeCapability>,
) -> NodeDefinition {
    let network = NodeNetwork {
        link_local_ip_vector: vec![text("fe80::1")],
        node_ip_option: Some(text("10.18.0.2")),
        wireguard_pub_key_option: Some(text("wireguard-key")),
        wireguard_proxy_vector: vec![WireguardProxy {
            wireguard_pub_key: text("proxy-key"),
            string: text("vpn.example:51820"),
            node_ip: text("10.0.0.2"),
        }],
        router_interfaces_option: Some(RouterInterfaces {
            first_interface: text("wan0"),
            second_interface: text("wlan0"),
            wlan_band: WlanBand::FiveG,
            integer: 36,
            wlan_standard: WlanStandard::Wifi6,
            secret_reference_option: Some(SecretReference {
                secret_name: text("router-wifi"),
            }),
            backup_wireless_option: Some(BackupWireless {
                interface: text("wlan1"),
                wireless_network_name: text("backup"),
                wlan_band: WlanBand::TwoG,
                integer: 1,
                wlan_standard: WlanStandard::Wifi4,
                secret_reference: SecretReference {
                    secret_name: text("backup-wifi"),
                },
            }),
        }),
    };
    NodeDefinition {
        node_name: text(name),
        node_variant: variant,
        first_magnitude: Magnitude::Max,
        second_magnitude: Magnitude::Max,
        machine_definition: machine,
        node_environment: NodeEnvironment {
            keyboard: Keyboard::Colemak,
            compressed_swap_option: Some(CompressedSwap { integer: 20 }),
        },
        node_network: network,
        node_keys: keys(),
        boolean_option: Some(true),
        capabilities,
        fixed_location_option: None,
    }
}
fn graphical_live() -> NodeDefinition {
    node(
        "live-install",
        NodeVariant::Live(LiveDefinition {}),
        metal(),
        vec![
            NodeCapability::Graphical(NoSettings {}),
            NodeCapability::Center(NoSettings {}),
            NodeCapability::LargeAi(NoSettings {}),
        ],
    )
}
fn minimal_live() -> NodeDefinition {
    NodeDefinition {
        node_name: text("live-install-minimal"),
        node_variant: NodeVariant::Live(LiveDefinition {}),
        first_magnitude: Magnitude::Min,
        second_magnitude: Magnitude::Min,
        machine_definition: metal(),
        node_environment: NodeEnvironment {
            keyboard: Keyboard::Qwerty,
            compressed_swap_option: None,
        },
        node_network: NodeNetwork {
            link_local_ip_vector: vec![],
            node_ip_option: None,
            wireguard_pub_key_option: None,
            wireguard_proxy_vector: vec![],
            router_interfaces_option: None,
        },
        node_keys: keys(),
        boolean_option: Some(true),
        capabilities: vec![],
        fixed_location_option: None,
    }
}
fn installation() -> NodeDefinition {
    node(
        "zeus",
        NodeVariant::Installation(Installation {
            bootloader: Bootloader::Uefi,
            disk_layout_vector: vec![DiskLayout {
                device_path: text("/dev/nvme0n1p1"),
                mount_path: text("/"),
                fs_type: FsType::Btrfs,
                string_vector: vec![text("compress=zstd")],
            }],
            swap_device_vector: vec![SwapDevice {
                device_path: text("/swapfile"),
                integer_option: Some(4096),
            }],
        }),
        metal(),
        vec![
            NodeCapability::Router(NoSettings {}),
            NodeCapability::NixBuilder(Some(6)),
            NodeCapability::VmHost(VmHost_Data {
                tap_subnet: text("169.254.100.0/22"),
                kvm_availability: KvmAvailability::Available,
                integer_option: Some(4),
            }),
            NodeCapability::VmTesting(VmTesting_Data {
                boolean: true,
                string: text("Spice"),
                string_option: Some(text("1002:73bf")),
            }),
            NodeCapability::WebHost(vec![HostedSite {
                served_domain: text("example.com"),
                site_source: text("github:org/site/rev"),
                site_renderer: SiteRenderer::MarkdownStatic,
            }]),
        ],
    )
}
fn local_vm() -> NodeDefinition {
    node(
        "mercury",
        NodeVariant::Live(LiveDefinition {}),
        MachineDefinition::VirtualMachine(VirtualMachine_Data {
            virtual_machine_host: VirtualMachineHost::Cluster(Cluster_Data {
                node_name: text("zeus"),
                node_name_vector: vec![],
                user_name_option: Some(text("li")),
                architecture_option: None,
            }),
            hardware: hardware(),
            integer_option: Some(30),
        }),
        vec![NodeCapability::TestVm(NoSettings {})],
    )
}
fn external_vm() -> NodeDefinition {
    node(
        "doris",
        NodeVariant::Live(LiveDefinition {}),
        MachineDefinition::VirtualMachine(VirtualMachine_Data {
            virtual_machine_host: VirtualMachineHost::External(External_Data {
                string: text("digitalocean-fra1"),
                architecture: Architecture::X86_64,
            }),
            hardware: hardware(),
            integer_option: Some(40),
        }),
        vec![NodeCapability::CloudNode(NoSettings {})],
    )
}

fn definition(selected: Vec<String>, local_nodes: Vec<NodeDefinition>) -> HorizonDefinition {
    HorizonDefinition {
        horizon_configuration: HorizonConfiguration {
            generic_nodes: vec![graphical_live(), minimal_live()],
            domain_configuration: DomainConfiguration {
                string: text("criome"),
                domain_name_vector: vec![text("goldragon.criome.net")],
            },
        },
        cluster_definition: ClusterDefinition {
            cluster_name: text("goldragon"),
            cluster_nodes: local_nodes,
            generic_node_names: selected,
            users: vec![UserDefinition {
                user_name: text("li"),
                user_role: UserRole::Code,
                magnitude: Magnitude::Max,
                keyboard: Keyboard::Colemak,
                style: Style::Emacs,
                github_id_option: Some(text("li")),
                boolean_option: Some(true),
                user_pub_key_vector: vec![UserPubKey {
                    node_name: text("zeus"),
                    ssh_pub_key: text("AAAAuser"),
                    keygrip: text("keygrip"),
                }],
                editor_option: Some(Editor::Emacs),
                text_size_option: Some(TextSize::Large),
            }],
            domains: vec![DomainDefinition {
                domain_name: text("goldragon.criome.net"),
                domain_provider: DomainProvider::Cloudflare,
            }],
            cluster_trust: ClusterTrust {
                magnitude: Magnitude::Max,
                cluster_trust_entry_vector: vec![ClusterTrustEntry {
                    cluster_name: text("sibling"),
                    magnitude: Magnitude::Medium,
                }],
                node_trust_entry_vector: vec![NodeTrustEntry {
                    node_name: text("zeus"),
                    magnitude: Magnitude::Max,
                }],
                user_trust_entry_vector: vec![UserTrustEntry {
                    user_name: text("li"),
                    magnitude: Magnitude::Max,
                }],
            },
        },
    }
}

#[test]
fn complete_production_shape_round_trips_and_projects_both_selected_live_installers() {
    let definition = definition(
        vec![text("live-install"), text("live-install-minimal")],
        vec![installation(), local_vm(), external_vm()],
    );
    let encoded = encode(&definition);
    let decoded = decode(encoded.as_ref()).expect("complete definition decodes");
    assert_eq!(encode(&decoded), encoded);

    let graphical = decoded
        .project("live-install")
        .expect("graphical selected generic projects");
    assert!(graphical.node.is_live);
    assert!(graphical.node.installation_disks.is_empty());
    assert_eq!(graphical.node.keyboard, "Colemak");
    assert_eq!(graphical.node.compressed_swap_memory_percent, Some(20));
    assert_eq!(
        graphical.node.criome_domain_name,
        "live-install.goldragon.criome"
    );
    assert_eq!(graphical.node.system, "x86_64-linux");
    assert!(graphical.node.behaves_as.center);
    assert_eq!(
        graphical.node.builder_configs[0].host_name,
        "zeus.goldragon.criome"
    );
    assert_eq!(
        graphical.node.admin_ssh_public_keys,
        ["ssh-ed25519 AAAAuser"]
    );
    assert!(matches!(
        graphical.node.capabilities.as_slice(),
        [
            Capability::Graphical,
            Capability::Center,
            Capability::LargeAi
        ]
    ));
    assert_eq!(
        graphical.ex_nodes["zeus"]
            .installation
            .as_ref()
            .expect("installation")
            .disks[0]
            .options,
        ["compress=zstd"]
    );
    assert!(
        matches!(graphical.ex_nodes["zeus"].capabilities[2], Capability::VmHost { ref kvm, .. } if kvm == "Available")
    );
    assert!(matches!(
        graphical.ex_nodes["zeus"].capabilities[3],
        Capability::VmTesting {
            gpu_passthrough: true,
            ref display,
            gpu: Some(ref gpu),
        } if display == "Spice" && gpu == "1002:73bf"
    ));
    assert!(matches!(
        graphical.ex_nodes["mercury"].capabilities.as_slice(),
        [Capability::TestVm]
    ));
    assert_eq!(
        graphical.ex_nodes["zeus"]
            .network
            .router_interfaces
            .as_ref()
            .expect("router")
            .backup_wireless
            .as_ref()
            .expect("backup")
            .network_name,
        "backup"
    );
    assert_eq!(graphical.users[0].public_keys[0].keygrip, "keygrip");
    assert_eq!(graphical.users[0].email_address, "li@goldragon.criome.net");
    assert_eq!(graphical.users[0].preferred_editor, "Emacs");
    assert_eq!(graphical.domains[0].provider, "Cloudflare");
    assert_eq!(graphical.trust.nodes[0].name, "zeus");
    assert_eq!(
        graphical.ex_nodes["mercury"].machine.host.as_deref(),
        Some("zeus")
    );
    assert_eq!(graphical.ex_nodes["mercury"].machine.architecture, "x86_64");
    assert_eq!(graphical.ex_nodes["doris"].machine.kind, "VirtualMachine");
    assert_eq!(
        graphical.ex_nodes["doris"].machine.host.as_deref(),
        Some("digitalocean-fra1")
    );

    let minimal = decoded
        .project("live-install-minimal")
        .expect("minimal selected generic projects");
    assert!(minimal.node.is_live);
    assert_eq!(minimal.node.keyboard, "Qwerty");
    assert!(minimal.node.capabilities.is_empty());
}

#[test]
fn unselected_catalogue_node_is_not_cluster_membership() {
    let definition = definition(Vec::new(), vec![installation()]);
    assert!(
        matches!(definition.project("live-install"), Err(Error::UnknownNode(name)) if name == "live-install")
    );
}

#[test]
fn local_and_selected_generic_name_collision_is_refused() {
    let definition = definition(vec![text("live-install")], vec![graphical_live()]);
    assert!(
        matches!(definition.resolve(), Err(Error::NodeCollision(name)) if name == "live-install")
    );
}

#[test]
fn local_vm_requires_an_existing_cluster_host() {
    let mut vm = local_vm();
    vm.machine_definition = MachineDefinition::VirtualMachine(VirtualMachine_Data {
        virtual_machine_host: VirtualMachineHost::Cluster(Cluster_Data {
            node_name: text("missing"),
            node_name_vector: Vec::new(),
            user_name_option: None,
            architecture_option: None,
        }),
        hardware: hardware(),
        integer_option: Some(30),
    });
    let definition = definition(vec![text("live-install")], vec![installation(), vm]);
    assert!(
        matches!(definition.resolve(), Err(Error::UnknownVmHost { node, host }) if node == "mercury" && host == "missing")
    );
}

#[test]
fn domain_and_github_defaults_follow_the_selected_cluster() {
    let mut definition = definition(vec![text("live-install")], vec![installation()]);
    definition
        .horizon_configuration
        .domain_configuration
        .domain_name_vector
        .clear();
    definition.cluster_definition.users[0].github_id_option = None;
    let horizon = definition
        .project("live-install")
        .expect("defaults project");
    assert_eq!(horizon.users[0].email_address, "li@goldragon.criome.net");
    assert_eq!(horizon.users[0].github_id.as_deref(), Some("li"));
}

#[test]
fn fixed_location_projects_without_changing_hardware_location() {
    let mut installation = installation();
    installation.fixed_location_option = Some(FixedLocation {
        latitude: 16.736944,
        longitude: -92.6375,
        altitude: 2121.0,
        accuracy: 1000.0,
    });
    let definition = definition(Vec::new(), vec![installation]);
    let encoded = encode(&definition);
    let decoded = decode(encoded.as_ref()).expect("fixed location decodes");
    let horizon = decoded.project("zeus").expect("fixed location projects");

    assert_eq!(
        horizon.node.fixed_location,
        Some(FixedLocationView {
            latitude: 16.736944,
            longitude: -92.6375,
            altitude: 2121.0,
            accuracy: 1000.0,
        })
    );
    assert_eq!(horizon.node.machine.hardware.location.as_deref(), Some("home-lab"));
}

#[test]
fn local_vm_architecture_inference_is_single_hop() {
    let mut chained = local_vm();
    chained.node_name = text("chained");
    chained.machine_definition = MachineDefinition::VirtualMachine(VirtualMachine_Data {
        virtual_machine_host: VirtualMachineHost::Cluster(Cluster_Data {
            node_name: text("mercury"),
            node_name_vector: Vec::new(),
            user_name_option: None,
            architecture_option: None,
        }),
        hardware: hardware(),
        integer_option: Some(30),
    });
    let definition = definition(
        vec![text("live-install")],
        vec![installation(), local_vm(), chained],
    );
    assert!(
        matches!(definition.resolve(), Err(Error::VmHostArchitecture { node }) if node == "chained")
    );
}
