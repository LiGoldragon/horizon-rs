//! Proposal-side `ClusterProposal` — the cluster-shaped input goldragon
//! emits as `datom.nota`.
//!
//! `ClusterProposal::project` is the single entry-point that produces
//! a typed `view::Horizon` from a viewpoint `(cluster, node)`.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::horizon_proposal::HorizonProposal;
use crate::magnitude::Magnitude;
use crate::name::{ClusterName, DomainName, NodeName, UserName};
use crate::proposal::ai::AiProvider;
use crate::proposal::domain::DomainProposal;
use crate::proposal::node::{NodeProjection, NodeProposal};
use crate::proposal::secret::{ClusterSecretBinding, SecretBackend, SecretName};
use crate::proposal::services::{TailnetConfig, TailnetControllerRole};
use crate::proposal::user::{UserProjection, UserProposal};
use crate::proposal::vpn::VpnProfile;
use crate::view::cluster::Cluster;
use crate::view::horizon::{Horizon, Viewpoint};
use crate::view::node::{Node, ViewpointFill};
use crate::view::user::User;

/// The proposal a cluster owner emits.
#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ClusterProposal {
    #[serde(default)]
    pub nodes: BTreeMap<NodeName, NodeProposal>,
    #[serde(default)]
    pub users: BTreeMap<UserName, UserProposal>,
    #[serde(default)]
    pub domains: BTreeMap<DomainName, DomainProposal>,
    pub trust: ClusterTrust,
    /// Resolves logical `SecretReference` names that appear on node-level
    /// records (Wi-Fi passwords, VPN credentials, etc.) to a concrete
    /// `SecretBackend`. Empty default keeps existing datom records
    /// parsing; nodes that author secret references require matching
    /// entries here, validated at projection time once a consumer is in
    /// place. Must stay near the tail so existing positional nota
    /// records keep decoding.
    #[serde(default)]
    pub secret_bindings: Vec<ClusterSecretBinding>,
    /// Optional per-cluster tailnet trust material. The base domain is
    /// derived by Horizon from the pan-horizon internal suffix.
    #[serde(default)]
    pub tailnet: Option<TailnetConfig>,
    /// AI providers the cluster advertises to consumers. Each entry
    /// selects a CriomOS-owned provider profile and the node hosting it.
    #[serde(default)]
    pub ai_providers: Vec<AiProvider>,
    /// VPN provider selections (NordVPN today; WireguardMesh later).
    /// Server catalogs and client defaults live in CriomOS.
    #[serde(default)]
    pub vpn_profiles: Vec<VpnProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrust {
    pub cluster: Magnitude,
    #[serde(default)]
    pub clusters: BTreeMap<ClusterName, Magnitude>,
    #[serde(default)]
    pub nodes: BTreeMap<NodeName, Magnitude>,
    #[serde(default)]
    pub users: BTreeMap<UserName, Magnitude>,
}

impl ClusterProposal {
    /// Project this proposal from a viewpoint into a typed `view::Horizon`.
    pub fn project(&self, horizon: &HorizonProposal, viewpoint: &Viewpoint) -> Result<Horizon> {
        if !self.nodes.contains_key(&viewpoint.node) {
            return Err(Error::NodeNotInCluster(viewpoint.node.clone()));
        }

        let cluster_trust_floor = self.trust.cluster;
        let tailnet_controller = self.validate_tailnet_topology(cluster_trust_floor)?;
        let secret_bindings = self.resolve_secret_bindings()?;
        let router_ssid = horizon.router_ssid(&viewpoint.cluster)?;

        // Build every Node (no viewpoint fill yet).
        let mut nodes: BTreeMap<NodeName, Node> = BTreeMap::new();
        for (name, proposal) in &self.nodes {
            let trust = self.node_trust(name, proposal.trust, cluster_trust_floor);
            if matches!(trust, Magnitude::Zero) {
                // trust=Zero marks a node as actively distrusted; drop it
                // from the horizon entirely.
                continue;
            }
            let resolved_arch = proposal.resolve_arch(name, &self.nodes)?;
            let ctx = NodeProjection {
                name: name.clone(),
                cluster: &viewpoint.cluster,
                cluster_domain: horizon.internal_domain(),
                router_ssid: router_ssid.clone(),
                trust,
                resolved_arch,
            };
            nodes.insert(name.clone(), proposal.project(ctx));
        }

        // Build every User (per-viewpoint). Users need the viewpoint
        // node's `behaves_as.center` to compute `enable_linger`, so
        // look it up once before the loop.
        let viewpoint_behaves_as_center = nodes
            .get(&viewpoint.node)
            .expect("viewpoint node was projected above")
            .behaves_as
            .center;
        let viewpoint_node_size = self
            .nodes
            .get(&viewpoint.node)
            .expect("viewpoint node proposal exists")
            .size;
        let mut users: BTreeMap<UserName, User> = BTreeMap::new();
        for (name, proposal) in &self.users {
            let trust = self
                .trust
                .users
                .get(name)
                .copied()
                .unwrap_or(Magnitude::Min);
            if matches!(trust, Magnitude::Zero) {
                continue;
            }
            let ctx = UserProjection {
                name: name.clone(),
                cluster: &viewpoint.cluster,
                cluster_public_domain: horizon.public_domain(),
                viewpoint_node: &viewpoint.node,
                trust,
                viewpoint_behaves_as_center,
                viewpoint_node_size,
            };
            users.insert(name.clone(), proposal.project(ctx));
        }

        let router_node = self.router_node_name(cluster_trust_floor);
        let lan = router_node
            .as_ref()
            .map(|router| horizon.lan_network(&viewpoint.cluster, router))
            .transpose()?;
        let resolver = horizon.resolver_policy(lan.as_ref())?;
        let tailnet = if self.tailnet.is_some() || tailnet_controller.is_some() {
            Some(crate::view::cluster::TailnetConfig {
                base_domain: horizon.tailnet_base_domain(&viewpoint.cluster)?,
                tls: self.tailnet.as_ref().and_then(|tailnet| tailnet.tls.clone()),
            })
        } else {
            None
        };

        // Cluster-level roll-up.
        let cluster = Cluster {
            name: viewpoint.cluster.clone(),
            domain: horizon.internal_domain().clone(),
            trusted_build_pub_keys: nodes
                .values()
                .filter_map(|n| n.nix_pub_key_line.clone())
                .collect(),
            lan,
            resolver,
            tailnet,
            ai_providers: self.ai_providers.clone(),
            vpn_profiles: self.vpn_profiles.clone(),
            secret_bindings,
        };

        // Clone the viewpoint node so we can fill it while the full
        // `nodes` map (including the viewpoint itself) is still available
        // — admin-ssh-key derivation needs the viewpoint node visible
        // among the fully-trusted ones.
        let mut viewpoint_node = nodes
            .get(&viewpoint.node)
            .expect("viewpoint node was checked to exist above")
            .clone();

        let proposal_io = self
            .nodes
            .get(&viewpoint.node)
            .expect("viewpoint node proposal exists")
            .io
            .clone();
        let wireguard_untrusted_proxies = self
            .nodes
            .get(&viewpoint.node)
            .expect("viewpoint node proposal exists")
            .wireguard_untrusted_proxies
            .clone();

        let fill = ViewpointFill {
            proposal_io,
            all_nodes: &nodes,
            all_users: &users,
            wireguard_untrusted_proxies,
        };
        viewpoint_node.fill_viewpoint(fill);

        // Walk every node proposal: if its placement names this
        // viewpoint as host, surface it as a contained_node.
        let mut contained_nodes = BTreeMap::new();
        for (name, proposal) in &self.nodes {
            let crate::proposal::placement::NodePlacement::Contained {
                host,
                user,
                substrate,
                resources,
                network,
                state,
                trust,
                user_namespace_policy,
            } = &proposal.placement
            else {
                continue;
            };
            if host != &viewpoint.node {
                continue;
            }
            contained_nodes.insert(
                name.clone(),
                crate::view::ProjectedNodeView {
                    name: name.clone(),
                    user: user.clone(),
                    cores: resources.cores,
                    ram_gb: Some(resources.ram_gb),
                    substrate: substrate.clone(),
                    resources: resources.clone(),
                    network: network.clone(),
                    state: state.clone(),
                    trust: *trust,
                    user_namespace_policy: *user_namespace_policy,
                },
            );
        }

        // Now remove the viewpoint from the map so `ex_nodes` excludes it.
        nodes.remove(&viewpoint.node);

        Ok(Horizon {
            cluster,
            node: viewpoint_node,
            ex_nodes: nodes,
            users,
            contained_nodes,
        })
    }

    /// Fold the proposal's authored `Vec<ClusterSecretBinding>` into
    /// the view's lookup-shaped `BTreeMap<SecretName, SecretBackend>`.
    /// Duplicate names loud-fail at projection time — the binding
    /// table is a resolution function, not a multi-set.
    fn resolve_secret_bindings(&self) -> Result<BTreeMap<SecretName, SecretBackend>> {
        let mut resolved: BTreeMap<SecretName, SecretBackend> = BTreeMap::new();
        for ClusterSecretBinding { name, backend } in &self.secret_bindings {
            if resolved.contains_key(name) {
                return Err(Error::DuplicateSecretBinding { name: name.clone() });
            }
            resolved.insert(name.clone(), backend.clone());
        }
        Ok(resolved)
    }

    fn validate_tailnet_topology(&self, cluster_trust_floor: Magnitude) -> Result<Option<NodeName>> {
        let mut server: Option<NodeName> = None;

        for (name, proposal) in &self.nodes {
            let trust = self.node_trust(name, proposal.trust, cluster_trust_floor);
            if matches!(trust, Magnitude::Zero) {
                continue;
            }
            if !matches!(
                proposal.services.tailnet_controller,
                Some(TailnetControllerRole::Server { .. })
            ) {
                continue;
            }

            if let Some(first) = server {
                return Err(Error::MultipleTailnetControllers {
                    first,
                    second: name.clone(),
                });
            }

            server = Some(name.clone());
        }

        Ok(server)
    }

    fn router_node_name(&self, cluster_trust_floor: Magnitude) -> Option<NodeName> {
        self.nodes.iter().find_map(|(name, proposal)| {
            let trust = self.node_trust(name, proposal.trust, cluster_trust_floor);
            if matches!(trust, Magnitude::Zero) || proposal.router_interfaces.is_none() {
                None
            } else {
                Some(name.clone())
            }
        })
    }

    /// `min(input_trust, self.trust.nodes[name], cluster_trust)`.
    fn node_trust(
        &self,
        name: &NodeName,
        input_trust: Magnitude,
        cluster_trust: Magnitude,
    ) -> Magnitude {
        let per_node = self
            .trust
            .nodes
            .get(name)
            .copied()
            .unwrap_or(Magnitude::Max);
        input_trust.min(per_node).min(cluster_trust)
    }
}
