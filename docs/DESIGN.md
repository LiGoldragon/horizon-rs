# Horizon Datom contract

Horizon materializes exactly one `HorizonDefinition` from two explicit typed
documents. `HorizonConfiguration` is an external catalogue of generic nodes
and domain configuration. `ClusterDefinition` owns cluster-local nodes,
selected generic node names, users, domains, and trust. A catalogue entry is
not a cluster member until its name appears in `generic_node_names`.

`horizon-compose 'Compose.{ configuration.datom cluster-definition.datom }'`
decodes both documents, rejects unknown/duplicate selections and name
collisions, and writes canonical `HorizonDefinition` Datom. Consumers decode
that one file with `horizon_lib::decode`, then call
`definition.project(node_name)`.

`NodeDefinition` has a `Live` or `Installation` variant. Only Installation
carries bootloader, persistent disks, and persistent swap. Keyboard and
compressed swap live in `NodeEnvironment`, so a Live installer retains both.
Node capabilities carry configured services and decomposed profile behavior;
they do not infer roles from names. `MachineDefinition` distinguishes Metal,
cluster-hosted VMs, and explicitly externally hosted VMs. The old Pod machine
was a VM guest and maps to `VirtualMachine`, never Container.

The generated Rust types are produced from `lib/ethos/horizon.ethos` using the
pinned Ethos generator. The contract suite round-trips the full network,
service, machine, user, domain, trust, and installation payload and exercises
generic-node selection, collisions, VM host validation, and composition.
