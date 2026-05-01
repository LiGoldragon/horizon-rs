# Agent instructions — horizon-rs

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

You **MUST** read CriomOS's AGENTS.md (sibling repo) — CriomOS-cluster rules apply here.

## Repo role

Owns the horizon schema, type-checking, and method computation for CriomOS. Reads a cluster proposal in nota (from goldragon), projects it from a viewpoint `(cluster, node)`, emits an enriched horizon nota.

CLI: `horizon-cli --cluster <C> --node <N> < proposal.nota > horizon.nota`.

Spec: `docs/DESIGN.md`. Build-cores derivation rationale: `docs/BUILD_CORES.md`.

First thing: run `bd list --status open`.
