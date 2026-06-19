//! Output `Node`: per-node view with every computed field present.
//!
//! Two kinds of fields:
//! - **Always-derived**: present on every `Node` (viewpoint and ex-nodes).
//! - **Viewpoint-only**: `Some` on `horizon.node`, `None` on entries
//!   in `horizon.ex_nodes`. Filled by `Node::fill_viewpoint`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::address::{LinkLocalAddress, NodeIp, YggAddress, YggSubnet};
use crate::domain::{DomainConfiguration, NodeDomainContext};
use crate::error::{Error, Result};
use crate::io::Io;
use crate::machine::Machine;
use crate::magnitude::{AtLeast, Magnitude};
use crate::name::{ClusterName, CriomeDomainName, ModelName, NodeName, UserName};
use crate::proposal::{
    NodeProposal, NodeService, NodeServiceKind, RouterInterfaces, VmHostCapability, WireguardProxy,
};
use crate::pub_key::{
    NixPubKey, NixPubKeyLine, SshPubKey, SshPubKeyLine, WireguardPubKey, YggPubKey,
};
use crate::species::{Arch, KnownModel, NodeSpecies, System};
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
    /// Router interface roles for router nodes. `None` for non-router
    /// nodes and for invalid proposals that validation rejects upstream.
    pub router_interfaces: Option<RouterInterfaces>,
    /// Per-node service roles. Projected from proposal data; never
    /// inferred from the node name.
    pub services: Vec<NodeService>,

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
    pub is_remote_nix_builder: bool,
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
    /// SCOPED image-exchange signing keys: the Nix signing-key lines of
    /// the hosts in this node's declared host-set
    /// (`machine.host_set()` = `{super_node} ∪ super_nodes`). These are
    /// the only hosts permitted to hold and exchange this node's image,
    /// so the trust edge they form is tighter than the cluster-wide
    /// signing-key pool (`Cluster.trusted_build_pub_keys`): a non-co-host
    /// node's key is absent. CriomOS emits these as
    /// `extra-trusted-public-keys` scoped to the co-hosting hosts in a
    /// later unit. `Some(empty)` for a non-Pod (no host-set) or a host
    /// whose hosts carry no signing key; the order follows the host-set
    /// (primary first).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image_exchange_pub_keys: Option<Vec<NixPubKeyLine>>,
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
    /// The guest's own lean test-VM profile gate. True only for a
    /// `NodeSpecies::TestVm` node. CriomOS gates the guest's minimal
    /// config on this facet; it is orthogonal to `virtual_machine`
    /// (which flags "runs on a host" for the host's substrate wiring).
    pub test_vm: bool,
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
    pub test_vm: bool,
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
            test_vm: matches!(s, NodeSpecies::TestVm),
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
        // A TestVm derives a deliberately lean profile: it carries
        // `test_vm` plus the `virtual_machine` it already gets from its
        // Pod substrate, and nothing else. It is NOT edge/center/router
        // — those facets stay false purely because `NodeSpecies::TestVm`
        // sets none of the `type_is` flags they read from, keeping the
        // guest config minimal.
        let test_vm = type_is.test_vm;
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
            test_vm,
        }
    }

    /// systemd-logind lid-switch policy derived from the node's
    /// behaves-as flags. Centers ignore lid events entirely; edges
    /// lock when docked; otherwise the policy depends on the
    /// power state.
    fn lid_switch_policy(&self) -> LidSwitchPolicy {
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

struct LidSwitchPolicy {
    on_battery: LidSwitchAction,
    on_external_power: LidSwitchAction,
    docked: LidSwitchAction,
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

pub struct NodeProjection<'a> {
    pub name: NodeName,
    pub cluster: &'a ClusterName,
    pub domain_configuration: &'a DomainConfiguration,
    pub trust: Magnitude,
    pub resolved_arch: Arch,
}

impl NodeProposal {
    /// Project a single node from the proposal. Viewpoint-only fields
    /// are left as `None`; call `Node::fill_viewpoint` afterwards on
    /// the viewpoint node to populate them.
    pub fn project(&self, ctx: NodeProjection<'_>) -> Node {
        let criome_domain_name = ctx
            .domain_configuration
            .criome_domain_name(NodeDomainContext {
                node: &ctx.name,
                cluster: ctx.cluster,
            });

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

        let online = self.online.unwrap_or(true);
        let is_remote_nix_builder = self.has_service(NodeServiceKind::NixBuilder)
            && online
            && is_fully_trusted
            && has_base_pub_keys;
        let is_dispatcher = !behaves_as.center && is_fully_trusted && sized_at_least.min;
        let is_nix_cache = self.has_service(NodeServiceKind::NixCache)
            && online
            && is_fully_trusted
            && has_base_pub_keys;
        let is_large_edge = sized_at_least.large && behaves_as.edge;
        let enable_network_manager =
            sized_at_least.min && !behaves_as.iso && !behaves_as.center && !behaves_as.router;
        let has_video_output = behaves_as.edge;

        let lid_policy = behaves_as.lid_switch_policy();

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
        // NixBuilder's optional capacity knob drives both
        // `nix.buildMachines.<n>.maxJobs` from dispatcher viewpoints
        // and the local dedicated-builder `nix.settings.cores` value.
        // Absence means single-job-at-a-time.
        let max_jobs = self.nix_builder_maximum_jobs().unwrap_or(1);
        let build_cores = max_jobs;
        let model_is_thinkpad = self
            .machine
            .model
            .as_ref()
            .and_then(ModelName::known)
            .is_some_and(KnownModel::is_thinkpad);

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
            router_interfaces: self.router_interfaces.clone(),
            services: self.services.clone(),

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
            is_remote_nix_builder,
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

            handle_lid_switch: lid_policy.on_battery,
            handle_lid_switch_external_power: lid_policy.on_external_power,
            handle_lid_switch_docked: lid_policy.docked,

            io: None,
            use_colemak: None,
            computer_is: None,
            builder_configs: None,
            cache_urls: None,
            ex_nodes_ssh_pub_keys: None,
            dispatchers_ssh_pub_keys: None,
            admin_ssh_pub_keys: None,
            image_exchange_pub_keys: None,
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

        let cache_urls: Vec<String> = ex_nodes.iter().filter_map(|n| n.nix_url.clone()).collect();

        let ex_nodes_ssh_pub_keys: Vec<SshPubKeyLine> = ex_nodes
            .iter()
            .map(|n| n.ssh_pub_key_line.clone())
            .collect();

        let dispatchers_ssh_pub_keys: Vec<SshPubKeyLine> = ex_nodes
            .iter()
            .filter(|n| n.is_dispatcher)
            .map(|n| n.ssh_pub_key_line.clone())
            .collect();

        // imageExchangePubKeys: the SCOPED image-exchange trust edge.
        // The keys of exactly this node's declared host-set
        // (`{super_node} ∪ super_nodes`) — the hosts permitted to hold
        // and exchange this node's image. Looked up in the full node map,
        // mapped to each host's signing-key line, in host-set order
        // (primary first). This is tighter than the cluster-wide
        // signing-key pool (`Cluster.trusted_build_pub_keys`): only
        // co-hosting hosts appear, so a non-co-host node's key is absent.
        // Empty for a single-host node whose one host carries no signing
        // key, and for a non-Pod (whose host-set is empty).
        let image_exchange_pub_keys: Vec<NixPubKeyLine> = self
            .machine
            .host_set()
            .iter()
            .filter_map(|host| fill.all_nodes.get(host))
            .filter_map(|host_node| host_node.nix_pub_key_line.clone())
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

        self.io = Some(fill.proposal_io);
        self.use_colemak = Some(use_colemak);
        self.computer_is = Some(computer_is);
        self.builder_configs = Some(builder_configs);
        self.cache_urls = Some(cache_urls);
        self.ex_nodes_ssh_pub_keys = Some(ex_nodes_ssh_pub_keys);
        self.dispatchers_ssh_pub_keys = Some(dispatchers_ssh_pub_keys);
        self.admin_ssh_pub_keys = Some(admin_ssh_pub_keys);
        self.image_exchange_pub_keys = Some(image_exchange_pub_keys);
        self.wireguard_untrusted_proxies = Some(fill.wireguard_untrusted_proxies);
    }

    /// This node's `VmHost` capability, if it declares one. Exposes the
    /// cluster-authored tap subnet, KVM availability, and capacity
    /// ceiling on the projection so the VM-test generator reads them
    /// off `horizon.node.services` exactly as the guest fold reads its
    /// facts off `horizon.ex_nodes`.
    pub fn vm_host_capability(&self) -> Option<VmHostCapability<'_>> {
        self.services.iter().find_map(NodeService::vm_host)
    }
}

impl NodeProposal {
    /// Resolve this proposal's machine arch — concrete if specified,
    /// otherwise looked up from the super-node's arch (single hop;
    /// no chained pods). `name` identifies this proposal in the
    /// surrounding map for error reporting.
    pub fn resolve_arch(
        &self,
        name: &NodeName,
        proposals: &BTreeMap<NodeName, NodeProposal>,
    ) -> Result<Arch> {
        if let Some(a) = self.machine.arch {
            return Ok(a);
        }
        let super_name = self
            .machine
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

    /// Invariant: a `Pod` machine substrate must name a host-set whose
    /// EVERY member exists in the cluster. The host→guest graph is total
    /// — no test-VM guest may name a host that isn't there. The host-set
    /// is `{super_node} ∪ super_nodes` (`Machine::host_set`), so this
    /// covers both the primary host and every additional declared host.
    /// Checked independently of arch resolution: `resolve_arch` returns
    /// early when `machine.arch` is explicit and so never reaches the
    /// existence check, but a Pod with an explicit arch and an absent
    /// host is still a broken cluster. The first missing host (primary
    /// first, then additional in declared order) reports
    /// `Error::MissingSuperNode`. `name` identifies this proposal for
    /// error reporting.
    pub fn validate_pod_super_node(
        &self,
        name: &NodeName,
        proposals: &BTreeMap<NodeName, NodeProposal>,
    ) -> Result<()> {
        if !matches!(self.machine.species, crate::species::MachineSpecies::Pod) {
            return Ok(());
        }
        let host_set = self.machine.host_set();
        if host_set.is_empty() {
            // A Pod with neither a primary super_node nor any additional
            // hosts cannot resolve its arch or its host — the same broken
            // shape `resolve_arch` rejects.
            return Err(Error::UnresolvableArch(name.clone()));
        }
        for host in host_set {
            if !proposals.contains_key(host) {
                return Err(Error::MissingSuperNode(name.clone(), host.clone()));
            }
        }
        Ok(())
    }

    /// Invariant: every host in this Pod's declared host-set must resolve
    /// to the SAME architecture. A guest image is one closure; every host
    /// permitted to hold and exchange it must share its arch, or the
    /// image cannot run there. The primary host's arch is the reference
    /// (arch resolution itself stays the single hop on `super_node`); any
    /// additional host whose arch diverges fails with
    /// `Error::HostSetArchMismatch`. A no-op for the single-host majority
    /// (a one-element host-set is trivially uniform) and for non-Pods.
    /// Call after `validate_pod_super_node` has proven every host exists.
    /// `name` identifies this proposal for error reporting.
    pub fn validate_host_set_single_arch(
        &self,
        name: &NodeName,
        proposals: &BTreeMap<NodeName, NodeProposal>,
    ) -> Result<()> {
        if !matches!(self.machine.species, crate::species::MachineSpecies::Pod) {
            return Ok(());
        }
        let mut reference: Option<(&NodeName, Arch)> = None;
        for host in self.machine.host_set() {
            let host_proposal = proposals
                .get(host)
                .ok_or_else(|| Error::MissingSuperNode(name.clone(), host.clone()))?;
            let host_arch = host_proposal.resolve_arch(host, proposals)?;
            match reference {
                None => reference = Some((host, host_arch)),
                Some((first_host, first_arch)) if first_arch != host_arch => {
                    return Err(Error::HostSetArchMismatch {
                        node: name.clone(),
                        first_host: first_host.clone(),
                        first_arch,
                        second_host: host.clone(),
                        second_arch: host_arch,
                    });
                }
                Some(_) => {}
            }
        }
        Ok(())
    }
}
