//! Typed name newtypes. Each kind of name is a distinct type so a
//! `NodeName` cannot be confused with a `UserName` or a `ClusterName`.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

macro_rules! string_newtype {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn try_new(s: impl Into<String>) -> Result<Self> {
                let s = s.into();
                if s.is_empty() {
                    Err(Error::EmptyName { kind: $kind })
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
            fn from_str(s: &str) -> Result<Self> {
                Self::try_new(s)
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

/// Derived: `<node>.<cluster>.criome` — and also `nix.<criomeDomain>` for caches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CriomeDomainName(String);

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
pub struct Keygrip(String);

impl Keygrip {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
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

impl TryFrom<String> for Keygrip {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl From<Keygrip> for String {
    fn from(k: Keygrip) -> Self {
        k.0
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
