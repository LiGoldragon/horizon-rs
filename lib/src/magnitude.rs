//! `Magnitude` — the size and trust ladder.
//!
//! Four points on a single ordinal scale: `None` (0, absent), `Min` (1),
//! `Med` (2), `Max` (3). Used for both `size` (capacity) and `trust`.
//!
//! `AtLeast` is the typed form of asking "is this magnitude at least
//! min/med/max?" — three booleans, true monotonically.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_monotonic() {
        assert_eq!(Magnitude::None.ladder(), AtLeast { min: false, med: false, max: false });
        assert_eq!(Magnitude::Min.ladder(), AtLeast { min: true, med: false, max: false });
        assert_eq!(Magnitude::Med.ladder(), AtLeast { min: true, med: true, max: false });
        assert_eq!(Magnitude::Max.ladder(), AtLeast { min: true, med: true, max: true });
    }

    #[test]
    fn floor_picks_lower() {
        assert_eq!(Magnitude::Max.floor(Magnitude::Med), Magnitude::Med);
        assert_eq!(Magnitude::Min.floor(Magnitude::Max), Magnitude::Min);
    }

    #[test]
    fn at_least_compares() {
        assert!(Magnitude::Med.at_least(Magnitude::Min));
        assert!(Magnitude::Med.at_least(Magnitude::Med));
        assert!(!Magnitude::Med.at_least(Magnitude::Max));
    }
}
