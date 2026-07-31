# Agent instructions — horizon-rs

You **MUST** read CriomOS's AGENTS.md (sibling repo) — CriomOS-cluster rules apply here.

## Repo role

Owns the horizon schema, type-checking, and method computation for CriomOS. Reads a cluster proposal in dotos (from goldragon), projects it from a viewpoint `(cluster, node)`, emits an enriched horizon dotos.

CLI: `horizon-cli --cluster <C> --node <N> < proposal.dotos > horizon.dotos`.

Spec: `docs/DESIGN.md`. Build-cores derivation rationale: `docs/BUILD_CORES.md`.

First thing: run `bd list --status open`.
