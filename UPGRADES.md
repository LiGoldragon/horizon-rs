# Upgrades

## 0.10.1 to 0.11.0

The producer chain repins to the arity-split substrate: `datom-codec`
`6dccc76b75918a91d3370a9d4fe88aa7dd567876` (0.27.0, was `627db67f2655…` /
0.26.3) and `ethos-zero` `b232d35e03011161fe7ec9129ad99a9914413348` (9.0.0,
was `de3d9928b156…` / 8.0.1). `protos` is unchanged at
`171b21f65337983ab624b7b906397a4f1f92c5a3` (0.30.1). Horizon declares no
`signal` dependency, so that repin does not apply here.

`datom-codec` 0.27.0 gives arity back to `Compositional` — it now carries
`const ARITY` and `from_positions`, and names the kind a datom composes into
`Composing`. `ethos-zero` 9.0.0 emits the renamed derive, so every committed
ethos-zero-generated file changed: `lib/src/generated/horizon.rs` no longer
wraps its `#[cfg_attr(feature = "datom", derive(...))]` onto four lines and
now derives `datom_codec::Composing` in place of `datom_codec::Compositional`.
`lib/build.rs` asserts the regeneration is byte-identical to the committed
file on every build.

This is a breaking change to Horizon's own public Rust surface: the
`datom`-feature derive on every generated type changes name, and
`horizon_lib::DatomDecoding`'s supertrait bound moves from
`datom_codec::Compositional` to `datom_codec::Composing`. A consumer naming
`datom_codec::Compositional` against a Horizon generated type, or bounding
against it directly, must move to `datom_codec::Composing`. The Datom wire
form and the authored contract are unchanged.

## 0.10.0 to 0.10.1

The producer chain settles on its final heads: `protos`
`171b21f65337983ab624b7b906397a4f1f92c5a3` (0.30.1), `datom-codec`
`627db67f2655efd9f786864009955005fd8ab2ad` (0.26.3), `ethos-zero`
`de3d9928b156f2e1a92d060b7817af201abfdbef` (8.0.1). No authored type, no Datom
wire form and no Rust surface changed: `lib/src/generated/horizon.rs`
regenerates byte-identical under ethos-zero 8.0.1, which `lib/build.rs` asserts
on every build.

`Cargo.lock` also collapses to one revision of each of our crates. It had
carried two `datom-codec` 0.25.7 entries — the one this crate pinned and a
second, `f2cc06858d38a4028c928d33323a6d682e7c222f`, reached through
ethos-zero 6.1.6. A consumer taking this revision inherits a single codec.

Consumers repin the revision and change nothing else.

## 0.9.0 to 0.10.0

Horizon's production Rust now homes every verb in a trait: `lib/src` and
`cli/src` carry no module-level free function but `fn main`, and no inherent
`impl` block. Two Nix checks, `no-free-functions` and `no-inherent-methods`,
enforce this. Nothing about the Datom wire form, the authored contract, or the
projected JSON changed; only the Rust surface moved.

Mechanical consumer migration:

- `horizon_lib::decode(text)` becomes `HorizonDefinition::decode(text)`,
  `horizon_lib::decode_configuration(text)` becomes
  `HorizonConfiguration::decode(text)`, `horizon_lib::decode_cluster(text)`
  becomes `ClusterDefinition::decode(text)`, and
  `horizon_lib::decode_composition_request(text)` becomes
  `CompositionCommand::decode(text)`. All four are the one method of the new
  `horizon_lib::DatomDecoding` trait, which must be in scope; each document
  kind carries its own `DatomDecoding::BUDGET`.
- `horizon_lib::compose(configuration, cluster)` becomes
  `configuration.compose(cluster)`, the one method of the new
  `horizon_lib::Composing` trait on `HorizonConfiguration`.
- `HorizonDefinition::resolve` and `HorizonDefinition::project` are no longer
  inherent. They are the two methods of the new `horizon_lib::Projecting`
  trait; call sites are unchanged, but the trait must be in scope. A consumer
  importing `horizon_lib::*` needs no edit; one naming
  `use horizon_lib::HorizonDefinition;` must add `Projecting`.
- `horizon_lib::NodeDefinitions` names the `BTreeMap<String, NodeDefinition>`
  that `ResolvedCluster::nodes` has always been; the field's type is
  unchanged.

Signatures, error variants, and every generated and view type are unchanged.

## 0.8.0 to 0.9.0

`NodeDefinition` gains a trailing optional `FixedLocation` value. Migrate every
authored node by appending `None`, except nodes with a deliberate static
GeoClue override, which append `Some.{ latitude longitude altitude accuracy }`.
Latitude and longitude are Decimal degrees; altitude and accuracy are Decimal
metres. This is an operator-declared fixed city position, not a measured device
position.

Publish the Horizon revision and update every direct `horizon-lib` consumer
before supplying a migrated definition to Lojix. The active 0.21 Lojix route
accepts only an externally composed `horizon-definition.datom`; legacy
`proposal.datom` is not that artifact and must not be used as a substitute.

## 0.7.0 — Current structural Datom stack

- Migrates the authored Horizon Library contract to Ethos Zero 6.1.6 and its named generated fields.
- Replaces the retired Protos text wrapper and direct textualization with the explicit Datom → Protos → text conversion chain.
- Pins Protos 0.29.1 and Datom 0.25.6 while preserving Horizon definition validation and projection behavior.

## 0.6.0 — Datom composition and node definitions

This release replaces the legacy `ClusterProposal`/Datomic API with generated
Datom `HorizonConfiguration`, `ClusterDefinition`, and `HorizonDefinition`.
Publish the external configuration and cluster definition as separate pinned
inputs, materialize `horizon-definition.datom` with `horizon-compose`, then
have consumers decode that one file and call `HorizonDefinition::project`.

Existing `MachineSpecies::Pod` data must migrate to `VirtualMachine` with a
`Cluster` host choice. Do not migrate it to Container. Persistent layout goes
only in `Installation`; Live nodes retain environment/network/key/capability
facts without disk layout.
