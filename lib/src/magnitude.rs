//! `Magnitude` — the size and trust ladder.
//!
//! Four points on a single ordinal scale: `None` (0, absent), `Min` (1),
//! `Med` (2), `Max` (3). Used internally for both `size` (capacity) and
//! `trust`.
//!
//! Consumers don't see `Magnitude` directly — they see `AtLeast`, the
//! monotonic boolean ladder (`atLeastMin` / `atLeastMed` / `atLeastMax`)
//! that tells them whether a magnitude meets each threshold. This is
//! the only public shape of magnitude-valued fields on `Node` / `User`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Magnitude {
    None,
    Min,
    Med,
    Max,
}

impl Magnitude {
    pub fn at_least(self, other: Magnitude) -> bool {
        self >= other
    }

    pub fn ladder(self) -> AtLeast {
        AtLeast {
            at_least_min: self >= Magnitude::Min,
            at_least_med: self >= Magnitude::Med,
            at_least_max: self >= Magnitude::Max,
        }
    }

    pub fn floor(self, other: Magnitude) -> Magnitude {
        std::cmp::min(self, other)
    }
}

/// Monotonic ladder of at-least predicates for a `Magnitude`.
///
/// The three booleans are the public shape of `Node.size`, `Node.trust`,
/// `User.size`, `User.trust`. They are monotonic — if `at_least_med` is
/// true then `at_least_min` is also true — so consumers can branch on
/// the threshold they actually care about without knowing the raw
/// `Magnitude` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtLeast {
    pub at_least_min: bool,
    pub at_least_med: bool,
    pub at_least_max: bool,
}
