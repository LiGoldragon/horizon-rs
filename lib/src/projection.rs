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
    pub wpa3_sae_password: Option<String>,
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
    pub password: String,
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
        let ex_nodes = resolved
            .nodes
            .iter()
            .filter(|(name, _)| name.as_str() != node)
            .map(|(name, definition)| {
                Ok((name.clone(), project_node(definition, &resolved.nodes)?))
            })
            .collect::<Result<_, Error>>()?;
        Ok(Horizon {
            cluster: resolved.name,
            node: project_node(viewpoint, &resolved.nodes)?,
            ex_nodes,
            users: self
                .1
                .3
                .iter()
                .filter_map(|user| {
                    let trust = effective_user_trust(&self.1.5, user.0.as_ref());
                    (!matches!(trust, Magnitude::Zero)).then(|| project_user(user, trust))
                })
                .collect(),
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
        resolved_architecture(name, nodes, &mut BTreeSet::new())?;
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
    visiting: &mut BTreeSet<String>,
) -> Result<Architecture, Error> {
    if !visiting.insert(name.to_owned()) {
        return Err(Error::VmHostCycle(name.to_owned()));
    }
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
                let observed = resolved_architecture(&host, nodes, visiting)?;
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
    visiting.remove(name);
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
    })
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
            architecture: architecture_name(&resolved_architecture(
                node,
                nodes,
                &mut BTreeSet::new(),
            )?)
            .into(),
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
                architecture: architecture_name(&resolved_architecture(
                    node,
                    nodes,
                    &mut BTreeSet::new(),
                )?)
                .into(),
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
        wpa3_sae_password: value.5.as_ref().map(|v| v.0.as_ref().to_owned()),
        backup_wireless: value.6.as_ref().map(|v| BackupWirelessView {
            interface: v.0.as_ref().to_owned(),
            network_name: v.1.as_ref().to_owned(),
            band: wlan_band(&v.2).into(),
            channel: v.3,
            standard: wlan_standard(&v.4).into(),
            password: v.5.0.as_ref().to_owned(),
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
fn project_user(value: &UserDefinition, trust: Magnitude) -> User {
    User {
        name: value.0.as_ref().to_owned(),
        role: user_role(&value.1).into(),
        size: magnitude(&value.2).into(),
        trust: magnitude(&trust).into(),
        keyboard: keyboard(&value.3).into(),
        style: style(&value.4).into(),
        github_id: value.5.as_ref().map(|v| v.as_ref().to_owned()),
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
