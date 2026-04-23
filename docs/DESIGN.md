# horizon-rs — design

This is the design we're agreeing on before any code lands. Read together
with `~/git/CriomOS/docs/HORIZON.md` (the schema) and `example-horizon.json`
(the golden output for ouranos@maisiliym).

## Scope

horizon-rs takes a **cluster proposal** (the JSON-ified shape of a
maisiliym `datom.nix`) and a viewpoint `(cluster, node)` and produces an
**enriched horizon**: that node's cluster + node + exNodes + users with
the full `methods.*` DAG computed.

It does not:

- talk to maisiliym or any source repo (the input is given on stdin),
- read any environment / filesystem state,
- emit anything other than enriched horizon JSON.

## Crate shape

Two-crate workspace, already in place:

```
horizon-rs/
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
├── lib.rs              # crate-level //! + re-exports
├── error.rs            # Error enum (thiserror)
├── name.rs             # ClusterName, NodeName, UserName, ModelName
├── precriome.rs        # YggPreCriome, NixPreCriome, SshPreCriome,
│                       # WireguardPreCriome, Keygrip
├── address.rs          # YggAddress, YggSubnet, NodeIp, LinkLocalIp,
│                       # CriomeDomainName
├── magnitude.rs        # Magnitude (0..3) + SizedAtLeast
├── species.rs          # NodeSpecies, UserSpecies, MachineSpecies,
│                       # Keyboard, Style enums
├── machine.rs          # Machine, Arch, System
├── io.rs               # Io, Disk, FsType, Bootloader
├── proposal.rs         # ClusterProposal + its node/user children
├── horizon.rs          # Horizon (top-level) + projection entry-point
├── cluster.rs          # Cluster + ClusterMethods
├── node.rs             # Node + NodeMethods + LocalNodeMethods +
│                       # BehavesAs + TypeIs + ComputerIs + BuilderConfig
└── user.rs             # User + UserMethods
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
| `System`               | `String`          | from arch   | `x86_64-linux` / `aarch64-linux` |
| `Arch`                 | enum              | (closed)    | see "Enums" |
| `YggAddress`           | `Ipv6Addr`-shaped | parse       | wraps `std::net::Ipv6Addr` |
| `YggSubnet`            | `String`          | parse       | `300:…` form |
| `YggPreCriome`         | `String`          | hex         | public key |
| `NixPreCriome`         | `String`          | base64      | signing key (raw, no domain prefix) |
| `SshPreCriome`         | `String`          | base64      | the `AAAAC3…` portion |
| `WireguardPreCriome`   | `String`          | base64      | |
| `NodeIp`               | `String`          | CIDR-ish    | `5::3/128` etc. |
| `LinkLocalIp`          | struct            | parse       | `species` + `suffix` → `fe80::…%iface` |
| `BuildCores`           | `u32`             | ≥ 1         | |
| `Magnitude`            | `u8` enum         | 0..3        | size / trust |

The `*PreCriome` types are deliberately distinct even though they all
wrap base64-ish strings — substituting a Wireguard preCriome where a
Nix preCriome is expected is exactly the bug typed newtypes prevent.

The `name.rs`-grouped names (`ClusterName`, `NodeName`, …) are
written out one impl block per type, not behind a macro: they will
grow per-name validation and per-name conversions.

## Enums — closed sets

Per the input schema (mkCrioSphere/speciesModule.nix in archive):

- `NodeSpecies`: `Center`, `Edge`, `EdgeTesting`, `Hybrid`, `Router`,
  `LargeAi`, `LargeAiRouter`, `MediaBroadcast`, `RouterTesting`.
  serde-renames to the kebab/camel forms in JSON
  (`largeAI`, `largeAI-router`).
- `UserSpecies`: `Code`, `Multimedia`, `Unlimited`, … (need full list
  from speciesModule.nix; placeholder).
- `MachineSpecies`: `Metal`, `Pod`.
- `Keyboard`: `Colemak`, `Qwerty`, … (placeholder).
- `Style`: `Emacs`, … (placeholder).
- `Arch`: `X86_64`, `Arm64`. Maps to `System` via a method.
- `Bootloader`: `Uefi`, `Bios`, `None`.
- `FsType`: `Ext4`, `Btrfs`, `Vfat`, `Tmpfs`, …

For every enum, `Display` and `FromStr` are derived/impl'd so JSON
round-trips cleanly.

## Top-level types

### `proposal::Cluster` (input shape)

Mirrors the cluster proposal that maisiliym emits when its
`NodeProposal` is JSON-serialized. Fields match
`mkCrioSphere/clustersModule.nix`:

```rust
pub struct ClusterProposal {
    pub nodes:   HashMap<NodeName, NodeProposal>,
    pub users:   HashMap<UserName, UserProposal>,
    pub domains: HashMap<DomainName, DomainProposal>,
    pub trust:   ClusterTrust,
}

pub struct NodeProposal {
    pub species:               NodeSpecies,
    pub size:                  Magnitude,
    pub trust:                 Magnitude,
    pub machine:               Machine,
    pub io:                    Io,
    pub pre_criomes:           NodePreCriomes,
    pub link_local_ips:        Vec<LinkLocalIp>,
    pub node_ip:               Option<NodeIp>,
    pub wireguard_pre_criome:  Option<WireguardPreCriome>,
    pub nordvpn:               bool,
    pub wifi_cert:             bool,
    // …
}
```

The `*Proposal` suffix names the **input concept**, not a -Details
companion to a paired `Node`. They live in `proposal.rs` together so
the namespace is local.

### `Horizon` (output shape)

```rust
pub struct Horizon {
    pub cluster:  Cluster,
    pub node:     Node,
    pub ex_nodes: HashMap<NodeName, Node>,
    pub users:    HashMap<UserName, User>,
}
```

`Cluster`, `Node`, `User` here are the **enriched** shapes — they carry
their `methods` field directly. There are no separate
`EnrichedNode` / `NodeWithMethods` types.

### `Cluster`, `Node`, `User`

Each is a struct of the schema's data fields plus `methods: …Methods`.
`Node` additionally carries `local_methods: Option<LocalNodeMethods>` —
present only when this is the viewpoint's `horizon.node` (carries
`builderConfigs`, `cacheURLs`, `adminSshPreCriomes`, `computerIs`, …).
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
    fn methods(&self) -> NodeMethods { … }                 // base methods on every node
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
map for `builderConfigs`, the user list for `adminSshPreCriomes`).

Every method in `~/git/CriomOS/docs/HORIZON.md` lands as one named
function on the type that owns it. The HORIZON.md table is the
test-equivalence target.

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

No `Box<dyn Error>`, no `anyhow`. Library returns
`Result<T, horizon_lib::Error>` everywhere.

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

- `serde` (derive) + `serde_json` — JSON I/O.
- `thiserror` — error enum derive.
- `clap` (derive) — CLI parsing.
- `std::net::Ipv6Addr` — `YggAddress` inner.
- No `anyhow`, no `tokio`, no `rkyv` (this crate is the JSON boundary,
  not a binary-contract participant).

## Open questions

1. **`Magnitude` representation.** `u8` (0..3) with a constructor that
   rejects `> 3`, or a 4-variant enum (`Zero`, `One`, `Two`, `Three`)?
   Enum is more typed; `u8` round-trips cheaper to JSON. Recommendation:
   `u8`-newtype with `TryFrom<u8>`.

2. **`Md5`-style addresses.** `YggAddress` wraps `std::net::Ipv6Addr`
   (real parsing) vs `String` (verbatim). Recommendation: `Ipv6Addr`
   so we catch malformed addresses at projection time, not at consumer
   parse-time. Same for `NodeIp` (currently CIDR-shaped strings like
   `5::3/128`) — wrap an `IpNet` from `ipnet`?

3. **Names for input vs output types.** Three options on the table:
   (a) `ClusterProposal` / `Cluster`,
   (b) `proposal::Cluster` / `horizon::Cluster` (module-namespaced
   same name), (c) `RawCluster` / `Cluster` (excluded — `Raw-` is on
   the banned-suffix list). The doc currently shows (a). (b) reads
   cleaner at use sites but doubles the import bookkeeping.
   Recommendation: (a).

4. **Closed-enum exhaustiveness.** `NodeSpecies`, `UserSpecies`,
   `Keyboard`, `MachineSpecies` are listed in
   `mkCrioSphere/speciesModule.nix` in the archive. Do we mirror that
   exact set, or define the canonical list here and have maisiliym
   conform? Recommendation: mirror; the cluster-proposal owner
   defines the species.

5. **`LocalNodeMethods` placement.** Two options:
   (a) `Option<LocalNodeMethods>` field on every `Node` (what's
   shown), (b) a distinct `LocalNode` type that holds a `Node` plus
   the local methods — used only at `horizon.node`. (a) keeps the
   single-Node-type discipline; (b) makes "this is the
   viewpoint-node" a static guarantee instead of a runtime `Option`.
   Recommendation: (a) for simplicity, revisit if `unwrap_or_default`
   noise piles up at use sites.

6. **`docs/HORIZON.md` shape vs current `example-horizon.json`.** The
   example carries `behavesAs.lowPower`, `behavesAs.nextGen`, and
   `typeIs.{mediaBroadcast, routerTesting}` that aren't enumerated in
   HORIZON.md. The doc needs a sweep against the example before code
   lands; flagging here so we do that pass first.

## Implementation order

1. `name.rs`, `magnitude.rs`, `species.rs` — foundation newtypes /
   enums. Round-trip tests against fragments of `example-horizon.json`.
2. `precriome.rs`, `address.rs`, `machine.rs`, `io.rs` — the rest of
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
