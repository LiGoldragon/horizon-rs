//! horizon-rs — typed schema + projection for criome cluster horizons.
//!
//! Reads a `proposal::ClusterProposal` (from goldragon nota), produces
//! a viewpoint-scoped enriched `view::Horizon` with every method-derived
//! field already filled.
//!
//! Two namespaces, two beauty criteria:
//! - `proposal::*` — input boundary; types authored in `datom.nota`.
//!   Beauty here is typed-correctness: data-bearing variants, no
//!   stringly-typed dispatch, perfect specificity.
//! - `view::*` — output boundary; types serialised as JSON and read by
//!   Nix modules. Beauty here is consumer ergonomics: predicate-named
//!   flags read as English at gate sites; derivation lives once in
//!   projection.
//!
//! Pure value modules (`species`, `name`, `magnitude`, `address`,
//! `pub_key`, `disk`, `error`) are shared between proposal and view —
//! they have no per-side derivation pressure.
//!
//! Spec: `docs/DESIGN.md`. Style: `~/git/lore/rust/style.md`.

pub mod address;
pub mod disk;
pub mod error;
pub mod magnitude;
pub mod name;
pub mod proposal;
pub mod pub_key;
pub mod species;
pub mod view;

pub use error::{Error, Result};
pub use proposal::cluster::ClusterProposal;
pub use view::horizon::{Horizon, Viewpoint};
