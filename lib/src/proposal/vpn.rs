//! Cluster-level VPN provider selections.
//!
//! Server catalogs, DNS defaults, and client defaults are CriomOS
//! inventory. The cluster chooses providers, credentials, and optional
//! location preferences.

use nota_codec::{NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::proposal::secret::SecretReference;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum VpnProfile {
    NordvpnProfile(NordvpnProfile),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NordvpnProfile {
    pub credentials: SecretReference,
    #[serde(default)]
    pub preferred_locations: Vec<NordvpnLocationPreference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NordvpnLocationPreference {
    pub country: VpnCountryCode,
    #[serde(default)]
    pub city: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent,
)]
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
