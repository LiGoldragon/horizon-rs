//! View-side `Node` — per-node view with every computed field present.
//!
//! Two kinds of fields:
//! - **Always-derived**: present on every `Node` (viewpoint and ex-nodes).
//! - **Viewpoint-only**: `Some` on `horizon.node`, `None` on entries
//!   in `horizon.ex_nodes`. Filled by `Node::fill_viewpoint`.
//!
//! `NodeProposal::project` (in `proposal::node`) is the constructor.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::address::{LinkLocalAddress, NodeIp};
use crate::magnitude::AtLeast;
use crate::name::{CriomeDomainName, ModelName, NodeName, UserName};
use crate::proposal::{NodePlacement, NodeServices, RouterInterfaces, WireguardProxy};
use crate::pub_key::{
    NixPubKey, NixPubKeyLine, SshPubKey, SshPubKeyLine, WireguardPubKey,
};
use crate::species::{KnownModel, NodeSpecies, System};
use crate::view::io::Io;
use crate::view::machine::Machine;
use crate::view::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    // input pass-through (always present)
    pub name: NodeName,
    pub species: NodeSpecies,
    pub size: AtLeast,
    pub trust: AtLeast,
    pub machine: Machine,
    pub link_local_ips: Vec<LinkLocalAddress>,
    pub node_ip: Option<NodeIp>,
    pub wireguard_pub_key: Option<WireguardPubKey>,
    pub nordvpn: bool,
    pub wifi_cert: bool,
    /// Operator opt-in for the printer driver bundle (hplip, samsung,
    /// epson, gutenprint). Default false — toggle on per node.
    pub wants_printing: bool,
    /// Operator opt-in for HW-accelerated video decode. Modules pick
    /// the codec driver based on `machine.chip_gen`.
    pub wants_hw_video_accel: bool,
    /// Router interface roles for router nodes. `None` for non-router
    /// nodes and for invalid proposals that validation rejects upstream.
    pub router_interfaces: Option<RouterInterfaces>,
    /// Per-node service roles. Projected from proposal data; never
    /// inferred from the node name.
    pub services: NodeServices,
    pub placement: NodePlacement,

    // identity / connectivity (always derived)
    pub criome_domain_name: CriomeDomainName,
    pub system: System,

    /// `nix.buildMachines.<this>.maxJobs` from this viewpoint:
    /// how many derivations Nix dispatches in parallel to this builder.
    /// Derived from cores + role + size.
    pub max_jobs: u32,
    /// `nix.settings.cores` for this builder. `0` = use all cores per
    /// individual derivation. Universal default.
    pub build_cores: u32,

    // pubkey shadow from input pub_keys
    pub ssh_pub_key: SshPubKey,
    pub nix_pub_key: Option<NixPubKey>,
    /// Yggdrasil presence as a typed sub-record (pub_key + address +
    /// subnet travel together). `None` when this node is not on the
    /// mesh. Replaces the previous `ygg_pub_key` / `ygg_address` /
    /// `ygg_subnet` sibling fields per step 14 (address grouping).
    pub yggdrasil: Option<crate::proposal::YggPubKeyEntry>,

    // computed booleans (always derived)
    pub is_fully_trusted: bool,
    pub is_remote_nix_builder: bool,
    pub is_dispatcher: bool,
    pub is_large_edge: bool,
    pub enable_network_manager: bool,
    pub has_video_output: bool,
    pub chip_is_intel: bool,
    pub model_is_thinkpad: bool,

    // computed power-policy (systemd logind lid-switch actions)
    pub handle_lid_switch: LidSwitchAction,
    pub handle_lid_switch_external_power: LidSwitchAction,
    pub handle_lid_switch_docked: LidSwitchAction,

    // computed strings
    pub ssh_pub_key_line: SshPubKeyLine,
    pub nix_pub_key_line: Option<NixPubKeyLine>,
    /// Nix binary cache served by this node (for the viewpoint node)
    /// or by an ex-node (when iterating siblings). `None` when the
    /// node does not serve a binary cache. Replaces the previous
    /// `is_nix_cache` / `nix_cache_domain` / `nix_url` sibling fields:
    /// presence ⇔ serves a cache, and the entry carries the data.
    pub nix_cache: Option<NixCache>,

    // grouped flags
    pub behaves_as: BehavesAs,
    pub type_is: TypeIs,

    // viewpoint-only fields
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub io: Option<Io>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub use_colemak: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub computer_is: Option<ComputerIs>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub builder_configs: Option<Vec<BuilderConfig>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cache_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ex_nodes_ssh_pub_keys: Option<Vec<SshPubKeyLine>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dispatchers_ssh_pub_keys: Option<Vec<SshPubKeyLine>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub admin_ssh_pub_keys: Option<Vec<SshPubKeyLine>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub wireguard_untrusted_proxies: Option<Vec<WireguardProxy>>,
}

/// Nix binary cache served by a node. Presence on `Node.nix_cache`
/// signals that the node serves a binary cache; absence means it
/// does not. Replaces the previous `is_nix_cache: bool` +
/// `nix_cache_domain: Option<...>` + `nix_url: Option<String>` trio
/// (same yggdrasil-style collapse from step 14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NixCache {
    pub domain: CriomeDomainName,
    pub url: String,
}

/// systemd-logind lid-switch policy. Serialises to the lowercase
/// strings systemd accepts directly (`"ignore" | "suspend" | "lock"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LidSwitchAction {
    Ignore,
    Suspend,
    Lock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BehavesAs {
    pub center: bool,
    pub router: bool,
    pub edge: bool,
    pub next_gen: bool,
    pub low_power: bool,
    pub bare_metal: bool,
    pub virtual_machine: bool,
    pub iso: bool,
    pub large_ai: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeIs {
    pub center: bool,
    pub edge: bool,
    pub edge_testing: bool,
    pub hybrid: bool,
    pub large_ai: bool,
    pub large_ai_router: bool,
    pub media_broadcast: bool,
    pub router: bool,
    pub router_testing: bool,
    pub publication: bool,
}

impl TypeIs {
    pub(crate) fn from_species(s: NodeSpecies) -> Self {
        TypeIs {
            center: matches!(s, NodeSpecies::Center),
            edge: matches!(s, NodeSpecies::Edge),
            edge_testing: matches!(s, NodeSpecies::EdgeTesting),
            hybrid: matches!(s, NodeSpecies::Hybrid),
            large_ai: matches!(s, NodeSpecies::LargeAi),
            large_ai_router: matches!(s, NodeSpecies::LargeAiRouter),
            media_broadcast: matches!(s, NodeSpecies::MediaBroadcast),
            router: matches!(s, NodeSpecies::Router),
            router_testing: matches!(s, NodeSpecies::RouterTesting),
            publication: matches!(s, NodeSpecies::Publication),
        }
    }
}

impl BehavesAs {
    pub(crate) fn derive(
        type_is: &TypeIs,
        placement: &NodePlacement,
        io_disks_empty: bool,
    ) -> Self {
        let large_ai = type_is.large_ai || type_is.large_ai_router;
        let router = type_is.hybrid || type_is.router || type_is.large_ai_router;
        let edge = type_is.edge || type_is.hybrid || type_is.edge_testing;
        let center = type_is.center || large_ai;
        let next_gen = type_is.edge_testing || type_is.hybrid;
        let low_power = type_is.edge || type_is.edge_testing;
        let bare_metal = matches!(placement, NodePlacement::Metal {});
        let virtual_machine = matches!(placement, NodePlacement::Contained { .. });
        let iso = !virtual_machine && io_disks_empty;
        BehavesAs {
            center,
            router,
            edge,
            next_gen,
            low_power,
            bare_metal,
            virtual_machine,
            iso,
            large_ai,
        }
    }

    /// systemd-logind lid-switch policy derived from the node's
    /// behaves-as flags. Centers ignore lid events entirely; edges
    /// lock when docked; otherwise the policy depends on the
    /// power state.
    pub(crate) fn lid_switch_policy(&self) -> LidSwitchPolicy {
        let on_battery = if self.center {
            LidSwitchAction::Ignore
        } else {
            LidSwitchAction::Suspend
        };
        let on_external_power = if self.center {
            LidSwitchAction::Ignore
        } else if self.low_power {
            LidSwitchAction::Suspend
        } else {
            LidSwitchAction::Lock
        };
        let docked = if self.edge {
            LidSwitchAction::Lock
        } else {
            LidSwitchAction::Ignore
        };
        LidSwitchPolicy {
            on_battery,
            on_external_power,
            docked,
        }
    }
}

pub(crate) struct LidSwitchPolicy {
    pub(crate) on_battery: LidSwitchAction,
    pub(crate) on_external_power: LidSwitchAction,
    pub(crate) docked: LidSwitchAction,
}

/// Closed set of computer-model flags downstream consumers gate on.
/// Add a variant here when a new model warrants a config branch.
/// Field names emit as camelCase per nota convention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputerIs {
    pub thinkpad_t14_gen2_intel: bool,
    pub thinkpad_t14_gen5_intel: bool,
    pub thinkpad_x230: bool,
    pub thinkpad_x240: bool,
    pub rpi3b: bool,
}

impl ComputerIs {
    pub(crate) fn from_model(model: Option<&ModelName>) -> Self {
        let known = model.and_then(ModelName::known);
        ComputerIs {
            thinkpad_t14_gen2_intel: known == Some(KnownModel::ThinkPadT14Gen2Intel),
            thinkpad_t14_gen5_intel: known == Some(KnownModel::ThinkPadT14Gen5Intel),
            thinkpad_x230: known == Some(KnownModel::ThinkPadX230),
            thinkpad_x240: known == Some(KnownModel::ThinkPadX240),
            rpi3b: known == Some(KnownModel::Rpi3B),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderConfig {
    pub host_name: CriomeDomainName,
    pub ssh_user: String,
    pub ssh_key: String,
    pub supported_features: Vec<String>,
    pub system: System,
    pub systems: Vec<System>,
    pub max_jobs: u32,
    /// Builder's SSH host pubkey, base64 form (no `ssh-ed25519 ` prefix).
    /// Maps to `nix.buildMachines.<n>.publicHostKey` which the consumer
    /// uses to verify the builder's identity at SSH-connect time. Without
    /// this populated, the dispatcher's nix-daemon (no-TTY root context)
    /// cannot answer the host-trust prompt and the build silently hangs.
    pub public_host_key: SshPubKey,
    /// Same key in line form (`ssh-ed25519 AAAA...== <comment>`). Maps to
    /// `programs.ssh.knownHosts.<host>.publicKey`. Populating both
    /// `publicHostKey` (in `nix.buildMachines`) and `programs.ssh.knownHosts`
    /// makes the SSH path fully declarative — no manual `ssh-keyscan`.
    pub public_host_key_line: SshPubKeyLine,
}

impl BuilderConfig {
    fn from_node(node: &Node) -> Self {
        // Legacy emitted "i686-linux" as a sibling system on x86_64 nodes. We
        // don't model i686 as a first-class `System` variant; the list stays
        // empty until a consumer actually needs it.
        let systems = Vec::new();
        BuilderConfig {
            host_name: node.criome_domain_name.clone(),
            // `nix-ssh` is the user `nix.sshServe.enable = true` creates on
            // the builder. Match it on the dispatcher side so `sshServe.keys`
            // and `buildMachines.<n>.sshUser` line up out of the box.
            ssh_user: "nix-ssh".to_string(),
            // The dispatcher's nix-daemon runs as root with no provisioned
            // user keys (NixOS doesn't auto-generate `/root/.ssh/id_*`).
            // Use the host's SSH host key as the daemon's identity instead —
            // sshd auto-generates `ssh_host_ed25519_key` at first boot, mode
            // 600 root-owned. The builder authorizes the dispatcher's host
            // *pub*key as if it were a user key in `nix.sshServe.keys`.
            ssh_key: "/etc/ssh/ssh_host_ed25519_key".to_string(),
            supported_features: if node.type_is.edge {
                Vec::new()
            } else {
                // `big-parallel`: required by LLVM, kernels, chromium, etc.
                // `kvm`: required by `nixos-tests` that boot VMs. Cheap to
                // claim on a non-edge bare-metal host (always has /dev/kvm).
                vec!["big-parallel".to_string(), "kvm".to_string()]
            },
            system: node.system,
            systems,
            max_jobs: node.max_jobs,
            public_host_key: node.ssh_pub_key.clone(),
            public_host_key_line: node.ssh_pub_key_line.clone(),
        }
    }
}

pub struct ViewpointFill<'a> {
    pub proposal_io: crate::proposal::Io,
    pub all_nodes: &'a BTreeMap<NodeName, Node>,
    pub all_users: &'a BTreeMap<UserName, User>,
    pub wireguard_untrusted_proxies: Vec<WireguardProxy>,
}

impl Node {
    /// Fill viewpoint-only fields on the viewpoint node. Idempotent.
    pub fn fill_viewpoint(&mut self, fill: ViewpointFill<'_>) {
        let use_colemak = matches!(fill.proposal_io.keyboard, crate::species::Keyboard::Colemak);

        let computer_is = ComputerIs::from_model(self.machine.model.as_ref());

        // Sibling nodes only, in deterministic order.
        let ex_nodes: Vec<&Node> = fill
            .all_nodes
            .iter()
            .filter(|(name, _)| **name != self.name)
            .map(|(_, n)| n)
            .collect();

        let builder_configs: Vec<BuilderConfig> = ex_nodes
            .iter()
            .filter(|n| n.is_remote_nix_builder)
            .map(|n| BuilderConfig::from_node(n))
            .collect();

        let cache_urls: Vec<String> = ex_nodes
            .iter()
            .filter_map(|n| n.nix_cache.as_ref().map(|c| c.url.clone()))
            .collect();

        let ex_nodes_ssh_pub_keys: Vec<SshPubKeyLine> = ex_nodes
            .iter()
            .map(|n| n.ssh_pub_key_line.clone())
            .collect();

        let dispatchers_ssh_pub_keys: Vec<SshPubKeyLine> = ex_nodes
            .iter()
            .filter(|n| n.is_dispatcher)
            .map(|n| n.ssh_pub_key_line.clone())
            .collect();

        // adminSshPubKeys: for each user with trust=Max, walk their pubKeys; for each
        // entry whose node is fully trusted, take that ssh line. Dedup preserving order.
        let mut admin_ssh_pub_keys: Vec<SshPubKeyLine> = Vec::new();
        for user in fill.all_users.values().filter(|u| u.trust.max) {
            for (node_name, entry) in &user.pub_keys {
                let is_trusted_node = fill
                    .all_nodes
                    .get(node_name)
                    .is_some_and(|n| n.is_fully_trusted);
                if is_trusted_node {
                    let line = entry.ssh.line();
                    if !admin_ssh_pub_keys.contains(&line) {
                        admin_ssh_pub_keys.push(line);
                    }
                }
            }
        }

        self.io = Some(Io::from(fill.proposal_io));
        self.use_colemak = Some(use_colemak);
        self.computer_is = Some(computer_is);
        self.builder_configs = Some(builder_configs);
        self.cache_urls = Some(cache_urls);
        self.ex_nodes_ssh_pub_keys = Some(ex_nodes_ssh_pub_keys);
        self.dispatchers_ssh_pub_keys = Some(dispatchers_ssh_pub_keys);
        self.admin_ssh_pub_keys = Some(admin_ssh_pub_keys);
        self.wireguard_untrusted_proxies = Some(fill.wireguard_untrusted_proxies);
    }
}
