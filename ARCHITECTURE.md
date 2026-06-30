# ARCHITECTURE — horizon-rs

The horizon projection library. Rust types and source files for
nixos modules; linked in-process by
[lojix-cli](https://github.com/LiGoldragon/lojix-cli)'s deploy path.

## Role

When forge materialises a CriomOS configuration, it walks
horizon-rs's projection types to compute the nixos-rebuild
inputs. Today, this is in-process — a Rust dep, not a daemon
boundary.

Detailed design lives in [`docs/DESIGN.md`](docs/DESIGN.md) and
[`docs/BUILD_CORES.md`](docs/BUILD_CORES.md).

## What goes in a `ClusterProposal`

A value belongs in `ClusterProposal` (or any record reachable from
it) only when **all three** answers are yes:

1. **Variability.** Would another cluster owner using this same
   horizon author a different value here?
2. **Authority.** Is the cluster owner the authority on this value
   — not the horizon operator, not CriomOS, not a provider?
3. **Non-derivable.** Does the projection genuinely need to be
   *told* this, rather than computing it from already-authored
   data?

A "no" on any of these means the value lives somewhere else:

| Bucket | Lives in | Examples |
|---|---|---|
| **Cluster fact** | `ClusterProposal` / `NodeProposal` | Node names, trust, hardware, router interface roles, secret references, provider *selections*, regulatory country. |
| **Horizon constant** | pan-horizon authored config or `lib/src/` constants | Internal DNS suffix (`criome`), public DNS suffix (`criome.net`), LAN address pool, reserved subdomain labels. |
| **Horizon derivation** | `lib/src/view/` projection code | Node domain, tailnet base domain, LAN CIDR / gateway / DHCP pool, router SSID, resolver listen addresses. |
| **CriomOS-side** | `CriomOS-lib` constants, CriomOS Nix module defaults, or catalog packages | Service ports, DNS upstream choice, AI runtime config, AI model catalog, NordVPN server catalog, DHCP lease TTL. |

## Service Roles Are Variants

Cluster data selects optional node roles with self-describing
variants, not positional booleans and not records that smuggle
implementation details into the proposal.

Good proposal shape:

```text
[
  TailnetClient
  TailnetController
  NixBuilder { maximum_jobs }
  NixCache
  PersonaDevelopment { capabilities = [GitoliteServer] }
]
```

Bad proposal shape:

```text
NodeServices Client (Server 8443 "tailnet.goldragon.criome") true
```

The variant says *what role the node has*. Horizon may derive
cluster-relative names from that role. CriomOS decides how the role
is implemented: which service package is used, which fixed port it
listens on, which firewall ports open, and which systemd units start.
Those implementation constants do not belong to the cluster owner.

**Smell — "replaces the literals scattered across CriomOS".** When
a new proposal record carries this phrase in its doc comment, the
shape is almost always wrong. The literals were a CriomOS
implementation choice; they belong in CriomOS defaults (or a
CriomOS Nix package for catalog data). What moves to horizon is
the *projection that derives the value* — not the literal itself.
A proposal record that transcribes the literals onto the cluster
authoring surface makes the cluster owner author the operating
system.

**Smell — composite that fails the rule.** A field can fail the
rule for half its content while passing it for the other half. An
"AI provider" entry is a cluster *selection* (`{ name,
serving_node, profile_ref, credentials_ref }`) **plus** a
CriomOS-side *implementation* (protocol, port, base path, model
catalog, runtime config). The selection authors per cluster; the
implementation does not. Split composites along the bucket
boundary.

The full audit driving this rule lives in primary's
`reports/designer/207-horizon-boundary-audit-and-lean-down-plan-2026-05-17.md`;
the brainstorm for the pan-horizon authored config is in
`reports/designer/208-pan-horizon-configuration-brainstorm-2026-05-17.md`.

## Node I/O policy is cluster-authored

Node I/O policy is cluster-authored when it is hardware/safety inventory.
Filesystems, swap devices, swapfile sizing, and compressed-swap sizing are
projected through Horizon so CriomOS can render them — into NixOS swap/zram
options — without node-name predicates. The cluster owner authors the
hardware/safety facts; CriomOS renders them.

## VM hosting is cluster-data-generated

The test-VM host carries an explicit `NodeService::VmHost` role — VM testing
is cluster-data-generated, not cluster-specific. A host that runs test VMs
declares a `VmHost` service (sibling to `NixBuilder`) carrying the
cluster-authored host data the VM-test generator reads: the guest tap subnet
(one sliced `TapSubnet` CIDR), KVM availability, and a maximum-guests ceiling.
This replaces the bespoke hardcoded `169.254.100+index.1` subnet and `inputs ?
microvm` probe invented in the Nix layer, giving the predictable interface a
readable OS/home-profile test suite is built on. The host→guest graph is
total: a `Pod` substrate must name a `super_node` that exists in the cluster
(`Error::MissingSuperNode`). This follows the recorded principle in primary's
`reports/cloud-designer/50-general-vm-testing-interface/intent-capture.md`.

A test-VM node may declare **multiple** vmhosts; the declared host-set is the
scoped image-exchange trust boundary. Beyond the primary `super_node`, a Pod
may carry an additive `super_nodes` tail — `Machine::host_set()` = `{super_node}
∪ super_nodes`, deduped, primary first; an empty `super_nodes` is the
single-host majority, unchanged. The host→guest existence invariant extends to
every host in the set, and a single-arch invariant requires every host to share
one architecture (a guest image is one closure; `Error::HostSetArchMismatch`).
The co-hosting hosts — and only they — trust each other's Nix signing keys for
that node's image: the projection derives a scoped `image_exchange_pub_keys` on
the output `Node` from the host-set, tighter than the cluster-wide
`Cluster.trusted_build_pub_keys` pool. A non-co-host node's key is absent.
CriomOS emits these as scoped `extra-trusted-public-keys` in a later unit. This
follows primary's report 54
(`reports/cloud-designer/54-lojix-test-op/4-proposal.md`, psyche decisions A
additive + B scoped).

## Boundaries

Owns:

- Projection types and projection helpers.
- A small CLI (under `cli/`) for ad-hoc projection.

Does not own:

- The deploy pipeline — that's
  [lojix-cli](https://github.com/LiGoldragon/lojix-cli).
- The nixos-rebuild driver — also forge.
- Sema records — though horizon-rs's role may eventually be
  absorbed into a records-authored projection over sema.

## Status

CANON. Active. Long-term: parts may migrate into forge's
in-process actors when the forge family unifies.

## Cross-cutting context

- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
