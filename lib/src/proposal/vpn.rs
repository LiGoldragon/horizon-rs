//! Cluster-level VPN profiles.
//!
//! Replaces the per-CriomOS `data/config/nordvpn/servers-lock.json`
//! file (DNS, client config, server catalog with name/hostname/
//! endpoint/publicKey/country/city). The cluster authors which VPN
//! profiles exist; node-level `nordvpn: bool` opt-in selects whether
//! a node configures the NordVPN client at all (which profile it
//! uses is the cluster's choice — currently the single profile in
//! the list).
//!
//! Source: `~/primary/reports/system-specialist/119-horizon-data-needed-to-purge-criomos-literals.md` §6.

use nota_codec::{NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::proposal::secret::SecretReference;
use crate::pub_key::WireguardPubKey;

/// Closed sum of supported VPN provider profiles. Currently only
/// NordVPN; new providers add new variants. The variant name
/// equals the payload type name (NotaSum convention).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum VpnProfile {
    NordvpnProfile(NordvpnProfile),
}

/// One NordVPN profile: cluster-level catalog of servers + DNS +
/// client-address policy + a SecretReference to the account
/// credentials. Consumed by `CriomOS modules/nixos/network/nordvpn.nix`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NordvpnProfile {
    pub dns: VpnDns,
    pub client: VpnClient,
    pub servers: Vec<NordvpnServer>,
    /// Account/token credentials. Resolved through the cluster's
    /// `secret_bindings` at projection time. Module loud-fails until
    /// the secret-backend resolver is wired (same pattern as
    /// AiProvider.api_key).
    pub credentials: SecretReference,
}

/// VPN-side DNS pair. NordVPN ships a primary + secondary that
/// resolve via the tunnel; non-tunnel resolvers stay on the node's
/// regular `cluster.resolver` policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct VpnDns {
    pub primary: VpnIpAddress,
    pub secondary: VpnIpAddress,
}

/// Static client-side WireGuard config the cluster assigns. Not a
/// secret — it's the address/port the local interface binds to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct VpnClient {
    /// CIDR string (e.g. "10.5.0.2/32"). Validated as non-empty.
    pub address: VpnClientAddress,
    pub port: u16,
}

/// One NordVPN server entry. The set is a snapshot from NordVPN's
/// recommendations API; the operator updates it by re-running their
/// regen tool and rewriting the datom (no separate lockfile).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NordvpnServer {
    pub name: NordvpnServerName,
    pub hostname: String,
    pub endpoint: String,
    pub public_key: WireguardPubKey,
    /// ISO 3166-1 alpha-2 country code.
    pub country: VpnCountryCode,
    pub city: String,
}

/// Operator-chosen identifier for one server entry. Letters,
/// digits, dashes — used to compose NetworkManager connection
/// names like `nordvpn-<name>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct NordvpnServerName(pub(crate) String);

impl NordvpnServerName {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::InvalidNordvpnServerName { got: s });
        }
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidNordvpnServerName { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for NordvpnServerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NordvpnServerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// ISO 3166-1 alpha-2 country code (two ASCII uppercase letters).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct VpnCountryCode(pub(crate) String);

impl VpnCountryCode {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.len() != 2 || !s.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(Error::InvalidVpnCountryCode { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for VpnCountryCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VpnCountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// IP address as a free string. Schema validation is minimal —
/// CriomOS' nordvpn module reads these straight into NetworkManager
/// configs where parsing happens.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct VpnIpAddress(pub(crate) String);

impl VpnIpAddress {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::EmptyName { kind: "VPN IP address" });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for VpnIpAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VpnIpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Client CIDR string. Non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct VpnClientAddress(pub(crate) String);

impl VpnClientAddress {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::EmptyName { kind: "VPN client address" });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for VpnClientAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VpnClientAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
