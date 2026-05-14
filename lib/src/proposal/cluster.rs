//! Proposal-side `ClusterProposal` — the cluster-shaped input goldragon
//! emits as `datom.nota`.
//!
//! `ClusterProposal::project` is the single entry-point that produces
//! a typed `view::Horizon` from a viewpoint `(cluster, node)`.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::magnitude::Magnitude;
use crate::name::{ClusterName, DomainName, NodeName, UserName};
use crate::proposal::domain::DomainProposal;
use crate::proposal::network::{LanNetwork, ResolverPolicy};
use crate::proposal::node::{NodeProjection, NodeProposal};
use crate::proposal::secret::ClusterSecretBinding;
use crate::proposal::services::TailnetControllerRole;
use crate::proposal::user::{UserProjection, UserProposal};
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
    /// Per-cluster LAN configuration (subnet, gateway, DHCP pool,
    /// lease policy). `None` means the cluster has no horizon-authored
    /// LAN policy; CriomOS modules fall back to whatever current
    /// implementation defaults exist until the second pass of step 4
    /// rewrites them to read this field. Tail position for positional
    /// nota compatibility.
    #[serde(default)]
    pub lan: Option<LanNetwork>,
    /// Per-cluster DNS-resolver policy (upstreams, fallbacks, local
    /// listen addresses). Same `None` semantics as `lan` above. Tail
    /// position for positional nota compatibility.
    #[serde(default)]
    pub resolver: Option<ResolverPolicy>,
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
    pub fn project(&self, viewpoint: &Viewpoint) -> Result<Horizon> {
        if !self.nodes.contains_key(&viewpoint.node) {
            return Err(Error::NodeNotInCluster(viewpoint.node.clone()));
        }

        let cluster_trust_floor = self.trust.cluster;
        self.validate_tailnet_controller_singleton(cluster_trust_floor)?;

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
                viewpoint_node: &viewpoint.node,
                trust,
                viewpoint_behaves_as_center,
                viewpoint_node_size,
            };
            users.insert(name.clone(), proposal.project(ctx));
        }

        // Cluster-level roll-up.
        let cluster = Cluster {
            name: viewpoint.cluster.clone(),
            trusted_build_pub_keys: nodes
                .values()
                .filter_map(|n| n.nix_pub_key_line.clone())
                .collect(),
            lan: self.lan.clone(),
            resolver: self.resolver.clone(),
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

        // Now remove the viewpoint from the map so `ex_nodes` excludes it.
        nodes.remove(&viewpoint.node);

        Ok(Horizon {
            cluster,
            node: viewpoint_node,
            ex_nodes: nodes,
            users,
        })
    }

    fn validate_tailnet_controller_singleton(&self, cluster_trust_floor: Magnitude) -> Result<()> {
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

        Ok(())
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
