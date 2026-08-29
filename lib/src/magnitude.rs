//! `Magnitude` — the size and trust ladder.

use serde::{Deserialize, Serialize};

/// The public value declaration is hand-owned for serde/projection semantics;
/// its Datomic anatomy is emitted from `lib/ethos/horizon.ethos` into `generated`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Magnitude {
    Zero,
    Min,
    Medium,
    Large,
    Max,
}

impl Magnitude {
    pub fn at_least(self, other: Magnitude) -> bool {
        self >= other
    }

    pub fn ladder(self) -> AtLeast {
        AtLeast {
            min: self >= Magnitude::Min,
            medium: self >= Magnitude::Medium,
            large: self >= Magnitude::Large,
            max: self >= Magnitude::Max,
        }
    }

    pub(crate) fn default_zero() -> Self {
        Self::Zero
    }

    pub(crate) fn default_min() -> Self {
        Self::Min
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtLeast {
    pub min: bool,
    pub medium: bool,
    pub large: bool,
    pub max: bool,
}
