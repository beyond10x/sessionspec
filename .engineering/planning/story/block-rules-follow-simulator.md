---
format: aep.planning-md/3
id: story:block-rules-follow-simulator
kind: story
status: implemented
title: Two architecture block rules follow the simulator (D1 = A)
relations:
- decomposes: epic:building-blocks
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

NotReachedByPrincipal refuses a border part only where the architecture has a principal and counts a Grant link as reach; QuestionWithoutAnswerer refuses only past one forwarding hop; sim check gains not_reached_by_principal.

## Delivered

a605d99, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of a605d99 pass in `slot task check`.
