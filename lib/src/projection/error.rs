//! Every way resolution or projection of a Horizon definition can refuse.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "datom")]
    #[error("datom: {0:?}")]
    Datom(datom_codec::Error),
    #[error("generic catalogue contains duplicate node {0:?}")]
    DuplicateGenericNode(String),
    #[error("cluster contains duplicate local node {0:?}")]
    DuplicateClusterNode(String),
    #[error("cluster selects generic node {0:?} more than once")]
    DuplicateGenericSelection(String),
    #[error("cluster selects unknown generic node {0:?}")]
    UnknownGenericNode(String),
    #[error("selected generic node {0:?} conflicts with a cluster-local node")]
    NodeCollision(String),
    #[error("resolved cluster has no node {0:?}")]
    UnknownNode(String),
    #[error("metal node {0:?} has no architecture")]
    MissingMetalArchitecture(String),
    #[error("external VM {0:?} has no architecture")]
    MissingExternalVmArchitecture(String),
    #[error("VM {node:?} names unknown cluster host {host:?}")]
    UnknownVmHost { node: String, host: String },
    #[error("VM {node:?} names hosts with different architectures")]
    VmHostArchitecture { node: String },
    #[error("VM host graph cycles at {0:?}")]
    VmHostCycle(String),
    #[error("trusted cluster has more than one TailnetController: {first:?}, {second:?}")]
    MultipleTailnetControllers { first: String, second: String },
}
