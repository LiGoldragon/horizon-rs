//! Cluster-level AI provider selections.
//!
//! The cluster names which endpoint exists and which node hosts it.
//! CriomOS owns the model catalog, protocol, ports, and runtime serving
//! defaults for each provider profile.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::name::NodeName;
use crate::proposal::secret::SecretReference;

/// Identifier the operator gives to one AI provider entry. Distinct
/// from the profile; one profile can be instantiated by many clusters.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent,
)]
#[serde(transparent)]
pub struct AiProviderName(pub(crate) String);

impl AiProviderName {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::InvalidAiProviderName { got: s });
        }
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidAiProviderName { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AiProviderName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AiProviderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
pub enum AiProviderProfile {
    CriomosLocalLlama,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct AiProvider {
    pub name: AiProviderName,
    pub serving_node: NodeName,
    pub profile: AiProviderProfile,
    /// `None` for endpoints that need no key. `Some(reference)` for
    /// endpoints whose runtime credential lives in the cluster's
    /// secret backend.
    #[serde(default)]
    pub api_key: Option<SecretReference>,
}
