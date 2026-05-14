//! Input schema: `proposal::*` types form the authored shape goldragon
//! emits as `datom.nota`. Beauty here is typed-correctness: data-bearing
//! variants, no stringly-typed dispatch, perfect specificity.
//!
//! `ClusterProposal::project` is the single entry-point; it produces a
//! typed `view::Horizon` from a viewpoint `(cluster, node)`.

pub mod cluster;
pub mod domain;
pub mod io;
pub mod machine;
pub mod network;
pub mod node;
pub mod pub_keys;
pub mod router;
pub mod secret;
pub mod services;
pub mod user;
pub mod wireguard;

pub use cluster::{ClusterProposal, ClusterTrust};
pub use domain::DomainProposal;
pub use io::Io;
pub use machine::Machine;
pub use network::{DhcpPool, LanCidr, LanNetwork, LeasePolicy, ResolverPolicy};
pub use node::{NodeProjection, NodeProposal};
pub use pub_keys::{NodePubKeys, YggPubKeyEntry};
pub use router::{RouterInterfaces, WlanBand, WlanStandard};
pub use secret::{
    ClusterSecretBinding, SecretBackend, SecretName, SecretPurpose, SecretReference, SopsFilePath,
    SopsKeyPath,
};
pub use services::{NodeServices, TailnetControllerRole, TailnetMembership};
pub use user::{UserProjection, UserProposal, UserPubKeyEntry};
pub use wireguard::WireguardProxy;
