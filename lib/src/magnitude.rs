//! `Magnitude` — the size and trust ladder.
//!
//! Five points on a single ordinal scale: `Zero` (0, absent / actively
//! distrusted / no capacity), `Min` (1), `Medium` (2), `Large` (3),
//! `Max` (4). Used internally for both `size` (capacity) and `trust`.
//!
//! Consumers don't see `Magnitude` directly — they see `AtLeast`, the
//! monotonic boolean ladder (`min` / `medium` / `large` / `max`) that
//! tells them whether a magnitude meets each threshold. This is the
//! only public shape of magnitude-valued fields on `Node` / `User`.

use nota_next::{NotaDecode, NotaEncode};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    NotaDecode,
    NotaEncode,
)]
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
}

/// Monotonic ladder of at-least predicates for a `Magnitude`.
///
/// The four booleans are the public shape of `Node.size`, `Node.trust`,
/// `User.size`, `User.trust`. They are monotonic — if `medium` is true
/// then `min` is also true — so consumers can branch on the threshold
/// they actually care about without knowing the raw `Magnitude`
/// variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct AtLeast {
    pub min: bool,
    pub medium: bool,
    pub large: bool,
    pub max: bool,
}
