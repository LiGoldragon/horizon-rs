# Upgrades

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
