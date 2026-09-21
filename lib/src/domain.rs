//! Cluster domain configuration and derived domain names.

use datomic::{Datomic, DatomicString, Fault, FaultProblem, PortionBuilding, PortionViewing};
use protos::{Portion, StructuralEnclosure};
use serde::{Deserialize, Serialize};

use crate::name::{ClusterName, CriomeDomainName, DomainName, NodeName, UserName};

/// Cluster-authored domain settings used by projection to derive local
/// and public identities. The input proposal may leave the public list
/// empty; projection resolves that to `<cluster>.criome.net` so the
/// output always carries a concrete public cluster domain.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainConfiguration {
    #[serde(default = "InternalDomainSuffix::default_criome")]
    pub internal_suffix: InternalDomainSuffix,
    #[serde(default)]
    pub public_cluster_domains: Vec<PublicClusterDomain>,
}

/// Internal DNS suffix for cluster-local names. The default `criome`
/// preserves the existing `<node>.<cluster>.criome` names while making
/// the suffix data instead of a hardcoded projection literal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InternalDomainSuffix(String);

/// Public DNS domain assigned to a cluster, such as
/// `goldragon.criome.net`. User email/Matrix identities and
/// phone-friendly public aliases derive from this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PublicClusterDomain(String);

#[derive(Clone, Copy)]
pub struct NodeDomainContext<'a> {
    pub node: &'a NodeName,
    pub cluster: &'a ClusterName,
}

#[derive(Clone, Copy)]
pub struct UserDomainContext<'a> {
    pub user: &'a UserName,
    pub cluster: &'a ClusterName,
}

impl DomainConfiguration {
    pub fn with_cluster_defaults(&self, cluster: &ClusterName) -> Self {
        let public_cluster_domains = if self.public_cluster_domains.is_empty() {
            vec![PublicClusterDomain::for_cluster(cluster)]
        } else {
            self.public_cluster_domains.clone()
        };
        Self {
            internal_suffix: self.internal_suffix.clone(),
            public_cluster_domains,
        }
    }

    pub fn tailnet_base_domain(&self, cluster: &ClusterName) -> DomainName {
        DomainName::try_new(format!(
            "tailnet.{cluster}.{}",
            self.internal_suffix.as_str()
        ))
        .expect("derived tailnet domain is non-empty")
    }

    pub fn criome_domain_name(&self, context: NodeDomainContext<'_>) -> CriomeDomainName {
        CriomeDomainName::try_new(format!(
            "{}.{}.{}",
            context.node,
            context.cluster,
            self.internal_suffix.as_str()
        ))
        .expect("derived node domain is non-empty")
    }

    pub fn public_node_domain_name(&self, context: NodeDomainContext<'_>) -> Option<DomainName> {
        self.public_cluster_domains.first().map(|domain| {
            DomainName::try_new(format!("{}.{}", context.node, domain.as_str()))
                .expect("derived public node domain is non-empty")
        })
    }

    pub fn email_address(&self, context: UserDomainContext<'_>) -> String {
        format!(
            "{}@{}",
            context.user,
            self.primary_public_cluster_domain(context.cluster).as_str()
        )
    }

    pub fn matrix_id(&self, context: UserDomainContext<'_>) -> String {
        format!(
            "@{}:{}",
            context.user,
            self.primary_public_cluster_domain(context.cluster).as_str()
        )
    }

    fn primary_public_cluster_domain(&self, cluster: &ClusterName) -> PublicClusterDomain {
        self.public_cluster_domains
            .first()
            .cloned()
            .unwrap_or_else(|| PublicClusterDomain::for_cluster(cluster))
    }
}

impl Default for InternalDomainSuffix {
    fn default() -> Self {
        Self::default_criome()
    }
}

impl InternalDomainSuffix {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn default_criome() -> Self {
        Self::new("criome")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PublicClusterDomain {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn for_cluster(cluster: &ClusterName) -> Self {
        Self::new(format!("{cluster}.criome.net"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for InternalDomainSuffix {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::fmt::Display for PublicClusterDomain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Datomic for InternalDomainSuffix {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.clone())
            .expect("domain suffix must be Datomic-representable");
        Datomic::portion(&value)
    }
}

impl Datomic for PublicClusterDomain {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.clone())
            .expect("public domain must be Datomic-representable");
        Datomic::portion(&value)
    }
}

impl Datomic for DomainConfiguration {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [internal_suffix, public_cluster_domains] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            internal_suffix: InternalDomainSuffix::embody(internal_suffix)?,
            public_cluster_domains: Vec::<PublicClusterDomain>::embody(public_cluster_domains)?,
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.internal_suffix.portion(),
                self.public_cluster_domains.portion(),
            ],
        )
    }
}
