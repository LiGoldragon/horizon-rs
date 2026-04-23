//! Output `Cluster`: cluster-level identity and roll-ups.

use serde::{Deserialize, Serialize};

use crate::name::ClusterName;
use crate::pub_key::NixPubKeyLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub name: ClusterName,
    /// One entry per node that has a nix signing key.
    pub trusted_build_pub_keys: Vec<NixPubKeyLine>,
}
