//! Cluster-level identity and roll-ups in the projected view.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::name::{ClusterDomain, ClusterName};
use crate::proposal::ai::AiProvider;
use crate::proposal::network::{LanNetwork, ResolverPolicy};
use crate::proposal::secret::{SecretBackend, SecretName};
use crate::proposal::services::TailnetConfig;
use crate::proposal::vpn::VpnProfile;
use crate::pub_key::NixPubKeyLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub name: ClusterName,
    pub domain: ClusterDomain,
    /// One entry per node that has a nix signing key.
    pub trusted_build_pub_keys: Vec<NixPubKeyLine>,
    /// Cluster LAN policy (subnet, gateway, DHCP pool, lease policy)
    /// passed through from the proposal. `None` means CriomOS
    /// modules use their current implementation defaults.
    pub lan: Option<LanNetwork>,
    /// Cluster DNS-resolver policy (upstreams, fallbacks, listen
    /// addresses) passed through from the proposal. `None` means
    /// CriomOS modules use their current implementation defaults.
    pub resolver: Option<ResolverPolicy>,
    /// Cluster tailnet configuration (base DNS domain plus optional
    /// CA-trust material). Required when any node hosts a tailnet
    /// controller; validated at projection time. `None` means the
    /// cluster has no tailnet.
    pub tailnet: Option<TailnetConfig>,
    /// AI providers the cluster advertises. Empty means the cluster
    /// has no AI inference endpoints; consumers (pi-models, future
    /// task routers) gate on `aiProviders != []`.
    pub ai_providers: Vec<AiProvider>,
    /// VPN provider profiles (NordVPN, future WireguardMesh). Empty
    /// means the cluster has no VPN catalog; CriomOS nordvpn.nix
    /// is inert when there is no Nordvpn variant in this list AND
    /// `node.nordvpn` is false.
    pub vpn_profiles: Vec<VpnProfile>,
    /// Resolved lookup table: logical `SecretName` → concrete
    /// `SecretBackend`. Projected from the proposal-side
    /// `Vec<ClusterSecretBinding>` so consumers can dispatch on the
    /// backend variant in O(log n) without reconstructing a map.
    ///
    /// The proposal carries an authored list (ordered, duplicates
    /// caught at projection time); the view exposes the resolution
    /// table shape that every consumer needs ("given this name, what
    /// backend?"). The Nix side reads it as a positional attrset:
    /// `horizon.cluster.secretBindings.${secretRef.name}`.
    ///
    /// Empty when the cluster has no secrets bound; consumer modules
    /// loud-fail at activation if any node-level `SecretReference`
    /// names a key absent from this map.
    pub secret_bindings: BTreeMap<SecretName, SecretBackend>,
}
