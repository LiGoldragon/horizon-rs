# Horizon agent notes

Horizon owns the typed Datom boundary between externally authored generic node
catalogues, cluster-owned membership/facts, and consumers such as Lojix.

`HorizonConfiguration` carries generic node definitions and domain settings.
`ClusterDefinition` carries local nodes, explicit selected generic names,
users, domains, and trust. `horizon-compose` accepts exactly one typed
`Compose.{ configuration-path cluster-definition-path }` request and writes
the validated one-file `HorizonDefinition`. `HorizonDefinition::project(node)`
resolves only selected generic names before producing the selected projection.

The generated source in `lib/src/generated/` comes only from
`lib/ethos/horizon.ethos` through the pinned Ethos generator. Do not edit it
by hand. `MachineSpecies::Pod` migrated to `VirtualMachine`; there is no
Container variant. Live definitions have no persistent layout, while keyboard
and compressed swap remain node environment facts for both variants.
