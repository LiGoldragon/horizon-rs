//! Output schema: `view::*` types form the wire shape consumed by Nix
//! modules in CriomOS / CriomOS-home through `inputs.horizon`. Beauty
//! here is consumer ergonomics: predicate-named flags read as English
//! at gate sites; derivation lives once in projection.

pub mod cluster;
pub mod horizon;
pub mod io;
pub mod machine;
pub mod node;
pub mod projected_node;
pub mod user;

pub use cluster::Cluster;
pub use horizon::{Horizon, Viewpoint};
pub use io::Io;
pub use machine::Machine;
pub use node::{BehavesAs, BuilderConfig, NixCache, Node, ViewpointFill};
pub use projected_node::ProjectedNodeView;
pub use user::User;
