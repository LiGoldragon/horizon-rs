//! Composing the two authored documents, and projecting one node's Horizon.

use std::collections::{BTreeMap, BTreeSet};

use super::error::Error;
use super::node::{NodeCatalogue, NodeDefinitions, NodeGraph, NodeProjection};
use super::trust::TrustResolving;
use super::user::{UserProjection, Viewpoint};
use super::viewpoint::NodeDeriving;
use super::views::Projection;
use crate::*;

/// The cluster membership an authored definition actually resolves to.
#[derive(Clone, Debug)]
pub struct ResolvedCluster {
    pub name: String,
    pub nodes: NodeDefinitions,
}

/// Combining the generic-node catalogue with one cluster's own definition.
pub trait Composing {
    /// Combines the two explicit source documents and validates the
    /// membership seam before a consumer can receive the resulting
    /// single-file document.
    fn compose(self, cluster: ClusterDefinition) -> Result<HorizonDefinition, Error>;
}

impl Composing for HorizonConfiguration {
    fn compose(self, cluster: ClusterDefinition) -> Result<HorizonDefinition, Error> {
        let definition = HorizonDefinition {
            horizon_configuration: self,
            cluster_definition: cluster,
        };
        definition.resolve()?;
        Ok(definition)
    }
}

/// Resolving a Horizon definition's membership, and projecting the viewpoint
/// of one of its nodes.
pub trait Projecting {
    /// Resolves only explicitly selected generic names; catalogue presence
    /// does not imply cluster membership.
    fn resolve(&self) -> Result<ResolvedCluster, Error>;

    /// The whole Horizon as the named node sees it.
    fn project(&self, node: &str) -> Result<Horizon, Error>;
}

impl Projecting for HorizonDefinition {
    fn resolve(&self) -> Result<ResolvedCluster, Error> {
        let generic = self
            .horizon_configuration
            .generic_nodes
            .catalogue(Error::DuplicateGenericNode)?;
        let mut nodes = self
            .cluster_definition
            .cluster_nodes
            .catalogue(Error::DuplicateClusterNode)?;
        let mut selected = BTreeSet::new();
        for name in &self.cluster_definition.generic_node_names {
            let name = name.clone();
            if !selected.insert(name.clone()) {
                return Err(Error::DuplicateGenericSelection(name));
            }
            let definition = generic
                .get(&name)
                .ok_or_else(|| Error::UnknownGenericNode(name.clone()))?;
            if nodes.insert(name.clone(), definition.clone()).is_some() {
                return Err(Error::NodeCollision(name));
            }
        }
        let trust = &self.cluster_definition.cluster_trust;
        nodes.retain(|name, definition| {
            !matches!(
                trust.node_trust(name, &definition.second_magnitude),
                Magnitude::Zero
            )
        });
        nodes.validate_tailnet_controller()?;
        nodes.validate_machines()?;
        Ok(ResolvedCluster {
            name: self.cluster_definition.cluster_name.clone(),
            nodes,
        })
    }

    fn project(&self, node: &str) -> Result<Horizon, Error> {
        let resolved = self.resolve()?;
        let trust = &self.cluster_definition.cluster_trust;
        let definition = resolved
            .nodes
            .get(node)
            .ok_or_else(|| Error::UnknownNode(node.to_owned()))?;
        let mut ex_nodes: BTreeMap<String, Node> = resolved
            .nodes
            .iter()
            .filter(|(name, _)| name.as_str() != node)
            .map(|(name, definition)| Ok((name.clone(), definition.project_in(&resolved.nodes)?)))
            .collect::<Result<_, Error>>()?;
        let mut projected_node = definition.project_in(&resolved.nodes)?;
        let internal_suffix = self
            .horizon_configuration
            .domain_configuration
            .string
            .as_str();
        let public_domain = self
            .horizon_configuration
            .domain_configuration
            .domain_name_vector
            .first()
            .cloned()
            .unwrap_or_else(|| format!("{}.criome.net", self.cluster_definition.cluster_name));
        for (name, projected) in &mut ex_nodes {
            let peer = resolved.nodes.get(name).expect("resolved node exists");
            projected.derive(
                &resolved.name,
                internal_suffix,
                trust.node_trust(name, &peer.second_magnitude),
            );
        }
        projected_node.derive(
            &resolved.name,
            internal_suffix,
            trust.node_trust(node, &definition.second_magnitude),
        );
        let trusted_build_public_keys = std::iter::once(&projected_node)
            .chain(ex_nodes.values())
            .filter_map(|node| node.nix_public_key_line.clone())
            .collect::<Vec<String>>();
        let viewpoint = Viewpoint {
            node,
            public_domain: &public_domain,
            center: projected_node.behaves_as.center,
            size: &projected_node.size,
        };
        let users = self
            .cluster_definition
            .users
            .iter()
            .filter_map(|user| {
                let granted = trust.user_trust(&user.user_name);
                (!matches!(granted, Magnitude::Zero))
                    .then(|| user.project_from(granted, &viewpoint))
            })
            .collect::<Vec<User>>();
        projected_node.fill_viewpoint(&ex_nodes, &users);
        Ok(Horizon {
            cluster: resolved.name,
            tailnet_base_domain: format!(
                "tailnet.{}.{}",
                self.cluster_definition.cluster_name, internal_suffix
            ),
            trusted_build_public_keys,
            node: projected_node,
            ex_nodes,
            users,
            domains: self
                .cluster_definition
                .domains
                .iter()
                .map(Projection::project)
                .collect(),
            trust: trust.project(),
            domain_configuration: DomainConfigurationView {
                internal_suffix: self
                    .horizon_configuration
                    .domain_configuration
                    .string
                    .clone(),
                public_cluster_domains: self
                    .horizon_configuration
                    .domain_configuration
                    .domain_name_vector
                    .to_vec(),
            },
        })
    }
}
