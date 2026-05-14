//! Per-node public-key bundles authored in the proposal.
//!
//! `NodePubKeys` carries the per-node identity bundle (SSH always,
//! Nix and Yggdrasil optional). `YggPubKeyEntry` carries the
//! yggdrasil-mesh tuple (public key + address + subnet).

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::address::{YggAddress, YggSubnet};
use crate::pub_key::{NixPubKey, SshPubKey, YggPubKey};

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodePubKeys {
    pub ssh: SshPubKey,
    #[serde(default)]
    pub nix: Option<NixPubKey>,
    #[serde(default)]
    pub yggdrasil: Option<YggPubKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct YggPubKeyEntry {
    pub pub_key: YggPubKey,
    pub address: YggAddress,
    pub subnet: YggSubnet,
}
