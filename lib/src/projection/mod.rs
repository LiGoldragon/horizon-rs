//! Resolving an authored Horizon definition into one node's viewpoint.

mod composition;
#[cfg(feature = "datom")]
mod decoding;
mod error;
mod names;
mod node;
mod trust;
mod user;
mod viewpoint;
mod views;

pub use composition::{Composing, Projecting, ResolvedCluster};
#[cfg(feature = "datom")]
pub use decoding::DatomDecoding;
pub use error::Error;
pub use node::NodeDefinitions;
