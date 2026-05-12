//! Typed secret references.
//!
//! Horizon names secrets logically. The cluster-level binding (which
//! backend stores which secret) lives separately on `Cluster`. Nodes
//! never see backend choice; CriomOS modules read both records to
//! render a runtime path.
//!
//! Spec: `reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
//! §P1.1 "SecretReference is a logical name, not a backend".

use nota_codec::NotaTransparent;
use serde::{Deserialize, Serialize};

/// A logical reference to a secret, decoupled from the backend that
/// stores it. The `name` is the operator-stable handle (matches a
/// binding on the cluster); the `purpose` is the typed declaration of
/// what role the secret plays.
///
/// First-slice note: `NotaRecord` derive will be added when this type
/// is wired into a proposal record (it currently isn't). Plain serde
/// derives are sufficient for the types-exist milestone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    pub name: SecretName,
    pub purpose: SecretPurpose,
}

/// Stable logical name for a secret. Matches an entry in the
/// cluster's `ClusterSecretBinding` map. Validation is permissive
/// at the newtype boundary; the binding lookup is the authoritative
/// presence check.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct SecretName(pub(crate) String);

impl SecretName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
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

/// Closed set of roles a secret can play in the cluster. One variant
/// per documented secret-bearing position. Adding a new variant is
/// the explicit declaration that a new kind of secret-bearing
/// position exists.
///
/// This is a typed declaration, not a free string — it forces the
/// reader to know which slot in the system the secret feeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, nota_codec::NotaEnum)]
pub enum SecretPurpose {
    BinaryCacheSigning,
    WireguardPrivateKey,
    NordvpnCredentials,
    GhostMailerPassword,
    GhostStripeKey,
}
