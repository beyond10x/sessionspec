---
format: aep.planning-md/3
id: story:sim-score
kind: story
status: implemented
title: 'Score one architecture for a shared search: sim score'
relations:
- decomposes: epic:simulator-and-search
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-11-28T20:14:19Z", actor: "human:operator", revision: 2, decided_on: {"recorded":{"test_result":1}}}
- {from: "proposed", to: "active", at: "2026-11-28T20:14:20Z", actor: "human:operator", revision: 3, decided_on: {"recorded":{"test_result":1}}}
- {from: "active", to: "implemented", at: "2026-11-28T20:14:20Z", actor: "human:operator", revision: 4, decided_on: {"recorded":{"test_result":1}}}
---
## Outcome

A shared search tool scores one candidate architecture of this repository with one call:
`agenticspec sim score --profile <profile.yaml> --seed <n> -- <architecture.yaml>` simulates the
document once, as written (a marked place `{value, vary}` at its `value`, no search), and prints its
KPIs as JSON on standard output. The search itself lives in the shared tool; this repository only
scores.

## Scope

- `spec/cli.yaml`, `spec/domains/cli.yaml`: the command `sim score`, its input, result and refusals
  (cited: the other `sim` commands are declared there).
- `crates/agenticspec-cli-contract/`: regenerated, never edited (cited).
- `crates/agenticspec-sim/src/score.rs` (new), `crates/agenticspec-sim/src/cli.rs`: the score of one
  document; the baseline of `search.rs` rebuilt from its public parts, `search.rs`, `best.rs` and
  `ladder.rs` unchanged (inferred: `Experiment::model` and `run_seed` are private).
- `crates/agenticspec/src/main.rs`: the handler, the error object on standard output and the exit
  code of a refused document (inferred: the generated adapter knows only exit 0, 1, 2 and 130).
- `docs/`, `CHANGELOG.md`, `AGENTS.md`: the command, the JSON shape and a manifest fragment.

## Contract

- Arguments: the fixed arguments first (`--profile`, `--seed`, optional `--horizon`, `--batch`),
  then the documents after `--`; one document may also be given without `--`.
- Answer of one document: `{"kpis": {<id>: <number>}, "per_part": {<part>: {<id>: <number>}}}`,
  the ids those of `kpis/catalogue.yaml`; a simulator count or gate count the catalogue does not
  list keeps the name `sim run` gives it.
- Refused document: `{"error": {"code": <code>, "message": <text>}}` on standard output, exit 3;
  the codes of `sim check` (`document_unreadable`, `architecture_refused`, `ports_unreadable`,
  `kpis_unreadable`) and `simulation_refused`.
- A refusal of the fixed arguments (profile, horizon): the same object, exit 1.
- Batch (`--batch`, or more than one document): `{"results": [<answer>, ...]}` in the order of the
  paths, each entry byte-equal to the answer of its path alone, exit 0.

## Acceptance

- `sim score` of a committed architecture prints its KPIs and per-part KPIs, exit 0.
- The KPIs equal those `sim run` reports for the baseline of an experiment over the same
  architecture and profile with `seed: <n>` and `seeds: 1`.
- The same document and arguments give byte-equal standard output.
- A document `sim check` refuses answers the error object with exit 3.
- A batch answers one entry per path, in order, each byte-equal to the single run of that path,
  a refused document as an entry holding its error.
- Each test ends within 60 s; the crate's test step stays within its budget.
