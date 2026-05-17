//! Router-interface roles for nodes that behave as routers.
//!
//! Deployment facts, not machine-model facts: two machines with the
//! same model may have different interface names.

use nota_codec::{NotaEnum, NotaRecord, NotaTryTransparent};
use serde::{Deserialize, Serialize};

use crate::address::Interface;
use crate::error::{Error, Result};
use crate::proposal::secret::SecretReference;

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfaces {
    pub wan: Interface,
    pub wlan: Interface,
    pub wlan_band: WlanBand,
    pub wlan_channel: u16,
    pub wlan_standard: WlanStandard,
    pub wpa3_sae_password: SecretReference,
    /// ISO 3166-1 alpha-2 country code (e.g. `PL`, `ES`) for hostapd's
    /// regulatory domain. Type enforces the format at the boundary.
    pub country: IsoCountryCode,
}

/// ISO 3166-1 alpha-2 country code. Validation: exactly two ASCII
/// uppercase letters. The doc comment used to carry this rule on a
/// `String` field; the type now carries it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct IsoCountryCode(pub(crate) String);

impl IsoCountryCode {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        let bytes = s.as_bytes();
        if bytes.len() == 2 && bytes[0].is_ascii_uppercase() && bytes[1].is_ascii_uppercase() {
            Ok(Self(s))
        } else {
            Err(Error::InvalidIsoCountryCode { got: s })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for IsoCountryCode {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl AsRef<str> for IsoCountryCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IsoCountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaEnum)]
pub enum WlanBand {
    #[serde(rename = "2g")]
    TwoG,
    #[serde(rename = "5g")]
    FiveG,
    #[serde(rename = "6g")]
    SixG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaEnum)]
#[serde(rename_all = "camelCase")]
pub enum WlanStandard {
    Wifi4,
    Wifi6,
    Wifi7,
}
