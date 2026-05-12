//! Node placement — how and where a node exists.
//!
//! Distinct from `NodeSpecies` (what the node is for) and
//! `NodeCapabilities` (what the node can provide). A node has exactly
//! one placement; placement names the substrate, host, and isolation
//! shape the node lives in.
//!
//! Spec: `reports/system-assistant/04-dedicated-cloud-host-plan-second-revision.md`
//! §P1.1 (typed placement, no `WorkloadSubstrate`) and report 05
//! (workload is always native NixOS — no axis to choose along).

//! Wire format: `NodePlacement` and `UserNamespacePolicy` are `NotaSum`s
//! (head-identifier dispatch on the variant name). `UserNamespacePolicy::
//! PrivateUsersPick {}` is an empty struct variant rather than a unit
//! variant so it fits `NotaSum`'s shape (unit variants belong on
//! `NotaEnum`, not on a sum-with-data enum). Construct as
//! `UserNamespacePolicy::PrivateUsersPick {}` and match as
//! `UserNamespacePolicy::PrivateUsersPick {}`.

// nota-codec's `NexusVerb` derive is the "head-identifier dispatched
// sum-with-data" macro. It was renamed to `NotaSum` in nota-codec
// commit adbdb6f; this file uses the older name because horizon-rs's
// pinned nota-codec rev (333e73a) predates that rename. TODO: rename
// when horizon-rs bumps nota-codec to >= adbdb6f.
use nota_codec::{NexusVerb, NotaEnum, NotaRecord, NotaTransparent};
use serde::{Deserialize, Serialize};

use crate::magnitude::AtLeast;
use crate::name::{ModelName, NodeName, UserName};
use crate::species::{Arch, MotherBoard};

/// How and where a node exists. Exactly one placement per node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NexusVerb)]
#[serde(rename_all = "camelCase")]
pub enum NodePlacement {
    Metal(Metal),
    Contained(Contained),
}

/// Bare-metal node: physical hardware on its own boot path. The
/// existing `Machine` record covers the same data and stays during
/// the migration cycle for backward compatibility; `Metal`
/// is the new authoritative form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Metal {
    pub arch: Arch,
    #[serde(default)]
    pub model: Option<ModelName>,
    #[serde(default)]
    pub motherboard: Option<MotherBoard>,
    #[serde(default)]
    pub ram_gb: Option<u32>,
}

/// Contained node: lives inside another node via a containment
/// substrate. Carries enough data for the host to materialize the
/// container and route traffic to it without consulting any other
/// records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct Contained {
    pub host: NodeName,
    pub substrate: ContainmentSubstrate,
    pub resources: ContainerResources,
    /// `None` for legacy Pod-shaped proposals that did not author
    /// network details. New proposals should populate this.
    #[serde(default)]
    pub network: Option<ContainerNetwork>,
    /// `None` for legacy Pod-shaped proposals that did not author
    /// state details. New proposals should populate this.
    #[serde(default)]
    pub state: Option<ContainerState>,
    pub trust: AtLeast,
    pub user_namespace_policy: UserNamespacePolicy,
    /// User the contained node runs under on the host, if any. Migrated
    /// from the legacy `Machine.super_user` pod-only field.
    #[serde(default)]
    pub super_user: Option<UserName>,
}

/// Closed set of containment substrates that confer node identity.
///
/// **Identity vs workload.** Only substrates that give the contained
/// thing its own address, keys, and lifecycle are listed here — that
/// is what makes a node a node. OCI containers, plain systemd
/// services, and similar are *workload* implementations that run
/// inside an identity-bearing substrate. Per
/// `skills/nix-discipline.md` §"Services are NixOS modules, not OCI
/// workloads", workloads are always native NixOS; there is no
/// `WorkloadSubstrate` to choose along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, NotaEnum)]
pub enum ContainmentSubstrate {
    /// Declarative `containers.<name>` via `systemd-nspawn`.
    NixosContainer,
    /// `microvm.nix`-managed MicroVM with a real kernel boundary.
    MicroVm,
}

/// User-namespace mapping policy for a contained node. The default in
/// proposals is `PrivateUsersPick`. Host-root mapping (the equivalent
/// of `containers.<name>.privateUsers = false`) is opt-in by data
/// variant: it requires an explicit `HostRootMappingAllowed` with a
/// reason and an approver. Trust level is not the gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NexusVerb)]
#[serde(rename_all = "camelCase")]
pub enum UserNamespacePolicy {
    /// `privateUsers = "pick"` — automatic user-namespace mapping.
    /// Empty struct variant rather than unit so `NotaSum` accepts it
    /// alongside `HostRootMappingAllowed`. Construct as
    /// `UserNamespacePolicy::PrivateUsersPick {}`.
    PrivateUsersPick {},
    /// `privateUsers = false` — container UIDs map to host UIDs.
    /// Required for some workloads but unsafe for public-facing
    /// services. Must declare a reason and an approver.
    HostRootMappingAllowed {
        reason: String,
        approved_by: UserName,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResources {
    pub cores: u32,
    pub ram_gb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ContainerNetwork {
    /// Address inside the container, on the host bridge.
    pub local_address: ContainerLocalAddress,
    /// Host-side address of the bridge interface for this child.
    pub host_address: ContainerLocalAddress,
}

/// IPv4 or IPv6 address used inside the host bridge. Stored as a
/// string newtype during the first cut; promote to a typed address
/// when the address vocabulary in `address.rs` grows a private-range
/// variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, NotaTransparent)]
#[serde(transparent)]
pub struct ContainerLocalAddress(pub(crate) String);

impl ContainerLocalAddress {
    pub fn new(addr: impl Into<String>) -> Self {
        Self(addr.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ContainerLocalAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, NotaRecord)]
#[serde(rename_all = "camelCase")]
pub struct ContainerState {
    /// Paths inside the container that must persist across host
    /// rebuilds. The host materializes these as bind-mounted host
    /// directories, owned by the contained node.
    pub persistent_paths: Vec<String>,
}
