//! Decoding one authored Datom document into the typed value it names.

use datom_codec::{Actualizing, Budget, Composing as DatomComposing, Potential};
use protos::ReaderBudget;

use super::error::Error;
use super::names::Named;
use crate::*;

fn validate_ordering(scope: &str, ordering: &[LinkKind]) -> Result<(), Error> {
    for expected in [LinkKind::Wired, LinkKind::Wireless, LinkKind::Tether] {
        let count = ordering.iter().filter(|kind| **kind == expected).count();
        if count == 0 {
            return Err(Error::MissingLinkKind {
                scope: scope.to_owned(),
                kind: expected.name().to_owned(),
            });
        }
        if count > 1 {
            return Err(Error::DoubledLinkKind {
                scope: scope.to_owned(),
                kind: expected.name().to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_node_ordering(node: &NodeDefinition) -> Result<(), Error> {
    if let Some(ordering) = &node.node_network.link_kind_ordering_option {
        validate_ordering(&format!("node {:?}", node.node_name), ordering)?;
    }
    Ok(())
}

/// A document kind Horizon accepts in authored Datom text, together with the
/// decoding budget that kind is allowed to spend.
pub trait DatomDecoding: DatomComposing + Sized {
    /// The largest decode this document kind may cost.
    const BUDGET: i64;

    fn validate_document(&self) -> Result<(), Error>;

    /// Decodes one authored document of this kind.
    fn decode(text: &str) -> Result<Self, Error> {
        let reader_budget =
            usize::try_from(Self::BUDGET).expect("positive decode budget fits usize");
        let value = Potential::<Self>::from(text)
            .actualize(&mut Budget {
                remaining: Self::BUDGET,
                reader: ReaderBudget {
                    remaining: reader_budget,
                },
                depth: 0,
                maximum_depth: 256,
            })
            .map_err(Error::Datom)?;
        value.validate_document()?;
        Ok(value)
    }
}

/// The materialized, single-file Horizon proposal.
impl DatomDecoding for HorizonDefinition {
    const BUDGET: i64 = 16_384;
    fn validate_document(&self) -> Result<(), Error> {
        self.horizon_configuration.validate_document()?;
        self.cluster_definition.validate_document()
    }
}

/// The externally authored generic-node catalogue.
impl DatomDecoding for HorizonConfiguration {
    const BUDGET: i64 = 16_384;
    fn validate_document(&self) -> Result<(), Error> {
        self.generic_nodes
            .iter()
            .try_for_each(validate_node_ordering)
    }
}

/// The cluster-owned membership and cluster facts.
impl DatomDecoding for ClusterDefinition {
    const BUDGET: i64 = 16_384;
    fn validate_document(&self) -> Result<(), Error> {
        validate_ordering("cluster", &self.link_kind_ordering)?;
        self.cluster_nodes
            .iter()
            .try_for_each(validate_node_ordering)
    }
}

/// The sole typed command accepted by `horizon-compose`.
impl DatomDecoding for CompositionCommand {
    const BUDGET: i64 = 1_024;
    fn validate_document(&self) -> Result<(), Error> {
        Ok(())
    }
}
