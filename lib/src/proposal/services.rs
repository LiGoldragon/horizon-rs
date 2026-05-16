//! Per-node service-role records + cluster-level tailnet config.
//!
//! Per-node:
//! - `NodeServices` containers `tailnet` (membership) and
//!   `tailnet_controller` (which node hosts the controller).
//! - `TailnetMembership` and `TailnetControllerRole` name the role
//!   declaratively; CriomOS renders them with concrete services
//!   (Tailscale, Headscale) at deploy time.
//!
//! Cluster-level:
//! - `TailnetConfig` carries the cluster's base DNS domain for
//!   tailnet hosts plus optional CA-trust material so consumers
//!   stop self-signing on first boot.
//! - `TlsTrustPolicy` carries the CA certificate plus optional
//!   controller server certificate and private-key reference.
//! - `PublicCertificate` is a typed PEM newtype.

use nota_codec::{NotaEnum, NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::name::DomainName;
use crate::proposal::secret::SecretReference;

#[derive(Debug, Clone, Default, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodeServices {
    /// Whether this node should join the cluster tailnet. CriomOS
    /// currently renders this with Tailscale, but the proposal names the
    /// role rather than deriving it from node identity.
    #[serde(default)]
    pub tailnet: Option<TailnetMembership>,

    /// Whether this node hosts the cluster tailnet controller service.
    /// CriomOS currently renders this with Headscale. The controller's
    /// base DNS domain lives once on `Cluster.tailnet.base_domain`,
    /// not per-controller — collapsed in step 11 of the horizon
    /// re-engineering arc.
    #[serde(default)]
    pub tailnet_controller: Option<TailnetControllerRole>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaEnum)]
pub enum TailnetMembership {
    Client,
}

/// Per-node tailnet-controller role. The previous shape carried
/// `base_domain` per controller; that field collapsed onto
/// `Cluster.tailnet.base_domain` (one cluster, one tailnet domain).
/// `port` stays per-controller because future clusters might host
/// controllers on different ports for testing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum TailnetControllerRole {
    Server { port: u16 },
}

/// Cluster-level tailnet configuration. `base_domain` is required
/// when any node hosts a tailnet controller (validated at
/// projection). `tls` is optional during the migration period —
/// once the operator generates a CA and authors it in datom,
/// CriomOS modules read the cert from horizon instead of
/// self-signing on first boot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct TailnetConfig {
    pub base_domain: DomainName,
    #[serde(default)]
    pub tls: Option<TlsTrustPolicy>,
}

/// TLS trust material for the cluster's tailnet controller. The CA
/// certificate is public trust material; optional server material lets
/// the cluster author pin the serving certificate and bind the private
/// key through the cluster secret table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct TlsTrustPolicy {
    pub ca_certificate: PublicCertificate,
    #[serde(default)]
    pub server_certificate: Option<PublicCertificate>,
    #[serde(default)]
    pub server_private_key: Option<SecretReference>,
}

/// PEM-encoded X.509 public certificate. Validated by checking
/// that the value starts with the standard PEM begin marker —
/// deeper validation (parsing the cert) happens at consumer time
/// where errors can name the right service.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct PublicCertificate(pub(crate) String);

impl PublicCertificate {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if !s.starts_with("-----BEGIN CERTIFICATE-----") {
            return Err(Error::InvalidPublicCertificate { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PublicCertificate {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PublicCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
