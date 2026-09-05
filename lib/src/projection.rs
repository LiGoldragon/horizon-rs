use std::collections::{BTreeMap, BTreeSet};

use datom_codec::{Actualizable, IncorporationBudget, Potential};
use serde::Serialize;
use thiserror::Error;

use crate::*;

/// Decodes the materialized, single-file Horizon proposal.
pub fn decode(text: &str) -> Result<HorizonDefinition, Error> {
    actualize(text, 16_384)
}

/// Decodes the externally authored generic-node catalogue.
pub fn decode_configuration(text: &str) -> Result<HorizonConfiguration, Error> {
    actualize(text, 16_384)
}

/// Decodes the cluster-owned membership and cluster facts.
pub fn decode_cluster(text: &str) -> Result<ClusterDefinition, Error> {
    actualize(text, 16_384)
}

/// Decodes the sole typed command accepted by `horizon-compose`.
pub fn decode_composition_request(text: &str) -> Result<CompositionCommand, Error> {
    actualize(text, 1_024)
}

fn actualize<T: datom_codec::Datomic>(text: &str, budget: i64) -> Result<T, Error> {
    Potential::<T>::from(text)
        .actualize(IncorporationBudget::try_from(budget).expect("positive fixed budget"))
        .map_err(Error::Datom)
}

/// Combines the two explicit source documents and validates the membership
/// seam before a consumer can receive the resulting single-file document.
pub fn compose(
    configuration: HorizonConfiguration,
    cluster: ClusterDefinition,
) -> Result<HorizonDefinition, Error> {
    let definition = HorizonDefinition(configuration, cluster);
    definition.resolve()?;
    Ok(definition)
}

#[derive(Clone, Debug)]
pub struct ResolvedCluster {
    pub name: String,
    pub nodes: BTreeMap<String, NodeDefinition>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Horizon {
    pub cluster: String,
    pub tailnet_base_domain: String,
    pub trusted_build_public_keys: Vec<String>,
    pub node: Node,
    pub ex_nodes: BTreeMap<String, Node>,
    pub users: Vec<User>,
    pub domains: Vec<Domain>,
    pub trust: Trust,
    pub domain_configuration: DomainConfigurationView,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,
    pub variant: String,
    pub is_live: bool,
    pub is_installation: bool,
    pub installation: Option<InstallationView>,
    pub installation_disks: Vec<Disk>,
    pub size: String,
    pub trust: String,
    pub online: Option<bool>,
    pub keyboard: String,
    pub compressed_swap_memory_percent: Option<i64>,
    pub machine: Machine,
    pub network: Network,
    pub keys: Keys,
    pub capabilities: Vec<Capability>,
    pub criome_domain_name: String,
    pub system: String,
    pub max_jobs: i64,
    pub build_cores: i64,
    pub is_fully_trusted: bool,
    pub is_remote_nix_builder: bool,
    pub is_dispatcher: bool,
    pub is_nix_cache: bool,
    pub is_large_edge: bool,
    pub enable_network_manager: bool,
    pub has_base_public_keys: bool,
    pub nix_public_key_line: Option<String>,
    pub nix_cache_domain: Option<String>,
    pub nix_url: Option<String>,
    pub behaves_as: BehavesAs,
    pub ssh_public_key_line: String,
    pub builder_configs: Vec<BuilderConfig>,
    pub cache_urls: Vec<String>,
    pub ex_nodes_ssh_public_keys: Vec<String>,
    pub dispatchers_ssh_public_keys: Vec<String>,
    pub admin_ssh_public_keys: Vec<String>,
    pub image_exchange_public_keys: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationView {
    pub bootloader: String,
    pub disks: Vec<Disk>,
    pub swap_devices: Vec<SwapDeviceView>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Disk {
    pub device: String,
    pub mount: String,
    pub fs_type: String,
    pub options: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDeviceView {
    pub device: String,
    pub size_mebibytes: Option<i64>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub kind: String,
    pub architecture: String,
    pub host: Option<String>,
    pub additional_hosts: Vec<String>,
    pub user: Option<String>,
    pub disk_gib: Option<i64>,
    pub hardware: HardwareView,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareView {
    pub cores: i64,
    pub model: Option<String>,
    pub motherboard: Option<String>,
    pub chip_generation: Option<i64>,
    pub ram_gib: Option<i64>,
    pub location: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BehavesAs {
    pub center: bool,
    pub router: bool,
    pub edge: bool,
    pub next_generation: bool,
    pub low_power: bool,
    pub bare_metal: bool,
    pub virtual_machine: bool,
    pub iso: bool,
    pub large_ai: bool,
    pub test_vm: bool,
    pub cloud_node: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderConfig {
    pub host_name: String,
    pub ssh_user: String,
    pub ssh_key: String,
    pub supported_features: Vec<String>,
    pub system: String,
    pub max_jobs: i64,
    pub public_host_key: String,
    pub public_host_key_line: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub link_local_ips: Vec<String>,
    pub node_ip: Option<String>,
    pub wireguard_public_key: Option<String>,
    pub wireguard_proxies: Vec<WireguardProxyView>,
    pub router_interfaces: Option<RouterInterfacesView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WireguardProxyView {
    pub public_key: String,
    pub endpoint: String,
    pub interface_ip: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfacesView {
    pub wan: String,
    pub wlan: String,
    pub wlan_band: String,
    pub wlan_channel: i64,
    pub wlan_standard: String,
    pub wpa3_sae_password_reference: Option<String>,
    pub backup_wireless: Option<BackupWirelessView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWirelessView {
    pub interface: String,
    pub network_name: String,
    pub band: String,
    pub channel: i64,
    pub standard: String,
    pub password_reference: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub ssh: String,
    pub nix: Option<String>,
    pub yggdrasil: Option<YggdrasilKeyView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YggdrasilKeyView {
    pub public_key: String,
    pub address: String,
    pub subnet: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Capability {
    Graphical,
    Center,
    LargeAi,
    Router,
    Edge,
    NextGeneration,
    LowPower,
    TestVm,
    CloudNode,
    Printing,
    HardwareVideo,
    Nordvpn,
    WifiCertificate,
    TailnetClient,
    TailnetController,
    NixBuilder {
        maximum_jobs: Option<i64>,
    },
    NixCache,
    PersonaDevelopment {
        capabilities: Vec<String>,
    },
    VmHost {
        guest_subnet: String,
        kvm: String,
        maximum_guests: Option<i64>,
    },
    WebHost {
        sites: Vec<HostedSiteView>,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct HostedSiteView {
    pub domain: String,
    pub source: String,
    pub renderer: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub role: String,
    pub size: String,
    pub trust: String,
    pub keyboard: String,
    pub style: String,
    pub github_id: Option<String>,
    pub fast_repeat: Option<bool>,
    pub public_keys: Vec<UserPubKeyView>,
    pub editor: Option<String>,
    pub text_size: Option<String>,
    pub has_public_key: bool,
    pub email_address: String,
    pub matrix_id: String,
    pub git_signing_key: Option<String>,
    pub use_colemak: bool,
    pub use_fast_repeat: bool,
    pub is_multimedia_dev: bool,
    pub is_code_dev: bool,
    pub preferred_editor: String,
    pub resolved_text_size: String,
    pub ssh_public_keys: Vec<String>,
    pub ssh_public_key: Option<String>,
    pub extra_groups: Vec<String>,
    pub enable_linger: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPubKeyView {
    pub node: String,
    pub ssh: String,
    pub keygrip: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Domain {
    pub name: String,
    pub provider: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trust {
    pub cluster: String,
    pub clusters: Vec<TrustEntry>,
    pub nodes: Vec<TrustEntry>,
    pub users: Vec<TrustEntry>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TrustEntry {
    pub name: String,
    pub magnitude: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainConfigurationView {
    pub internal_suffix: String,
    pub public_cluster_domains: Vec<String>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("datom: {0:?}")]
    Datom(datom_codec::Fault),
    #[error("generic catalogue contains duplicate node {0:?}")]
    DuplicateGenericNode(String),
    #[error("cluster contains duplicate local node {0:?}")]
    DuplicateClusterNode(String),
    #[error("cluster selects generic node {0:?} more than once")]
    DuplicateGenericSelection(String),
    #[error("cluster selects unknown generic node {0:?}")]
    UnknownGenericNode(String),
    #[error("selected generic node {0:?} conflicts with a cluster-local node")]
    NodeCollision(String),
    #[error("resolved cluster has no node {0:?}")]
    UnknownNode(String),
    #[error("metal node {0:?} has no architecture")]
    MissingMetalArchitecture(String),
    #[error("external VM {0:?} has no architecture")]
    MissingExternalVmArchitecture(String),
    #[error("VM {node:?} names unknown cluster host {host:?}")]
    UnknownVmHost { node: String, host: String },
    #[error("VM {node:?} names hosts with different architectures")]
    VmHostArchitecture { node: String },
    #[error("VM host graph cycles at {0:?}")]
    VmHostCycle(String),
    #[error("trusted cluster has more than one TailnetController: {first:?}, {second:?}")]
    MultipleTailnetControllers { first: String, second: String },
}

impl HorizonDefinition {
    /// Resolves only explicitly selected generic names; catalogue presence does
    /// not imply cluster membership.
    pub fn resolve(&self) -> Result<ResolvedCluster, Error> {
        let generic = named_nodes(&self.0.0, Error::DuplicateGenericNode)?;
        let mut nodes = named_nodes(&self.1.1, Error::DuplicateClusterNode)?;
        let mut selected = BTreeSet::new();
        for name in &self.1.2 {
            let name = name.as_ref().to_owned();
            if !selected.insert(name.clone()) {
                return Err(Error::DuplicateGenericSelection(name));
            }
            let definition = generic
                .get(&name)
                .ok_or_else(|| Error::UnknownGenericNode(name.clone()))?;
            if nodes.insert(name.clone(), definition.clone()).is_some() {
                return Err(Error::NodeCollision(name));
            }
        }
        nodes.retain(|name, definition| {
            !matches!(
                effective_node_trust(&self.1.5, name, &definition.3),
                Magnitude::Zero
            )
        });
        validate_tailnet_controller(&nodes)?;
        validate_machines(&nodes)?;
        Ok(ResolvedCluster {
            name: self.1.0.as_ref().to_owned(),
            nodes,
        })
    }

    pub fn project(&self, node: &str) -> Result<Horizon, Error> {
        let resolved = self.resolve()?;
        let viewpoint = resolved
            .nodes
            .get(node)
            .ok_or_else(|| Error::UnknownNode(node.to_owned()))?;
        let ex_nodes: BTreeMap<String, Node> = resolved
            .nodes
            .iter()
            .filter(|(name, _)| name.as_str() != node)
            .map(|(name, definition)| {
                Ok((name.clone(), project_node(definition, &resolved.nodes)?))
            })
            .collect::<Result<_, Error>>()?;
        let mut ex_nodes = ex_nodes;
        let mut projected_node = project_node(viewpoint, &resolved.nodes)?;
        let internal_suffix = self.0.1.0.as_ref();
        let public_domain = self
            .0
            .1
            .1
            .first()
            .map(|value| value.as_ref().to_owned())
            .unwrap_or_else(|| format!("{}.criome.net", self.1.0.as_ref()));
        for (name, projected) in &mut ex_nodes {
            let definition = resolved.nodes.get(name).expect("resolved node exists");
            derive_node(
                projected,
                &resolved.name,
                internal_suffix,
                effective_node_trust(&self.1.5, name, &definition.3),
            );
        }
        derive_node(
            &mut projected_node,
            &resolved.name,
            internal_suffix,
            effective_node_trust(&self.1.5, node, &viewpoint.3),
        );
        let trusted_build_public_keys = std::iter::once(&projected_node)
            .chain(ex_nodes.values())
            .filter_map(|node| node.nix_public_key_line.clone())
            .collect::<Vec<String>>();
        let users = self
            .1
            .3
            .iter()
            .filter_map(|user| {
                let trust = effective_user_trust(&self.1.5, user.0.as_ref());
                (!matches!(trust, Magnitude::Zero)).then(|| {
                    project_user(
                        user,
                        trust,
                        node,
                        &public_domain,
                        projected_node.behaves_as.center,
                        &projected_node.size,
                    )
                })
            })
            .collect::<Vec<User>>();
        fill_viewpoint(&mut projected_node, &ex_nodes, &users);
        Ok(Horizon {
            cluster: resolved.name,
            tailnet_base_domain: format!("tailnet.{}.{}", self.1.0.as_ref(), internal_suffix),
            trusted_build_public_keys,
            node: projected_node,
            ex_nodes,
            users,
            domains: self.1.4.iter().map(project_domain).collect(),
            trust: project_trust(&self.1.5),
            domain_configuration: DomainConfigurationView {
                internal_suffix: self.0.1.0.as_ref().to_owned(),
                public_cluster_domains: self
                    .0
                    .1
                    .1
                    .iter()
                    .map(|value| value.as_ref().to_owned())
                    .collect(),
            },
        })
    }
}

fn fill_viewpoint(viewpoint: &mut Node, ex_nodes: &BTreeMap<String, Node>, users: &[User]) {
    viewpoint.builder_configs = ex_nodes
        .values()
        .filter(|node| node.is_remote_nix_builder)
        .map(builder_config)
        .collect();
    viewpoint.cache_urls = ex_nodes
        .values()
        .filter_map(|node| node.nix_url.clone())
        .collect();
    viewpoint.ex_nodes_ssh_public_keys = ex_nodes
        .values()
        .map(|node| node.ssh_public_key_line.clone())
        .collect();
    viewpoint.dispatchers_ssh_public_keys = ex_nodes
        .values()
        .filter(|node| node.is_dispatcher)
        .map(|node| node.ssh_public_key_line.clone())
        .collect();
    let mut nodes: BTreeMap<&str, &Node> = ex_nodes
        .iter()
        .map(|(name, node)| (name.as_str(), node))
        .collect();
    nodes.insert(viewpoint.name.as_str(), viewpoint);
    let mut admin_ssh_public_keys = Vec::new();
    for user in users.iter().filter(|user| user.trust == "Max") {
        for key in &user.public_keys {
            if nodes
                .get(key.node.as_str())
                .is_some_and(|node| node.is_fully_trusted)
            {
                let line = format!("ssh-ed25519 {}", key.ssh);
                if !admin_ssh_public_keys.contains(&line) {
                    admin_ssh_public_keys.push(line);
                }
            }
        }
    }
    let mut image_exchange_public_keys = Vec::new();
    if viewpoint.machine.kind == "VirtualMachine" {
        for host in viewpoint
            .machine
            .host
            .iter()
            .chain(viewpoint.machine.additional_hosts.iter())
        {
            if let Some(node) = nodes.get(host.as_str()) {
                if let Some(key) = &node.nix_public_key_line {
                    image_exchange_public_keys.push(key.clone());
                }
            }
        }
    }
    drop(nodes);
    viewpoint.admin_ssh_public_keys = admin_ssh_public_keys;
    viewpoint.image_exchange_public_keys = image_exchange_public_keys;
}

fn builder_config(node: &Node) -> BuilderConfig {
    BuilderConfig {
        host_name: node.criome_domain_name.clone(),
        ssh_user: "nix-ssh".into(),
        ssh_key: "/etc/ssh/ssh_host_ed25519_key".into(),
        supported_features: if node.behaves_as.edge {
            Vec::new()
        } else {
            vec!["big-parallel".into(), "kvm".into()]
        },
        system: node.system.clone(),
        max_jobs: node.max_jobs,
        public_host_key: node.keys.ssh.clone(),
        public_host_key_line: node.ssh_public_key_line.clone(),
    }
}

fn named_nodes(
    definitions: &[NodeDefinition],
    duplicate: fn(String) -> Error,
) -> Result<BTreeMap<String, NodeDefinition>, Error> {
    let mut nodes = BTreeMap::new();
    for definition in definitions {
        let name = definition.0.as_ref().to_owned();
        if nodes.insert(name.clone(), definition.clone()).is_some() {
            return Err(duplicate(name));
        }
    }
    Ok(nodes)
}

fn validate_machines(nodes: &BTreeMap<String, NodeDefinition>) -> Result<(), Error> {
    for name in nodes.keys() {
        resolved_architecture(name, nodes)?;
    }
    Ok(())
}

fn validate_tailnet_controller(nodes: &BTreeMap<String, NodeDefinition>) -> Result<(), Error> {
    let mut controllers = nodes.iter().filter(|(_, node)| {
        node.9
            .iter()
            .any(|capability| matches!(capability, NodeCapability::TailnetController(_)))
    });
    let Some((first, _)) = controllers.next() else {
        return Ok(());
    };
    if let Some((second, _)) = controllers.next() {
        return Err(Error::MultipleTailnetControllers {
            first: first.clone(),
            second: second.clone(),
        });
    }
    Ok(())
}

fn effective_node_trust(trust: &ClusterTrust, name: &str, input: &Magnitude) -> Magnitude {
    let floor = trust
        .2
        .iter()
        .find(|entry| entry.0.as_ref() == name)
        .map(|entry| &entry.1)
        .unwrap_or(&Magnitude::Max);
    let cluster_limited = minimum_magnitude(floor, &trust.0);
    minimum_magnitude(input, &cluster_limited)
}

fn effective_user_trust(trust: &ClusterTrust, name: &str) -> Magnitude {
    let input = trust
        .3
        .iter()
        .find(|entry| entry.0.as_ref() == name)
        .map(|entry| &entry.1)
        .unwrap_or(&Magnitude::Min);
    minimum_magnitude(input, &trust.0)
}

fn minimum_magnitude(left: &Magnitude, right: &Magnitude) -> Magnitude {
    if magnitude_rank(left) <= magnitude_rank(right) {
        left.clone()
    } else {
        right.clone()
    }
}

fn magnitude_rank(value: &Magnitude) -> u8 {
    match value {
        Magnitude::Zero => 0,
        Magnitude::Min => 1,
        Magnitude::Medium => 2,
        Magnitude::Large => 3,
        Magnitude::Max => 4,
    }
}

fn resolved_architecture(
    name: &str,
    nodes: &BTreeMap<String, NodeDefinition>,
) -> Result<Architecture, Error> {
    let definition = nodes.get(name).ok_or_else(|| Error::UnknownVmHost {
        node: name.to_owned(),
        host: name.to_owned(),
    })?;
    let result = match &definition.4 {
        MachineDefinition::Metal(architecture, _) => Ok(architecture.clone()),
        MachineDefinition::VirtualMachine(VirtualMachineHost::External(_, architecture), _, _) => {
            Ok(architecture.clone())
        }
        MachineDefinition::VirtualMachine(
            VirtualMachineHost::Cluster(host, additional_hosts, _, architecture),
            _,
            _,
        ) => {
            let mut hosts = vec![host.as_ref().to_owned()];
            hosts.extend(
                additional_hosts
                    .iter()
                    .map(|value| value.as_ref().to_owned()),
            );
            let mut host_architecture = None;
            for host in hosts {
                if !nodes.contains_key(&host) {
                    return Err(Error::UnknownVmHost {
                        node: name.to_owned(),
                        host,
                    });
                }
                let host_definition = nodes.get(&host).expect("checked host exists");
                let observed = match &host_definition.4 {
                    MachineDefinition::Metal(architecture, _) => architecture.clone(),
                    MachineDefinition::VirtualMachine(
                        VirtualMachineHost::External(_, architecture),
                        _,
                        _,
                    ) => architecture.clone(),
                    MachineDefinition::VirtualMachine(
                        VirtualMachineHost::Cluster(_, _, _, Some(architecture)),
                        _,
                        _,
                    ) => architecture.clone(),
                    MachineDefinition::VirtualMachine(
                        VirtualMachineHost::Cluster(_, _, _, None),
                        _,
                        _,
                    ) => {
                        return Err(Error::VmHostArchitecture {
                            node: name.to_owned(),
                        });
                    }
                };
                if host_architecture
                    .as_ref()
                    .is_some_and(|expected| expected != &observed)
                {
                    return Err(Error::VmHostArchitecture {
                        node: name.to_owned(),
                    });
                }
                host_architecture = Some(observed);
            }
            let inherited = host_architecture.expect("cluster VM has a primary host");
            if architecture
                .as_ref()
                .is_some_and(|declared| declared != &inherited)
            {
                return Err(Error::VmHostArchitecture {
                    node: name.to_owned(),
                });
            }
            Ok(architecture.clone().unwrap_or(inherited))
        }
    };
    result
}

fn project_node(
    definition: &NodeDefinition,
    nodes: &BTreeMap<String, NodeDefinition>,
) -> Result<Node, Error> {
    let (variant, installation) = match &definition.1 {
        NodeVariant::Live(_) => ("Live".to_owned(), None),
        NodeVariant::Installation(value) => (
            "Installation".to_owned(),
            Some(InstallationView {
                bootloader: bootloader(&value.0).to_owned(),
                disks: value.1.iter().map(project_disk).collect(),
                swap_devices: value
                    .2
                    .iter()
                    .map(|swap| SwapDeviceView {
                        device: swap.0.as_ref().to_owned(),
                        size_mebibytes: swap.1,
                    })
                    .collect(),
            }),
        ),
    };
    let installation_disks = installation
        .as_ref()
        .map_or_else(Vec::new, |value| value.disks.clone());
    Ok(Node {
        name: definition.0.as_ref().to_owned(),
        is_live: installation.is_none(),
        is_installation: installation.is_some(),
        variant,
        installation,
        installation_disks,
        size: magnitude(&definition.2).to_owned(),
        trust: magnitude(&definition.3).to_owned(),
        online: definition.8,
        keyboard: keyboard(&definition.5.0).to_owned(),
        compressed_swap_memory_percent: definition.5.1.as_ref().map(|value| value.0),
        machine: project_machine(&definition.0.as_ref().to_owned(), &definition.4, nodes)?,
        network: project_network(&definition.6),
        keys: project_keys(&definition.7),
        capabilities: definition.9.iter().map(project_capability).collect(),
        criome_domain_name: String::new(),
        system: String::new(),
        max_jobs: 0,
        build_cores: 0,
        is_fully_trusted: false,
        is_remote_nix_builder: false,
        is_dispatcher: false,
        is_nix_cache: false,
        is_large_edge: false,
        enable_network_manager: false,
        has_base_public_keys: false,
        nix_public_key_line: None,
        nix_cache_domain: None,
        nix_url: None,
        behaves_as: BehavesAs {
            center: false,
            router: false,
            edge: false,
            next_generation: false,
            low_power: false,
            bare_metal: false,
            virtual_machine: false,
            iso: false,
            large_ai: false,
            test_vm: false,
            cloud_node: false,
        },
        ssh_public_key_line: String::new(),
        builder_configs: Vec::new(),
        cache_urls: Vec::new(),
        ex_nodes_ssh_public_keys: Vec::new(),
        dispatchers_ssh_public_keys: Vec::new(),
        admin_ssh_public_keys: Vec::new(),
        image_exchange_public_keys: Vec::new(),
    })
}

fn derive_node(node: &mut Node, cluster: &str, suffix: &str, trust: Magnitude) {
    let has = |kind: fn(&Capability) -> bool| node.capabilities.iter().any(kind);
    let center = has(|capability| matches!(capability, Capability::Center));
    let router = has(|capability| matches!(capability, Capability::Router));
    let edge = has(|capability| matches!(capability, Capability::Edge));
    let next_generation = has(|capability| matches!(capability, Capability::NextGeneration));
    let low_power = has(|capability| matches!(capability, Capability::LowPower));
    let large_ai = has(|capability| matches!(capability, Capability::LargeAi));
    let test_vm = has(|capability| matches!(capability, Capability::TestVm));
    let cloud_node = has(|capability| matches!(capability, Capability::CloudNode));
    let virtual_machine = node.machine.kind == "VirtualMachine";
    let bare_metal = !virtual_machine;
    let iso = node.is_live && bare_metal;
    node.behaves_as = BehavesAs {
        center,
        router,
        edge,
        next_generation,
        low_power,
        bare_metal,
        virtual_machine,
        iso,
        large_ai,
        test_vm,
        cloud_node,
    };
    node.criome_domain_name = format!("{}.{}.{}", node.name, cluster, suffix);
    node.system = match node.machine.architecture.as_str() {
        "x86_64" => "x86_64-linux",
        "aarch64" => "aarch64-linux",
        _ => "unknown",
    }
    .to_owned();
    node.trust = magnitude(&trust).to_owned();
    node.is_fully_trusted = matches!(trust, Magnitude::Max);
    node.has_base_public_keys = node.keys.nix.is_some() && node.keys.yggdrasil.is_some();
    let maximum_jobs = node
        .capabilities
        .iter()
        .find_map(|capability| match capability {
            Capability::NixBuilder { maximum_jobs } => *maximum_jobs,
            _ => None,
        })
        .unwrap_or(1);
    node.max_jobs = maximum_jobs;
    node.build_cores = maximum_jobs;
    let online = node.online.unwrap_or(true);
    node.is_remote_nix_builder = node
        .capabilities
        .iter()
        .any(|capability| matches!(capability, Capability::NixBuilder { .. }))
        && online
        && node.is_fully_trusted
        && node.has_base_public_keys;
    node.is_dispatcher = !center && node.is_fully_trusted && !matches!(node.size.as_str(), "Zero");
    node.is_nix_cache = node
        .capabilities
        .iter()
        .any(|capability| matches!(capability, Capability::NixCache))
        && online
        && node.is_fully_trusted
        && node.has_base_public_keys;
    node.is_large_edge = matches!(node.size.as_str(), "Large" | "Max") && edge;
    node.enable_network_manager =
        !matches!(node.size.as_str(), "Zero") && !iso && !center && !router;
    node.nix_public_key_line = node
        .keys
        .nix
        .as_ref()
        .map(|key| format!("{}:{key}", node.criome_domain_name));
    node.nix_cache_domain = node
        .is_nix_cache
        .then(|| format!("nix.{}", node.criome_domain_name));
    node.nix_url = node
        .nix_cache_domain
        .as_ref()
        .map(|domain| format!("http://{domain}"));
    node.ssh_public_key_line = format!("ssh-ed25519 {}", node.keys.ssh);
}

fn project_disk(value: &DiskLayout) -> Disk {
    Disk {
        device: value.0.as_ref().to_owned(),
        mount: value.1.as_ref().to_owned(),
        fs_type: fs_type(&value.2).to_owned(),
        options: value.3.iter().map(|v| v.as_ref().to_owned()).collect(),
    }
}
fn project_machine(
    node: &str,
    value: &MachineDefinition,
    nodes: &BTreeMap<String, NodeDefinition>,
) -> Result<Machine, Error> {
    match value {
        MachineDefinition::Metal(_, hardware) => Ok(Machine {
            kind: "Metal".into(),
            architecture: architecture_name(&resolved_architecture(node, nodes)?).into(),
            host: None,
            additional_hosts: vec![],
            user: None,
            disk_gib: None,
            hardware: project_hardware(hardware),
        }),
        MachineDefinition::VirtualMachine(host_choice, hardware, disk_gib) => {
            let (host, additional_hosts, user) = match host_choice {
                VirtualMachineHost::Cluster(host, additional_hosts, user, _) => (
                    Some(host.as_ref().to_owned()),
                    additional_hosts
                        .iter()
                        .map(|v| v.as_ref().to_owned())
                        .collect(),
                    user.as_ref().map(|v| v.as_ref().to_owned()),
                ),
                VirtualMachineHost::External(host, _) => {
                    (Some(host.as_ref().to_owned()), Vec::new(), None)
                }
            };
            Ok(Machine {
                kind: "VirtualMachine".into(),
                architecture: architecture_name(&resolved_architecture(node, nodes)?).into(),
                host,
                additional_hosts,
                user,
                disk_gib: *disk_gib,
                hardware: project_hardware(hardware),
            })
        }
    }
}
fn project_hardware(value: &Hardware) -> HardwareView {
    HardwareView {
        cores: value.0,
        model: value.1.as_ref().map(|v| v.as_ref().to_owned()),
        motherboard: value.2.as_ref().map(motherboard).map(str::to_owned),
        chip_generation: value.3,
        ram_gib: value.4,
        location: value.5.as_ref().map(|v| v.as_ref().to_owned()),
    }
}
fn project_network(value: &NodeNetwork) -> Network {
    Network {
        link_local_ips: value.0.iter().map(|v| v.as_ref().to_owned()).collect(),
        node_ip: value.1.as_ref().map(|v| v.as_ref().to_owned()),
        wireguard_public_key: value.2.as_ref().map(|v| v.as_ref().to_owned()),
        wireguard_proxies: value
            .3
            .iter()
            .map(|p| WireguardProxyView {
                public_key: p.0.as_ref().to_owned(),
                endpoint: p.1.as_ref().to_owned(),
                interface_ip: p.2.as_ref().to_owned(),
            })
            .collect(),
        router_interfaces: value.4.as_ref().map(project_router),
    }
}
fn project_router(value: &RouterInterfaces) -> RouterInterfacesView {
    RouterInterfacesView {
        wan: value.0.as_ref().to_owned(),
        wlan: value.1.as_ref().to_owned(),
        wlan_band: wlan_band(&value.2).into(),
        wlan_channel: value.3,
        wlan_standard: wlan_standard(&value.4).into(),
        wpa3_sae_password_reference: value.5.as_ref().map(|v| v.0.as_ref().to_owned()),
        backup_wireless: value.6.as_ref().map(|v| BackupWirelessView {
            interface: v.0.as_ref().to_owned(),
            network_name: v.1.as_ref().to_owned(),
            band: wlan_band(&v.2).into(),
            channel: v.3,
            standard: wlan_standard(&v.4).into(),
            password_reference: v.5.0.as_ref().to_owned(),
        }),
    }
}
fn project_keys(value: &NodeKeys) -> Keys {
    Keys {
        ssh: value.0.as_ref().to_owned(),
        nix: value.1.as_ref().map(|v| v.as_ref().to_owned()),
        yggdrasil: value.2.as_ref().map(|v| YggdrasilKeyView {
            public_key: v.0.as_ref().to_owned(),
            address: v.1.as_ref().to_owned(),
            subnet: v.2.as_ref().to_owned(),
        }),
    }
}
fn project_capability(value: &NodeCapability) -> Capability {
    match value {
        NodeCapability::Graphical(_) => Capability::Graphical,
        NodeCapability::Center(_) => Capability::Center,
        NodeCapability::LargeAi(_) => Capability::LargeAi,
        NodeCapability::Router(_) => Capability::Router,
        NodeCapability::Edge(_) => Capability::Edge,
        NodeCapability::NextGeneration(_) => Capability::NextGeneration,
        NodeCapability::LowPower(_) => Capability::LowPower,
        NodeCapability::TestVm(_) => Capability::TestVm,
        NodeCapability::CloudNode(_) => Capability::CloudNode,
        NodeCapability::Printing(_) => Capability::Printing,
        NodeCapability::HardwareVideo(_) => Capability::HardwareVideo,
        NodeCapability::Nordvpn(_) => Capability::Nordvpn,
        NodeCapability::WifiCertificate(_) => Capability::WifiCertificate,
        NodeCapability::TailnetClient(_) => Capability::TailnetClient,
        NodeCapability::TailnetController(_) => Capability::TailnetController,
        NodeCapability::NixBuilder(maximum_jobs) => Capability::NixBuilder {
            maximum_jobs: *maximum_jobs,
        },
        NodeCapability::NixCache(_) => Capability::NixCache,
        NodeCapability::PersonaDevelopment(values) => Capability::PersonaDevelopment {
            capabilities: values
                .iter()
                .map(persona_capability)
                .map(str::to_owned)
                .collect(),
        },
        NodeCapability::VmHost(subnet, kvm, maximum_guests) => Capability::VmHost {
            guest_subnet: subnet.as_ref().to_owned(),
            kvm: kvm_availability(kvm).into(),
            maximum_guests: *maximum_guests,
        },
        NodeCapability::WebHost(sites) => Capability::WebHost {
            sites: sites
                .iter()
                .map(|site| HostedSiteView {
                    domain: site.0.as_ref().to_owned(),
                    source: site.1.as_ref().to_owned(),
                    renderer: site_renderer(&site.2).into(),
                })
                .collect(),
        },
    }
}
fn project_user(
    value: &UserDefinition,
    trust: Magnitude,
    viewpoint: &str,
    public_domain: &str,
    viewpoint_center: bool,
    viewpoint_size: &str,
) -> User {
    let viewpoint_key = value.7.iter().find(|key| key.0.as_ref() == viewpoint);
    let is_code_dev = matches!(value.1, UserRole::Code | UserRole::Unlimited);
    let is_multimedia_dev = matches!(value.1, UserRole::Multimedia | UserRole::Unlimited);
    let is_max_trusted = matches!(trust, Magnitude::Max);
    let mut extra_groups = vec!["audio".to_owned()];
    if matches!(trust, Magnitude::Medium | Magnitude::Large | Magnitude::Max) {
        extra_groups.push("video".to_owned());
    }
    if is_max_trusted {
        extra_groups.extend(
            [
                "adbusers",
                "nixdev",
                "systemd-journal",
                "dialout",
                "plugdev",
                "power",
                "storage",
                "libvirtd",
            ]
            .map(str::to_owned),
        );
    }
    let resolved_size = if magnitude_rank(&value.2) <= size_rank(viewpoint_size) {
        magnitude(&value.2).to_owned()
    } else {
        viewpoint_size.to_owned()
    };
    User {
        name: value.0.as_ref().to_owned(),
        role: user_role(&value.1).into(),
        size: resolved_size,
        trust: magnitude(&trust).into(),
        keyboard: keyboard(&value.3).into(),
        style: style(&value.4).into(),
        github_id: Some(
            value
                .5
                .as_ref()
                .map(|v| v.as_ref().to_owned())
                .unwrap_or_else(|| value.0.as_ref().to_owned()),
        ),
        fast_repeat: value.6,
        public_keys: value
            .7
            .iter()
            .map(|v| UserPubKeyView {
                node: v.0.as_ref().to_owned(),
                ssh: v.1.as_ref().to_owned(),
                keygrip: v.2.as_ref().to_owned(),
            })
            .collect(),
        editor: value.8.as_ref().map(editor).map(str::to_owned),
        text_size: value.9.as_ref().map(text_size).map(str::to_owned),
        has_public_key: viewpoint_key.is_some(),
        email_address: format!("{}@{public_domain}", value.0.as_ref()),
        matrix_id: format!("@{}:{public_domain}", value.0.as_ref()),
        git_signing_key: viewpoint_key.map(|key| format!("&{}", key.2.as_ref())),
        use_colemak: matches!(value.3, Keyboard::Colemak),
        use_fast_repeat: value.6.unwrap_or(true),
        is_multimedia_dev,
        is_code_dev,
        preferred_editor: value
            .8
            .as_ref()
            .map(editor)
            .unwrap_or(if is_code_dev { "Emacs" } else { "Codium" })
            .to_owned(),
        resolved_text_size: value
            .9
            .as_ref()
            .map(text_size)
            .unwrap_or("Medium")
            .to_owned(),
        ssh_public_keys: value
            .7
            .iter()
            .map(|key| format!("ssh-ed25519 {}", key.1.as_ref()))
            .collect(),
        ssh_public_key: viewpoint_key.map(|key| format!("ssh-ed25519 {}", key.1.as_ref())),
        extra_groups,
        enable_linger: is_max_trusted && viewpoint_center,
    }
}

fn size_rank(value: &str) -> u8 {
    match value {
        "Zero" => 0,
        "Min" => 1,
        "Medium" => 2,
        "Large" => 3,
        "Max" => 4,
        _ => 0,
    }
}
fn project_domain(value: &DomainDefinition) -> Domain {
    Domain {
        name: value.0.as_ref().to_owned(),
        provider: domain_provider(&value.1).into(),
    }
}
fn project_trust(value: &ClusterTrust) -> Trust {
    Trust {
        cluster: magnitude(&value.0).into(),
        clusters: value
            .1
            .iter()
            .map(|v| TrustEntry {
                name: v.0.as_ref().to_owned(),
                magnitude: magnitude(&v.1).into(),
            })
            .collect(),
        nodes: value
            .2
            .iter()
            .map(|v| TrustEntry {
                name: v.0.as_ref().to_owned(),
                magnitude: magnitude(&v.1).into(),
            })
            .collect(),
        users: value
            .3
            .iter()
            .map(|v| TrustEntry {
                name: v.0.as_ref().to_owned(),
                magnitude: magnitude(&v.1).into(),
            })
            .collect(),
    }
}

fn architecture_name(value: &Architecture) -> &'static str {
    match value {
        Architecture::X86_64 => "x86_64",
        Architecture::Arm64 => "aarch64",
    }
}
fn magnitude(value: &Magnitude) -> &'static str {
    match value {
        Magnitude::Zero => "Zero",
        Magnitude::Min => "Min",
        Magnitude::Medium => "Medium",
        Magnitude::Large => "Large",
        Magnitude::Max => "Max",
    }
}
fn keyboard(value: &Keyboard) -> &'static str {
    match value {
        Keyboard::Qwerty => "Qwerty",
        Keyboard::Colemak => "Colemak",
    }
}
fn style(value: &Style) -> &'static str {
    match value {
        Style::Vim => "Vim",
        Style::Emacs => "Emacs",
    }
}
fn editor(value: &Editor) -> &'static str {
    match value {
        Editor::Codium => "Codium",
        Editor::Emacs => "Emacs",
    }
}
fn text_size(value: &TextSize) -> &'static str {
    match value {
        TextSize::ExtraSmall => "ExtraSmall",
        TextSize::Small => "Small",
        TextSize::Medium => "Medium",
        TextSize::Large => "Large",
        TextSize::ExtraLarge => "ExtraLarge",
    }
}
fn bootloader(value: &Bootloader) -> &'static str {
    match value {
        Bootloader::Uefi => "Uefi",
        Bootloader::Mbr => "Mbr",
        Bootloader::Uboot => "Uboot",
    }
}
fn fs_type(value: &FsType) -> &'static str {
    match value {
        FsType::Ext2 => "Ext2",
        FsType::Ext3 => "Ext3",
        FsType::Ext4 => "Ext4",
        FsType::Btrfs => "Btrfs",
        FsType::Xfs => "Xfs",
        FsType::Zfs => "Zfs",
        FsType::F2fs => "F2fs",
        FsType::Bcachefs => "Bcachefs",
        FsType::Vfat => "Vfat",
        FsType::Exfat => "Exfat",
        FsType::Ntfs => "Ntfs",
        FsType::Tmpfs => "Tmpfs",
    }
}
fn motherboard(value: &MotherBoard) -> &'static str {
    match value {
        MotherBoard::Ondyfaind => "Ondyfaind",
    }
}
fn wlan_band(value: &WlanBand) -> &'static str {
    match value {
        WlanBand::TwoG => "2g",
        WlanBand::FiveG => "5g",
        WlanBand::SixG => "6g",
    }
}
fn wlan_standard(value: &WlanStandard) -> &'static str {
    match value {
        WlanStandard::Wifi4 => "Wifi4",
        WlanStandard::Wifi6 => "Wifi6",
        WlanStandard::Wifi7 => "Wifi7",
    }
}
fn kvm_availability(value: &KvmAvailability) -> &'static str {
    match value {
        KvmAvailability::Available => "Available",
        KvmAvailability::Absent => "Absent",
    }
}
fn site_renderer(value: &SiteRenderer) -> &'static str {
    match value {
        SiteRenderer::MarkdownStatic => "MarkdownStatic",
    }
}
fn persona_capability(value: &PersonaCapability) -> &'static str {
    match value {
        PersonaCapability::GitoliteServer => "GitoliteServer",
    }
}
fn user_role(value: &UserRole) -> &'static str {
    match value {
        UserRole::Code => "Code",
        UserRole::Multimedia => "Multimedia",
        UserRole::Unlimited => "Unlimited",
    }
}
fn domain_provider(value: &DomainProvider) -> &'static str {
    match value {
        DomainProvider::Cloudflare => "Cloudflare",
    }
}
