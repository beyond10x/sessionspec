---
format: aep.planning-md/3
id: story:renderer-reference-checks
kind: story
status: implemented
title: The renderer refuses a port, mark or node region that names nothing
relations:
- decomposes: epic:docs-site
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:21Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:21Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:21Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

Page::check refuses port_node_unknown, mark_target_unknown and node_region_unknown; a mark may target a region; ui derive gives an Inbox to every principal kind of the model.

## Delivered

e560fbf, ea2b01a, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of e560fbf, ea2b01a pass in `slot task check`.
