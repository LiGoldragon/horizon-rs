//! One-level-deep view of a contained node, as the host sees it.
//!
//! `Horizon.contained_nodes[name]` lets a host know which containers
//! it must launch and the resources to give them.

use serde::{Deserialize, Serialize};

use crate::magnitude::Magnitude;
use crate::name::{NodeName, UserName};
use crate::proposal::{
    ContainedNetwork, ContainedState, Resources, Substrate, UserNamespacePolicy,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedNodeView {
    pub name: NodeName,
    pub user: UserName,
    pub cores: u32,
    pub ram_gb: Option<u32>,
    pub substrate: Substrate,
    pub resources: Resources,
    pub network: ContainedNetwork,
    pub state: ContainedState,
    pub trust: Magnitude,
    pub user_namespace_policy: UserNamespacePolicy,
}
