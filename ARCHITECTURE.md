# ARCHITECTURE — horizon-rs

Typed schema and projection for criome cluster horizons. Reads an
authored `ClusterProposal` (NOTA-encoded `datom.nota`), validates it,
and produces a viewpoint-scoped `view::Horizon` whose JSON wire form
is consumed by Nix modules in CriomOS / CriomOS-home (via
`inputs.horizon`) and by `lojix-daemon` over the in-process
`horizon-lib` dependency.

## What goes in a `ClusterProposal` — the boundary rule

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
| **Cluster fact** | `ClusterProposal` / `NodeProposal` | node names, trust, hardware, secret references, provider *selections*, regulatory country |
| **Horizon constant** | pan-horizon authored config or `lib/src/` constants | internal DNS suffix (`criome`), public DNS suffix (`criome.net`), temporary exact IPv4 LAN |
| **Horizon derivation** | `lib/src/view/` projection code | node domain, tailnet base domain, router SSID, resolver listen addresses |
| **CriomOS-side** | CriomOS Nix module default or catalog package | DNS upstream choice, AI runtime config, AI model catalog, NordVPN server catalog, DHCP lease TTL |

**Smell — "replaces the literals scattered across CriomOS".** When
a new proposal record carries this phrase in its doc comment, the
shape is almost always wrong. The literals were a CriomOS
implementation choice; they belong in CriomOS defaults (or a
CriomOS Nix package for catalog data). What moves to horizon is
the *projection that derives the value* — not the literal itself.
A proposal record that simply transcribes the literals onto the
cluster authoring surface makes the cluster owner author the
operating system.

**Smell — composite that fails the rule.** A field can fail the
rule for half its content while passing it for the other half. An
"AI provider" entry is a cluster *selection* (`{ name,
serving_node, profile_ref, credentials_ref }`) **plus** a
CriomOS-side *implementation* (protocol, port, base path, model
catalog, runtime config). The selection authors per cluster; the
implementation does not. Split composites along the bucket
boundary.

The full audit driving this rule lives in
`~/primary/reports/designer/207-horizon-boundary-audit-and-lean-down-plan-2026-05-17.md`;
the brainstorm for the pan-horizon authored config is in
`~/primary/reports/designer/208-pan-horizon-configuration-brainstorm-2026-05-17.md`.

## Status

CANON. Active on the `horizon-leaner-shape` branch — a sibling of
`horizon-re-engineering` carrying the structural cleanups proposed
in `reports/designer-assistant/101-horizon-rs-overbuild-audit-2026-05-16.md`.
The legacy ARCH on `main` is 41 lines, names `lojix-cli` as the
consumer, and points at `docs/DESIGN.md` instead of describing the
system. This document replaces both.

## Consumers

| Consumer | How it reads the projection |
|---|---|
| `lojix-daemon` | In-process Rust dep (`horizon-lib`). Calls `ClusterProposal::project(&horizon_proposal, &viewpoint)` directly in its deploy actor. |
| `cli/` (`horizon`) | Same crate, binary entry point. Decodes a `datom.nota` from a path, projects from a `(cluster, node)` viewpoint, writes JSON to stdout. |
| CriomOS / CriomOS-home Nix modules | Read the JSON output via `inputs.horizon.cluster.{node, exNodes, users, …}`. Schema is the camelCase serialization of `view::*` records. |

`lojix-cli` (the legacy monolithic deploy tool) is in retirement;
new consumers integrate with `lojix-daemon`. The CLI binary stays
useful for ad-hoc projection (debugging, schema introspection,
generating fixtures).

## Shape — input and output namespaces

```mermaid
flowchart LR
    nota[datom.nota] -->|nota-codec decode| input[ClusterProposal]
    input -->|project(&HorizonProposal, &Viewpoint)| output[view::Horizon]
    output -->|serde_json| json[JSON]
    json --> nix[Nix modules]
    output --> lojix[lojix-daemon actor]
```

Two namespaces:

- **`proposal::*`** — the authored input shape. Records and validated
  newtypes. Decoded from NOTA. The boundary every cluster owner
  authors against.
- **`view::*`** — the projected output shape. Records that *only*
  appear after projection — derived booleans, resolved lookups,
  viewpoint-only fields. Records that *don't* differ from the input
  (e.g. `Machine`, `Io`) stay in `proposal::*` and are re-used
  directly; the view doesn't shadow them.

The decision rule for whether a record earns a `view::` type:
*does the projection genuinely change its shape, or just pass it
through?* If pass-through, no view type. The split is honest about
what the projection actually does, not architecturally enforced
fragmentation.

## Owned records — `proposal::*`

| Record | What it carries |
|---|---|
| `ClusterProposal` | Top-level: nodes, users, domains, trust, secret bindings, tailnet / AI / VPN policy, cluster + public domain. |
| `ClusterTrust` | Per-cluster trust floor + per-cluster / per-node / per-user overrides. |
| `NodeProposal` | Per-node authored shape: species, size, trust, machine, IO, pub keys, capability opt-ins, services, placement. |
| `UserProposal` | Per-user authored shape: species, size, keyboard, style, pub keys, editor, text size. |
| `Machine` | Hardware: arch, cores, model, motherboard, chip-gen, RAM. (Same type used by `view::Node`.) |
| `Io` | Filesystem + boot config. (Same type used by `view::Node`.) |
| `NodePlacement` | `Metal` vs `Contained { host, user, substrate, … }`. |
| `RouterInterfaces` | Per-router WAN/WLAN interface roles + WLAN config (IsoCountryCode + band + channel + standard + WPA3-SAE password reference). |
| `NodeServices` | Per-node service roles (tailnet membership + controller). |
| `AiProvider` | Cluster-advertised AI inference endpoints. |
| `VpnProfile` | VPN provider profiles (NordVPN today; WireguardMesh later). |
| `ClusterSecretBinding` / `SecretReference` / `SecretBackend` | Logical secret names + per-cluster backend resolution. |

## Projected records — `view::*`

| Record | What's new vs the proposal |
|---|---|
| `Horizon` | The output root. `{ cluster, node, ex_nodes, users, contained_nodes }`. |
| `Cluster` | Cluster-level roll-up: trusted-build-pub-keys list, resolved `secret_bindings: BTreeMap<SecretName, SecretBackend>` (the proposal carries the authored `Vec`), passthrough of LAN / resolver / tailnet / AI providers / VPN profiles. |
| `LanNetwork` / `LanCidr` / `DhcpPool` / `ResolverPolicy` | Projected network records derived from pan-horizon config. The current IPv4 LAN is an explicit temporary value, not an allocator. |
| `RouterInterfaces` / `Ssid` | Projected router interfaces with derived SSID. |
| `Node` | Per-node projected view: every passthrough field + computed booleans (`is_remote_nix_builder`, `is_dispatcher`, `is_large_edge`, `enable_network_manager`, `is_fully_trusted`, `chip_is_intel`, `model_is_thinkpad`), sub-records (`BehavesAs`, `Option<NixCache>`), derived strings (`ssh_pub_key_line`, `nix_pub_key_line`, `criome_domain_name`), and viewpoint-only optionals (`io`, `use_colemak`, `builder_configs`, `cache_urls`, ex-node / dispatcher / admin SSH lines, `wireguard_untrusted_proxies`). |
| `User` | Per-user projected view: trust ladder, computed booleans (`use_colemak`, `use_fast_repeat`, `is_multimedia_dev`, `is_code_dev`, `has_pub_key`, `enable_linger`), typed identifiers (`EmailAddress`, `MatrixId`), resolved keys, derived `extra_groups`. |
| `ProjectedNodeView` | One level of detail for contained nodes (nodes whose `placement = Contained { host: <viewpoint> }`). Populated only on the host's `horizon.contained_nodes` map. |
| `BehavesAs` | Composed flags derived from `NodeSpecies + NodePlacement`: `center`, `router`, `edge`, `next_gen`, `low_power`, `bare_metal`, `virtual_machine`, `iso`, `large_ai`. The composition lives once on the view side; Nix consumers gate on these. |
| `NixCache` | Presence on `Node.nix_cache` ⇔ node serves a binary cache; the entry carries `domain` and `url`. |
| `BuilderConfig` | One per remote Nix builder visible from the viewpoint. Resolved SSH user / key / pub-host-key + supported features + max-jobs. |

## Projection contract

`ClusterProposal::project(&HorizonProposal, &Viewpoint) -> Result<Horizon>` is the
single entry-point.

```rust
let horizon_proposal: HorizonProposal = decode_nota(pan_horizon_text)?;
let proposal: ClusterProposal = decode_nota(datom_text)?;
let viewpoint = Viewpoint { cluster, node };
let horizon: Horizon = proposal.project(&horizon_proposal, &viewpoint)?;
let json: String = serde_json::to_string(&horizon)?;
```

What `project` does, in order:

1. **Validates the cluster** — viewpoint node must exist; tailnet
   topology must be consistent (one server max; controller requires
   `cluster.tailnet`); secret bindings must be unique (duplicate
   names → typed `Error::DuplicateSecretBinding`).
2. **Projects every node** in trust order. Nodes with effective
   `trust = Zero` are dropped from the horizon entirely (actively
   distrusted). For each surviving node, resolves arch (via
   placement host if `machine.arch = None`), composes `BehavesAs`,
   computes the seven derived booleans, materialises `Option<NixCache>`.
3. **Projects every user** in trust order. Same `Zero` filter.
   Computes `EmailAddress` / `MatrixId` from cluster + public domain.
   Resolves the user's git signing key + ssh line at the viewpoint
   node. Derives `extra_groups` from trust + `enable_linger` from
   trust × viewpoint behaves-as-center.
4. **Fills viewpoint-only fields** on the viewpoint node:
   `io`, `use_colemak`, per-ex-node `builder_configs`, `cache_urls`,
   SSH pub-key lists (ex-nodes, dispatchers, admins). These are
   `Option<…>` everywhere — `Some(…)` on `horizon.node`, `None` on
   each entry of `horizon.ex_nodes`.
5. **Surfaces contained nodes** — every node whose placement names
   this viewpoint as its host appears in `horizon.contained_nodes`
   as a `ProjectedNodeView`.

## Validation guarantees

Every constraint below is enforced at projection time and returns a
typed `Error` variant. No silent fallback, no string-based dispatch
on errors.

- Viewpoint node exists in cluster (`NodeNotInCluster`).
- At most one tailnet-controller-server per cluster
  (`MultipleTailnetControllers`).
- A tailnet controller requires `cluster.tailnet` to be set
  (`TailnetControllerWithoutClusterConfig`).
- Every authored `ClusterSecretBinding.name` is unique
  (`DuplicateSecretBinding`).
- Every pod node either declares its own `machine.arch` or has a
  resolvable super-node via `placement.host` (`UnresolvableArch`,
  `MissingSuperNode`).
- Every newtype validates at the boundary: `EmailAddress` requires
  `local@host`, `MatrixId` requires `@user:domain`, `IsoCountryCode`
  is two ASCII uppercase letters, `Ssid` is 1–32 bytes, `Magnitude`
  is a closed ladder, `NodeSpecies` / `KnownModel` are closed enums.

## JSON wire shape

Every record serializes via `serde_json` with:

- `#[serde(rename_all = "camelCase")]` on every struct;
- typed enums as PascalCase strings (`NodeSpecies::Center` →
  `"Center"`; `KnownModel::GmktecEvoX2` → `"GmktecEvoX2"`);
- `System` as Nix tuple strings (`"x86_64-linux"` /
  `"aarch64-linux"`) via per-variant `serde(rename)`;
- `Option<…>` viewpoint-only fields with
  `#[serde(skip_serializing_if = "Option::is_none", default)]` —
  absent in JSON when `None`, present when `Some`;
- Newtype identifiers as transparent strings (`EmailAddress`,
  `MatrixId`, `IsoCountryCode`, `Ssid`, `ClusterDomain`,
  `PublicDomain`, …).

Nix consumers read these as `inputs.horizon.<path>`. Gate sites use
direct field reads (no dispatch on string content):

```nix
# Was: node.typeIs.center → before TypeIs retired.
# Now: equality against the species tag string.
isCenter = config.horizon.node.species == "Center";

# Was: node.computerIs.gmktecEvoX2 → before ComputerIs retired.
# Now: equality against the known-model tag (None ⇔ unknown).
isGmktec = (config.horizon.node.machine.model or null) == "GmktecEvoX2";

# behaves_as composes multiple inputs and stays on the view.
isRouter = config.horizon.node.behavesAs.router;
```

## Constraints

Each becomes a green test (per
`~/primary/ESSENCE.md` §"Constraints become tests").

- **C1 — record round-trip.** Every `proposal::*` record decodes from
  NOTA, re-encodes, and round-trips byte-stable. Tests live in
  `lib/tests/{proposal, secret, ai, vpn, tailnet, …}.rs`.
- **C2 — view round-trip.** Every `view::*` record serializes to JSON,
  parses back, and re-serializes byte-stable. Tests live in
  `lib/tests/view_json_roundtrip.rs`. camelCase keys asserted
  per-record.
- **C3 — `skip_serializing_if` correctness.** Every viewpoint-only
  `Option<…>` field is *absent* from the JSON object when `None`
  (asserted in `node_view_round_trips_through_json_with_only_always_derived_fields`).
- **C4 — projection contract.** End-to-end projection witnessed in
  `lib/tests/horizon.rs`: a real cluster proposal projects to a
  real horizon; the viewpoint node has populated viewpoint-only
  fields; ex-nodes have them absent.
- **C5 — validation surfaces typed errors.** Every error path above
  has at least one negative test that constructs the failing input
  and asserts the typed error.
- **C6 — positional NOTA tail safety.** New fields added to existing
  records land at the *tail* of the struct and carry `#[serde(default)]`
  (or `#[serde(default = "…")]` for a non-empty default) so existing
  authored datoms keep decoding without a schema-version bump.
- **C7 — newtype validation at the boundary.** Every newtype with
  format validation has a positive (valid input) and a negative
  (invalid input) test.

## Code map

```
lib/
  src/
    lib.rs                 — module entries + re-exports
    error.rs               — crate-typed Error enum
    address.rs             — IP / interface / link-local newtypes
    disk.rs                — Disk / MountPath / FsType / SwapDevice
    magnitude.rs           — Magnitude ladder + AtLeast
    name.rs                — String newtypes (ClusterName, NodeName, …,
                              PublicDomain, EmailAddress, MatrixId,
                              CriomeDomainName, Keygrip)
    pub_key.rs             — SshPubKey, NixPubKey, WireguardPubKey, …
    species.rs             — NodeSpecies, UserSpecies, Arch, System,
                              Keyboard, Style, KnownModel, …
    proposal.rs            — module entry + re-exports
    proposal/
      ai.rs                — AiProvider + AiModel + AiServingConfig
      cluster.rs           — ClusterProposal + ClusterTrust + project()
      domain.rs            — DomainProposal
      io.rs                — Io  (shared with view side)
      machine.rs           — Machine  (shared with view side)
      node.rs              — NodeProposal + project() + resolve_arch()
      placement.rs         — NodePlacement (Metal | Contained)
      pub_keys.rs          — NodePubKeys + YggPubKeyEntry
      router.rs            — RouterInterfaces + WlanBand + WlanStandard
                              + IsoCountryCode
      secret.rs            — ClusterSecretBinding + SecretReference
                              + SecretBackend + SecretPurpose
      services.rs          — NodeServices + TailnetConfig
                              + TailnetControllerRole + TlsTrustPolicy
                              + PublicCertificate
      user.rs              — UserProposal + UserPubKeyEntry + project()
      vpn.rs               — VpnProfile + NordvpnProfile + ...
      wireguard.rs         — WireguardProxy
    view.rs                — module entry + re-exports
    view/
      cluster.rs           — Cluster (the projected roll-up)
      horizon.rs           — Horizon + Viewpoint
      network.rs           — LanNetwork + LanCidr + DhcpPool
                              + ResolverPolicy
      node.rs              — Node + BehavesAs + BuilderConfig + NixCache
                              + ViewpointFill
      projected_node.rs    — ProjectedNodeView (contained-node detail)
      router.rs            — projected RouterInterfaces + Ssid
      user.rs              — User (the projected user view)
  tests/                   — 21 integration test files
cli/
  src/main.rs              — `horizon` binary: nota → JSON projection
```

## Boundaries

**Owns:**

- The proposal NOTA schema (the source of truth for what a cluster
  owner authors).
- The projection semantics (validation rules, trust-zero filter,
  arch resolution, secret-binding resolution).
- The JSON wire shape consumed by Nix and lojix-daemon.
- The CLI binary for ad-hoc projection.

**Doesn't own:**

- The deploy pipeline — `lojix-daemon` reads horizon, drives `nix
  build` / `nix copy` / `nix switch`.
- The Nix module rendering — CriomOS / CriomOS-home modules
  translate `inputs.horizon.*` into NixOS / Home Manager
  configuration.
- The proposal authoring — `goldragon`'s `datom.nota` is the
  cluster owner's authored source.
- Secret values — only secret *references* travel through
  horizon. The backend (sops-nix / systemd-credential / agenix)
  resolves the value at activation time.
- The cluster-trust runtime — separate component, not yet in this
  workspace.

## Versioning + cross-cutting context

- Workspace `~/primary/ESSENCE.md` is upstream of every rule.
- The `horizon-leaner-shape` branch implements the audit findings
  in `~/primary/reports/designer-assistant/101-horizon-rs-overbuild-audit-2026-05-16.md`
  on a sibling of `horizon-re-engineering` (the system-specialist's
  active branch). Downstream consumer rewrites (CriomOS / CriomOS-home
  Nix modules, lojix-daemon's horizon-lib pin) need to follow on
  matching sibling branches before this lands on `main`.
- The legacy `lojix-cli` deploy path stays at the current schema for
  the duration of the horizon-re-engineering arc; retires after
  CriomOS migrates to `lojix-daemon`.
