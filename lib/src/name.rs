//! Typed name newtypes. Each kind of name is a distinct type so a
//! `NodeName` cannot be confused with a `UserName` or a `ClusterName`.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result as HorizonResult};
use crate::species::KnownModel;

macro_rules! string_newtype {
    ($name:ident, $kind:literal) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, NotaEncode,
        )]
        #[serde(transparent)]
        pub struct $name(pub(crate) String);

        impl $name {
            pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
                let s = s.into();
                if s.is_empty() {
                    Err(Error::EmptyName { kind: $kind })
                } else if s.contains('"') {
                    Err(Error::QuotationMarkInName {
                        kind: $kind,
                        got: s,
                    })
                } else {
                    Ok(Self(s))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = Error;
            fn from_str(s: &str) -> HorizonResult<Self> {
                Self::try_new(s)
            }
        }

        impl NotaDecode for $name {
            fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
                let value = NotaBlock::new(block).parse_string()?;
                Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
                    type_name: stringify!($name),
                    value,
                    reason: error.to_string(),
                })
            }
        }
    };
}

string_newtype!(ClusterName, "cluster name");
string_newtype!(NodeName, "node name");
string_newtype!(UserName, "user name");
string_newtype!(ModelName, "model name");
string_newtype!(GithubId, "github id");
string_newtype!(DomainName, "domain name");
string_newtype!(SecretName, "secret name");
string_newtype!(WirelessNetworkName, "wireless network name");

impl DomainName {
    pub fn for_tailnet(cluster: &ClusterName) -> Self {
        Self(format!("tailnet.{cluster}.criome"))
    }
}

impl ModelName {
    /// Parse this model name into its `KnownModel` form, if it
    /// matches one. Unknown model strings return `None` — the
    /// projection treats them as "no model-specific config branch."
    pub fn known(&self) -> Option<KnownModel> {
        match self.0.as_str() {
            "ThinkPadX230" => Some(KnownModel::ThinkPadX230),
            "ThinkPadX240" => Some(KnownModel::ThinkPadX240),
            "ThinkPadT14Gen2Intel" => Some(KnownModel::ThinkPadT14Gen2Intel),
            "ThinkPadT14Gen5Intel" => Some(KnownModel::ThinkPadT14Gen5Intel),
            "rpi3B" => Some(KnownModel::Rpi3B),
            _ => None,
        }
    }
}

/// Derived: `<node>.<cluster>.criome` — and also `nix.<criomeDomain>` for caches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(transparent)]
pub struct CriomeDomainName(pub(crate) String);

impl CriomeDomainName {
    pub fn for_node(node: &NodeName, cluster: &ClusterName) -> Self {
        Self(format!("{node}.{cluster}.criome"))
    }

    pub fn nix_subdomain(&self) -> Self {
        Self(format!("nix.{}", self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CriomeDomainName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CriomeDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// GPG keygrip: 40 hex chars.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Keygrip(pub(crate) String);

impl TryFrom<String> for Keygrip {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl Keygrip {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.to_ascii_uppercase()))
        } else {
            Err(Error::InvalidKeygrip { got: s })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for Keygrip {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_string()?;
        Self::try_new(value.clone()).map_err(|error| NotaDecodeError::InvalidValue {
            type_name: "Keygrip",
            value,
            reason: error.to_string(),
        })
    }
}

impl NotaEncode for Keygrip {
    fn to_nota(&self) -> String {
        self.0.to_nota()
    }
}

impl From<Keygrip> for String {
    fn from(keygrip: Keygrip) -> Self {
        keygrip.0
    }
}

impl AsRef<str> for Keygrip {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Keygrip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
