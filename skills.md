# Skill — horizon-rs

*The horizon schema, type-checking, and method computation for
CriomOS. Reads a cluster proposal in nota; projects it from one
viewpoint `(cluster, node)`; emits an enriched horizon that
downstream Nix consumes verbatim.*

---

## What this skill is for

Use this when adding, modifying, or debugging horizon
projection. The repo owns:

- the typed proposal schema (`ClusterProposal`, `NodeProposal`,
  `UserProposal`, `ClusterTrust`, `Magnitude`, every `Species`
  enum) — the input shape goldragon emits;
- the projected schema (`Horizon`, `Node`, `User`, `Cluster`,
  `BehavesAs`, `TypeIs`, `ComputerIs`, `BuilderConfig`) — the
  output shape Nix consumes;
- the projection logic (`ClusterProposal::project`,
  `NodeProposal::project`, `UserProposal::project`,
  `Node::fill_viewpoint`);
- a small CLI (`horizon-cli`) for ad-hoc projection.

Architectural shape lives in `ARCHITECTURE.md`; the design and
the build-cores rationale live under `docs/`.

---

## Three layers, three shapes

Read in this order to understand the projection surface:

1. **Input — `proposal.rs`.** `NodeProposal`, `UserProposal`,
   `DomainProposal`, `ClusterTrust`. These mirror what
   goldragon's `datom.nota` files declare. Pass-through types,
   no derived fields.
2. **Method computation — `node.rs`, `user.rs`,
   `horizon.rs`.** Each `*Proposal::project` consumes the input
   and adds derived booleans (`is_remote_nix_builder`,
   `behaves_as`, `type_is`, `enable_linger`, etc.) plus
   typed identifiers (`criome_domain_name`, `system`,
   `nix_pub_key_line`).
3. **Output — `node.rs::Node`, `user.rs::User`,
   `cluster.rs::Cluster`, `horizon.rs::Horizon`.** Flat shape
   the Nix consumer reads via `builtins.fromJSON`. No method
   calls cross the boundary.

---

## The wire is the schema

Every public type that participates in the wire — every
proposal record, every projected record — is a typed Rust
struct or enum with `serde` derives plus the matching
`nota_codec::Nota*` derive (`NotaRecord`, `NotaEnum`,
`NotaTransparent`, `NotaTryTransparent`).

The same Rust definition serves three audiences:

- **goldragon's `datom.nota`** decodes via `NotaDecode`.
- **The projected horizon JSON** serialises via serde + the
  `#[serde(rename_all = "camelCase")]` attribute on output
  records.
- **Downstream Nix modules in CriomOS / CriomOS-home** read
  the JSON and gate their config on the typed booleans.

Reordering, renaming, or retypifying any field on a public
record is **a coordinated upstream-and-downstream change**:
goldragon's `datom.nota` files must update; CriomOS Nix
modules that read the JSON field must update; lojix-cli's
pinned `horizon-rs` rev must bump in lockstep.

---

## Field-add discipline

When extending `NodeProposal`, `UserProposal`, or any other
proposal record:

- **New fields go at the tail.** Positional Nota records
  parse by source-declaration order. Inserting a field in the
  middle is a wire break.
- **`#[serde(default)]` on every new field.** Existing
  `datom.nota` files must keep parsing without the new
  positional slot. Use `Option<T>` (default `None`),
  `Vec<T>` (default empty), or `bool` (default `false`) so
  the absence is meaningful.
- **Document the gate.** New fields that drive Nix config
  branches need a doc comment naming the consumer (e.g.
  "drives `nix.settings.maxJobs`") and the fallback when
  absent.

When extending an output record (`Node`, `User`,
`Cluster`):

- New fields appear in the projection output JSON. Nix
  consumers read them — coordinate the addition with the
  CriomOS / CriomOS-home modules that gate on them.
- Derived booleans go in the same family as their siblings
  — `is_*` predicates, `has_*_pub_key` shadows, `enable_*`
  switches, `handle_*` policy actions.

---

## The four-bucket sorter

Before adding any field to `ClusterProposal`, `NodeProposal`,
or any record reachable from them, name which of four buckets
the value lives in. **Only the first bucket lives on the
proposal surface.**

| Bucket | Lives in | Examples |
|---|---|---|
| **Cluster fact** | `ClusterProposal` / `NodeProposal` | Node inventory, trust, hardware, secret references, provider *selections*, regulatory country. |
| **Horizon constant** | pan-horizon authored config or `lib/src/horizon_constants.rs` | Internal DNS suffix (`criome`), public DNS suffix (`criome.net`), temporary exact IPv4 LAN. |
| **Horizon derivation** | `lib/src/view/` projection code | Node domain, tailnet base domain, router SSID, resolver listen addresses. |
| **CriomOS-side** | CriomOS Nix module default or catalog package | DNS upstream choice, AI runtime config, AI model catalog, NordVPN server catalog, lease TTL. |

The bucket rule, expanded:

1. **Variability.** Would another cluster owner author a
   different value here?
2. **Authority.** Is the cluster owner the authority — not the
   horizon operator, not CriomOS, not a provider?
3. **Non-derivable.** Must the projection be *told* this, or
   can it compute it from other authored data?

A "no" on any of these means the field doesn't belong in
`ClusterProposal`.

### Smells that mean you're misclassifying

- **"Replaces the literals scattered across CriomOS"** in a doc
  comment for a new `proposal/*.rs` record. The phrase usually
  means the literals belong in CriomOS defaults (or a CriomOS
  Nix package for catalog data), and what moves to horizon is
  the *projection that derives the value* — not the literal
  itself. A proposal record that transcribes the literals onto
  the cluster surface makes the cluster owner author the
  operating system.
- **Field whose value never varies across clusters in this
  horizon.** It's a horizon constant masquerading as a cluster
  fact (`domain = "criome"` for every cluster).
- **Field that carries the same value as another authored
  field plus a constant** (`tailnet.base_domain =
  "tailnet." + cluster_name + "." + cluster_domain`). It's a
  derivation masquerading as data; the projection should
  compute it.
- **Composite that mixes a cluster selection with a provider
  implementation.** An `AiProvider` carrying both `{ name,
  serving_node }` (cluster choice) and `{ models[], serving_config,
  protocol, port }` (CriomOS implementation) is two records
  glued together. Split along the bucket boundary; only the
  selection stays.
- **A wire newtype validating a value that won't be authored
  anymore.** When the value moves to derivation or CriomOS,
  the validation moves with it — the newtype retires from
  `proposal/*.rs`.

The full audit and lean-down plan that prompted this section
lives in
`~/primary/reports/designer/207-horizon-boundary-audit-and-lean-down-plan-2026-05-17.md`.
The pan-horizon-config brainstorm (a destination for several
of the **Horizon constant** values) lives in
`~/primary/reports/designer/208-pan-horizon-configuration-brainstorm-2026-05-17.md`.

---

## Magnitude is the size-and-trust ladder

`Magnitude` is a five-point ordinal scale:
`None < Min < Med < Large < Max`. It carries both `size`
(capacity) and `trust` (authority). Consumers don't see
`Magnitude` directly — they see `AtLeast`, the monotonic
ladder of booleans (`at_least_min`, `at_least_med`,
`at_least_large`, `at_least_max`).

The `at_least_*` shape is what gets written to JSON for
every `size` and `trust` field on `Node` and `User`. Nix
consumers branch on whichever threshold matches their need
without knowing the raw `Magnitude` variant — adding a new
`Magnitude` point would only require a new ladder field.

The `Magnitude::None` variant is not "missing data" — it is
the explicit zero-point on the scale. A node with `trust =
None` is *actively distrusted* and gets dropped from the
projected horizon entirely (per `horizon.rs::project`).

---

## Pod arch resolution

A `Machine` may have `arch: None` when its species is
`Pod` (a virtual machine hosted by a parent node). The
projection resolves the concrete arch by looking up the
super-node's arch (single hop; no chained pods). The
resolution lives in `node::resolve_arch`; it errors with
`UnresolvableArch` or `MissingSuperNode` rather than
silently defaulting.

---

## CLI is for ad-hoc projection only

`horizon-cli --cluster <C> --node <N> < proposal.nota` is a
debugging tool — it reads stdin, projects, prints JSON or
nota. The real consumer is `lojix-cli`, which links
`horizon-lib` in-process and feeds the projected horizon
into the Nix flake-input pipeline (per
[lojix-cli](https://github.com/LiGoldragon/lojix-cli)'s
`ARCHITECTURE.md`).

The Nota output mode is currently a stub
(`Format::Nota` → "not implemented"); JSON is the
production output format.

---

## Hard rules in this repo

- Methods on types — no free functions outside `main`. Per
  primary's `skills/abstractions.md`.
- Errors are one `thiserror` enum (`error::Error`); foreign
  errors wrap via `#[from]`. Per primary's
  `skills/rust-discipline.md`.
- Domain values are typed newtypes — `NodeName`,
  `ClusterName`, `Magnitude`, `SshPubKey`, `NixPubKey`,
  `YggAddress`, `NodeIp`. Bare `String` only inside a
  newtype's wrapped field.
- Tests live under `tests/`, never `#[cfg(test)] mod
  tests`. Per primary's `skills/rust-discipline.md`.
- Edition 2024.

---

## See also

- this repo's `ARCHITECTURE.md` — what the projection
  exists to do and what it does not own.
- `docs/DESIGN.md` — the projection spec.
- `docs/BUILD_CORES.md` — the build-cores derivation
  rationale.
- [lojix-cli](https://github.com/LiGoldragon/lojix-cli)'s
  `skills.md` — the consumer of this projection.
- primary's `skills/rust-discipline.md` — the Rust
  discipline this repo follows.
- primary's `skills/system-specialist.md` — the role that
  owns this repo.
- primary's `skills/autonomous-agent.md` — the gateway
  skill listing every required-read.
