//! Projected router interface roles.

use crate::address::Interface;
use nota_codec::NotaTryTransparent;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::proposal::router::{
    IsoCountryCode, RouterInterfaces as RouterInterfaceProposal, WlanBand, WlanStandard,
};
use crate::proposal::secret::SecretReference;

/// Wi-Fi SSID. Validation: 1 to 32 bytes (IEEE 802.11 limit).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTryTransparent)]
#[serde(try_from = "String", into = "String")]
pub struct Ssid(pub(crate) String);

impl Ssid {
    pub fn try_new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();
        let length = s.len();
        if length == 0 || length > 32 {
            Err(Error::InvalidSsid { got: s })
        } else {
            Ok(Self(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Ssid {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::try_new(s)
    }
}

impl AsRef<str> for Ssid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Ssid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfaces {
    pub wan: Interface,
    pub wlan: Interface,
    pub wlan_band: WlanBand,
    pub wlan_channel: u16,
    pub wlan_standard: WlanStandard,
    pub wpa3_sae_password: SecretReference,
    pub ssid: Ssid,
    pub country: IsoCountryCode,
}

impl RouterInterfaces {
    pub fn project(proposal: &RouterInterfaceProposal, ssid: Ssid) -> Self {
        Self {
            wan: proposal.wan.clone(),
            wlan: proposal.wlan.clone(),
            wlan_band: proposal.wlan_band,
            wlan_channel: proposal.wlan_channel,
            wlan_standard: proposal.wlan_standard,
            wpa3_sae_password: proposal.wpa3_sae_password.clone(),
            ssid,
            country: proposal.country.clone(),
        }
    }
}
