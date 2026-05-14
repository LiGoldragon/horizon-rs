//! Per-node service-role records authored as cluster-proposal data.
//!
//! Names the role (e.g. tailnet membership, tailnet-controller server)
//! rather than deriving it from node identity. CriomOS renders these
//! with concrete services (Tailscale, Headscale) at deploy time.

use nota_codec::{NotaEnum, NotaRecord, NotaSum};
use serde::{Deserialize, Serialize};

use crate::name::DomainName;

#[derive(Debug, Clone, Default, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodeServices {
    /// Whether this node should join the cluster tailnet. CriomOS
    /// currently renders this with Tailscale, but the proposal names the
    /// role rather than deriving it from node identity.
    #[serde(default)]
    pub tailnet: Option<TailnetMembership>,

    /// Whether this node hosts the cluster tailnet controller service.
    /// CriomOS currently renders this with Headscale.
    #[serde(default)]
    pub tailnet_controller: Option<TailnetControllerRole>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaEnum)]
pub enum TailnetMembership {
    Client,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum TailnetControllerRole {
    Server { port: u16, base_domain: DomainName },
}
