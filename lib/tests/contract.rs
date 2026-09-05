use datom_codec::Textualizable;
use horizon_lib::*;

fn text(value: &str) -> protos::Text {
    protos::Text::try_from(value).expect("fixture text")
}
fn hardware() -> Hardware {
    Hardware(
        16,
        Some(text("ThinkPadT14Gen5Intel")),
        Some(MotherBoard::Ondyfaind),
        Some(12),
        Some(32),
        Some(text("home-lab")),
    )
}
fn metal() -> MachineDefinition {
    MachineDefinition::Metal(Architecture::X86_64, hardware())
}
fn keys() -> NodeKeys {
    NodeKeys(
        text("ssh-ed25519 AAAAfixture"),
        Some(text("cache-key:fixture")),
        Some(YggdrasilKey(
            text("ygg-key"),
            text("200:1::1"),
            text("200:1::/64"),
        )),
    )
}
fn node(
    name: &str,
    variant: NodeVariant,
    machine: MachineDefinition,
    capabilities: Vec<NodeCapability>,
) -> NodeDefinition {
    let network = NodeNetwork(
        vec![text("fe80::1")],
        Some(text("10.18.0.2")),
        Some(text("wireguard-key")),
        vec![WireguardProxy(
            text("proxy-key"),
            text("vpn.example:51820"),
            text("10.0.0.2"),
        )],
        Some(RouterInterfaces(
            text("wan0"),
            text("wlan0"),
            WlanBand::FiveG,
            36,
            WlanStandard::Wifi6,
            Some(SecretReference(text("router-wifi"))),
            Some(BackupWireless(
                text("wlan1"),
                text("backup"),
                WlanBand::TwoG,
                1,
                WlanStandard::Wifi4,
                SecretReference(text("backup-wifi")),
            )),
        )),
    );
    NodeDefinition(
        text(name),
        variant,
        Magnitude::Max,
        Magnitude::Max,
        machine,
        NodeEnvironment(Keyboard::Colemak, Some(CompressedSwap(20))),
        network,
        keys(),
        Some(true),
        capabilities,
    )
}
fn graphical_live() -> NodeDefinition {
    node(
        "live-install",
        NodeVariant::Live(LiveDefinition()),
        metal(),
        vec![
            NodeCapability::Graphical(NoSettings()),
            NodeCapability::Center(NoSettings()),
            NodeCapability::LargeAi(NoSettings()),
        ],
    )
}
fn minimal_live() -> NodeDefinition {
    NodeDefinition(
        text("live-install-minimal"),
        NodeVariant::Live(LiveDefinition()),
        Magnitude::Min,
        Magnitude::Min,
        metal(),
        NodeEnvironment(Keyboard::Qwerty, None),
        NodeNetwork(vec![], None, None, vec![], None),
        keys(),
        Some(true),
        vec![],
    )
}
fn installation() -> NodeDefinition {
    node(
        "zeus",
        NodeVariant::Installation(Installation(
            Bootloader::Uefi,
            vec![DiskLayout(
                text("/dev/nvme0n1p1"),
                text("/"),
                FsType::Btrfs,
                vec![text("compress=zstd")],
            )],
            vec![SwapDevice(text("/swapfile"), Some(4096))],
        )),
        metal(),
        vec![
            NodeCapability::Router(NoSettings()),
            NodeCapability::NixBuilder(Some(6)),
            NodeCapability::VmHost(
                text("169.254.100.0/22"),
                KvmAvailability::Available,
                Some(4),
            ),
            NodeCapability::WebHost(vec![HostedSite(
                text("example.com"),
                text("github:org/site/rev"),
                SiteRenderer::MarkdownStatic,
            )]),
        ],
    )
}
fn local_vm() -> NodeDefinition {
    node(
        "mercury",
        NodeVariant::Live(LiveDefinition()),
        MachineDefinition::VirtualMachine(
            VirtualMachineHost::Cluster(text("zeus"), vec![], Some(text("li")), None),
            hardware(),
            Some(30),
        ),
        vec![NodeCapability::TestVm(NoSettings())],
    )
}
fn external_vm() -> NodeDefinition {
    node(
        "doris",
        NodeVariant::Live(LiveDefinition()),
        MachineDefinition::VirtualMachine(
            VirtualMachineHost::External(text("digitalocean-fra1"), Architecture::X86_64),
            hardware(),
            Some(40),
        ),
        vec![NodeCapability::CloudNode(NoSettings())],
    )
}

fn definition(selected: Vec<protos::Text>, local_nodes: Vec<NodeDefinition>) -> HorizonDefinition {
    HorizonDefinition(
        HorizonConfiguration(
            vec![graphical_live(), minimal_live()],
            DomainConfiguration(text("criome"), vec![text("goldragon.criome.net")]),
        ),
        ClusterDefinition(
            text("goldragon"),
            local_nodes,
            selected,
            vec![UserDefinition(
                text("li"),
                UserRole::Code,
                Magnitude::Max,
                Keyboard::Colemak,
                Style::Emacs,
                Some(text("li")),
                Some(true),
                vec![UserPubKey(text("zeus"), text("AAAAuser"), text("keygrip"))],
                Some(Editor::Emacs),
                Some(TextSize::Large),
            )],
            vec![DomainDefinition(
                text("goldragon.criome.net"),
                DomainProvider::Cloudflare,
            )],
            ClusterTrust(
                Magnitude::Max,
                vec![ClusterTrustEntry(text("sibling"), Magnitude::Medium)],
                vec![NodeTrustEntry(text("zeus"), Magnitude::Max)],
                vec![UserTrustEntry(text("li"), Magnitude::Max)],
            ),
        ),
    )
}

#[test]
fn complete_production_shape_round_trips_and_projects_both_selected_live_installers() {
    let definition = definition(
        vec![text("live-install"), text("live-install-minimal")],
        vec![installation(), local_vm(), external_vm()],
    );
    let encoded = definition.textualize();
    let decoded = decode(encoded.as_ref()).expect("complete definition decodes");
    assert_eq!(decoded.textualize(), encoded);

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
    vm.4 = MachineDefinition::VirtualMachine(
        VirtualMachineHost::Cluster(text("missing"), Vec::new(), None, None),
        hardware(),
        Some(30),
    );
    let definition = definition(vec![text("live-install")], vec![installation(), vm]);
    assert!(
        matches!(definition.resolve(), Err(Error::UnknownVmHost { node, host }) if node == "mercury" && host == "missing")
    );
}

#[test]
fn domain_and_github_defaults_follow_the_selected_cluster() {
    let mut definition = definition(vec![text("live-install")], vec![installation()]);
    definition.0.1.1.clear();
    definition.1.3[0].5 = None;
    let horizon = definition
        .project("live-install")
        .expect("defaults project");
    assert_eq!(horizon.users[0].email_address, "li@goldragon.criome.net");
    assert_eq!(horizon.users[0].github_id.as_deref(), Some("li"));
}

#[test]
fn local_vm_architecture_inference_is_single_hop() {
    let mut chained = local_vm();
    chained.0 = text("chained");
    chained.4 = MachineDefinition::VirtualMachine(
        VirtualMachineHost::Cluster(text("mercury"), Vec::new(), None, None),
        hardware(),
        Some(30),
    );
    let definition = definition(
        vec![text("live-install")],
        vec![installation(), local_vm(), chained],
    );
    assert!(
        matches!(definition.resolve(), Err(Error::VmHostArchitecture { node }) if node == "chained")
    );
}
