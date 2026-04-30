//! Input shape: what goldragon emits as a nota cluster proposal.
//!
//! `ClusterProposal::project(viewpoint)` is the single entry-point;
//! it produces the typed `Horizon`. Proposal types carry only raw
//! data — no derived fields appear here.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::address::{Iface, LinkLocalIp, NodeIp};
use crate::address::{YggAddress, YggSubnet};
use crate::io::Io;
use crate::machine::Machine;
use crate::magnitude::Magnitude;
use crate::name::{ClusterName, DomainName, GithubId, Keygrip, NodeName, UserName};
use crate::pub_key::{NixPubKey, SshPubKey, WireguardPubKey, YggPubKey};
use crate::species::{DomainSpecies, Keyboard, NodeSpecies, Style, UserSpecies};

/// The proposal a cluster owner emits.
#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ClusterProposal {
    #[serde(default)]
    pub nodes: BTreeMap<NodeName, NodeProposal>,
    #[serde(default)]
    pub users: BTreeMap<UserName, UserProposal>,
    #[serde(default)]
    pub domains: BTreeMap<DomainName, DomainProposal>,
    pub trust: ClusterTrust,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodeProposal {
    pub species: NodeSpecies,
    #[serde(default = "Magnitude::default_none")]
    pub size: Magnitude,
    #[serde(default = "Magnitude::default_min")]
    pub trust: Magnitude,
    pub machine: Machine,
    pub io: Io,
    pub pub_keys: NodePubKeys,
    #[serde(default)]
    pub link_local_ips: Vec<LinkLocalIp>,
    #[serde(default)]
    pub node_ip: Option<NodeIp>,
    #[serde(default)]
    pub wireguard_pub_key: Option<WireguardPubKey>,
    #[serde(default)]
    pub nordvpn: bool,
    #[serde(default)]
    pub wifi_cert: bool,
    #[serde(default)]
    pub wireguard_untrusted_proxies: Vec<WireguardProxy>,
    /// Operator opt-in for the printer driver bundle (hplip, samsung,
    /// epson, gutenprint). Default false. Must stay near the end of
    /// this struct so existing positional nota files still parse.
    #[serde(default)]
    pub wants_printing: bool,
    /// Operator opt-in for hardware-accelerated video decode (browser
    /// playback, mpv, etc.). Modules pick the codec driver based on
    /// `machine.chip_gen`: Gen >= 12 → `vpl-gpu-rt` (AV1/HEVC); older
    /// Intel → `intel-vaapi-driver`. Default false; software fallback
    /// is silent.
    #[serde(default)]
    pub wants_hw_video_accel: bool,

    /// Router interface roles for nodes that behave as routers. These
    /// are deployment facts, not machine-model facts: two machines with
    /// the same model may have different interface names.
    #[serde(default)]
    pub router_interfaces: Option<RouterInterfaces>,

    /// Whether this node is currently reachable on the network.
    /// `None` (= default `Some(true)`) means online; `Some(false)`
    /// declares the node as administratively offline so dispatchers
    /// don't list it in `nix.buildMachines` and stall on TCP timeouts
    /// trying to reach it. Nodes that are offline still get projected
    /// (so other consumers can see they exist) but the `is_builder`
    /// predicate is gated on `online`.
    #[serde(default)]
    pub online: Option<bool>,

    /// `nix.buildMachines.<this>.maxJobs` from each dispatcher's
    /// viewpoint when this node acts as a remote builder; also drives
    /// `nix.settings.build-cores` locally on the node itself. `None`
    /// (= default `Some(1)`) means single-job-at-a-time, matching
    /// nix's default. Bump this up on big builders (e.g. prometheus
    /// at 6) to unlock parallel dispatch.
    #[serde(default)]
    pub nb_of_build_cores: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfaces {
    pub wan: Iface,
    pub wlan: Iface,
    pub wlan_band: WlanBand,
    pub wlan_channel: u16,
    pub wlan_standard: WlanStandard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, nota_codec::NotaEnum)]
pub enum WlanBand {
    #[serde(rename = "2g")]
    TwoG,
    #[serde(rename = "5g")]
    FiveG,
    #[serde(rename = "6g")]
    SixG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, nota_codec::NotaEnum)]
#[serde(rename_all = "camelCase")]
pub enum WlanStandard {
    Wifi4,
    Wifi6,
    Wifi7,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct NodePubKeys {
    pub ssh: SshPubKey,
    #[serde(default)]
    pub nix: Option<NixPubKey>,
    #[serde(default)]
    pub yggdrasil: Option<YggPubKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct YggPubKeyEntry {
    pub pub_key: YggPubKey,
    pub address: YggAddress,
    pub subnet: YggSubnet,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct UserProposal {
    pub species: UserSpecies,
    #[serde(default = "Magnitude::default_none")]
    pub size: Magnitude,
    pub keyboard: Keyboard,
    pub style: Style,
    #[serde(default)]
    pub github_id: Option<GithubId>,
    /// `None` means default-true; preserved to distinguish absent from explicit-true.
    #[serde(default)]
    pub fast_repeat: Option<bool>,
    #[serde(default)]
    pub pub_keys: BTreeMap<NodeName, UserPubKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct UserPubKeyEntry {
    pub ssh: SshPubKey,
    pub keygrip: Keygrip,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct DomainProposal {
    pub species: DomainSpecies,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ClusterTrust {
    pub cluster: Magnitude,
    #[serde(default)]
    pub clusters: BTreeMap<ClusterName, Magnitude>,
    #[serde(default)]
    pub nodes: BTreeMap<NodeName, Magnitude>,
    #[serde(default)]
    pub users: BTreeMap<UserName, Magnitude>,
}

/// An external WireGuard proxy this node tunnels through. Becomes a
/// peer on the `wgProxies` interface; downstream nix module routes
/// `0.0.0.0/0` through it. One per VPN connection (NordVPN, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct WireguardProxy {
    pub pub_key: WireguardPubKey,
    /// `host:port` form.
    pub endpoint: String,
    /// Address assigned to our wireguard interface for this proxy.
    pub interface_ip: NodeIp,
}

// Free-fn helpers used by serde defaults; not exposed.
impl Magnitude {
    pub(crate) fn default_none() -> Self {
        Magnitude::None
    }
    pub(crate) fn default_min() -> Self {
        Magnitude::Min
    }
}
