# Agent Bootstrap — horizon-rs

## First thing

Run `bd list --status open` to see what's already on the table.

## Scope

Owns the horizon schema, type-checking, and method computation for
CriomOS. Reads a cluster proposal in TOML (from goldragon), projects it
from a viewpoint `(cluster, node)`, and emits an enriched horizon TOML.

Spec: [docs/DESIGN.md](docs/DESIGN.md).
Build-cores derivation rationale: [docs/BUILD_CORES.md](docs/BUILD_CORES.md).

CLI: `horizon-cli --cluster <C> --node <N> < proposal.toml > horizon.toml`.

## Rust style

Follow [~/git/tools-documentation/rust/style.md](../tools-documentation/rust/style.md):
methods on types, typed newtypes, single-object I/O, `thiserror`-derived
`Error` (no `anyhow`, no `eyre`), trait-domain rule.

## Hard process rules

- Jujutsu only. Never `git` CLI.
- Push immediately after every change.
- Mentci three-tuple commit format:
  `(("CommitType", "scope"), ("Action", "what"), ("Verdict", "why"))`.
