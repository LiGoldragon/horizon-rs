//! The facts a projected node holds only relative to its cluster.

use std::collections::BTreeMap;

use super::names::Named;
use crate::*;

/// A projected node completed against the cluster it belongs to.
pub(crate) trait NodeDeriving {
    /// Fills every fact that follows from the node's capabilities and the
    /// cluster's naming and trust.
    fn derive(&mut self, cluster: &str, suffix: &str, trust: Magnitude);

    /// The remote-builder entry this node offers its peers.
    fn builder_config(&self) -> BuilderConfig;

    /// Fills the facts a node holds only as the viewpoint: what it may build
    /// on, fetch from, and admit.
    fn fill_viewpoint(&mut self, ex_nodes: &BTreeMap<String, Node>, users: &[User]);
}

impl NodeDeriving for Node {
    fn derive(&mut self, cluster: &str, suffix: &str, trust: Magnitude) {
        let has = |kind: fn(&Capability) -> bool| self.capabilities.iter().any(kind);
        let center = has(|capability| matches!(capability, Capability::Center));
        let router = has(|capability| matches!(capability, Capability::Router));
        let edge = has(|capability| matches!(capability, Capability::Edge));
        let next_generation = has(|capability| matches!(capability, Capability::NextGeneration));
        let low_power = has(|capability| matches!(capability, Capability::LowPower));
        let large_ai = has(|capability| matches!(capability, Capability::LargeAi));
        let test_vm = has(|capability| matches!(capability, Capability::TestVm));
        let cloud_node = has(|capability| matches!(capability, Capability::CloudNode));
        let virtual_machine = self.machine.kind == "VirtualMachine";
        let bare_metal = !virtual_machine;
        let iso = self.is_live && bare_metal;
        self.behaves_as = BehavesAs {
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
        self.criome_domain_name = format!("{}.{}.{}", self.name, cluster, suffix);
        self.system = match self.machine.architecture.as_str() {
            "x86_64" => "x86_64-linux",
            "aarch64" => "aarch64-linux",
            _ => "unknown",
        }
        .to_owned();
        self.trust = trust.name().to_owned();
        self.is_fully_trusted = matches!(trust, Magnitude::Max);
        self.has_base_public_keys = self.keys.nix.is_some() && self.keys.yggdrasil.is_some();
        let maximum_jobs = self
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                Capability::NixBuilder { maximum_jobs } => *maximum_jobs,
                _ => None,
            })
            .unwrap_or(1);
        self.max_jobs = maximum_jobs;
        self.build_cores = maximum_jobs;
        let online = self.online.unwrap_or(true);
        self.is_remote_nix_builder = self
            .capabilities
            .iter()
            .any(|capability| matches!(capability, Capability::NixBuilder { .. }))
            && online
            && self.is_fully_trusted
            && self.has_base_public_keys;
        self.is_dispatcher =
            !center && self.is_fully_trusted && !matches!(self.size.as_str(), "Zero");
        self.is_nix_cache = self
            .capabilities
            .iter()
            .any(|capability| matches!(capability, Capability::NixCache))
            && online
            && self.is_fully_trusted
            && self.has_base_public_keys;
        self.is_large_edge = matches!(self.size.as_str(), "Large" | "Max") && edge;
        self.enable_network_manager =
            !matches!(self.size.as_str(), "Zero") && !iso && !center && !router;
        self.nix_public_key_line = self
            .keys
            .nix
            .as_ref()
            .map(|key| format!("{}:{key}", self.criome_domain_name));
        self.nix_cache_domain = self
            .is_nix_cache
            .then(|| format!("nix.{}", self.criome_domain_name));
        self.nix_url = self
            .nix_cache_domain
            .as_ref()
            .map(|domain| format!("http://{domain}"));
        self.ssh_public_key_line = format!("ssh-ed25519 {}", self.keys.ssh);
    }

    fn builder_config(&self) -> BuilderConfig {
        BuilderConfig {
            host_name: self.criome_domain_name.clone(),
            ssh_user: "nix-ssh".into(),
            ssh_key: "/etc/ssh/ssh_host_ed25519_key".into(),
            supported_features: if self.behaves_as.edge {
                Vec::new()
            } else {
                vec!["big-parallel".into(), "kvm".into()]
            },
            system: self.system.clone(),
            max_jobs: self.max_jobs,
            public_host_key: self.keys.ssh.clone(),
            public_host_key_line: self.ssh_public_key_line.clone(),
        }
    }

    fn fill_viewpoint(&mut self, ex_nodes: &BTreeMap<String, Node>, users: &[User]) {
        self.builder_configs = ex_nodes
            .values()
            .filter(|node| node.is_remote_nix_builder)
            .map(NodeDeriving::builder_config)
            .collect();
        self.cache_urls = ex_nodes
            .values()
            .filter_map(|node| node.nix_url.clone())
            .collect();
        self.ex_nodes_ssh_public_keys = ex_nodes
            .values()
            .map(|node| node.ssh_public_key_line.clone())
            .collect();
        self.dispatchers_ssh_public_keys = ex_nodes
            .values()
            .filter(|node| node.is_dispatcher)
            .map(|node| node.ssh_public_key_line.clone())
            .collect();
        let mut nodes: BTreeMap<&str, &Node> = ex_nodes
            .iter()
            .map(|(name, node)| (name.as_str(), node))
            .collect();
        nodes.insert(self.name.as_str(), self);
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
        if self.machine.kind == "VirtualMachine" {
            for host in self
                .machine
                .host
                .iter()
                .chain(self.machine.additional_hosts.iter())
            {
                if let Some(node) = nodes.get(host.as_str())
                    && let Some(key) = &node.nix_public_key_line
                {
                    image_exchange_public_keys.push(key.clone());
                }
            }
        }
        drop(nodes);
        self.admin_ssh_public_keys = admin_ssh_public_keys;
        self.image_exchange_public_keys = image_exchange_public_keys;
    }
}
