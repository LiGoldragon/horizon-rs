//! Node placement — where a node physically (Metal) or logically
//! (Contained inside another node) lives.

use nota_codec::{NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaSum, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::address::IpAddress;
use crate::magnitude::Magnitude;
use crate::name::{NodeName, UserName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum NodePlacement {
    Metal {},
    Contained {
        host: NodeName,
        user: UserName,
        substrate: Substrate,
        resources: Resources,
        network: ContainedNetwork,
        state: ContainedState,
        trust: Magnitude,
        user_namespace_policy: UserNamespacePolicy,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaSum)]
#[serde(rename_all_fields = "camelCase")]
pub enum Substrate {
    NixosContainer {},
    MicrovmCloudHypervisor {},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub cores: u32,
    pub ram_gb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ContainedNetwork {
    pub local_address: VirtualIp,
    pub host_address: VirtualIp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ContainedState {
    pub persistent_paths: Vec<PersistentPath>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct PersistentPath(pub(crate) String);

impl PersistentPath {
    pub fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PersistentPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PersistentPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaEnum)]
pub enum UserNamespacePolicy {
    PrivateUsersPick,
    PrivateUsersIdentity,
    PrivateUsersOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct VirtualIp(IpAddress);

impl VirtualIp {
    pub fn try_new(address: impl Into<String>) -> crate::Result<Self> {
        Ok(Self(IpAddress::try_new(address)?))
    }

    pub fn ip_address(self) -> IpAddress {
        self.0
    }
}

impl TryFrom<String> for VirtualIp {
    type Error = crate::Error;

    fn try_from(address: String) -> crate::Result<Self> {
        Self::try_new(address)
    }
}

impl From<VirtualIp> for String {
    fn from(address: VirtualIp) -> Self {
        address.0.to_string()
    }
}

impl std::fmt::Display for VirtualIp {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl NotaEncode for VirtualIp {
    fn encode(&self, encoder: &mut nota_codec::Encoder) -> nota_codec::Result<()> {
        encoder.write_string(&self.0.to_string())
    }
}

impl NotaDecode for VirtualIp {
    fn decode(decoder: &mut nota_codec::Decoder<'_>) -> nota_codec::Result<Self> {
        let address = decoder.read_string()?;
        VirtualIp::try_new(address.clone()).map_err(|error| nota_codec::Error::Validation {
            type_name: "VirtualIp",
            message: format!("invalid VirtualIp {address:?}: {error}"),
        })
    }
}
