//! Proposal-side `NodeProposal` — the per-node input shape goldragon
//! emits.
//!
//! `NodeProposal::project` is the constructor for `view::Node`;
//! `NodeProposal::resolve_arch` resolves a pod's arch via its
//! super-node when not explicitly set.

use std::collections::BTreeMap;

use nota_codec::NotaRecord;
use serde::{Deserialize, Serialize};

use crate::address::{LinkLocalIp, NodeIp};
use crate::error::{Error, Result};
use crate::magnitude::Magnitude;
use crate::name::{ClusterDomain, ClusterName, CriomeDomainName, ModelName, NodeName};
use crate::proposal::io::Io;
use crate::proposal::machine::Machine;
use crate::proposal::placement::NodePlacement;
use crate::proposal::pub_keys::NodePubKeys;
use crate::proposal::router::RouterInterfaces;
use crate::proposal::services::NodeServices;
use crate::proposal::wireguard::WireguardProxy;
use crate::pub_key::WireguardPubKey;
use crate::species::{Arch, KnownModel, NodeSpecies};
use crate::view;

#[derive(Debug, Clone, Serialize, Deserialize, NotaRecord)]
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

    /// `nix.buildMachines.<this>.maxJobs` from each dispatcher's
    /// viewpoint when this node acts as a remote builder; also drives
    /// `nix.settings.build-cores` locally on the node itself. `None`
    /// (= default `Some(1)`) means single-job-at-a-time, matching
    /// nix's default. Bump this up on large builders to unlock
    /// parallel dispatch.
    #[serde(default)]
    pub number_of_build_cores: Option<u32>,

    /// Per-node service roles. This is cluster role data: consumers must
    /// not infer it from node names.
    #[serde(default)]
    pub services: NodeServices,
    pub placement: NodePlacement,
}

pub struct NodeProjection<'a> {
    pub name: NodeName,
    pub cluster: &'a ClusterName,
    pub cluster_domain: &'a ClusterDomain,
    pub trust: Magnitude,
    pub resolved_arch: Arch,
}

impl NodeProposal {
    /// Project a single node from the proposal. Viewpoint-only fields
    /// are left as `None`; call `view::Node::fill_viewpoint` afterwards
    /// on the viewpoint node to populate them.
    pub fn project(&self, ctx: NodeProjection<'_>) -> view::Node {
        let criome_domain_name =
            CriomeDomainName::for_node(&ctx.name, ctx.cluster, ctx.cluster_domain);

        let nix_pub_key = self.pub_keys.nix.clone();
        // Step 14: yggdrasil presence travels as one typed sub-record on
        // the view side rather than three sibling fields; consumers gate
        // on `node.yggdrasil != null`.
        let yggdrasil = self.pub_keys.yggdrasil.clone();

        // Step 7b: has_*_pub_key sibling fields deleted. Where derivation
        // logic in this projector still needs the boolean form, compute
        // it locally (this stays Rust-side only). Consumers gate on the
        // underlying typed field directly: `node.nixPubKey != null`,
        // `node.yggdrasil != null`, `node.wireguardPubKey != null`,
        // `node.nordvpn`, `node.wifiCert`. SSH is required at the proposal
        // schema (see proposal/pub_keys.rs:NodePubKeys.ssh, non-optional);
        // the old `has_ssh_pub_key` was always true and was deleted in step 14.
        let has_base_pub_keys = nix_pub_key.is_some() && yggdrasil.is_some();

        let is_fully_trusted = matches!(ctx.trust, Magnitude::Max);
        let sized_at_least = self.size.ladder();

        let io_disks_empty = self.io.disks.is_empty();

        // Machine carries `arch: Option<Arch>`; on the input side a pod
        // node may leave it None (the projection resolves it from the
        // host); on the projected node it's always Some.
        let mut machine = self.machine.clone();
        machine.arch = Some(ctx.resolved_arch);

        let behaves_as = view::BehavesAs::derive(self.species, &self.placement, io_disks_empty);

        let online = self.online.unwrap_or(true);
        // Strict-Edge gate: only the literal `Edge` species is excluded
        // from acting as a remote nix builder. `EdgeTesting` and `Hybrid`
        // also `behaves_as.edge` but remain eligible builders.
        let is_remote_nix_builder = online
            && !matches!(self.species, NodeSpecies::Edge)
            && is_fully_trusted
            && (sized_at_least.medium || behaves_as.center)
            && has_base_pub_keys;
        let is_dispatcher = !behaves_as.center && is_fully_trusted && sized_at_least.min;
        let is_nix_cache = behaves_as.center && sized_at_least.min && has_base_pub_keys;
        let is_large_edge = sized_at_least.large && behaves_as.edge;
        let enable_network_manager =
            sized_at_least.min && !behaves_as.iso && !behaves_as.center && !behaves_as.router;

        let nix_pub_key_line = nix_pub_key.as_ref().map(|k| k.line(&criome_domain_name));
        let nix_cache = if is_nix_cache {
            let domain = criome_domain_name.nix_subdomain();
            let url = format!("http://{domain}");
            Some(view::NixCache { domain, url })
        } else {
            None
        };

        let ssh_pub_key = self.pub_keys.ssh.clone();
        let ssh_pub_key_line = ssh_pub_key.line();

        let chip_is_intel = ctx.resolved_arch.is_intel();
        // Per-node `number_of_build_cores` from the datom drives both
        // `nix.buildMachines.<n>.maxJobs` (when this node acts as a
        // remote builder) and `nix.settings.build-cores` locally on the
        // node itself — one number, one wire field. `None` defaults to
        // 1 — matches nix's out-of-the-box single-job-at-a-time and
        // keeps the wire backward-compat with datoms that don't set
        // the field.
        let max_jobs = self.number_of_build_cores.unwrap_or(1);
        let model_is_thinkpad = self
            .machine
            .model
            .as_ref()
            .and_then(ModelName::known)
            .is_some_and(KnownModel::is_thinkpad);

        let link_local_ips = self.link_local_ips.iter().map(|l| l.render()).collect();

        view::Node {
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
            placement: self.placement.clone(),

            criome_domain_name,
            system: ctx.resolved_arch.system(),
            max_jobs,

            ssh_pub_key,
            nix_pub_key,
            yggdrasil,

            is_fully_trusted,
            is_remote_nix_builder,
            is_dispatcher,
            is_large_edge,
            enable_network_manager,
            chip_is_intel,
            model_is_thinkpad,

            ssh_pub_key_line,
            nix_pub_key_line,
            nix_cache,

            behaves_as,

            io: None,
            use_colemak: None,
            builder_configs: None,
            cache_urls: None,
            ex_nodes_ssh_pub_keys: None,
            dispatchers_ssh_pub_keys: None,
            admin_ssh_pub_keys: None,
            wireguard_untrusted_proxies: None,
        }
    }

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
        let host = match &self.placement {
            NodePlacement::Metal {} => {
                return Err(Error::UnresolvableArch(name.clone()));
            }
            NodePlacement::Contained { host, .. } => host,
        };
        let host_proposal = proposals
            .get(host)
            .ok_or_else(|| Error::MissingSuperNode(name.clone(), host.clone()))?;
        host_proposal
            .machine
            .arch
            .ok_or_else(|| Error::UnresolvableArch(name.clone()))
    }
}
