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

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result as HorizonResult};
use crate::name::CriomeDomainName;

const NIX_PUBKEY_LEN: usize = 44;
const WG_PUBKEY_LEN: usize = 44;
const YGG_PUBKEY_LEN: usize = 64;

struct PublicKeyText<'text> {
    text: &'text str,
}

impl<'text> PublicKeyText<'text> {
    fn new(text: &'text str) -> Self {
        Self { text }
    }

    fn is_base64(&self) -> bool {
        self.text.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || character == '+'
                || character == '/'
                || character == '='
        })
    }

    fn is_hex(&self) -> bool {
        self.text
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct SshPubKey(String);

impl TryFrom<String> for SshPubKey {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl SshPubKey {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if !s.is_empty() && PublicKeyText::new(&s).is_base64() {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key {
                expected_len: 0,
                got: s,
            })
        }
    }

    pub fn line(&self) -> SshPubKeyLine {
        SshPubKeyLine(format!("ssh-ed25519 {}", self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for SshPubKey {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_string()?;
        Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
            type_name: "SshPubKey",
            value,
            reason: error.to_string(),
        })
    }
}

impl NotaEncode for SshPubKey {
    fn to_nota(&self) -> String {
        self.0.to_nota()
    }
}

impl From<SshPubKey> for String {
    fn from(key: SshPubKey) -> Self {
        key.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct YggPubKey(String);

impl TryFrom<String> for YggPubKey {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl YggPubKey {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if s.len() == YGG_PUBKEY_LEN && PublicKeyText::new(&s).is_hex() {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(Error::InvalidHexKey {
                expected_len: YGG_PUBKEY_LEN,
                got: s,
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for YggPubKey {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_string()?;
        Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
            type_name: "YggPubKey",
            value,
            reason: error.to_string(),
        })
    }
}

impl NotaEncode for YggPubKey {
    fn to_nota(&self) -> String {
        self.0.to_nota()
    }
}

impl From<YggPubKey> for String {
    fn from(key: YggPubKey) -> Self {
        key.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NixPubKey(String);

impl TryFrom<String> for NixPubKey {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl NixPubKey {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if s.len() == NIX_PUBKEY_LEN && PublicKeyText::new(&s).is_base64() {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key {
                expected_len: NIX_PUBKEY_LEN,
                got: s,
            })
        }
    }

    pub fn line(&self, domain: &CriomeDomainName) -> NixPubKeyLine {
        NixPubKeyLine(format!("{}:{}", domain, self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for NixPubKey {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_string()?;
        Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
            type_name: "NixPubKey",
            value,
            reason: error.to_string(),
        })
    }
}

impl NotaEncode for NixPubKey {
    fn to_nota(&self) -> String {
        self.0.to_nota()
    }
}

impl From<NixPubKey> for String {
    fn from(key: NixPubKey) -> Self {
        key.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WireguardPubKey(String);

impl TryFrom<String> for WireguardPubKey {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl WireguardPubKey {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if s.len() == WG_PUBKEY_LEN && PublicKeyText::new(&s).is_base64() {
            Ok(Self(s))
        } else {
            Err(Error::InvalidBase64Key {
                expected_len: WG_PUBKEY_LEN,
                got: s,
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for WireguardPubKey {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_string()?;
        Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
            type_name: "WireguardPubKey",
            value,
            reason: error.to_string(),
        })
    }
}

impl NotaEncode for WireguardPubKey {
    fn to_nota(&self) -> String {
        self.0.to_nota()
    }
}

impl From<WireguardPubKey> for String {
    fn from(key: WireguardPubKey) -> Self {
        key.0
    }
}

/// Pre-rendered SSH known-hosts / authorized_keys line:
/// `ssh-ed25519 <pubKey>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode)]
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
