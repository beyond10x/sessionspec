---
format: aep.planning-md/3
id: story:settings-canonical-spelling
kind: story
status: implemented
title: Settings read in one canonical spelling and checked as the simulator checks them
relations:
- decomposes: epic:simulator-and-search
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:22Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:22Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:22Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

An enum setting in the block's spelling is read as its canonical spelling everywhere, also by sim check; rate_limit_per_minute keeps its fraction; count, capacity and job_limit start at 1; model_share and the settings table are checked when read.

## Delivered

54eb347, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of 54eb347 pass in `slot task check`.
