//! Input shape: what goldragon emits as a nota cluster proposal.
//!
//! `ClusterProposal::project(viewpoint)` is the single entry-point;
//! it produces the typed `Horizon`. Proposal types carry only raw
//! data — no derived fields appear here.

use std::collections::BTreeMap;

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use serde::{Deserialize, Serialize};

use crate::address::{Interface, LinkLocalIp, NodeIp, TapSubnet};
use crate::address::{YggAddress, YggSubnet};
use crate::io::Io;
use crate::machine::Machine;
use crate::magnitude::Magnitude;
use crate::name::{
    ClusterName, DomainName, GithubId, Keygrip, NodeName, SecretName, UserName, WirelessNetworkName,
};
use crate::pub_key::{NixPubKey, SshPubKey, WireguardPubKey, YggPubKey};
use crate::species::{DomainSpecies, Editor, Keyboard, NodeSpecies, Style, TextSize, UserSpecies};

/// The proposal a cluster owner emits.
#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
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

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct NodeProposal {
    pub species: NodeSpecies,
    #[serde(default = "Magnitude::default_zero")]
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
    /// (so other consumers can see they exist) but the
    /// `is_remote_nix_builder` predicate is gated on `online`.
    #[serde(default)]
    pub online: Option<bool>,

    /// Per-node service roles. This is cluster role data: consumers
    /// must not infer it from node names, and role variants must not
    /// carry CriomOS-standard ports, domains, or implementation
    /// defaults.
    #[serde(default)]
    pub services: Vec<NodeService>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum NodeService {
    /// Join the cluster tailnet. CriomOS currently renders this with
    /// Tailscale.
    TailnetClient {},
    /// Host the cluster tailnet controller. CriomOS derives the
    /// Headscale port and MagicDNS base domain.
    TailnetController {},
    /// Receive remote Nix builds. `maximum_jobs` is cluster-authored
    /// capacity policy; absent means one job at a time.
    NixBuilder {
        #[serde(default)]
        maximum_jobs: Option<u32>,
    },
    /// Serve a cluster Nix binary cache. CriomOS owns the service port
    /// and signing-key path.
    NixCache {},
    /// Host Persona development infrastructure. Nested capabilities
    /// select sub-roles without making the cluster author CriomOS
    /// implementation details.
    PersonaDevelopment {
        #[serde(default)]
        capabilities: Vec<PersonaDevelopmentCapability>,
    },
    /// Run cluster test VMs. The host's VM substrate (tap subnet, KVM
    /// availability, capacity ceiling) is cluster-authored here rather
    /// than invented in the Nix layer. Guests are still discovered by
    /// the `ex_nodes` fold (`super_node == this_node` &&
    /// `behaves_as.test_vm`); this carries only the host's own
    /// capability data. Sibling to `NixBuilder` — both are opt-in
    /// per-node capabilities, never inferred from the node name.
    VmHost {
        /// CIDR the per-guest taps live in. Replaces the hardcoded
        /// `169.254.100+index.1` host-endpoint scheme. The generator
        /// derives each guest's host endpoint and route from this
        /// subnet plus the guest index.
        guest_subnet: TapSubnet,
        /// Hardware acceleration availability (`/dev/kvm`). When
        /// `Absent` the generator emits a TCG (software) substrate.
        kvm: KvmAvailability,
        /// Maximum concurrent guests this host advertises. `None` means
        /// no declared ceiling; the generator asserts the hosted set
        /// fits when a ceiling is present.
        #[serde(default)]
        maximum_guests: Option<MaximumGuests>,
    },
}

/// Whether a VM host offers hardware acceleration (`/dev/kvm`). A
/// closed-set domain value rather than a bare `bool`: the projection
/// and the generator switch substrate on it, so it earns a name and a
/// type the wire renders as `Available` / `Absent` instead of an
/// anonymous boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
pub enum KvmAvailability {
    /// `/dev/kvm` is present; guests boot under hardware acceleration.
    Available,
    /// No `/dev/kvm`; guests fall back to a software (TCG) substrate.
    Absent,
}

impl KvmAvailability {
    pub fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// Maximum concurrent test-VM guests a host advertises. A typed count
/// rather than a bare `u32` so the capacity ceiling cannot be confused
/// with any other small integer the projection carries (cores, jobs,
/// guest index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(transparent)]
pub struct MaximumGuests(u32);

impl MaximumGuests {
    pub fn new(count: u32) -> Self {
        Self(count)
    }

    pub fn count(self) -> u32 {
        self.0
    }
}

/// A borrowed view of a host's `VmHost` capability — the exact
/// cluster-authored data the VM-test generator reads off a host's
/// projection. One object out of `NodeService::vm_host`, so a consumer
/// pattern-matches the service once and reads tap subnet, KVM
/// availability, and capacity together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmHostCapability<'a> {
    pub guest_subnet: &'a TapSubnet,
    pub kvm: KvmAvailability,
    pub maximum_guests: Option<MaximumGuests>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum PersonaDevelopmentCapability {
    /// Host the Git repository receive surface used by Persona
    /// development.
    GitoliteServer {},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeServiceKind {
    TailnetClient,
    TailnetController,
    NixBuilder,
    NixCache,
    PersonaDevelopment,
    VmHost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaDevelopmentCapabilityKind {
    GitoliteServer,
}

impl NodeService {
    pub fn kind(&self) -> NodeServiceKind {
        match self {
            Self::TailnetClient {} => NodeServiceKind::TailnetClient,
            Self::TailnetController {} => NodeServiceKind::TailnetController,
            Self::NixBuilder { .. } => NodeServiceKind::NixBuilder,
            Self::NixCache {} => NodeServiceKind::NixCache,
            Self::PersonaDevelopment { .. } => NodeServiceKind::PersonaDevelopment,
            Self::VmHost { .. } => NodeServiceKind::VmHost,
        }
    }

    pub fn is_kind(&self, kind: NodeServiceKind) -> bool {
        self.kind() == kind
    }

    pub fn nix_builder_maximum_jobs(&self) -> Option<u32> {
        match self {
            Self::NixBuilder { maximum_jobs } => *maximum_jobs,
            _ => None,
        }
    }

    /// The cluster-authored VM-host capability data, if this service is
    /// a `VmHost`. The VM-test generator reads the host's tap subnet,
    /// KVM availability, and capacity ceiling through this.
    pub fn vm_host(&self) -> Option<VmHostCapability<'_>> {
        match self {
            Self::VmHost {
                guest_subnet,
                kvm,
                maximum_guests,
            } => Some(VmHostCapability {
                guest_subnet,
                kvm: *kvm,
                maximum_guests: *maximum_guests,
            }),
            _ => None,
        }
    }

    pub fn has_persona_development_capability(
        &self,
        kind: PersonaDevelopmentCapabilityKind,
    ) -> bool {
        match self {
            Self::PersonaDevelopment { capabilities } => capabilities
                .iter()
                .any(|capability| capability.is_kind(kind)),
            _ => false,
        }
    }
}

impl NotaEncode for NodeService {
    fn to_nota(&self) -> String {
        match self {
            NodeService::TailnetClient {} => {
                Delimiter::Parenthesis.wrap(["TailnetClient".to_owned()])
            }
            NodeService::TailnetController {} => {
                Delimiter::Parenthesis.wrap(["TailnetController".to_owned()])
            }
            NodeService::NixBuilder { maximum_jobs } => {
                Delimiter::Parenthesis.wrap(["NixBuilder".to_owned(), maximum_jobs.to_nota()])
            }
            NodeService::NixCache {} => Delimiter::Parenthesis.wrap(["NixCache".to_owned()]),
            NodeService::PersonaDevelopment { capabilities } => Delimiter::Parenthesis
                .wrap(["PersonaDevelopment".to_owned(), capabilities.to_nota()]),
            NodeService::VmHost {
                guest_subnet,
                kvm,
                maximum_guests,
            } => Delimiter::Parenthesis.wrap([
                "VmHost".to_owned(),
                guest_subnet.to_nota(),
                kvm.to_nota(),
                maximum_guests.to_nota(),
            ]),
        }
    }
}

impl NotaDecode for NodeService {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_delimited(Delimiter::Parenthesis, "NodeService")?;
        let variant = fields.first().and_then(Block::demote_to_string).ok_or(
            NotaDecodeError::ExpectedAtom {
                type_name: "NodeService",
            },
        )?;
        let service = match variant {
            "TailnetClient" => {
                Self::expect_service_arity(fields, variant, 1)?;
                NodeService::TailnetClient {}
            }
            "TailnetController" => {
                Self::expect_service_arity(fields, variant, 1)?;
                NodeService::TailnetController {}
            }
            "NixBuilder" => {
                Self::expect_service_arity(fields, variant, 2)?;
                NodeService::NixBuilder {
                    maximum_jobs: Option::<u32>::from_nota_block(&fields[1])?,
                }
            }
            "NixCache" => {
                Self::expect_service_arity(fields, variant, 1)?;
                NodeService::NixCache {}
            }
            "PersonaDevelopment" => {
                Self::expect_service_arity(fields, variant, 2)?;
                NodeService::PersonaDevelopment {
                    capabilities: Vec::<PersonaDevelopmentCapability>::from_nota_block(&fields[1])?,
                }
            }
            "VmHost" => {
                Self::expect_service_arity(fields, variant, 4)?;
                NodeService::VmHost {
                    guest_subnet: TapSubnet::from_nota_block(&fields[1])?,
                    kvm: KvmAvailability::from_nota_block(&fields[2])?,
                    maximum_guests: Option::<MaximumGuests>::from_nota_block(&fields[3])?,
                }
            }
            other => {
                return Err(NotaDecodeError::UnknownVariant {
                    enum_name: "NodeService",
                    variant: other.to_string(),
                });
            }
        };
        Ok(service)
    }
}

impl NodeService {
    fn expect_service_arity(
        fields: &[Block],
        variant: &str,
        expected: usize,
    ) -> Result<(), NotaDecodeError> {
        if fields.len() != expected {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name: match variant {
                    "TailnetClient" => "TailnetClient",
                    "TailnetController" => "TailnetController",
                    "NixBuilder" => "NixBuilder",
                    "NixCache" => "NixCache",
                    "PersonaDevelopment" => "PersonaDevelopment",
                    "VmHost" => "VmHost",
                    _ => "NodeService",
                },
                expected,
                found: fields.len(),
            });
        }
        Ok(())
    }
}

impl PersonaDevelopmentCapability {
    pub fn kind(&self) -> PersonaDevelopmentCapabilityKind {
        match self {
            Self::GitoliteServer {} => PersonaDevelopmentCapabilityKind::GitoliteServer,
        }
    }

    pub fn is_kind(&self, kind: PersonaDevelopmentCapabilityKind) -> bool {
        self.kind() == kind
    }
}

impl NotaEncode for PersonaDevelopmentCapability {
    fn to_nota(&self) -> String {
        match self {
            PersonaDevelopmentCapability::GitoliteServer {} => {
                Delimiter::Parenthesis.wrap(["GitoliteServer".to_owned()])
            }
        }
    }
}

impl NotaDecode for PersonaDevelopmentCapability {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields = NotaBlock::new(block)
            .expect_delimited(Delimiter::Parenthesis, "PersonaDevelopmentCapability")?;
        if fields.len() != 1 {
            return Err(NotaDecodeError::ExpectedRootCount {
                type_name: "PersonaDevelopmentCapability",
                expected: 1,
                found: fields.len(),
            });
        }
        let variant = fields[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "PersonaDevelopmentCapability",
            })?;
        let capability = match variant {
            "GitoliteServer" => PersonaDevelopmentCapability::GitoliteServer {},
            other => {
                return Err(NotaDecodeError::UnknownVariant {
                    enum_name: "PersonaDevelopmentCapability",
                    variant: other.to_string(),
                });
            }
        };
        Ok(capability)
    }
}

impl NodeProposal {
    pub fn has_service(&self, kind: NodeServiceKind) -> bool {
        self.services.iter().any(|service| service.is_kind(kind))
    }

    pub fn nix_builder_maximum_jobs(&self) -> Option<u32> {
        self.services
            .iter()
            .find_map(NodeService::nix_builder_maximum_jobs)
    }

    /// The host's `VmHost` capability data, if it declares one. The
    /// VM-test generator reads the tap subnet, KVM availability, and
    /// capacity ceiling through this.
    pub fn vm_host_capability(&self) -> Option<VmHostCapability<'_>> {
        self.services.iter().find_map(NodeService::vm_host)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct RouterInterfaces {
    pub wan: Interface,
    pub wlan: Interface,
    pub wlan_band: WlanBand,
    pub wlan_channel: u16,
    pub wlan_standard: WlanStandard,
    /// Runtime secret reference for the transitional WPA3-SAE network.
    /// CriomOS resolves this to a sops-nix secret file and passes the
    /// decrypted path to hostapd's `saePasswordsFile`.
    #[serde(default)]
    pub wpa3_sae_password: Option<SecretReference>,
    /// Optional physically separate backup wireless access point. This
    /// is cluster-authored interface/SSID/secret data; CriomOS owns the
    /// actual hostapd service shape and routing policy.
    #[serde(default)]
    pub backup_wireless: Option<BackupWireless>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct BackupWireless {
    pub interface: Interface,
    pub network_name: WirelessNetworkName,
    pub band: WlanBand,
    pub channel: u16,
    pub standard: WlanStandard,
    pub password: SecretReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    pub name: SecretName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
pub enum WlanBand {
    #[serde(rename = "2g")]
    TwoG,
    #[serde(rename = "5g")]
    FiveG,
    #[serde(rename = "6g")]
    SixG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub enum WlanStandard {
    Wifi4,
    Wifi6,
    Wifi7,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct NodePubKeys {
    pub ssh: SshPubKey,
    #[serde(default)]
    pub nix: Option<NixPubKey>,
    #[serde(default)]
    pub yggdrasil: Option<YggPubKeyEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct YggPubKeyEntry {
    pub pub_key: YggPubKey,
    pub address: YggAddress,
    pub subnet: YggSubnet,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct UserProposal {
    pub species: UserSpecies,
    #[serde(default = "Magnitude::default_zero")]
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
    /// Preferred top-level editor application. `None` means use the
    /// projection's smart default (`Emacs` for code developers,
    /// `Codium` otherwise).
    #[serde(default)]
    pub editor: Option<Editor>,
    /// Preferred relative text size — drives terminal font, editor
    /// font, and editor UI zoom. `None` means use the default
    /// (`Medium`).
    #[serde(default)]
    pub text_size: Option<TextSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct UserPubKeyEntry {
    pub ssh: SshPubKey,
    pub keygrip: Keygrip,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
#[serde(rename_all = "camelCase")]
pub struct DomainProposal {
    pub species: DomainSpecies,
}

#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
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
#[derive(Debug, Clone, Serialize, Deserialize, NotaDecode, NotaEncode)]
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
    pub(crate) fn default_zero() -> Self {
        Magnitude::Zero
    }
    pub(crate) fn default_min() -> Self {
        Magnitude::Min
    }
}
