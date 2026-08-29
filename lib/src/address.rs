//! Network addresses: yggdrasil identifiers, node IPs, link-local
//! per-interface addresses.

use std::net::Ipv6Addr;

use datomic::{Datomic, DatomicString, Fault, FaultProblem, PortionBuilding, PortionViewing};
use ipnet::{IpNet, Ipv4Net};
use protos::{Portion, StructuralEnclosure};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result as HorizonResult};

/// Yggdrasil-mesh IPv6 address. Always within `200::/7`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct YggAddress(Ipv6Addr);

impl YggAddress {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidYggAddress { got: s, source: e })
    }

    pub fn ipv6(self) -> Ipv6Addr {
        self.0
    }
}

impl TryFrom<String> for YggAddress {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl From<YggAddress> for String {
    fn from(a: YggAddress) -> Self {
        a.0.to_string()
    }
}

impl std::fmt::Display for YggAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Datomic for YggAddress {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let value = DatomicString::embody(portion)?;
        Self::try_new(value.as_ref().to_owned()).map_err(|_| portion.fault(FaultProblem::Value))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.to_string())
            .expect("parsed IPv6 address must be Datomic-representable");
        Datomic::portion(&value)
    }
}

/// Yggdrasil subnet identifier (e.g. `300:ca41:6b12:fba`). Free-form
/// today — not a parsed CIDR — because the legacy data carries it as
/// the bare prefix without a prefix length. Promote to `Ipv6Net` when
/// goldragon emits canonical CIDRs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YggSubnet(pub(crate) String);

impl YggSubnet {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        if s.is_empty() {
            Err(Error::EmptyYggSubnet)
        } else {
            Ok(Self(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Datomic for YggSubnet {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let value = DatomicString::embody(portion)?;
        Self::try_new(value.as_ref().to_owned()).map_err(|_| portion.fault(FaultProblem::Value))
    }

    fn portion(&self) -> Portion {
        datomic_string_portion(&self.0, "Yggdrasil subnet")
    }
}

/// Internal-cluster routing IP, as a CIDR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct NodeIp(IpNet);

impl Datomic for NodeIp {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let value = DatomicString::embody(portion)?;
        Self::try_new(value.as_ref().to_owned()).map_err(|_| portion.fault(FaultProblem::Value))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.to_string())
            .expect("parsed IP network must be Datomic-representable");
        Datomic::portion(&value)
    }
}

impl NodeIp {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidNodeIp { got: s, source: e })
    }

    pub fn ipnet(self) -> IpNet {
        self.0
    }
}

impl TryFrom<String> for NodeIp {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl From<NodeIp> for String {
    fn from(a: NodeIp) -> Self {
        a.0.to_string()
    }
}

/// The IPv4 CIDR subnet a VM host slices per-guest taps out of. One
/// cluster-authored subnet (e.g. `169.254.100.0/22`); the VM-test
/// generator derives each guest's host endpoint and route from this
/// subnet plus the guest index — replacing the per-guest
/// `169.254.100+index.1` scheme previously invented in the Nix layer.
///
/// IPv4-only on purpose: the CriomOS Nix generator slices this subnet
/// on `.` as dotted-decimal, so an IPv6 net would parse the Rust type
/// then silently misbehave in Nix. `ipnet::Ipv4Net` owns the parse and
/// rejects IPv6 at the boundary into `Error::InvalidTapSubnet`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TapSubnet(Ipv4Net);

impl TapSubnet {
    pub fn try_new(s: impl Into<String>) -> HorizonResult<Self> {
        let s = s.into();
        s.parse()
            .map(Self)
            .map_err(|e| Error::InvalidTapSubnet { got: s, source: e })
    }

    /// The underlying IPv4 network. The generator reads `.network()` /
    /// `.hosts()` off this to derive per-guest endpoints.
    pub fn ipv4_net(&self) -> Ipv4Net {
        self.0
    }

    /// The same value widened to `IpNet`, for callers that render or
    /// compare against the generic address type.
    pub fn ipnet(&self) -> IpNet {
        IpNet::V4(self.0)
    }

    /// Count of usable host addresses in the subnet — the host endpoints
    /// the generator can hand to guests. Excludes network and broadcast
    /// for prefixes shorter than `/31`, matching `Ipv4Net::hosts()`. The
    /// Nix capacity assert compares the hosted guest count against this.
    pub fn usable_host_count(&self) -> u64 {
        self.0.hosts().count() as u64
    }

    /// Whether the subnet has enough usable host slots to address
    /// `count` guests. The generator asserts this so an over-subscribed
    /// host fails loudly at eval rather than silently slicing outside
    /// the declared network.
    pub fn can_host(&self, count: u64) -> bool {
        self.usable_host_count() >= count
    }
}

impl TryFrom<String> for TapSubnet {
    type Error = Error;
    fn try_from(s: String) -> HorizonResult<Self> {
        Self::try_new(s)
    }
}

impl From<TapSubnet> for String {
    fn from(subnet: TapSubnet) -> Self {
        subnet.0.to_string()
    }
}

impl std::fmt::Display for TapSubnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Datomic for TapSubnet {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let value = DatomicString::embody(portion)?;
        Self::try_new(value.as_ref().to_owned()).map_err(|_| portion.fault(FaultProblem::Value))
    }

    fn portion(&self) -> Portion {
        let value = DatomicString::try_from(self.0.to_string())
            .expect("parsed IPv4 network must be Datomic-representable");
        Datomic::portion(&value)
    }
}

/// Network interface name (`enp0s25`, `wlp3s0`, …). Hardware-dependent;
/// the proposal author specifies it per link-local entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Interface(pub(crate) String);

impl Interface {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Datomic for Interface {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        datomic_string_portion(&self.0, "interface")
    }
}

/// Raw input form of a link-local address: an interface plus a
/// 64-bit suffix. Renders as `fe80::<suffix>%<iface>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkLocalIp {
    pub iface: Interface,
    pub suffix: String,
}

impl LinkLocalIp {
    pub fn render(&self) -> LinkLocalAddress {
        LinkLocalAddress(format!("fe80::{}%{}", self.suffix, self.iface))
    }
}

impl Datomic for LinkLocalIp {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        let Some(parts) = portion.structural(StructuralEnclosure::Braced) else {
            return Err(portion.fault(FaultProblem::Shape));
        };
        let [iface, suffix] = parts else {
            return Err(portion.fault(FaultProblem::Arity));
        };
        Ok(Self {
            iface: Interface::embody(iface)?,
            suffix: DatomicString::embody(suffix)?.as_ref().to_owned(),
        })
    }

    fn portion(&self) -> Portion {
        PortionBuilding::structural(
            "",
            StructuralEnclosure::Braced,
            vec![
                self.iface.portion(),
                datomic_string_portion(&self.suffix, "link-local suffix"),
            ],
        )
    }
}

/// Projected (rendered) link-local address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LinkLocalAddress(pub(crate) String);

impl LinkLocalAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LinkLocalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Datomic for LinkLocalAddress {
    fn embody(portion: &Portion) -> std::result::Result<Self, Fault> {
        Ok(Self(DatomicString::embody(portion)?.as_ref().to_owned()))
    }

    fn portion(&self) -> Portion {
        datomic_string_portion(&self.0, "link-local address")
    }
}

fn datomic_string_portion(value: &str, kind: &str) -> Portion {
    let value = DatomicString::try_from(value.to_owned())
        .unwrap_or_else(|_| panic!("{kind} must be Datomic-representable"));
    Datomic::portion(&value)
}
