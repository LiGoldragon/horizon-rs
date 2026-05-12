//! Output `Cluster`: cluster-level identity and roll-ups.

use serde::{Deserialize, Serialize};

use crate::name::{ClusterName, ClusterTld};
use crate::pub_key::NixPubKeyLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub name: ClusterName,
    /// Cluster top-level domain — the label that follows `<cluster>` in
    /// every `<node>.<cluster>.<tld>` address. Default `"criome"`; new
    /// clusters may pick another label. CriomOS modules read this rather
    /// than hardcoding `"criome"`.
    pub tld: ClusterTld,
    /// One entry per node that has a nix signing key.
    pub trusted_build_pub_keys: Vec<NixPubKeyLine>,
}
