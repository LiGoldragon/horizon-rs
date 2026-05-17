//! Output schema: `view::*` types form the wire shape consumed by Nix
//! modules in CriomOS / CriomOS-home through `inputs.horizon`. Beauty
//! here is consumer ergonomics: predicate-named flags read as English
//! at gate sites; derivation lives once in projection.
//!
//! Shape-equivalent input types stay in `proposal::*` and are re-used
//! directly by the view side (`Machine`, `Io`). Only records that
//! genuinely diverge from the proposal — derived booleans, resolved
//! lookups, viewpoint-only fields — earn a `view::` type.

pub mod cluster;
pub mod horizon;
pub mod network;
pub mod node;
pub mod projected_node;
pub mod router;
pub mod user;

pub use cluster::Cluster;
pub use horizon::{Horizon, Viewpoint};
pub use network::{DhcpPool, LanCidr, LanNetwork, ResolverPolicy};
pub use node::{BehavesAs, BuilderConfig, NixCache, Node, ViewpointFill};
pub use projected_node::ProjectedNodeView;
pub use router::{RouterInterfaces, Ssid};
pub use user::User;
