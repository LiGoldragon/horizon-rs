//! The trust a cluster grants a node or a user, after every ceiling applies.

use super::names::MagnitudeRanking;
use crate::*;

/// A cluster's trust declaration read as the ceiling it is: a node or user
/// never holds more trust than the cluster, nor than its own entry allows.
pub(crate) trait TrustResolving {
    /// The trust a node holds, given the trust its own definition declares.
    fn node_trust(&self, name: &str, declared: &Magnitude) -> Magnitude;

    /// The trust a user holds; a user without an entry starts at `Min`.
    fn user_trust(&self, name: &str) -> Magnitude;
}

impl TrustResolving for ClusterTrust {
    fn node_trust(&self, name: &str, declared: &Magnitude) -> Magnitude {
        let ceiling = self
            .node_trust_entry_vector
            .iter()
            .find(|entry| entry.node_name == name)
            .map(|entry| &entry.magnitude)
            .unwrap_or(&Magnitude::Max);
        declared.minimum(&ceiling.minimum(&self.magnitude))
    }

    fn user_trust(&self, name: &str) -> Magnitude {
        let declared = self
            .user_trust_entry_vector
            .iter()
            .find(|entry| entry.user_name == name)
            .map(|entry| &entry.magnitude)
            .unwrap_or(&Magnitude::Min);
        declared.minimum(&self.magnitude)
    }
}
