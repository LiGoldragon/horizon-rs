# horizon-rs — design

The spec horizon-rs is built against. Integration tests against the
goldragon `datom.nota` are pending (tracked in beads — fixture lives in
the goldragon repo, not here).

## Scope

horizon-rs takes a **cluster proposal** (the goldragon nota) and a
viewpoint `(cluster, node)`, and produces an **enriched horizon**: the
viewpoint node's view of its cluster + node + exNodes + users with
every computed field already filled in.

It does not:

- talk to goldragon or any source repo (the input arrives on stdin),
- read any environment / filesystem state,
- emit anything other than enriched horizon nota.

## Wire format: nota

Both input and output are nota — see `~/git/nota/README.md` for the spec. nota-codec's typed `Decoder` + `Encoder` plus nota-derive's proc-macro derives (`NotaRecord`, `NotaEnum`, `NotaTransparent`, `NotaTryTransparent`).
JSON is gone.

## Schema rules

- **Clean break.** No `preCriome` anywhere; no `methods.` sub-namespace;
  no `*Methods` / `*Details` / `*Local` companion types; no serde
  rename shims for legacy field names.
- **Derived fields can never be populated by input.** The
  proposal types (input deserialization targets) carry only raw
  data. Derived fields exist only on the output types and are filled
  exclusively by horizon-rs's projection logic.
- **`Option` is fine for genuine optionality.** A field is `Option<T>`
  when its absence has a real meaning — it doesn't apply to this node,
  it doesn't apply from this viewpoint, etc. Examples: `nix_pub_key`
  (not every node has one), `nix_cache_domain` (only when this node is
  a cache), `Node`'s viewpoint-only roll-ups (`builder_configs`,
  `admin_ssh_pub_keys`, …) which are `Some(...)` for `horizon.node` and
  `None` for entries in `horizon.ex_nodes`.

## Crate shape

```
/home/li/git/horizon-rs/
├── Cargo.toml             # workspace
├── lib/                   # horizon-lib
├── cli/                   # horizon-cli
└── docs/DESIGN.md         # this file
```

## Module layout (`lib/src/`)

```
lib/src/
├── lib.rs        # crate-level //! + re-exports
├── error.rs      # Error enum (thiserror)
├── name.rs       # ClusterName, NodeName, UserName, ModelName, Keygrip, GithubId, CriomeDomainName, DomainName
├── pub_key.rs    # YggPubKey, NixPubKey, SshPubKey, WireguardPubKey
├── address.rs    # YggAddress, YggSubnet, NodeIp, LinkLocalIp + LinkLocalSpecies, Iface
├── magnitude.rs  # Magnitude (Non/Min/Med/Max), AtLeast
├── species.rs    # NodeSpecies, UserSpecies, MachineSpecies, Keyboard,
│                 # Style, Bootloader, Arch, System, MotherBoard
├── machine.rs    # Machine
├── io.rs         # Io, Disk, SwapDevice, FsType
├── proposal.rs   # ClusterProposal + child types (input)
├── horizon.rs    # Horizon + projection entry-point
├── cluster.rs    # Cluster
├── node.rs       # Node + BehavesAs + TypeIs + ComputerIs +
│                 # BuilderConfig + WireguardProxy
└── user.rs       # User
```

## Newtypes

| Newtype                | Inner             | Notes |
|------------------------|-------------------|-------|
| `ClusterName`          | `String`          | non-empty |
| `NodeName`             | `String`          | non-empty |
| `UserName`             | `String`          | non-empty |
| `ModelName`            | `String`          | hardware model |
| `Keygrip`              | `String`          | hex 40 |
| `GithubId`             | `String`          | non-empty |
| `CriomeDomainName`     | `String`          | derived: `<node>.<cluster>.criome` (also `nix.<criomeDomain>` for cache) |
| `DomainName`           | `String`          | external DNS domain owned by the cluster |
| `YggAddress`           | `Ipv6Addr`        | wraps `std::net::Ipv6Addr` |
| `YggSubnet`            | `String`          | `300:…` form |
| `YggPubKey`            | `String`          | hex |
| `NixPubKey`            | `String`          | base64 44 |
| `SshPubKey`            | `String`          | base64 (the `AAAAC3…` portion) |
| `WireguardPubKey`      | `String`          | base64 44 |
| `NodeIp`               | `IpNet`           | CIDR (`5::3/128`) |
| `Iface`                | `String`          | network interface name (`enp0s25`, `wlp3s0`) |
| `BuildCores`           | `u32`             | ≥ 1 |
| `SshPubKeyLine`        | `String`          | derived: `ssh-ed25519 <pubKey>` |
| `NixPubKeyLine`        | `String`          | derived: `<criomeDomain>:<rawNixPubKey>` (empty when no nix key) |

Fields are private. Construction goes through `TryFrom<&str>` for
validating types and `From<String>` for untyped pass-throughs.

## Enums

Mirroring [/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix](/home/li/git/criomos-archive/nix/mkCrioSphere/speciesModule.nix):

```rust
pub enum Magnitude { None, Min, Med, Max }    // 0..3 — size and trust ladder

pub enum NodeSpecies {
    Center, LargeAi, LargeAiRouter, Hybrid, Edge, EdgeTesting,
    MediaBroadcast, Router, RouterTesting,
}

pub enum UserSpecies   { Code, Multimedia, Unlimited }
pub enum MachineSpecies { Metal, Pod }
pub enum Keyboard      { Qwerty, Colemak }
pub enum Style         { Vim, Emacs }
pub enum Bootloader    { Uefi, Mbr, Uboot }
pub enum Arch          { X86_64, Arm64 }
pub enum System        { X86_64Linux, Aarch64Linux }   // derived from Arch
pub enum MotherBoard   { Ondyfaind }
pub enum FsType        { Ext2, Ext3, Ext4, Btrfs, Xfs, Zfs, F2fs,
                         Bcachefs, Vfat, Exfat, Ntfs, Tmpfs }
pub enum DomainSpecies { Cloudflare }
```

Variants serialize as their natural Rust spelling (PascalCase) per the
nota identifier convention. No serde rename annotations on enums.

## `Magnitude`

`None` = 0, `Min` = 1, `Med` = 2, `Max` = 3. Matches the Nix
`matchSize: ifNon ifMin ifMed ifMax` ladder.

```rust
pub struct AtLeast { pub min: bool, pub med: bool, pub max: bool }

impl Magnitude {
    pub fn at_least(&self) -> AtLeast { … }
}
```

## Input shape — `proposal::ClusterProposal`

The nota schema goldragon emits.

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
    pub wireguard_untrusted_proxies: Vec<WireguardProxy>,
    pub wants_printing:    bool,
    pub wants_hw_video_accel: bool,
    pub router_interfaces: Option<RouterInterfaces>,
    pub online:            Option<bool>,
    pub nb_of_build_cores: Option<u32>,
}

pub struct RouterInterfaces {
    pub wan:           Iface,
    pub wlan:          Iface,
    pub wlan_band:     WlanBand,
    pub wlan_channel:  u16,
    pub wlan_standard: WlanStandard,
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

pub struct UserProposal {
    pub species:    UserSpecies,
    pub size:       Magnitude,
    pub keyboard:   Keyboard,
    pub style:      Style,
    pub github_id:  Option<GithubId>,
    pub fast_repeat: Option<bool>,                 // defaults true when absent
    pub pub_keys:   HashMap<NodeName, UserPubKeyEntry>,
}

pub struct UserPubKeyEntry {
    pub ssh:     SshPubKey,
    pub keygrip: Keygrip,
}

pub struct DomainProposal {
    pub species: DomainSpecies,                    // currently just Cloudflare
}

pub enum DomainSpecies { Cloudflare }

pub struct ClusterTrust {
    pub cluster:  Magnitude,
    pub clusters: HashMap<ClusterName, Magnitude>,
    pub nodes:    HashMap<NodeName, Magnitude>,
    pub users:    HashMap<UserName, Magnitude>,
}
```

## Hardware / I/O

```rust
pub struct Machine {
    pub species:      MachineSpecies,             // Metal | Pod
    pub arch:         Arch,                        // resolved from input or pod's superNode
    pub cores:        u32,
    pub model:        Option<ModelName>,           // None for Pod
    pub mother_board: Option<MotherBoard>,
    pub super_node:   Option<NodeName>,            // only for Pod
    pub super_user:   Option<UserName>,            // only for Pod
}

pub struct Io {
    pub keyboard:     Keyboard,
    pub bootloader:   Bootloader,
    pub disks:        HashMap<MountPath, Disk>,
    pub swap_devices: Vec<SwapDevice>,
}

pub struct Disk {
    pub device:  String,
    pub fs_type: FsType,
    pub options: Vec<String>,                      // mount options; e.g. ["subvol=root"]
}

pub struct SwapDevice {
    pub device: String,
}

pub struct LinkLocalIp {
    pub species: LinkLocalSpecies,
    pub suffix:  String,                            // e.g. "aec6:ecad:34e0:b41f"
}

pub struct WireguardProxy {
    // Open shape; passes through from input. Refine when first proxy lands.
}

pub type MountPath = String;                       // "/", "/boot", "/nix", "/home", "/var"
```

`Iface` is derived from `LinkLocalSpecies` at projection time
(ethernet → `enp0s25`, wifi → `wlp3s0` — TODO move this mapping out
of code into per-node config when it stops being a constant).

## Output — `Cluster`, `Node`, `User`, `Horizon`

```rust
pub struct Horizon {
    pub cluster:  Cluster,
    pub node:     Node,
    pub ex_nodes: HashMap<NodeName, Node>,
    pub users:    HashMap<UserName, User>,
}

pub struct Cluster {
    pub name: ClusterName,
    pub trusted_build_pub_keys: Vec<NixPubKeyLine>,
}

pub struct Node {
    // input pass-through (always present)
    pub name:                NodeName,
    pub species:             NodeSpecies,
    pub size:                Magnitude,
    pub trust:               Magnitude,
    pub machine:             Machine,
    pub link_local_ips:      Vec<LinkLocalAddress>,    // already rendered "fe80::…%iface"
    pub node_ip:             Option<NodeIp>,
    pub wireguard_pub_key:   Option<WireguardPubKey>,
    pub nordvpn:             bool,
    pub wifi_cert:           bool,
    pub wants_printing:      bool,
    pub wants_hw_video_accel: bool,
    pub router_interfaces:   Option<RouterInterfaces>,

    // identity / connectivity (derived)
    pub criome_domain_name:  CriomeDomainName,
    pub system:              System,
    pub nb_of_build_cores:   BuildCores,

    // pubkey shadow flattened from input pubKeys
    pub ssh_pub_key:         SshPubKey,
    pub nix_pub_key:         Option<NixPubKey>,
    pub ygg_pub_key:         Option<YggPubKey>,
    pub ygg_address:         Option<YggAddress>,
    pub ygg_subnet:          Option<YggSubnet>,

    // computed booleans
    pub is_fully_trusted:    bool,
    pub sized_at_least:      AtLeast,
    pub is_builder:          bool,
    pub is_dispatcher:       bool,
    pub is_nix_cache:        bool,
    pub has_nix_pub_key:     bool,
    pub has_ygg_pub_key:     bool,
    pub has_ssh_pub_key:     bool,
    pub has_wireguard_pub_key: bool,
    pub has_nordvpn_pub_key: bool,
    pub has_wifi_cert_pub_key: bool,
    pub has_base_pub_keys:   bool,
    pub has_video_output:    bool,
    pub chip_is_intel:       bool,
    pub model_is_thinkpad:   bool,

    // computed strings (empty when not applicable)
    pub ssh_pub_key_line:    SshPubKeyLine,              // "ssh-ed25519 <key>"
    pub nix_pub_key_line:    NixPubKeyLine,              // "<criomeDomain>:<key>" or ""
    pub nix_cache_domain:    Option<CriomeDomainName>,   // Some only when is_nix_cache
    pub nix_url:             Option<String>,             // Some only when is_nix_cache

    // grouped flags
    pub behaves_as:          BehavesAs,
    pub type_is:             TypeIs,

    // viewpoint-only fields: Some on horizon.node, None on horizon.ex_nodes.<name>.
    pub io:                  Option<Io>,
    pub use_colemak:         Option<bool>,                  // io.keyboard == Colemak
    pub computer_is:         Option<ComputerIs>,
    pub builder_configs:     Option<Vec<BuilderConfig>>,
    pub cache_urls:          Option<Vec<String>>,           // each is `http://nix.<criomeDomain>`
    pub ex_nodes_ssh_pub_keys:     Option<Vec<SshPubKeyLine>>,
    pub dispatchers_ssh_pub_keys:  Option<Vec<SshPubKeyLine>>,
    pub admin_ssh_pub_keys:        Option<Vec<SshPubKeyLine>>,
    pub wireguard_untrusted_proxies: Option<Vec<WireguardProxy>>,
}

pub struct LinkLocalAddress(String);                     // rendered "fe80::<suffix>%<iface>"

pub struct BehavesAs {
    pub center:          bool,
    pub router:          bool,
    pub edge:            bool,
    pub next_gen:        bool,
    pub low_power:       bool,
    pub bare_metal:      bool,
    pub virtual_machine: bool,
    pub iso:             bool,
    pub large_ai:        bool,
}

pub struct TypeIs {
    pub center:           bool,
    pub edge:             bool,
    pub edge_testing:     bool,
    pub hybrid:           bool,
    pub large_ai:         bool,
    pub large_ai_router:  bool,
    pub media_broadcast:  bool,
    pub router:           bool,
    pub router_testing:   bool,
}

pub struct ComputerIs {
    pub thinkpad_t14_gen2_intel: bool,
    pub thinkpad_t14_gen5_intel: bool,
    pub thinkpad_x230:           bool,
    pub thinkpad_x240:           bool,
    pub rpi3b:                   bool,
    // serde-renames preserve the legacy keys: "ThinkPadT14Gen2Intel", "rpi3B", etc.
    // Add models here as the cluster grows.
}

pub struct BuilderConfig {
    pub host_name:          CriomeDomainName,
    pub ssh_user:           String,                       // "nixBuilder"
    pub ssh_key:            String,                       // "/etc/ssh/ssh_host_ed25519_key"
    pub supported_features: Vec<String>,                  // e.g. ["big-parallel"]
    pub system:             System,
    pub systems:            Vec<System>,                  // extras (i686-linux when system == x86_64-linux)
    pub max_jobs:           BuildCores,
}

pub struct User {
    // input pass-through
    pub name:        UserName,
    pub species:     UserSpecies,
    pub size:        Magnitude,
    pub trust:       Magnitude,
    pub keyboard:    Keyboard,
    pub style:       Style,
    pub github_id:   Option<GithubId>,
    pub pub_keys:    HashMap<NodeName, UserPubKeyEntry>,

    // computed
    pub sized_at_least:    AtLeast,
    pub has_pub_key:       bool,                          // viewpoint node specifically
    pub email_address:     String,                        // "<user>@<cluster>.criome.net"
    pub matrix_id:         String,                        // "@<user>:<cluster>.criome.net"
    pub git_signing_key:   Option<String>,                // "&<keygrip>" form, viewpoint-node keygrip
    pub use_colemak:       bool,
    pub use_fast_repeat:   bool,
    pub is_multimedia_dev: bool,
    pub is_code_dev:       bool,
    pub ssh_pub_keys:      Vec<SshPubKeyLine>,            // every node's line
    pub ssh_pub_key:       Option<SshPubKeyLine>,         // viewpoint-node line; only when has_pub_key
}
```

## Method computation

One entry point on `ClusterProposal`:

```rust
impl ClusterProposal {
    pub fn project(&self, viewpoint: Viewpoint) -> Result<Horizon, Error> { … }
}

pub struct Viewpoint {
    pub cluster: ClusterName,
    pub node:    NodeName,
}
```

Internally the constructors live on the output types:

```rust
impl Node {
    // Builds a Node with viewpoint-only fields = None (suitable for ex_nodes).
    fn from_proposal(p: &NodeProposal, name: NodeName, cluster: &ClusterName,
                     trust_floor: Magnitude) -> Self { … }

    // Fills the Some-variants of the viewpoint-only fields in place.
    // Called only on the viewpoint Node, after every other Node is built.
    fn fill_viewpoint(&mut self, ctx: &ZoneContext) { … }
}

impl User {
    fn from_proposal(p: &UserProposal, name: UserName, cluster: &ClusterName,
                     viewpoint_node: &NodeName, trust_floor: Magnitude) -> Self { … }
}

impl Cluster {
    fn from_nodes(name: ClusterName, nodes: &HashMap<NodeName, Node>) -> Self { … }
}

impl Horizon {
    fn from_proposal(proposal: &ClusterProposal, viewpoint: Viewpoint) -> Result<Self, Error> { … }
}
```

Per-node fields are filled in `Node::from_proposal`. The viewpoint-only
fields (`builder_configs`, `admin_ssh_pub_keys`, `io`, etc.) are filled
in `Node::fill_viewpoint`, called once on `horizon.node` after every
sibling `Node` is constructed. `ZoneContext` is internal to `node.rs`,
carries the full `nodes` map and the user list.

## Error type

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid cluster name: {0}")]
    InvalidClusterName(String),

    #[error("invalid node name: {0}")]
    InvalidNodeName(String),

    #[error("cluster has no node {node:?}")]
    NodeNotInCluster { node: NodeName },

    #[error("invalid yggdrasil address: {0}")]
    InvalidYggAddress(String),

    #[error("invalid hex (expected {expected_len} chars): {got}")]
    InvalidHex { expected_len: usize, got: String },

    #[error("invalid base64: {0}")]
    InvalidBase64(String),

    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("nota: {0}")]
    Nota(#[from] nota_codec::Error),
}
```

## CLI

```
horizon-cli --cluster <CLUSTER> --node <NODE> [--format nota|json]
            < proposal.nota > horizon.<ext>
```

- Reads cluster proposal **nota** from stdin (always nota — that's the
  source-of-truth format).
- Writes the enriched horizon to stdout in the chosen format. `nota`
  is the default (human-readable, canonical). **`json` is the
  format Nix consumers ask for**, since `builtins.fromJSON` exists in
  Nix and `builtins.fromNota` does not. Same data, different
  encoding; both are produced by serde over the same `Horizon` types.
- Exit codes: `0` success, `1` projection error, `2` usage error.
- `clap` derive. `main` is the only free function in the binary.

The IFD wiring in `CriomOS/lib.mkHorizon` calls
`horizon-cli … --format json` and reads the result via
`builtins.fromJSON (builtins.readFile drv)`.

## Actors

None. horizon-cli is a one-shot pure function.

## Dependencies

- `nota-codec` (typed `Decoder` + `Encoder`) + `nota-derive` (six proc-macro derives) — nota I/O.
- `serde` (derive) + `serde_json` — JSON output mode (Nix consumption path).
- `thiserror` — Error enum derive.
- `clap` (derive) — CLI parsing.
- `std::net::Ipv6Addr` — `YggAddress`.
- `ipnet` — `NodeIp`.
- No `anyhow`, no `tokio`, no `rkyv`.

## Status

Phase 1 implemented (unit tests only). End-to-end: `horizon-cli
--cluster <C> --node <N>` reads cluster proposal nota on stdin and
writes enriched horizon nota on stdout. Integration tests against
goldragon's `datom.nota` are pending the goldragon nota conversion.

Next phases:

- Goldragon datom in nota: convert `goldragon/datom.nix` into a nota
  proposal that horizon-cli can consume directly.
- Nix consumer: wire CriomOS's `lib.mkHorizon` to invoke horizon-cli
  via an IFD derivation, replacing the pure-Nix horizon derivation.
