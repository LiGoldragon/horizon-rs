//! Decoding one authored Datom document into the typed value it names.

use datom_codec::{Actualizing, Budget, Compositional, Potential};
use protos::ReaderBudget;

use super::error::Error;
use crate::*;

/// A document kind Horizon accepts in authored Datom text, together with the
/// decoding budget that kind is allowed to spend.
pub trait DatomDecoding: Compositional + Sized {
    /// The largest decode this document kind may cost.
    const BUDGET: i64;

    /// Decodes one authored document of this kind.
    fn decode(text: &str) -> Result<Self, Error> {
        let reader_budget =
            usize::try_from(Self::BUDGET).expect("positive decode budget fits usize");
        Potential::<Self>::from(text)
            .actualize(&mut Budget {
                remaining: Self::BUDGET,
                reader: ReaderBudget {
                    remaining: reader_budget,
                },
                depth: 0,
                maximum_depth: 256,
            })
            .map_err(Error::Datom)
    }
}

/// The materialized, single-file Horizon proposal.
impl DatomDecoding for HorizonDefinition {
    const BUDGET: i64 = 16_384;
}

/// The externally authored generic-node catalogue.
impl DatomDecoding for HorizonConfiguration {
    const BUDGET: i64 = 16_384;
}

/// The cluster-owned membership and cluster facts.
impl DatomDecoding for ClusterDefinition {
    const BUDGET: i64 = 16_384;
}

/// The sole typed command accepted by `horizon-compose`.
impl DatomDecoding for CompositionCommand {
    const BUDGET: i64 = 1_024;
}
