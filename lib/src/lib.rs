//! horizon-rs — typed schema + projection for criome cluster horizons.
//!
//! Reads a `ClusterProposal` (from goldragon TOML), produces a
//! viewpoint-scoped enriched `Horizon` with every method-derived
//! field already filled.
//!
//! Spec: `docs/DESIGN.md`. Style: `~/git/tools-documentation/rust/style.md`.

pub mod address;
pub mod cluster;
pub mod error;
pub mod horizon;
pub mod io;
pub mod machine;
pub mod magnitude;
pub mod name;
pub mod node;
pub mod proposal;
pub mod pub_key;
pub mod species;
pub mod user;

pub use error::{Error, Result};
pub use horizon::{Horizon, Viewpoint};
pub use proposal::ClusterProposal;
