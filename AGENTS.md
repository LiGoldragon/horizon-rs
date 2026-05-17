# Agent instructions — horizon-rs

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

You **MUST** read CriomOS's AGENTS.md (sibling repo) — CriomOS-cluster rules apply here.

## Repo role

Owns the horizon schema, type-checking, and method computation for
CriomOS. Reads a pan-horizon `HorizonProposal`, a cluster
`ClusterProposal`, and a caller-supplied `Viewpoint { cluster, node }`;
emits the viewpoint-scoped JSON horizon consumed by CriomOS,
CriomOS-home, and `lojix-daemon`.

CLI: `horizon-cli --horizon <horizon.nota> --proposal <datom.nota> --cluster <C> --node <N>`.

Spec: `ARCHITECTURE.md`. Older files under `docs/` are historical
unless `ARCHITECTURE.md` or `skills.md` names a section as current.
Build-cores derivation rationale: `docs/BUILD_CORES.md`.

First thing: run `bd list --status open`.
