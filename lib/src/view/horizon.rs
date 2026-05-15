//! `Horizon` — the projected view from one node.
//!
//! `ClusterProposal::project` (in `proposal::cluster`) is the
//! constructor that produces a `Horizon` from a typed cluster proposal
//! plus a `Viewpoint`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::name::{ClusterName, NodeName, UserName};
use crate::view::cluster::Cluster;
use crate::view::node::Node;
use crate::view::projected_node::ProjectedNodeView;
use crate::view::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Horizon {
    pub cluster: Cluster,
    pub node: Node,
    pub ex_nodes: BTreeMap<NodeName, Node>,
    pub users: BTreeMap<UserName, User>,
    /// Nodes whose `placement = Contained { host: <viewpoint> }`,
    /// projected one level deep. Empty for non-host viewpoints.
    pub contained_nodes: BTreeMap<NodeName, ProjectedNodeView>,
}

#[derive(Debug, Clone)]
pub struct Viewpoint {
    pub cluster: ClusterName,
    pub node: NodeName,
}
