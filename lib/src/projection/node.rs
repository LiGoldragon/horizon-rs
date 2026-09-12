//! The nodes of a cluster: catalogued by name, validated, and projected.

use std::collections::BTreeMap;

use datom_codec::Decimating;

use super::error::Error;
use super::names::Named;
use super::views::Projection;
use crate::*;

/// The nodes of one resolved cluster, by node name.
pub type NodeDefinitions = BTreeMap<String, NodeDefinition>;

/// An authored run of node definitions, read as a catalogue keyed by name.
pub(crate) trait NodeCatalogue {
    /// Keys every definition by its own name, refusing a repeated name with
    /// the caller's error for that catalogue.
    fn catalogue(&self, duplicate: fn(String) -> Error) -> Result<NodeDefinitions, Error>;
}

impl NodeCatalogue for [NodeDefinition] {
    fn catalogue(&self, duplicate: fn(String) -> Error) -> Result<NodeDefinitions, Error> {
        let mut nodes = NodeDefinitions::new();
        for definition in self {
            let name = definition.node_name.clone();
            if nodes.insert(name.clone(), definition.clone()).is_some() {
                return Err(duplicate(name));
            }
        }
        Ok(nodes)
    }
}

/// The whole set of a cluster's nodes, whose facts only hold against each
/// other: a virtual machine's architecture is its hosts', and a tailnet has
/// at most one controller.
pub(crate) trait NodeGraph {
    /// The architecture a node runs, inherited through its hosts when it is a
    /// virtual machine.
    fn architecture(&self, name: &str) -> Result<Architecture, Error>;

    /// Refuses a cluster whose every node cannot be given an architecture.
    fn validate_machines(&self) -> Result<(), Error>;

    /// Refuses a cluster naming more than one tailnet controller.
    fn validate_tailnet_controller(&self) -> Result<(), Error>;
}

impl NodeGraph for NodeDefinitions {
    fn architecture(&self, name: &str) -> Result<Architecture, Error> {
        let definition = self.get(name).ok_or_else(|| Error::UnknownVmHost {
            node: name.to_owned(),
            host: name.to_owned(),
        })?;
        let cluster = match &definition.machine_definition {
            MachineDefinition::Metal(data) => return Ok(data.architecture.clone()),
            MachineDefinition::VirtualMachine(data) => match &data.virtual_machine_host {
                VirtualMachineHost::External(external) => {
                    return Ok(external.architecture.clone());
                }
                VirtualMachineHost::Cluster(cluster) => cluster,
            },
        };
        let mut hosts = vec![cluster.node_name.clone()];
        hosts.extend(cluster.node_name_vector.iter().cloned());
        let mut host_architecture = None;
        for host in hosts {
            let Some(host_definition) = self.get(&host) else {
                return Err(Error::UnknownVmHost {
                    node: name.to_owned(),
                    host,
                });
            };
            let observed = match &host_definition.machine_definition {
                MachineDefinition::Metal(data) => data.architecture.clone(),
                MachineDefinition::VirtualMachine(data) => match &data.virtual_machine_host {
                    VirtualMachineHost::External(external) => external.architecture.clone(),
                    VirtualMachineHost::Cluster(host) => host
                        .architecture_option
                        .clone()
                        .ok_or_else(|| Error::VmHostArchitecture {
                            node: name.to_owned(),
                        })?,
                },
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
        if cluster
            .architecture_option
            .as_ref()
            .is_some_and(|declared| declared != &inherited)
        {
            return Err(Error::VmHostArchitecture {
                node: name.to_owned(),
            });
        }
        Ok(cluster.architecture_option.clone().unwrap_or(inherited))
    }

    fn validate_machines(&self) -> Result<(), Error> {
        for name in self.keys() {
            self.architecture(name)?;
        }
        Ok(())
    }

    fn validate_tailnet_controller(&self) -> Result<(), Error> {
        let mut controllers = self.iter().filter(|(_, node)| {
            node.capabilities
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
}

/// A machine whose view needs the cluster it sits in, because its
/// architecture is resolved through its hosts.
pub(crate) trait MachineProjection {
    /// `node` names the node this machine belongs to.
    fn project_in(&self, node: &str, nodes: &NodeDefinitions) -> Result<Machine, Error>;
}

impl MachineProjection for MachineDefinition {
    fn project_in(&self, node: &str, nodes: &NodeDefinitions) -> Result<Machine, Error> {
        let architecture = nodes.architecture(node)?.name().into();
        match self {
            MachineDefinition::Metal(data) => Ok(Machine {
                kind: "Metal".into(),
                architecture,
                host: None,
                additional_hosts: vec![],
                user: None,
                disk_gib: None,
                hardware: data.hardware.project(),
            }),
            MachineDefinition::VirtualMachine(data) => {
                let (host, additional_hosts, user) = match &data.virtual_machine_host {
                    VirtualMachineHost::Cluster(cluster) => (
                        Some(cluster.node_name.clone()),
                        cluster.node_name_vector.clone(),
                        cluster.user_name_option.clone(),
                    ),
                    VirtualMachineHost::External(external) => {
                        (Some(external.string.clone()), Vec::new(), None)
                    }
                };
                Ok(Machine {
                    kind: "VirtualMachine".into(),
                    architecture,
                    host,
                    additional_hosts,
                    user,
                    disk_gib: data.integer_option,
                    hardware: data.hardware.project(),
                })
            }
        }
    }
}

/// A node whose view needs the cluster it sits in.
pub(crate) trait NodeProjection {
    /// The node's own facts as a view; cluster-relative facts stay empty
    /// until [`super::viewpoint::NodeDeriving::derive`] fills them.
    fn project_in(&self, nodes: &NodeDefinitions) -> Result<Node, Error>;
}

impl NodeProjection for NodeDefinition {
    fn project_in(&self, nodes: &NodeDefinitions) -> Result<Node, Error> {
        let (variant, installation) = match &self.node_variant {
            NodeVariant::Live(_) => ("Live".to_owned(), None),
            NodeVariant::Installation(value) => (
                "Installation".to_owned(),
                Some(InstallationView {
                    bootloader: value.bootloader.name().to_owned(),
                    disks: value
                        .disk_layout_vector
                        .iter()
                        .map(Projection::project)
                        .collect(),
                    swap_devices: value
                        .swap_device_vector
                        .iter()
                        .map(|swap| SwapDeviceView {
                            device: swap.device_path.clone(),
                            size_mebibytes: swap.integer_option,
                        })
                        .collect(),
                }),
            ),
        };
        let installation_disks = installation
            .as_ref()
            .map_or_else(Vec::new, |value| value.disks.clone());
        Ok(Node {
            name: self.node_name.clone(),
            is_live: installation.is_none(),
            is_installation: installation.is_some(),
            variant,
            installation,
            installation_disks,
            size: self.first_magnitude.name().to_owned(),
            trust: self.second_magnitude.name().to_owned(),
            online: self.boolean_option,
            keyboard: self.node_environment.keyboard.name().to_owned(),
            compressed_swap_memory_percent: self
                .node_environment
                .compressed_swap_option
                .as_ref()
                .map(|value| value.integer),
            machine: self.machine_definition.project_in(&self.node_name, nodes)?,
            network: self.node_network.project(),
            keys: self.node_keys.project(),
            capabilities: self.capabilities.iter().map(Projection::project).collect(),
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
            fixed_location: self
                .fixed_location_option
                .as_ref()
                // A `Decimal` is finite by construction, so widening it back to
                // the `f64` the serialized view carries cannot produce a value
                // the view could not hold.
                .map(|value| FixedLocationView {
                    latitude: value.first_decimal.float(),
                    longitude: value.second_decimal.float(),
                    altitude: value.third_decimal.float(),
                    accuracy: value.fourth_decimal.float(),
                }),
        })
    }
}
