# ARCHITECTURE — horizon-rs

The horizon projection library. Rust types and source files for
nixos modules; linked in-process by
[lojix](https://github.com/LiGoldragon/lojix)'s deploy path.

## Role

When lojix-deploy materialises a CriomOS configuration, it walks
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
  [lojix](https://github.com/LiGoldragon/lojix).
- The nixos-rebuild driver — also lojix.
- Sema records — though horizon-rs's role may eventually be
  absorbed into a records-authored projection over sema.

## Status

CANON. Active. Long-term: parts may migrate into lojixd's
in-process actors when the lojix family unifies.

## Cross-cutting context

- Project-wide architecture:
  [mentci-next/docs/architecture.md](https://github.com/LiGoldragon/mentci-next/blob/main/docs/architecture.md)
