//! Projected router interface roles.

use serde::{Deserialize, Serialize};

use crate::address::Interface;
use crate::proposal::router::{
    IsoCountryCode, RouterInterfaces as RouterInterfaceProposal, Ssid, WlanBand, WlanStandard,
};
use crate::proposal::secret::SecretReference;

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
