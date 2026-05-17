//! Pan-horizon authored configuration.
//!
//! This is the second input to projection, alongside a per-cluster
//! `ClusterProposal`. It carries facts owned by the horizon operator:
//! domain suffixes and temporary pan-horizon network facts.

use nota_codec::{NotaRecord, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::address::IpAddress;
use crate::error::{Error, Result};
use crate::name::{ClusterDomain, ClusterName, DomainName, PublicDomain};
use crate::view::network::{DhcpPool, LanCidr, LanNetwork, ResolverPolicy};
use crate::view::router::Ssid;

const TAILNET_SERVICE_LABEL: &str = "tailnet";

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct HorizonProposal {
    pub operator: OperatorName,
    pub domain_suffixes: DomainSuffixes,
    pub transitional_ipv4_lan: TransitionalIpv4Lan,
    #[serde(default)]
    pub trusted_keys: Vec<HorizonTrustedKey>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct OperatorName(String);

impl OperatorName {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            Err(Error::EmptyName {
                kind: "operator name",
            })
        } else {
            Ok(Self(s))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct DomainSuffixes {
    pub internal: ClusterDomain,
    pub public: PublicDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct TransitionalIpv4Lan {
    pub cidr: LanCidr,
    pub gateway: IpAddress,
    pub dhcp_pool: DhcpPool,
    pub warning: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct HorizonTrustedKey(String);

impl HorizonTrustedKey {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            Err(Error::EmptyName {
                kind: "horizon trusted key",
            })
        } else {
            Ok(Self(s))
        }
    }
}

impl HorizonProposal {
    pub fn from_parts(
        operator: impl Into<String>,
        internal_domain: impl Into<String>,
        public_domain: impl Into<String>,
        lan_cidr: impl Into<String>,
        lan_gateway: impl Into<String>,
        dhcp_start: impl Into<String>,
        dhcp_end: impl Into<String>,
        warning: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            operator: OperatorName::try_new(operator)?,
            domain_suffixes: DomainSuffixes {
                internal: ClusterDomain::try_new(internal_domain)?,
                public: PublicDomain::try_new(public_domain)?,
            },
            transitional_ipv4_lan: TransitionalIpv4Lan {
                cidr: LanCidr::try_new(lan_cidr)?,
                gateway: IpAddress::try_new(lan_gateway)?,
                dhcp_pool: DhcpPool {
                    start: IpAddress::try_new(dhcp_start)?,
                    end: IpAddress::try_new(dhcp_end)?,
                },
                warning: warning.into(),
            },
            trusted_keys: Vec::new(),
        })
    }

    pub fn internal_domain(&self) -> &ClusterDomain {
        &self.domain_suffixes.internal
    }

    pub fn public_domain(&self) -> &PublicDomain {
        &self.domain_suffixes.public
    }

    pub fn router_ssid(&self, cluster: &ClusterName) -> Result<Ssid> {
        Ssid::try_new(format!("{cluster}.{}", self.internal_domain()))
    }

    pub fn tailnet_base_domain(&self, cluster: &ClusterName) -> Result<DomainName> {
        self.service_domain(cluster, TAILNET_SERVICE_LABEL)
    }

    pub fn service_domain(&self, cluster: &ClusterName, service: &str) -> Result<DomainName> {
        DomainName::try_new(format!("{service}.{cluster}.{}", self.internal_domain()))
    }

    pub fn lan_network(&self, _cluster: &ClusterName) -> Result<LanNetwork> {
        // Temporary single-router IPv4 LAN. Do not generalize this into
        // a hash allocator; replace it with the IPv6-first network design.
        Ok(LanNetwork {
            cidr: self.transitional_ipv4_lan.cidr.clone(),
            gateway: self.transitional_ipv4_lan.gateway,
            dhcp_pool: self.transitional_ipv4_lan.dhcp_pool.clone(),
        })
    }

    pub fn resolver_policy(&self, lan: Option<&LanNetwork>) -> Result<ResolverPolicy> {
        let mut listens = vec![IpAddress::try_new("::1")?, IpAddress::try_new("127.0.0.1")?];
        if let Some(lan) = lan {
            listens.push(lan.gateway);
        }
        Ok(ResolverPolicy { listens })
    }
}
