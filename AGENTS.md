# Agent instructions — horizon-rs

You **MUST** read CriomOS's AGENTS.md (sibling repo) — CriomOS-cluster rules apply here.

## Repo role

Owns the horizon schema, type-checking, and method computation for CriomOS. Reads a cluster proposal in Datomic (from goldragon), projects it from a viewpoint `(cluster, node)`, emits an enriched horizon Datomic.

CLI: `horizon-cli --cluster <C> --node <N> < proposal.Datomic > horizon.Datomic`.

Spec: `docs/DESIGN.md`. Build-cores derivation rationale: `docs/BUILD_CORES.md`.

First thing: run `bd list --status open`.

## Protos estate status

Stack: correct-new destination
Status: active component, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
