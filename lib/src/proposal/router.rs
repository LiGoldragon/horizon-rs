//! Router-interface roles for nodes that behave as routers.
//!
//! Deployment facts, not machine-model facts: two machines with the
//! same model may have different interface names.

use nota_codec::{NotaEnum, NotaRecord};
use serde::{Deserialize, Serialize};

use crate::address::Interface;
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
