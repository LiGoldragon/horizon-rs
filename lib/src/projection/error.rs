//! Every way resolution or projection of a Horizon definition can refuse.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("user {user:?} synchronization has identical master and replica node {node:?}")]
    SynchronizationSameNode { user: String, node: String },
    #[error("user {user:?} synchronization names unknown node {node:?}")]
    SynchronizationUnknownNode { user: String, node: String },
    #[error(
        "user {user:?} synchronization has no declared account or required master identity on node {node:?}"
    )]
    SynchronizationUserAbsent { user: String, node: String },
    #[error(
        "user {user:?} synchronization master key from {master:?} is not authorized by the account on replica {replica:?}"
    )]
    SynchronizationMasterKeyUnauthorized {
        user: String,
        master: String,
        replica: String,
    },
    #[error("user {user:?} synchronization has invalid home-relative tree {tree:?}")]
    SynchronizationInvalidTree { user: String, tree: String },
    #[error(
        "user {user:?} synchronizations on node {node:?} overlap trees {first:?} and {second:?}"
    )]
    SynchronizationOverlappingTrees {
        user: String,
        node: String,
        first: String,
        second: String,
    },
    #[error("node {node:?} requires one battery policy with 0 <= action < critical < low <= 100")]
    InvalidBatteryPowerPolicy { node: String },
    #[error("node {node:?} has an invalid RouterUsbParentPowerOn capability")]
    InvalidRouterUsbParentPowerOn { node: String },
    #[error("node {node:?} has an invalid PreferredWiredInternet capability")]
    InvalidPreferredWiredInternet { node: String },
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
