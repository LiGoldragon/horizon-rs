# Upgrades

## 0.4.0 — remove Agent Intercom node services

`AgentIntercomLocal` and `AgentIntercomGraphical` are no longer valid
`NodeService` values. Remove both entries from every cluster proposal before
using Horizon 0.4.0. There is no compatibility decoder or projection path.

Configure Agent Intercom wrappers and integrations in their consumers. Configure
graphical facilities through the Edge capability; do not replace either removed
service with another Horizon node-service variant.
