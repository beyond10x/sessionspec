---
format: aep.planning-md/3
id: story:studio-caller-header
kind: story
status: implemented
title: The studio records a call as the sender named by X-Studio-Caller; check:mutation runs grant-add
relations:
- decomposes: epic:agenticstudio
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:23Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

A body naming another caller than X-Studio-Caller is 403 (Breaking for the studio HTTP surface); the grant-add mutant class runs in check:mutation.

## Delivered

06e718d, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of 06e718d pass in `slot task check`.
