# Architecture

The Ethos declaration defines the wire contract and generates public Rust
Datom types. `projection` owns typed decoding, deterministic composition,
generic-node resolution, VM validation, trust filtering, and JSON-ready
projection. `horizon-compose` materializes the sole consumer artifact from
two pinned input files; Lojix consumes that artifact through the library API.

The catalogue/membership split is deliberate: external configuration may
provide generic definitions, while only the cluster selects them. Machine host
choice separates physical nodes, locally hosted guests, and external VMs.
