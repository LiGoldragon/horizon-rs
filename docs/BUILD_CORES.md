# Build cores: `max_jobs` + `build_cores` derivation

Background research feeding the `nix_concurrency` function in
[lib/src/node.rs](/home/li/git/horizon-rs/lib/src/node.rs).

## The two knobs in nix

Three settings interact; horizon-rs derives the first two per builder.

| Setting | What | Source |
|-|-|-|
| `nix.buildMachines.<n>.maxJobs` | Parallel-derivations cap on this builder. Nix dispatches at most this many derivations at once to it. Default `1`. | [Nix manual: distributed-builds](https://nix.dev/manual/nix/2.24/advanced-topics/distributed-builds.html) |
| `nix.settings.cores` | `NIX_BUILD_CORES` ambient on this builder. Per-derivation `make -j N`. `0` means "use all cores". | [Nix manual: cores-vs-jobs](https://nix.dev/manual/nix/2.24/advanced-topics/cores-vs-jobs) |
| nixpkgs `enableParallelBuilding` | Per-derivation opt-in that honors `NIX_BUILD_CORES`. | nixpkgs convention |

Total CPU pressure on a builder is approximately `maxJobs × build_cores`
(or `maxJobs × nproc` when `build_cores = 0`). The manual warns that
over-selling beyond physical cores degrades through context-switching.

## What community setups do

- `maxJobs = nproc, cores = 0` on a dedicated builder.
- `maxJobs = nproc / 2, cores = 0` on a workstation also used interactively.
- `maxJobs = 1, cores = 0` on tiny VMs / pods.
- RAM rule of thumb: budget 2–4 GB per concurrent C++/Rust job.
  Cap `maxJobs ≤ ram_gb / 4` on memory-constrained nodes.

## The horizon-rs formula

```rust
fn nix_concurrency(cores: u32, behaves_as_center: bool, size: Magnitude) -> (u32, u32) {
    let max_jobs = if cores <= 1 {
        1
    } else if matches!(size, Magnitude::None | Magnitude::Min) {
        1
    } else if behaves_as_center {
        cores
    } else {
        (cores / 2).max(1)
    };
    (max_jobs, 0)
}
```

`build_cores = 0` universally — every derivation gets the whole box via
`NIX_BUILD_CORES=0`. `enableParallelBuilding` does the right thing.

`max_jobs`:

- `cores <= 1` → `1` (pods, tiny VMs).
- `size = None | Min` → `1` (the node isn't meant to carry build load).
- `behaves_as.center` (large_ai, large_ai_router, center) → `cores`
  (dedicated builder; saturate it).
- Otherwise (interactive edge / hybrid) → `cores / 2` (leave headroom
  for the human at the keyboard).

## Cluster numbers

Applied to current cluster shapes:

| Node | Cores | Species | Size | `max_jobs` | Note |
|-|-|-|-|-|-|
| prometheus | 8 | largeAIRouter | max | 8 | dedicated AI/build host |
| ouranos | 12 | edgeTesting | max | 6 | interactive laptop |
| tiger | 4 | edgeTesting | max | 2 | interactive laptop |
| xerxes | 2 | hybrid | max | 1 | interactive laptop |
| zeus | 4 | edge | max | 2 | interactive laptop |
| klio | 2 | edge | max | 1 | interactive laptop |
| balboa | 4 | center | none | 1 | size=none → 1 regardless |
| asklepios | 1 | hybrid | min | 1 | live USB |
| eibetik | 1 | center | min | 1 | pod |

## Future extension: RAM cap

The formula doesn't yet cap by RAM. Add when memory-pressure incidents
appear:

```rust
let ram_cap = ram_gb / 4;          // ~4 GB per concurrent job
max_jobs = max_jobs.min(ram_cap);
```

Needs `ram_gb: u32` in `Machine`. Not in the input schema today.

## Sources

- [Nix manual: cores-vs-jobs](https://nix.dev/manual/nix/2.24/advanced-topics/cores-vs-jobs)
- [Nix manual: nix.conf (max-jobs, cores, builders)](https://nix.dev/manual/nix/2.24/command-ref/conf-file.html)
- [Nix manual: Remote builds](https://nix.dev/manual/nix/2.24/advanced-topics/distributed-builds.html)
- [nix.dev tutorial: Setting up distributed builds](https://nix.dev/tutorials/nixos/distributed-builds-setup.html)
- [NixOS Wiki: Distributed build](https://wiki.nixos.org/wiki/Distributed_build)
- [NixOS & Flakes Book: Distributed Building](https://nixos-and-flakes.thiscute.world/development/distributed-building)
