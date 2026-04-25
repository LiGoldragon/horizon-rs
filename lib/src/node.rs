//! Output `Node`: per-node view with every computed field present.
//!
//! Two kinds of fields:
//! - **Always-derived**: present on every `Node` (viewpoint and ex-nodes).
//! - **Viewpoint-only**: `Some` on `horizon.node`, `None` on entries
//!   in `horizon.ex_nodes`. Filled by `Node::fill_viewpoint`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::address::{LinkLocalAddress, NodeIp, YggAddress, YggSubnet};
use crate::error::{Error, Result};
use crate::io::Io;
use crate::machine::Machine;
use crate::magnitude::{AtLeast, Magnitude};
use crate::name::{ClusterName, CriomeDomainName, ModelName, NodeName};
use crate::proposal::{NodeProposal, WireguardProxy};
use crate::pub_key::{NixPubKey, NixPubKeyLine, SshPubKey, SshPubKeyLine, WireguardPubKey, YggPubKey};
use crate::species::{Arch, NodeSpecies, System};
use crate::user::User;

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

    // pubkey shadow flattened from input pub_keys
    pub ssh_pub_key: SshPubKey,
    pub nix_pub_key: Option<NixPubKey>,
    pub ygg_pub_key: Option<YggPubKey>,
    pub ygg_address: Option<YggAddress>,
    pub ygg_subnet: Option<YggSubnet>,

    // computed booleans (always derived)
    pub is_fully_trusted: bool,
    pub is_builder: bool,
    pub is_dispatcher: bool,
    pub is_nix_cache: bool,
    pub is_large_edge: bool,
    pub enable_network_manager: bool,
    pub has_nix_pub_key: bool,
    pub has_ygg_pub_key: bool,
    pub has_ssh_pub_key: bool,
    pub has_wireguard_pub_key: bool,
    pub has_nordvpn_pub_key: bool,
    pub has_wifi_cert_pub_key: bool,
    pub has_base_pub_keys: bool,
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
    pub nix_cache_domain: Option<CriomeDomainName>,
    /// `http://<nix_cache_domain>` when `is_nix_cache`.
    pub nix_url: Option<String>,

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
}

impl TypeIs {
    fn from_species(s: NodeSpecies) -> Self {
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
        }
    }
}

impl BehavesAs {
    fn derive(type_is: &TypeIs, machine: &Machine, io_disks_empty: bool) -> Self {
        let large_ai = type_is.large_ai || type_is.large_ai_router;
        let router = type_is.hybrid || type_is.router || type_is.large_ai_router;
        let edge = type_is.edge || type_is.hybrid || type_is.edge_testing;
        let center = type_is.center || large_ai;
        let next_gen = type_is.edge_testing || type_is.hybrid;
        let low_power = type_is.edge || type_is.edge_testing;
        let bare_metal = matches!(machine.species, crate::species::MachineSpecies::Metal);
        let virtual_machine = matches!(machine.species, crate::species::MachineSpecies::Pod);
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
    fn from_model(model: Option<&ModelName>) -> Self {
        let m = model.map(ModelName::as_str);
        ComputerIs {
            thinkpad_t14_gen2_intel: m == Some("ThinkPadT14Gen2Intel"),
            thinkpad_t14_gen5_intel: m == Some("ThinkPadT14Gen5Intel"),
            thinkpad_x230: m == Some("ThinkPadX230"),
            thinkpad_x240: m == Some("ThinkPadX240"),
            rpi3b: m == Some("rpi3B"),
        }
    }
}

const THINKPAD_MODELS: &[&str] = &[
    "ThinkPadX240",
    "ThinkPadX230",
    "ThinkPadT14Gen2Intel",
    "ThinkPadT14Gen5Intel",
];

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
}

impl BuilderConfig {
    fn from_node(node: &Node) -> Self {
        // Legacy emitted "i686-linux" as a sibling system on x86_64 nodes. We
        // don't model i686 as a first-class `System` variant; the list stays
        // empty until a consumer actually needs it.
        let systems = Vec::new();
        BuilderConfig {
            host_name: node.criome_domain_name.clone(),
            ssh_user: "nixBuilder".to_string(),
            ssh_key: "/etc/ssh/ssh_host_ed25519_key".to_string(),
            supported_features: if node.type_is.edge {
                Vec::new()
            } else {
                vec!["big-parallel".to_string()]
            },
            system: node.system,
            systems,
            max_jobs: node.max_jobs,
        }
    }
}

/// Compute `(max_jobs, build_cores)` for a builder.
///
/// `build_cores = 0` universally — each derivation is allowed to use
/// every core via `NIX_BUILD_CORES=0`. Per-derivation `enableParallelBuilding`
/// then runs `make -j$(nproc)`.
///
/// `max_jobs` (parallel-derivations-on-this-builder) is role + size aware:
/// - 1-core machines (pods) → 1.
/// - size = None or Min → 1 (don't fan out on a node not meant to carry load).
/// - dedicated builders (`behaves_as.center`) → all cores.
/// - everything else (interactive edge / hybrid) → cores / 2,
///   leaving headroom for the human at the keyboard.
pub(crate) fn nix_concurrency(
    cores: u32,
    behaves_as_center: bool,
    size: Magnitude,
) -> (u32, u32) {
    let max_jobs = if cores <= 1 {
        1
    } else if matches!(size, Magnitude::None | Magnitude::Min) {
        1
    } else if behaves_as_center {
        cores
    } else {
        (cores / 2).max(1)
    };
    (max_jobs, 0)
}

pub struct NodeProjection<'a> {
    pub name: NodeName,
    pub cluster: &'a ClusterName,
    pub trust: Magnitude,
    pub resolved_arch: Arch,
}

impl NodeProposal {
    /// Project a single node from the proposal. Viewpoint-only fields
    /// are left as `None`; call `Node::fill_viewpoint` afterwards on
    /// the viewpoint node to populate them.
    pub fn project(&self, ctx: NodeProjection<'_>) -> Node {
        let criome_domain_name = CriomeDomainName::for_node(&ctx.name, ctx.cluster);

        let nix_pub_key = self.pub_keys.nix.clone();
        let ygg_entry = self.pub_keys.yggdrasil.as_ref();
        let ygg_pub_key = ygg_entry.map(|e| e.pub_key.clone());
        let ygg_address = ygg_entry.map(|e| e.address.clone());
        let ygg_subnet = ygg_entry.map(|e| e.subnet.clone());

        let has_nix_pub_key = nix_pub_key.is_some();
        let has_ygg_pub_key = ygg_pub_key.is_some();
        let has_ssh_pub_key = true; // ssh is required in the proposal schema
        let has_wireguard_pub_key = self.wireguard_pub_key.is_some();
        let has_nordvpn_pub_key = self.nordvpn;
        let has_wifi_cert_pub_key = self.wifi_cert;
        let has_base_pub_keys = has_nix_pub_key && has_ygg_pub_key && has_ssh_pub_key;

        let is_fully_trusted = matches!(ctx.trust, Magnitude::Max);
        let sized_at_least = self.size.ladder();

        let type_is = TypeIs::from_species(self.species);
        let io_disks_empty = self.io.disks.is_empty();
        let behaves_as = BehavesAs::derive(&type_is, &self.machine, io_disks_empty);

        let is_builder = !type_is.edge
            && is_fully_trusted
            && (sized_at_least.at_least_med || behaves_as.center)
            && has_base_pub_keys;
        let is_dispatcher = !behaves_as.center && is_fully_trusted && sized_at_least.at_least_min;
        let is_nix_cache = behaves_as.center && sized_at_least.at_least_min && has_base_pub_keys;
        let is_large_edge = sized_at_least.at_least_large && behaves_as.edge;
        let enable_network_manager = sized_at_least.at_least_min
            && !behaves_as.iso
            && !behaves_as.center
            && !behaves_as.router;
        let has_video_output = behaves_as.edge;

        let handle_lid_switch = if behaves_as.center {
            LidSwitchAction::Ignore
        } else {
            LidSwitchAction::Suspend
        };
        let handle_lid_switch_external_power = if behaves_as.center {
            LidSwitchAction::Ignore
        } else if behaves_as.low_power {
            LidSwitchAction::Suspend
        } else {
            LidSwitchAction::Lock
        };
        let handle_lid_switch_docked = if behaves_as.edge {
            LidSwitchAction::Lock
        } else {
            LidSwitchAction::Ignore
        };

        let nix_pub_key_line = nix_pub_key.as_ref().map(|k| k.line(&criome_domain_name));
        let nix_cache_domain = if is_nix_cache {
            Some(criome_domain_name.nix_subdomain())
        } else {
            None
        };
        let nix_url = nix_cache_domain.as_ref().map(|d| format!("http://{d}"));

        let ssh_pub_key = self.pub_keys.ssh.clone();
        let ssh_pub_key_line = ssh_pub_key.line();

        let chip_is_intel = ctx.resolved_arch.is_intel();
        let (max_jobs, build_cores) =
            nix_concurrency(self.machine.cores, behaves_as.center, self.size);
        let model_is_thinkpad = self
            .machine
            .model
            .as_ref()
            .is_some_and(|m| THINKPAD_MODELS.contains(&m.as_str()));

        let mut machine = self.machine.clone();
        machine.arch = Some(ctx.resolved_arch);

        let link_local_ips = self.link_local_ips.iter().map(|l| l.render()).collect();

        Node {
            name: ctx.name,
            species: self.species,
            size: self.size.ladder(),
            trust: ctx.trust.ladder(),
            machine,
            link_local_ips,
            node_ip: self.node_ip.clone(),
            wireguard_pub_key: self.wireguard_pub_key.clone(),
            nordvpn: self.nordvpn,
            wifi_cert: self.wifi_cert,
            wants_printing: self.wants_printing,
            wants_hw_video_accel: self.wants_hw_video_accel,

            criome_domain_name,
            system: ctx.resolved_arch.system(),
            max_jobs,
            build_cores,

            ssh_pub_key,
            nix_pub_key,
            ygg_pub_key,
            ygg_address,
            ygg_subnet,

            is_fully_trusted,
            is_builder,
            is_dispatcher,
            is_nix_cache,
            is_large_edge,
            enable_network_manager,
            has_nix_pub_key,
            has_ygg_pub_key,
            has_ssh_pub_key,
            has_wireguard_pub_key,
            has_nordvpn_pub_key,
            has_wifi_cert_pub_key,
            has_base_pub_keys,
            has_video_output,
            chip_is_intel,
            model_is_thinkpad,

            ssh_pub_key_line,
            nix_pub_key_line,
            nix_cache_domain,
            nix_url,

            behaves_as,
            type_is,

            handle_lid_switch,
            handle_lid_switch_external_power,
            handle_lid_switch_docked,

            io: None,
            use_colemak: None,
            computer_is: None,
            builder_configs: None,
            cache_urls: None,
            ex_nodes_ssh_pub_keys: None,
            dispatchers_ssh_pub_keys: None,
            admin_ssh_pub_keys: None,
            wireguard_untrusted_proxies: None,
        }
    }
}

pub struct ViewpointFill<'a> {
    pub proposal_io: Io,
    pub all_nodes: &'a BTreeMap<NodeName, Node>,
    pub all_users: &'a BTreeMap<UserName, User>,
    pub wireguard_untrusted_proxies: Vec<WireguardProxy>,
}

use crate::name::UserName;

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
            .filter(|n| n.is_builder)
            .map(|n| BuilderConfig::from_node(n))
            .collect();

        let cache_urls: Vec<String> = ex_nodes
            .iter()
            .filter_map(|n| n.nix_url.clone())
            .collect();

        let ex_nodes_ssh_pub_keys: Vec<SshPubKeyLine> =
            ex_nodes.iter().map(|n| n.ssh_pub_key_line.clone()).collect();

        let dispatchers_ssh_pub_keys: Vec<SshPubKeyLine> = ex_nodes
            .iter()
            .filter(|n| n.is_dispatcher)
            .map(|n| n.ssh_pub_key_line.clone())
            .collect();

        // adminSshPubKeys: for each user with trust=Max, walk their pubKeys; for each
        // entry whose node is fully trusted, take that ssh line. Dedup preserving order.
        let mut admin_ssh_pub_keys: Vec<SshPubKeyLine> = Vec::new();
        for user in fill.all_users.values().filter(|u| u.trust.at_least_max) {
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

        self.io = Some(fill.proposal_io);
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

/// Resolve a machine's arch — concrete if specified, otherwise looked up
/// from its super-node's arch (single hop; no chained pods).
pub(crate) fn resolve_arch(
    name: &NodeName,
    machine: &Machine,
    proposals: &BTreeMap<NodeName, NodeProposal>,
) -> Result<Arch> {
    if let Some(a) = machine.arch {
        return Ok(a);
    }
    let super_name = machine
        .super_node
        .as_ref()
        .ok_or_else(|| Error::UnresolvableArch(name.clone()))?;
    let super_proposal = proposals
        .get(super_name)
        .ok_or_else(|| Error::MissingSuperNode(name.clone(), super_name.clone()))?;
    super_proposal
        .machine
        .arch
        .ok_or_else(|| Error::UnresolvableArch(name.clone()))
}
