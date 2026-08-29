# Upgrades

## 0.5.1 — package the authored Ethos map with `horizon-lib`

`lib/ethos/horizon.ethos` now travels inside the published `horizon-lib`
package. This fixes fresh Cargo/Nix vendoring for the map-owned regeneration
test; no data or public type shape changes. Consumers should repin to 0.5.1
before building from a clean source closure.

## 0.5.0 — Datomic ClusterProposal boundary

Horizon no longer depends on or parses Dotos. The sole ClusterProposal input
is typed Datomic text, and the CLI reads it with `Text<ClusterProposal>`.
Replace `goldragon/datom.dotos` with `goldragon/proposal.datomic` atomically;
the formats are not compatible and there is no shipped compatibility decoder.

The one-time conversion used Horizon 0.4.0's pinned decoder to produce JSON,
normalised only the intentional `GitoliteServer {}` to `GitoliteServer` unit
variant API break, and emitted the new text through Horizon 0.5.0's Datomic
anatomy. Re-run the current CLI against every proposal viewpoint before moving
a consumer pin.

`synchronizer.dotos` was not ClusterProposal data. Its separate
SynchronizerConfig migration now owns `synchronizer.datomic`; Horizon neither
interprets nor rewrites that configuration root.

## 0.4.0 — remove Agent Intercom node services

`AgentIntercomLocal` and `AgentIntercomGraphical` are no longer valid
`NodeService` values. Remove both entries from every cluster proposal before
using Horizon 0.4.0. There is no compatibility decoder or projection path.

Configure Agent Intercom wrappers and integrations in their consumers. Configure
graphical facilities through the Edge capability; do not replace either removed
service with another Horizon node-service variant.
