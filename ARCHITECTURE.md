# ARCHITECTURE — horizon-rs

The horizon projection library. Rust types and source files for
nixos modules; linked in-process by
[lojix-cli](https://github.com/LiGoldragon/lojix-cli)'s deploy path.

## Role

When forge-deploy materialises a CriomOS configuration, it walks
horizon-rs's projection types to compute the nixos-rebuild
inputs. Today, this is in-process — a Rust dep, not a daemon
boundary.

Detailed design lives in [`docs/DESIGN.md`](docs/DESIGN.md) and
[`docs/BUILD_CORES.md`](docs/BUILD_CORES.md).

## Boundaries

Owns:

- Projection types and projection helpers.
- A small CLI (under `cli/`) for ad-hoc projection.

Does not own:

- The deploy pipeline — that's
  [lojix-cli](https://github.com/LiGoldragon/lojix-cli).
- The nixos-rebuild driver — also forge.
- Sema records — though horizon-rs's role may eventually be
  absorbed into a records-authored projection over sema.

## Status

CANON. Active. Long-term: parts may migrate into forge's
in-process actors when the forge family unifies.

## Cross-cutting context

- Project-wide architecture:
  [criome/ARCHITECTURE.md](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
