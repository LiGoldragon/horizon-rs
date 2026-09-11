# Upgrades

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
