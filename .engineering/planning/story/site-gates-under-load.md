---
format: aep.planning-md/3
id: story:site-gates-under-load
kind: story
status: implemented
title: Two site gates hold on a loaded machine
relations:
- decomposes: epic:docs-site
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:24Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

SUF-008 waits for the region attributes set after hydration and the page sweep waits for a frame's title, so both pass under load and still fail on a page that never sets them.

## Delivered

5a97fcd, in batch 7 (integration/2026-11-28-e, PR #47, merge d34ea91).

## Acceptance

- The tests named in the commit messages of 5a97fcd pass in `slot task check`.
