# horizon-rs — design

This is the design we're agreeing on before any code lands. Read together
with [/home/li/git/CriomOS/docs/HORIZON.md](/home/li/git/CriomOS/docs/HORIZON.md)
(the schema) and [/home/li/git/horizon-rs/example-horizon.json](/home/li/git/horizon-rs/example-horizon.json)
(the golden output for ouranos@maisiliym).

## Scope

horizon-rs takes a **cluster proposal** (the JSON-ified shape of a
maisiliym/goldragon `datom.nix`) and a viewpoint `(cluster, node)`,
and produces an **enriched horizon**: that node's cluster + node +
exNodes + users with the full `methods.*` DAG computed.

It does not:

- talk to the data-source repo (the input arrives on stdin),
- read any environment / filesystem state,
- emit anything other than enriched horizon data.

## Naming: `pubKey`, not `preCriome`

Every `preCriome` / `Precriad` in the legacy schema is a public key
(SSH ed25519, Yggdrasil ed25519, Nix signing, WireGuard). The new
name is `pubKey`. horizon-rs uses `pubKey` throughout. The Nix side
will follow in a coordinated rename (tracked in beads on
`/home/li/git/CriomOS`).

Renames:

| Legacy | New |
|--------|-----|
| `preCriomes` (field) | `pubKeys` |
| `wireguardPreCriome` | `wireguardPubKey` |
| `yggPreCriome` / `yggdrasil.preCriome` | `yggPubKey` |
| `nixPreCriome` / `nixSigningPublicKey` | `nixPubKey` |
| `sshPreCriome` / `preCriomes.ssh` | `sshPubKey` |
| `hasNixPreCriad` | `hasNixPubKey` |
| `hasYggPrecriad` | `hasYggPubKey` |
| `hasSshPrecriad` | `hasSshPubKey` |
| `hasWireguardPrecriad` | `hasWireguardPubKey` |
| `hasNordvpnPrecriad` | `hasNordvpnPubKey` |
| `hasWifiCertPrecriad` | `hasWifiCertPubKey` |
| `hasBasePrecriads` | `hasBasePubKeys` |
| `sshPrecriome` (string method) | `sshPubKeyLine` |
| `nixPreCriome` (string method, "domain:key") | `nixPubKeyLine` |
| `trustedBuildPreCriomes` | `trustedBuildPubKeys` |
| `adminSshPreCriomes` | `adminSshPubKeys` |
| `exNodesSshPreCriomes` | `exNodesSshPubKeys` |
| `dispatchersSshPreCriomes` | `dispatchersSshPubKeys` |
| `sshCriomes` (user) | `sshPubKeys` |

`Keygrip` stays (it's a GPG key identifier, not a public key proper).
`gitSigningKey` stays (its value is `&<keygrip>`, a GPG configuration
form, not a key itself).

## Input source and wire format

The data source migrates from `maisiliym` to `goldragon` (already at
`github:LiGoldragon/goldragon`; old contents wiped, seeded with
maisiliym's data as the production starting point).

The input wire format is **TBD** — Nix and JSON are both ugly. Until
the new format is chosen, horizon-cli reads the existing JSON shape
(the `serde_json` derive of the proposal types) so we can develop
against today's data. Switching the wire format later is a
serde-side change that doesn't touch the typed schema.

## Crate shape

Two-crate workspace (already in place):

```
/home/li/git/horizon-rs/
├── Cargo.toml             # workspace
├── lib/
│   ├── Cargo.toml         # horizon-lib
│   └── src/…
├── cli/
│   ├── Cargo.toml         # horizon-cli (depends on horizon-lib)
│   └── src/main.rs
└── docs/DESIGN.md         # this file
```

`horizon-lib` is the typed schema + the projection. `horizon-cli` is a
thin binary that wires stdin/argv/stdout.

## Module layout (`lib/src/`)

```
lib/src/
├── lib.rs            # crate-level //! + re-exports
├── error.rs          # Error enum (thiserror)
├── name.rs           # ClusterName, NodeName, UserName, ModelName, Keygrip
├── pub_key.rs        # YggPubKey, NixPubKey, SshPubKey, WireguardPubKey
├── address.rs        # YggAddress, YggSubnet, NodeIp, LinkLocalIp,
│                     # CriomeDomainName
├── magnitude.rs      # Magnitude enum (Non, Min, Med, Max), AtLeast helper
├── species.rs        # NodeSpecies, UserSpecies, MachineSpecies,
│                     # Keyboard, Style, Bootloader, Arch, MotherBoard
├── machine.rs        # Machine, System
├── io.rs             # Io, Disk, FsType
├── proposal.rs       # ClusterProposal + its node/user children
├── horizon.rs        # Horizon (top-level) + projection entry-point
├── cluster.rs        # Cluster + ClusterMethods
├── node.rs           # Node + NodeMethods + LocalNodeMethods +
│                     # BehavesAs + TypeIs + ComputerIs + BuilderConfig
└── user.rs           # User + UserMethods
```

One concern per file. Impls live next to their types — no `node.rs` +
`node_impl.rs` split.

## Domain types — newtypes

Every value with semantic identity gets its own type. None of them have
public fields; construction goes through `TryFrom<&str>` (validating
ones) or `From<String>` (untyped pass-throughs).

| Newtype                | Inner             | Validation? | Notes |
|------------------------|-------------------|-------------|-------|
| `ClusterName`          | `String`          | non-empty   | |
| `NodeName`             | `String`          | non-empty   | |
| `UserName`             | `String`          | non-empty   | |
| `ModelName`            | `String`          | non-empty   | hardware model string |
| `Keygrip`              | `String`          | hex 40 chars | GPG keygrip |
| `GithubId`             | `String`          | non-empty   | |
| `CriomeDomainName`     | `String`          | derived only | `<node>.<cluster>.criome` |
| `System`               | enum              | from arch   | `X86_64Linux`, `Aarch64Linux` |
| `YggAddress`           | `Ipv6Addr`        | parse       | wraps `std::net::Ipv6Addr` |
| `YggSubnet`            | `String`          | parse       | `300:…` form (custom prefix length) |
| `YggPubKey`            | `String`          | hex 64+     | yggdrasil ed25519 public key |
| `NixPubKey`            | `String`          | base64 44   | nix signing public key (raw) |
| `SshPubKey`            | `String`          | base64      | the `AAAAC3…` portion |
| `WireguardPubKey`      | `String`          | base64 44   | |
| `NodeIp`               | `IpNet`-shaped    | parse       | `5::3/128` etc. (CIDR) |
| `LinkLocalIp`          | struct            | parse       | `species` + `suffix` → `fe80::…%iface` |
| `BuildCores`           | `u32`             | ≥ 1         | |

The four `*PubKey` types are deliberately distinct — substituting a
WireGuard pubkey where a Nix pubkey is expected is exactly the bug
typed newtypes prevent.

The `name.rs`-grouped names are written out one impl block per type,
not behind a macro: they will grow per-name validation and conversions.

## Enums — closed sets

Mirroring [/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix](/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix):

```rust
pub enum Magnitude { Non, Min, Med, Max }     // 0..3 — size and trust ladder

pub enum NodeSpecies {
    Center,
    LargeAi,            // serde-renamed to "largeAI"
    LargeAiRouter,      // serde-renamed to "largeAI-router"
    Hybrid,
    Edge,
    EdgeTesting,
    MediaBroadcast,
    Router,
    RouterTesting,
}

pub enum UserSpecies { Code, Multimedia, Unlimited }

pub enum MachineSpecies { Metal, Pod }

pub enum Keyboard { Qwerty, Colemak }

pub enum Style { Vim, Emacs }

pub enum Bootloader { Uefi, Mbr, Uboot }

pub enum Arch { X86_64, Arm64 }                // serde-renamed "x86-64", "arm64"

pub enum System { X86_64Linux, Aarch64Linux }  // derived from Arch

pub enum MotherBoard { Ondyfaind }             // singleton today; extend with the source
```

`FsType` is open-ish (ext4, btrfs, vfat, tmpfs, …) — keep as a `String`
newtype for now; promote to enum when the closed set is settled.

For every enum, `Display` and `FromStr` are derived/impl'd so JSON
round-trips cleanly via `#[serde(rename = "…")]` on each variant.

## `Magnitude` semantics

`Non` = 0 (absent / disabled), `Min` = 1, `Med` = 2, `Max` = 3.
Matches the Nix `matchSize: ifNon ifMin ifMed ifMax` ladder in
[/home/li/git/criomos-archive/criomos-lib.nix](/home/li/git/criomos-archive/criomos-lib.nix).

```rust
impl Magnitude {
    pub fn at_least(&self, other: Magnitude) -> bool { /* >= */ }
}

pub struct AtLeast { pub min: bool, pub med: bool, pub max: bool }

impl Magnitude {
    pub fn at_least_breakdown(&self) -> AtLeast { … }
}
```

`AtLeast` is the typed form of `sizedAtLeast.{min,med,max}` in the
output methods.

## Top-level types

### `proposal::ClusterProposal` (input shape)

Mirrors the JSON-ified maisiliym/goldragon proposal. Field names track
the new pubKey naming.

```rust
pub struct ClusterProposal {
    pub nodes:   HashMap<NodeName, NodeProposal>,
    pub users:   HashMap<UserName, UserProposal>,
    pub domains: HashMap<DomainName, DomainProposal>,
    pub trust:   ClusterTrust,
}

pub struct NodeProposal {
    pub species:           NodeSpecies,
    pub size:              Magnitude,
    pub trust:             Magnitude,
    pub machine:           Machine,
    pub io:                Io,
    pub pub_keys:          NodePubKeys,
    pub link_local_ips:    Vec<LinkLocalIp>,
    pub node_ip:           Option<NodeIp>,
    pub wireguard_pub_key: Option<WireguardPubKey>,
    pub nordvpn:           bool,
    pub wifi_cert:         bool,
}

pub struct NodePubKeys {
    pub ssh:       SshPubKey,
    pub nix:       Option<NixPubKey>,
    pub yggdrasil: Option<YggPubKeyEntry>,
}

pub struct YggPubKeyEntry {
    pub pub_key: YggPubKey,
    pub address: YggAddress,
    pub subnet:  YggSubnet,
}
```

The `*Proposal` suffix names the **input concept**; it's not a
-Details companion to a paired `Node`. They live in `proposal.rs`
together so the namespace is local.

### `Horizon` (output shape)

```rust
pub struct Horizon {
    pub cluster:  Cluster,
    pub node:     Node,
    pub ex_nodes: HashMap<NodeName, Node>,
    pub users:    HashMap<UserName, User>,
}
```

`Cluster`, `Node`, `User` are the **enriched** shapes — they carry
their `methods` directly. No separate `EnrichedNode` /
`NodeWithMethods` types.

### `Cluster`, `Node`, `User`

Each is a struct of the schema's data fields plus `methods: …Methods`.
`Node` additionally carries `local_methods: Option<LocalNodeMethods>` —
present only when this is the viewpoint's `horizon.node` (carries
`builderConfigs`, `cacheURLs`, `adminSshPubKeys`, `computerIs`, …).
`local_methods` for sibling `exNodes` is `None`.

## Method computation

The projection is one method on `ClusterProposal`:

```rust
impl ClusterProposal {
    pub fn project(&self, viewpoint: Viewpoint) -> Result<Horizon, Error> { … }
}

pub struct Viewpoint {
    pub cluster: ClusterName,
    pub node:    NodeName,
}
```

`project` walks the proposal in one pass and emits a `Horizon`.
Internally it composes per-type methods that compute their own pieces
of the DAG:

```rust
impl Node {
    fn methods(&self) -> NodeMethods { … }
    fn local_methods(&self, ctx: &ZoneContext) -> LocalNodeMethods { … }
}

impl User {
    fn methods(&self, viewpoint: &Viewpoint, cluster_name: &ClusterName) -> UserMethods { … }
}

impl Cluster {
    fn methods(&self, nodes: &HashMap<NodeName, Node>) -> ClusterMethods { … }
}
```

`ZoneContext` is an internal struct (in `node.rs`) carrying the small
amount of shared state the local-only methods need (the full `nodes`
map for `builderConfigs`, the user list for `adminSshPubKeys`).

Every method in [/home/li/git/CriomOS/docs/HORIZON.md](/home/li/git/CriomOS/docs/HORIZON.md)
lands as one named function on the type that owns it. The HORIZON.md
table is the test-equivalence target.

## Error type

```rust
// error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid cluster name: {0}")]
    InvalidClusterName(String),

    #[error("invalid node name: {0}")]
    InvalidNodeName(String),

    #[error("cluster {cluster:?} has no node {node:?}")]
    NodeNotInCluster { cluster: ClusterName, node: NodeName },

    #[error("invalid yggdrasil address: {0}")]
    InvalidYggAddress(String),

    #[error("invalid hex (expected {expected_len} chars): {got}")]
    InvalidHex { expected_len: usize, got: String },

    #[error("invalid base64: {0}")]
    InvalidBase64(String),

    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}
```

Library returns `Result<T, horizon_lib::Error>` everywhere. No
`anyhow`, no `Box<dyn Error>`.

## CLI

```
horizon-cli --cluster <CLUSTER> --node <NODE>   # < raw.json > horizon.json
```

- Reads cluster proposal JSON from stdin.
- Writes enriched horizon JSON to stdout (`serde_json::to_writer_pretty`).
- Exit codes:
  - `0` — success.
  - `1` — schema / projection error (printed to stderr).
  - `2` — usage error (missing flag, bad JSON parse).
- Flags via `clap` with `derive`. The CLI struct lives in
  `cli/src/main.rs`; `main` is the only free function in the crate.

## Actors?

No actors here. horizon-cli is a one-shot pure function:
`(input bytes, viewpoint) → output bytes`. No long-lived state, no
concurrent components, no logical units that warrant their own
protocol. Methods on types is the right grain.

If a later horizon-as-a-service surface materializes (long-lived
process, multiple in-flight projections, watchers on input changes),
that's where actors would land — and it would be a sibling crate, not
this one.

## Dependencies

- `serde` (derive) + `serde_json` — JSON I/O for Phase 1.
- `thiserror` — error enum derive.
- `clap` (derive) — CLI parsing.
- `std::net::Ipv6Addr` — `YggAddress` inner.
- `ipnet` — `NodeIp` inner (CIDR parsing).
- No `anyhow`, no `tokio`, no `rkyv`.

## Settled questions (from review)

1. **Magnitude as enum** — `Non / Min / Med / Max`, matches the Nix
   `matchSize` ladder.
2. **Typed addresses** — yes; `YggAddress` wraps `Ipv6Addr`, `NodeIp`
   wraps `IpNet`. Bad addresses fail at projection time, not at
   downstream parse.
3. **Input format** — open. Nix and JSON are both ugly; format will
   change. Phase 1 reads JSON for bootstrap (matches today's data).
4. **Enum exhaustiveness** — mirror the Nix species lists from
   [/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix](/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix).
   The cluster-proposal owner defines the species set (currently
   maisiliym; soon goldragon).
5. **Source-of-truth migration** — input source moves from
   maisiliym to goldragon. goldragon already exists at
   [github.com/LiGoldragon/goldragon](https://github.com/LiGoldragon/goldragon)
   (resetting now, seeded with maisiliym data).
6. **`pubKey` rename** — agreed. Nix-side rename tracked in beads on
   `/home/li/git/CriomOS`.

## Still open

1. **`LocalNodeMethods` placement.** `Option<LocalNodeMethods>` field
   on every `Node` (current proposal) vs a distinct `LocalNode` type
   that holds a `Node` plus the local methods (statically guaranteed
   to be present at `horizon.node`). Recommendation: stick with
   `Option`; revisit if `unwrap_or_default` noise piles up at use
   sites.

2. **`example-horizon.json` vs HORIZON.md drift.** The example carries
   `behavesAs.{lowPower, nextGen}` and `typeIs.{mediaBroadcast,
   routerTesting}` not enumerated in HORIZON.md. Need a sweep against
   the example before code lands. This is a HORIZON.md fix, not a
   horizon-rs change.

## Implementation order

1. `name.rs`, `magnitude.rs`, `species.rs` — foundation newtypes /
   enums. Round-trip tests against fragments of `example-horizon.json`.
2. `pub_key.rs`, `address.rs`, `machine.rs`, `io.rs` — the rest of
   the input-shape primitives.
3. `proposal.rs` — full `ClusterProposal` deserialization. Test:
   parse the maisiliym datom (JSON-converted) round-trip-clean.
4. `node.rs`, `user.rs`, `cluster.rs`, `horizon.rs` — output types,
   no methods yet.
5. Method DAG, one type at a time, each with golden tests against the
   corresponding slice of `example-horizon.json`.
6. `error.rs` final pass, `lib.rs` re-exports.
7. `cli/src/main.rs` — clap, stdin→stdout wiring.
8. End-to-end test: pipe the maisiliym JSON in, diff against
   `example-horizon.json`. Bytes-equal is the bar.
