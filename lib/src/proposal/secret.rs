//! Logical-name secret references + the per-cluster backend binding.
//!
//! `SecretReference` is what a node-level record carries when a field
//! needs a secret value (e.g. `WpaSae { password: SecretReference }`).
//! It is a typed *name*, not a path or a backend choice — nodes never
//! see where a secret physically lives. The cluster's
//! `ClusterSecretBinding` list resolves each name to a concrete
//! `SecretBackend` at projection time.
//!
//! Design source: `~/primary/reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md` §P1.1.

use nota_codec::{NotaEnum, NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// A logical, typed reference to a secret value. Resolved through the
/// cluster's `secret_bindings` list at projection time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    pub name: SecretName,
    pub purpose: SecretPurpose,
}

/// Identifier for a secret — non-empty ASCII letters, digits, and
/// dashes. Used as the join key between a node-level
/// `SecretReference` and the cluster-level `ClusterSecretBinding`.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaTransparent,
)]
#[serde(transparent)]
pub struct SecretName(pub(crate) String);

impl SecretName {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::InvalidSecretName { got: s });
        }
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidSecretName { got: s });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SecretName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SecretName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for SecretName {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::try_new(s)
    }
}

/// Closed set of documented secret-bearing roles. Each variant names
/// a kind of secret a node-level record may carry. Open list — add
/// new variants as new typed records introduce new secret-bearing
/// fields; never widen via a free-string fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
pub enum SecretPurpose {
    /// Nix binary cache signing key (gates `nix.settings.secret-key-files`).
    BinaryCacheSigning,
    /// WireGuard interface private key.
    WireguardPrivateKey,
    /// NordVPN account credentials.
    NordvpnCredentials,
    /// WPA3-SAE password for a Wi-Fi network.
    WifiPassword,
    /// EAP-TLS client identity private key.
    EapTlsClientKey,
    /// SMTP password for the Ghost mailer.
    GhostMailerPassword,
    /// Stripe API key for Ghost.
    GhostStripeKey,
    /// Cloud AI provider API key.
    AiProviderApiKey,
    /// TLS private key for a service certificate.
    TlsPrivateKey,
}

/// One entry in the cluster's secret-binding list. Maps a logical
/// `SecretName` to the concrete backend that stores its value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSecretBinding {
    pub name: SecretName,
    pub backend: SecretBackend,
}

/// The runtime mechanism that delivers a secret value to the
/// consuming service. Closed set — adding a new backend is a
/// workspace-level coordinated change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum SecretBackend {
    /// sops-nix encrypted file. `file` is the path to the sops YAML
    /// (relative to the cluster repo); `key` is the YAML key path
    /// within the file.
    Sops {
        file: SopsFilePath,
        key: SopsKeyPath,
    },
    /// systemd LoadCredential: the service unit imports a credential
    /// named `credential_name` and reads it from `$CREDENTIALS_DIRECTORY`.
    SystemdCredential { credential_name: String },
    /// agenix secret identified by `secret_id`.
    Agenix { secret_id: String },
}

/// Relative path to a sops-encrypted YAML file inside the cluster
/// repo. Non-empty. Validation stays minimal here so authors can
/// move files around without fighting the schema; semantic checks
/// (file exists, is sops-encrypted) happen in CriomOS modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct SopsFilePath(pub(crate) String);

impl SopsFilePath {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::EmptyName {
                kind: "sops file path",
            });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SopsFilePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SopsFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// YAML key path within a sops-encrypted file (slash-separated form
/// matching the way sops addresses nested fields). Non-empty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct SopsKeyPath(pub(crate) String);

impl SopsKeyPath {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.is_empty() {
            return Err(Error::EmptyName {
                kind: "sops key path",
            });
        }
        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SopsKeyPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SopsKeyPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
