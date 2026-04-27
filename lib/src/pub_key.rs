//! Public-key newtypes. Each protocol's key is its own type so they
//! cannot be confused at boundaries.
//!
//! - `SshPubKey` — the base64 portion after `ssh-ed25519`.
//! - `YggPubKey` — yggdrasil ed25519 hex (128 chars: 64-byte key).
//! - `NixPubKey` — nix signing key, base64 (44 chars: 32-byte key).
//! - `WireguardPubKey` — wireguard public key, base64 (44 chars).
//!
//! Derived line types (`SshPubKeyLine`, `NixPubKeyLine`) carry the
//! pre-rendered string form used by downstream consumers.

use nota_codec::{NotaTransparent, NotaTryTransparent};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::name::CriomeDomainName;

const NIX_PUBKEY_LEN: usize = 44;
const WG_PUBKEY_LEN: usize = 44;
const YGG_PUBKEY_LEN: usize = 64;

fn is_base64(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
}

fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct SshPubKey(String);

impl TryFrom<String> for SshPubKey {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl SshPubKey {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if !s.is_empty() && is_base64(&s) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key { expected_len: 0, got: s })
        }
    }

    pub fn line(&self) -> SshPubKeyLine {
        SshPubKeyLine(format!("ssh-ed25519 {}", self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct YggPubKey(String);

impl TryFrom<String> for YggPubKey {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl YggPubKey {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.len() == YGG_PUBKEY_LEN && is_hex(&s) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(Error::InvalidHexKey { expected_len: YGG_PUBKEY_LEN, got: s })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct NixPubKey(String);

impl TryFrom<String> for NixPubKey {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl NixPubKey {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.len() == NIX_PUBKEY_LEN && is_base64(&s) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key { expected_len: NIX_PUBKEY_LEN, got: s })
        }
    }

    pub fn line(&self, domain: &CriomeDomainName) -> NixPubKeyLine {
        NixPubKeyLine(format!("{}:{}", domain, self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct WireguardPubKey(String);

impl TryFrom<String> for WireguardPubKey {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl WireguardPubKey {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        if s.len() == WG_PUBKEY_LEN && is_base64(&s) {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key { expected_len: WG_PUBKEY_LEN, got: s })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


/// Pre-rendered SSH known-hosts / authorized_keys line:
/// `ssh-ed25519 <pubKey>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct SshPubKeyLine(String);

impl SshPubKeyLine {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SshPubKeyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Pre-rendered nix `trusted-public-keys` entry:
/// `<criomeDomain>:<rawNixPubKey>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct NixPubKeyLine(String);

impl NixPubKeyLine {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NixPubKeyLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
