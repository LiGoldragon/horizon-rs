# Upgrades

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
