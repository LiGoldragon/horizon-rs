# INTENT — horizon-rs

*What the psyche has explicitly intended for this project.
Synthesised from psyche statements and applicable workspace
constraints; not embellished. `ARCHITECTURE.md` says what
horizon-rs IS; this file says what the psyche wants it to BE.*

## Purpose

`horizon-rs` is the horizon projection library: the Rust types
and projection logic that turn a cluster proposal into the
per-`(cluster, node)` view (the "horizon") that CriomOS consumes
to build per-node OS configurations. It is the single source of
truth for the typed proposal schema. Today it is linked in-process
by the deploy path (`lojix-cli`, and the new `lojix` daemon);
long-term parts may migrate into forge's in-process actors.

## Constraints — the proposal boundary

- **A value belongs in `ClusterProposal` only when all three are
  true: variability, authority, non-derivable.** Would another
  cluster owner author a different value? Is the cluster owner the
  authority on it (not the horizon operator, not CriomOS, not a
  provider)? Does the projection genuinely need to be *told* it
  rather than computing it? A "no" on any of these sends the value
  elsewhere — a horizon constant, a horizon derivation, or a
  CriomOS-side default.
- **Service roles are self-describing variants, not positional
  booleans or smuggled implementation details.** Cluster data
  selects optional node roles (`TailnetController`, `NixBuilder {
  maximum_jobs }`, `PersonaDevelopment { capabilities }`); horizon
  may derive cluster-relative names from the role; CriomOS decides
  how the role is implemented (which package, port, firewall rule,
  systemd unit).
- **The cluster owner must not be made to author the operating
  system.** A proposal record whose doc comment says it "replaces
  the literals scattered across CriomOS" is almost always the
  wrong shape: the literals were a CriomOS implementation choice
  and belong in CriomOS defaults. What moves to horizon is the
  *projection that derives the value*, never the literal itself.
- **Split composites along the bucket boundary.** A field that is
  half cluster-selection and half CriomOS-implementation (e.g. an
  "AI provider") is split: the selection authors per cluster; the
  implementation does not.
- **Node I/O policy is cluster-authored when it is hardware/safety
  inventory.** Filesystems, swap devices, swapfile sizing, and
  compressed-swap sizing are projected through Horizon so CriomOS can
  render them without node-name predicates.
- **The test-VM host carries an explicit `NodeService::VmHost` role —
  VM testing is cluster-data-generated, not cluster-specific.** A host
  that runs test VMs declares a `VmHost` service (sibling to
  `NixBuilder`) carrying the cluster-authored host data the VM-test
  generator reads: the guest tap subnet (one sliced `TapSubnet` CIDR),
  KVM availability, and a maximum-guests ceiling. This replaces the
  bespoke hardcoded `169.254.100+index.1` subnet and `inputs ? microvm`
  probe invented in the Nix layer, giving the predictable interface a
  readable OS/home-profile test suite is built on. The host→guest graph
  is total: a `Pod` substrate must name a `super_node` that exists in
  the cluster (`Error::MissingSuperNode`). Per the recorded principle in
  `primary/reports/cloud-designer/50-general-vm-testing-interface/intent-capture.md`.

## Naming and stack discipline

- Full English words for every identifier; no crate-name prefix on
  types. Per `primary/skills/naming.md`.
- Projection types and helpers are methods on data-bearing nouns,
  not free functions; reach for `impl From` for conversions. Per
  `primary/skills/rust-discipline.md` and
  `primary/skills/rust/methods.md`.

## Scope — today, not eventually

horizon-rs is CANON and active on today's stack. horizon-rs's role
may eventually be absorbed into a records-authored projection over
Sema; today it is a Rust dependency, not a daemon boundary. Per
`primary/ESSENCE.md` §"Today and eventually".

*Source statements live in Spirit intent records, the project's
`ARCHITECTURE.md`, and primary's horizon-boundary audit reports.
Workspace-shape intent stays in `primary/INTENT.md` and the named
skills above.*
