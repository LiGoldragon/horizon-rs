use thiserror::Error;

use crate::name::NodeName;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid name: {kind} cannot be empty")]
    EmptyName { kind: &'static str },

    #[error("invalid keygrip: expected 40 hex chars, got {got:?}")]
    InvalidKeygrip { got: String },

    #[error("invalid hex public key (expected {expected_len} hex chars): {got:?}")]
    InvalidHexKey { expected_len: usize, got: String },

    #[error("invalid base64 public key (expected {expected_len} chars): {got:?}")]
    InvalidBase64Key { expected_len: usize, got: String },

    #[error("invalid yggdrasil address {got:?}: {source}")]
    InvalidYggAddress { got: String, source: std::net::AddrParseError },

    #[error("invalid node ip {got:?}: {source}")]
    InvalidNodeIp { got: String, source: ipnet::AddrParseError },

    #[error("unknown {kind}: {got:?}")]
    UnknownVariant { kind: &'static str, got: String },

    #[error("cluster has no node {0:?}")]
    NodeNotInCluster(NodeName),

    #[error("pod node {0:?} references missing super-node {1:?}")]
    MissingSuperNode(NodeName, NodeName),

    #[error("pod node {0:?} has no super-node and no arch of its own")]
    UnresolvableArch(NodeName),

    #[error("nota: {0}")]
    Nota(#[from] nota_serde::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
