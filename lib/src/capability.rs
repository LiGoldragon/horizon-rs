//! Node capabilities — what a node can provide.
//!
//! Distinct from `NodeSpecies` (what the node is for) and
//! `NodePlacement` (how/where the node exists). A node can carry
//! several capabilities at once: a single dedicated host can be a
//! build host, binary cache, container host, and public endpoint
//! simultaneously.
//!
//! Spec: `reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
//! §P1.1 (capability records, no `InfrastructureHost` marker, typed
//! endpoints). The "is this an infrastructure host" question is
//! derived (`binary_cache.is_some() || container_host.is_some() ||
//! public_endpoint.is_some()`); not a separate type.

//! First-slice note: `NotaRecord` / `NotaEnum` derives are intentionally
//! omitted here too. Sum-with-data enums (`PublicDomain`) need
//! hand-written `NotaEncode/NotaDecode`. Unit enums get `NotaEnum` once
//! the surrounding records become NotaRecords.

use nota_codec::NotaTransparent;
use serde::{Deserialize, Serialize};

use crate::magnitude::AtLeast;
use crate::name::{CriomeDomainName, NodeName};
use crate::placement::ContainmentSubstrate;
use crate::pub_key::NixPubKey;
use crate::secret::SecretReference;

/// All capabilities a node can advertise. Each field is `Option`:
/// `None` means "this node does not provide this capability".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCapabilities {
    pub build_host: Option<BuildHost>,
    pub binary_cache: Option<BinaryCache>,
    pub container_host: Option<ContainerHost>,
    pub public_endpoint: Option<PublicEndpoint>,
}

impl NodeCapabilities {
    pub fn empty() -> Self {
        Self {
            build_host: None,
            binary_cache: None,
            container_host: None,
            public_endpoint: None,
        }
    }

    /// True if any infrastructure-shaped capability is present.
    /// Replaces the deleted `InfrastructureHost` marker.
    pub fn is_infrastructure_host(&self) -> bool {
        self.binary_cache.is_some()
            || self.container_host.is_some()
            || self.public_endpoint.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildHost {
    pub max_jobs: u32,
    pub cores_per_job: u32,
    pub trust: AtLeast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryCache {
    pub endpoint: BinaryCacheEndpoint,
    pub signing_key: SecretReference,
    pub retention_policy: CacheRetentionPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinaryCacheEndpoint {
    pub scheme: CacheScheme,
    pub host: PublicDomain,
    pub port: u16,
    #[serde(default)]
    pub path_prefix: Option<String>,
    pub public_key: NixPubKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheScheme {
    Http,
    Https,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerHost {
    pub substrates: Vec<ContainmentSubstrate>,
    pub bridge_policy: BridgePolicy,
    pub public_endpoint_policy: PublicEndpointPolicy,
    /// Children whose `placement.host == this_node`. Derived during
    /// projection from `horizon.exNodes`; never authored directly in
    /// proposals. Carried as data so consumers don't re-walk
    /// `exNodes`.
    pub children: Vec<NodeName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgePolicy {
    /// The named bridge interface containers attach to. Comes from
    /// data, not a literal — host modules read this rather than
    /// hardcoding `"br0"`.
    pub bridge_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PublicEndpointPolicy {
    /// The host terminates TLS and reverse-proxies to children.
    HostTerminates,
    /// Children expose their own ports directly through the bridge.
    DirectPassthrough,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicEndpoint {
    pub domains: Vec<PublicDomain>,
    pub tls_policy: TlsPolicy,
    pub reverse_proxy_policy: ReverseProxyPolicy,
}

/// A public-facing domain. Either an internal `*.criome` name or an
/// external FQDN — Ghost-shaped services live at real domains, not
/// only inside the cluster's name space.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicDomain {
    Criome(CriomeDomainName),
    External(ExternalDomainName),
}

/// External fully-qualified domain name, e.g. `blog.example.com`.
/// Validation is permissive at the newtype boundary; DNS-side
/// presence is the authoritative check.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct ExternalDomainName(pub(crate) String);

impl ExternalDomainName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ExternalDomainName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TlsPolicy {
    /// ACME / Let's Encrypt provisioned via `security.acme`.
    AcmeLetsEncrypt,
    /// Self-signed certificate (Phase-1 / development).
    SelfSigned,
    /// Externally-provisioned certificate referenced by `SecretReference`.
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReverseProxyPolicy {
    /// Use the host's nginx/caddy as a TLS-terminating reverse proxy.
    HostTerminates,
    /// Pass through TLS to the contained node (SNI routing).
    PassThrough,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheRetentionPolicy {
    /// How many rolled-back system generations to keep rooted.
    pub rollback_window: u32,
    /// Grace TTL in seconds before recently-built closures are
    /// considered for collection — pairs with `nix.conf`'s
    /// `narinfo-cache-positive-ttl`.
    pub recent_grace_seconds: u32,
}
