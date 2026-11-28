---
format: aep.planning-md/3
id: story:blocks-adversarial-guards
kind: story
status: implemented
title: Guards from the adversarial review of the blocks
relations:
- decomposes: epic:building-blocks
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

A workspace proposal needs the lease holder; nothing is written or proposed past a lease's expiry; a proposal does not supersede itself; an unreadable answer after the deadline, an Allow under an approval grant, a schedule moved back and an unknown tool call are refused.

## Delivered

26a6bf3, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of 26a6bf3 pass in `slot task check`.
