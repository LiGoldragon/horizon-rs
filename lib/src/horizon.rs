//! `Horizon` — the projected view from one node, plus the
//! `ClusterProposal::project` entry-point.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::cluster::Cluster;
use crate::error::{Error, Result};
use crate::magnitude::Magnitude;
use crate::name::{ClusterName, NodeName, UserName};
use crate::node::{resolve_arch, Node, NodeProjection, ViewpointFill};
use crate::proposal::ClusterProposal;
use crate::user::{User, UserProjection};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Horizon {
    pub cluster: Cluster,
    pub node: Node,
    pub ex_nodes: BTreeMap<NodeName, Node>,
    pub users: BTreeMap<UserName, User>,
}

#[derive(Debug, Clone)]
pub struct Viewpoint {
    pub cluster: ClusterName,
    pub node: NodeName,
}

impl ClusterProposal {
    /// Project this proposal from a viewpoint into a typed `Horizon`.
    pub fn project(&self, viewpoint: &Viewpoint) -> Result<Horizon> {
        if !self.nodes.contains_key(&viewpoint.node) {
            return Err(Error::NodeNotInCluster(viewpoint.node.clone()));
        }

        let cluster_trust_floor = self.trust.cluster;

        // Build every Node (no viewpoint fill yet).
        let mut nodes: BTreeMap<NodeName, Node> = BTreeMap::new();
        for (name, proposal) in &self.nodes {
            let trust = node_trust(self, name, proposal.trust, cluster_trust_floor);
            if matches!(trust, Magnitude::Non) {
                continue; // legacy: trust=0 nodes are filtered out of horizon
            }
            let resolved_arch = resolve_arch(name, &proposal.machine, &self.nodes)?;
            let ctx = NodeProjection {
                name: name.clone(),
                cluster: &viewpoint.cluster,
                trust,
                resolved_arch,
            };
            nodes.insert(name.clone(), proposal.project(ctx));
        }

        // Build every User (per-viewpoint).
        let mut users: BTreeMap<UserName, User> = BTreeMap::new();
        for (name, proposal) in &self.users {
            let trust = self
                .trust
                .users
                .get(name)
                .copied()
                .unwrap_or(Magnitude::Min);
            if matches!(trust, Magnitude::Non) {
                continue;
            }
            let ctx = UserProjection {
                name: name.clone(),
                cluster: &viewpoint.cluster,
                viewpoint_node: &viewpoint.node,
                trust,
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
}

/// `min(node_input_trust, cluster.trust.nodes[node], cluster.trust.cluster)`.
fn node_trust(
    proposal: &ClusterProposal,
    name: &NodeName,
    input_trust: Magnitude,
    cluster_trust: Magnitude,
) -> Magnitude {
    let per_node = proposal
        .trust
        .nodes
        .get(name)
        .copied()
        .unwrap_or(Magnitude::Max);
    input_trust.floor(per_node).floor(cluster_trust)
}
