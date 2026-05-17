//! Pan-horizon authored configuration.
//!
//! This is the second input to projection, alongside a per-cluster
//! `ClusterProposal`. It carries facts owned by the horizon operator:
//! domain suffixes, the LAN allocation pool, and reserved service labels.

use std::net::Ipv4Addr;

use ipnet::{IpNet, Ipv4Net};
use nota_codec::{NotaRecord, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::address::IpAddress;
use crate::error::{Error, Result};
use crate::name::{ClusterDomain, ClusterName, DomainName, NodeName, PublicDomain};
use crate::proposal::network::{DhcpPool, LanCidr, LanNetwork, ResolverPolicy};
use crate::proposal::router::Ssid;

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct HorizonProposal {
    pub operator: OperatorName,
    pub domain_suffixes: DomainSuffixes,
    pub lan_pool: LanPool,
    pub reserved_subdomains: Vec<ReservedSubdomainLabel>,
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
pub struct LanPool {
    pub supernet: LanCidr,
    pub per_cluster_prefix_length: u8,
    pub hash_namespace: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct ReservedSubdomainLabel(String);

impl ReservedSubdomainLabel {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::EmptyName {
                kind: "reserved subdomain label",
            });
        }
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidReservedSubdomainLabel { got: s });
        }
        Ok(Self(s))
    }
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
        lan_supernet: impl Into<String>,
        per_cluster_prefix_length: u8,
        hash_namespace: impl Into<String>,
        reserved_subdomains: Vec<String>,
    ) -> Result<Self> {
        Ok(Self {
            operator: OperatorName::try_new(operator)?,
            domain_suffixes: DomainSuffixes {
                internal: ClusterDomain::try_new(internal_domain)?,
                public: PublicDomain::try_new(public_domain)?,
            },
            lan_pool: LanPool {
                supernet: LanCidr::try_new(lan_supernet)?,
                per_cluster_prefix_length,
                hash_namespace: hash_namespace.into(),
            },
            reserved_subdomains: reserved_subdomains
                .into_iter()
                .map(ReservedSubdomainLabel::try_new)
                .collect::<Result<Vec<_>>>()?,
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
        self.service_domain(cluster, "tailnet")
    }

    pub fn service_domain(&self, cluster: &ClusterName, service: &str) -> Result<DomainName> {
        DomainName::try_new(format!("{service}.{cluster}.{}", self.internal_domain()))
    }

    pub fn lan_network(
        &self,
        cluster: &ClusterName,
        router: &NodeName,
    ) -> Result<LanNetwork> {
        let supernet = match self.lan_pool.supernet.as_ipnet() {
            IpNet::V4(net) => net,
            IpNet::V6(_) => return Err(Error::LanPoolMustBeIpv4),
        };
        let target_prefix = self.lan_pool.per_cluster_prefix_length;
        let source_prefix = supernet.prefix_len();
        if target_prefix < source_prefix || target_prefix > 24 {
            return Err(Error::InvalidLanPoolPrefixLength {
                supernet_prefix: source_prefix,
                target_prefix,
            });
        }

        let subnet_count = 1u64 << u32::from(target_prefix - source_prefix);
        let hash = stable_hash_v1(&[
            self.lan_pool.hash_namespace.as_str(),
            cluster.as_str(),
            router.as_str(),
        ]);
        let subnet_index = hash % subnet_count;
        let host_bits = u32::from(32 - target_prefix);
        let base = u32::from(supernet.network());
        let network_address = Ipv4Addr::from(base + ((subnet_index as u32) << host_bits));
        let cidr = Ipv4Net::new(network_address, target_prefix)
            .expect("target_prefix was validated as an IPv4 prefix");
        let gateway = increment(network_address, 1);
        let dhcp_start = increment(network_address, 100);
        let dhcp_end = increment(network_address, 240);

        Ok(LanNetwork {
            cidr: LanCidr::try_new(cidr.to_string())?,
            gateway: IpAddress::try_new(gateway.to_string())?,
            dhcp_pool: DhcpPool {
                start: IpAddress::try_new(dhcp_start.to_string())?,
                end: IpAddress::try_new(dhcp_end.to_string())?,
            },
        })
    }

    pub fn resolver_policy(&self, lan: Option<&LanNetwork>) -> Result<ResolverPolicy> {
        let mut listens = vec![
            IpAddress::try_new("::1")?,
            IpAddress::try_new("127.0.0.1")?,
        ];
        if let Some(lan) = lan {
            listens.push(lan.gateway);
        }
        Ok(ResolverPolicy { listens })
    }
}

fn increment(address: Ipv4Addr, offset: u32) -> Ipv4Addr {
    Ipv4Addr::from(u32::from(address) + offset)
}

fn stable_hash_v1(parts: &[&str]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for part in parts {
        for byte in part.as_bytes().iter().copied().chain(std::iter::once(0)) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    hash
}
