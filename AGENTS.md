# Agent Bootstrap — horizon-rs

## Rust style

Follow [`~/git/tools-documentation/rust/style.md`](../tools-documentation/rust/style.md):
methods on types, typed newtypes for domain values (`ClusterName`,
`NodeName`, `Md5`, `YggAddress`, …), single-object I/O at every public
boundary, manual `Error` enum (no thiserror/anyhow), trait-domain rule.

Stub Rust currently in `lib/src/*.rs` is pre-style; rewrite when porting.

## Scope

Owns the horizon schema, type-checking, and method computation for
CriomOS. Reads raw horizon JSON (from `maisiliym`-style cluster
proposals), validates it, produces enriched horizon JSON with the full
`methods.*` DAG.

Reference: `~/git/CriomOS/docs/HORIZON.md` is the spec for what `methods.*`
must contain. `example-horizon.json` in this repo is the golden output for
ouranos@maisiliym.

Phase 1 is the CLI: `horizon-cli --cluster X --node Y < raw.json > horizon.json`.
Library and broader API come later.

## Hard process rules

- Jujutsu only. Never `git` CLI.
- Push immediately after every change.
- Mentci three-tuple commit format:
  `(("CommitType", "scope"), ("Action", "what"), ("Verdict", "why"))`.
