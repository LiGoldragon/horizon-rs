//! `Magnitude` — the size and trust ladder.
//!
//! Four points on a single ordinal scale: `None` (0, absent), `Min` (1),
//! `Med` (2), `Max` (3). Used for both `size` (capacity) and `trust`.
//!
//! `AtLeast` is the typed form of asking "is this magnitude at least
//! min/med/max?" — three booleans, true monotonically.

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
            min: self >= Magnitude::Min,
            med: self >= Magnitude::Med,
            max: self >= Magnitude::Max,
        }
    }

    pub fn floor(self, other: Magnitude) -> Magnitude {
        std::cmp::min(self, other)
    }
}

/// Three-bit decomposition of a `Magnitude` for downstream consumers
/// that want named flags rather than ordinal comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtLeast {
    pub min: bool,
    pub med: bool,
    pub max: bool,
}

/// `Mg` — a `Magnitude` bundled with its predicate ladder, so
/// downstream consumers can do `node.size.is.med` rather than
/// implementing the magnitude → ordinal conversion themselves.
/// `value` carries the raw enum for callers that need the full
/// magnitude; `is` is the precomputed `AtLeast` ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mg {
    pub value: Magnitude,
    pub is: AtLeast,
}

impl Mg {
    pub fn from(value: Magnitude) -> Self {
        Self { value, is: value.ladder() }
    }
}

impl From<Magnitude> for Mg {
    fn from(value: Magnitude) -> Self {
        Self::from(value)
    }
}

