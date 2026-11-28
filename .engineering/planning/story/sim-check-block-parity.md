---
format: aep.planning-md/3
id: story:sim-check-block-parity
kind: story
status: implemented
title: sim check refuses what the architecture block refuses
relations:
- decomposes: epic:simulator-and-search
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

sim check refuses empty_architecture, self_reference, not_a_channel, link_exists, gateway_without_model and acts_for_not_principal, as the architecture block does.

## Delivered

5731325, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of 5731325 pass in `slot task check`.
