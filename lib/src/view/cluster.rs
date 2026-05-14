//! Cluster-level identity and roll-ups in the projected view.

use serde::{Deserialize, Serialize};

use crate::name::ClusterName;
use crate::proposal::network::{LanNetwork, ResolverPolicy};
use crate::proposal::services::TailnetConfig;
use crate::pub_key::NixPubKeyLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub name: ClusterName,
    /// One entry per node that has a nix signing key.
    pub trusted_build_pub_keys: Vec<NixPubKeyLine>,
    /// Cluster LAN policy (subnet, gateway, DHCP pool, lease policy)
    /// passed through from the proposal. `None` means CriomOS
    /// modules use their current implementation defaults.
    pub lan: Option<LanNetwork>,
    /// Cluster DNS-resolver policy (upstreams, fallbacks, listen
    /// addresses) passed through from the proposal. `None` means
    /// CriomOS modules use their current implementation defaults.
    pub resolver: Option<ResolverPolicy>,
    /// Cluster tailnet configuration (base DNS domain plus optional
    /// CA-trust material). Required when any node hosts a tailnet
    /// controller; validated at projection time. `None` means the
    /// cluster has no tailnet.
    pub tailnet: Option<TailnetConfig>,
}
