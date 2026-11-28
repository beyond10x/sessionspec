# AGENTS.md — agenticspec

agenticspec lifts agent sessions of any harness into one IR (actors, commands, decisions, events,
state over time), redacts it into a persistable form, and evaluates KPIs on it. User
documentation: `docs/`. Internal design notes: `design/` (`design/design.md` first). Baseline
numbers: `research/`.

Organization-wide rules live in the workspace `AGENTS.md` (in the directory that holds the
organization's repositories side by side); this file adds what is specific to agenticspec.

This repository is public. No person's name, e-mail address, host path, private repository,
session name, employer or customer goes into a tracked file, a commit message or a fixture.

Documents state the product: what it does and why, in its own words. They never quote the owner or
a session, never cite a chat as the reason for a design, and carry no decision history ("accepted
on", "opened by"). Where a decision came from belongs in the commit message, not in a file the docs
site publishes.

`docs/` is end-user documentation and the only source of the docs site: what a user installs,
runs and reads, in plain words. No `§` references, no `file:line`, no internal module names a user
does not type, no workstation numbers, no design rationale. Design notes, decisions and
measurements go in `design/` and `research/`, which the site does not publish. A change to a
command, a flag, a KPI, an sq relation or a stored field updates its page in `docs/` in the same
change.

## Serves

- **O3 — any harness, observed and compared.** A session of any harness is lifted into one IR
  (actors, commands, decisions, events), so runs are recorded in one vocabulary and compared on the
  same KPIs.
- **O6 — self-improvement, built into all of it.** KPIs, sq checks and mined patterns over the
  store say where agent work loses time, tokens or the owner's attention, and re-measure after a
  change.
- **O2 — decisions as data, with evidence.** Every permission decision and every prompt carries
  who made it and how sure the record is (`Known`, `Inferred`, `NotRecorded`).

## Specification first (ESS)

- Every noun is declared in `spec/` before code uses it; `task check:spec` passes first.
- `spec/domains/session.yaml` is the session IR. `spec/domains/cli.yaml` and the `ess-cli/2`
  binding `spec/cli.yaml` are the command line, including the typed KPI report.
- Generated, never edited by hand, checked byte for byte by `task check:generated`:
  - `generated/`: `ess generate synthesize --target rust` (IR types, command behaviours, system,
    wire codecs). Its own workspace, excluded from the root one.
  - `crates/agenticspec-cli-contract/`: `ess generate cli`.
- After a change of `spec/`: `task generate`, then fix what the compiler names.
- Authored scenarios of `spec/` are in `spec/scenarios/`, listed in `spec/ess-inputs.yaml`;
  `task check:mutation` runs them against every mutant of `spec/`.
- `ui/agenticspec.ui.yaml` is the UI as an `ess-ui/1` document over the views of
  `agenticspec.view`, which the `session-view` component serves; `task check:ui` checks it against
  `spec/` and runs `ui/tests/` on the fixtures in `ui/fixtures/` (format: `ui/README.md`).

## Building blocks (`blocks/`) and their layers

- `blocks/` is a second ESS system, `agentic`: one domain per block, one directory per layer
  (`0-foundation` to `10-evaluation`). Design: `design/building-blocks.md`.
- Dependencies point down only. A domain depends on another through a type of it, the identifier
  of one of its entities (identifiers live in `agentic.shared`, layer 0, but count for the domain
  whose entity they identify), or a relation. `scripts/check-layers.sh` refuses:
  - a dependency on a higher layer;
  - a cycle between domains of one layer;
  - a domain outside a numbered layer directory.
- When a lower block needs to name a higher one, turn the edge around: the higher block holds
  the identifier of the lower one (an agent revision lists its knowledge; knowledge does not name
  agents).
- `task check:layers` runs the check and its negative fixtures (`scripts/fixtures/layers/`, each
  must fail). `task layers:graph` prints the dependency graph with counts.
- A new identifier goes into `agentic.shared` in the same change as the entity it identifies.
- `blocks/components.yaml` declares one component, `blocks`, that owns every domain and is
  `reached_by: network`, so an `ess-ui/1` document over `agentic` reads its views. Every view is
  readable by some actor: a domain whose actors may none of its commands has a `Viewer` that may
  its views (`ess ui check` refuses a section over a view no actor may read). A new view goes into
  such a `Viewer`'s `may` in the same change.
- The block panels (`ui/views/`) are generated from `blocks/` by `agenticspec ui panels`
  (`task ui:panels`): a change of `blocks/` that changes a view, an event or a type runs it in the
  same change, and `task check:ui` fails on any byte difference or a digest that no longer
  matches.
- `task check:blocks` runs the conformance suite of `blocks/` with every authored scenario that
  `blocks/ess-inputs.yaml` lists (`scripts/check-blocks-conform.sh`). Synthesis runs once per
  digest of those sources and is kept in `target/blocks-conform/` (the suite is about 9 MB, too
  large to commit); a run on the cached suite takes seconds. `task slow:blocks` synthesizes afresh
  and compares bytes with the cached suite.
- The docs pages of the blocks are generated by `crates/agenticspec-docs` from `blocks/`,
  `kpis/catalogue.yaml`, `architectures/` and the experiments that name them: `docs/blocks/`,
  `docs/architectures/`, `docs/kpis-catalogue.md`, `website/static/architectures/` and
  `website/src/generated/`. Never edited by hand: a change of those sources runs
  `task docs:generate` in the same change, and `task check:docs` (in the gate) fails on any byte
  difference. Generated pages carry their route and header in their own front matter, not in
  `pages` of `docusaurus.config.ts`, and draw with the components in `website/src/components/blocks/`.
  The short optimization run on an architecture page uses a fixed budget (`DOCS_BUDGET`), so the
  page changes when the simulator changes.
- The catalog (`docs/catalog/`, `website/static/catalog/<architecture>/`) is generated the same
  way, one entry per experiment: its full result page, the downloads (the files as written,
  `best.yaml` from `agenticspec_sim::best`, the experiment pointing at the files beside it,
  `result.json`) and the index cards. The results of the experiments at their own budget and seeds
  are committed inputs: `website/static/catalog/<entry>/result.json` with `sources.sha256` beside
  it (the experiment, architecture, profile, the simulator's sources that change a result, the
  part kinds and `carries` values of the architecture block, the names and types of
  `blocks/settings.yaml`, its dependency versions in
  `Cargo.lock`, `rust-toolchain.toml`). `task docs:generate` never runs a
  full experiment and refuses a result whose digest file no longer matches; `task catalog:generate`
  runs them (and refuses a `best.yaml` that does not score the best fitness), then
  `task docs:generate`; `task slow:catalog` runs them afresh and compares bytes. The catalog
  components are in `website/src/components/catalog/`; a ladder stage on a card comes from
  `STAGES` in `catalog.rs`; the sortable table of the architectures as a whole on the index is
  `ArchitectureKpis.tsx`, from `agenticspec_sim::whole`.
- Cross-block causation is declared as ESS bindings in `blocks/components.yaml` (one component
  owns every block); `scripts/check-layers.sh` counts a binding as a dependency of the event's
  domain on the command's, and `scripts/check-blocks-conform.sh` fails when a binding has no
  `<binding>/binding/...` scenario. A causation ESS cannot express gets a reproduction under
  `design/ess-repro/<gap>/` (a header comment says what `ess specify validate` answers).
- That component is `reached_by: network` and `accepts` each entry command an ingress may export:
  every command whose role in `blocks/ports.json` is intent or administration, or that the
  actor `agentic.architecture.Principal` may issue, and that some actor may issue (a test in
  `tests/cases/control_plane.rs` holds the list). A new such command goes into `accepts` in the
  same change.
- `blocks/ports.json` (`agenticspec-ports/1`) holds the ports derived from the compiled blocks
  and the surface an ingress selects from: each command's role, each domain's views and events,
  each actor's `may:` (`agenticspec sim ports`, `docs/contract.md`). A change of a file
  `blocks/ess-inputs.yaml` lists runs `task ports:generate` in the same change; `task check:ports`
  (in the gate) compares the digest without `ess`, `task slow:ports` derives afresh. The actors
  `agentic.architecture.Deployer` and `agentic.architecture.Principal` decide configuration and
  principal ports; a new command a deployer issues to end something goes into the deployer's
  `may:`.
- Each architecture page and catalog entry has a "Contract" section and a contract file
  (`website/static/architectures/<name>.contract.yaml`, `<entry>/<entry>.contract.yaml`), from
  `agenticspec_sim::contract::derive` over the committed `blocks/ports.json`.

## Packs (`packs/`) and fields (`fields/`)

- Three tiers of the `agentic` system, each in its own directories, dependencies only downward:
  - core: `blocks/`, what every agentic system has; it validates alone from
    `blocks/ess-inputs.yaml`;
  - packs: `packs/<pack>/` (`pack.yaml`, `blocks/`, `scenarios/`, `kpis.yaml`), concepts more
    than one field reuses, in the namespace `agentic.<word>` (`source-control` is `agentic.scm`,
    then `tracking`, `files`, `shell`); a pack depends on the core and on the packs its
    `pack.yaml` lists under `uses:`;
  - fields: `fields/<field>/`, one kind of work; a field lists the packs it uses and adds a block
    only for a concept no other field shares. `field.yaml` (`agenticspec-field/1`) lists its
    `packs`, `architectures`, `profiles` (behaviour profiles, the simulator's input), `kpis`
    (its catalogue), `rows` (`agenticspec-rows/1`: rows it registers in core catalog entities, as
    inputs of a core register command) and `words` (the words only it uses).
- The rule of the tiers: a term only one field uses is never an enum variant, entity or field of
  a core block. Where a core block must tell such values apart, it holds a catalog entity whose
  rows a field registers (`agentic.harness.PermissionMode`; the coding field registers
  `accept-edits`). The test for a new core term: would a security-review or AI-research setup need
  it under the same name? If not, it is a row, a KPI or a profile of a field.
- `scripts/check-neutral.sh` (`task check:neutral`, in `check:blocks`) refuses every word of a
  field's `words.txt` in `blocks/` and `kpis/catalogue.yaml`; each fixture under
  `scripts/fixtures/neutral/` must fail. A new coding term in a core block or core KPI goes into
  the field, or gets a neutral name in the core.
- Every experiment under `experiments/` names its `field:` and runs a profile that field lists
  (`scripts/check-packs.sh`); the catalog of the docs groups its entries by field.
- The root `ess-inputs.yaml` is the whole system: `packs/system.yaml` (the core domains and the
  pack domains), every core file, every pack file and the packs' authored scenarios. It pins
  `ess 0.99` (the minor line), so it does not take over every `ess` run at the root. A new block of
  a pack goes into its `pack.yaml`, the root `ess-inputs.yaml` and `packs/system.yaml` in one
  change; `scripts/check-packs.sh` holds the three together.
- `scripts/check-layers.sh --path .` refuses a core block that depends on a pack or a field, a
  pack that depends on a field or on a pack its `pack.yaml` does not list, and a field that
  depends on another field. The fixtures under `scripts/fixtures/layers/` fail, those under
  `scripts/fixtures/layers-pass/` pass.
- No product names under `packs/` or `fields/`: a forge, an issue tracker or an agent product is a
  binding, never a block. `scripts/check-packs.sh` greps for them.
- `task check:fields` validates the whole system, checks the manifests, runs its conformance suite
  (`scripts/check-blocks-conform.sh --path .`, cached in `target/system-conform/`; synthesis about
  70-100 s on a change of its sources, a run on the cached suite seconds) and runs
  `agenticspec sim check` on every architecture a field lists. `packs/known-synthesis-defects.txt`
  lists the synthesized scenarios ESS 0.99.2's interpreter fails because synthesis arranged them
  wrong; each names the authored scenario that walks its path. The run passes only when exactly
  those fail. `task slow:fields` synthesizes afresh and compares bytes.
- A refusal synthesis cannot reach gets an authored scenario in `packs/<pack>/scenarios/`.
- KPIs of a pack are in `packs/<pack>/kpis.yaml` (same format as `kpis/catalogue.yaml`, `block`
  is the domain without `agentic.`, at least 2 per block). KPIs of a field are in
  `fields/<field>/kpis.yaml`: `block` is the core block or pack block whose events it reads, no
  least number per block; a core KPI whose `block` is a field is refused.
  `scripts/check-kpis.sh` checks every catalogue.
- The docs pages of the packs and fields are generated with the others: `docs/packs/`,
  `docs/fields/`, `website/static/fields/`.

## agenticstudio nouns (`studio/`)

- A noun that is a building block of agentic development goes into `blocks/` (the patch and its
  application are `agentic.architecture.Patch` and `PatchApplication`, layer 9).
- A noun only the studio app has goes into `studio/`, a third ESS system (`studio`): drafts,
  studio sessions, tool calls, evaluations. Check: `ess specify validate --path studio
  --strict-requires`.
- ESS declares no relation across two systems: `studio/` names an entity of `agentic` by its
  identifier only (`studio/domains/shared.yaml`).
- Authored scenarios of `studio/` are in `studio/scenarios/`, listed in `studio/ess-inputs.yaml`;
  `task check:mutation` runs them against every mutant of `studio/`.

## Layout of the hand-written crates

`crates/agenticspec-records` is the library the gateway links (`design/interaction-records.md`);
`crates/agenticspec` is the command line over it and re-exports its modules.

| crate | module | what |
|---|---|---|
| `agenticspec-records` | `claude.rs` | Claude Code adapter: transcript files to IR commands, with actor and attribution |
| `agenticspec-records` | `redact.rs` | secret and personal-data detection; aliases, never values |
| `agenticspec-records` | `replay.rs` | storage ports and the replay loop through the generated component |
| `agenticspec-records` | `kpi.rs` | KPIs over published events only |
| `agenticspec-records` | `store.rs` | the local store `<state-dir>/sessions.db` (SQLite): `sync`, stored commands, view rows |
| `agenticspec-records` | `wire.rs` | the wire source: one finished gateway call to IR commands (`lift_call`), the session hash, `join` on the message id |
| `agenticspec-records` | `append.rs` | the non-blocking writer a gateway appends through (`Appender::spawn`, a bounded queue, drops counted) |
| `agenticspec-records` | `blocks.rs` | the lift of one session into rows of the building blocks (`lift`, the rules of `queries/blocks.sq` run by sq; fields the session does not record in `not_recorded`, identifiers derived from nodes so two lifts give the same bytes) and the draft scenario of its outside inputs (`draft`), joined through the receiving turn to the rows they caused |
| `agenticspec-records` | `view.rs`, `sq/`, `pricing.rs`, `time.rs` | view projection, the sq engine, API list prices, time helpers |
| `agenticspec` | `mine.rs`, `export.rs` | pattern mining, procedure export to canon |
| `agenticspec` | `draft.rs` | `agenticspec blocks lift\|scenario`: the lifted rows checked against the blocks built into the simulator, the draft scenario checked against its architecture and profile before it is written |
| `agenticspec` | `serve.rs` | `agenticspec serve`: the `session-view` views from the store, the built UI, SSE channels. It never lifts: each sync is a `agenticspec sync` child (`Syncer`), one at a time, and the server publishes what the child wrote |
| `agenticspec` | `html.rs`, `assets/dashboard.html` | the dashboard file `agenticspec analyze` writes |
| `agenticspec` | `main.rs` | handler behind the generated CLI; `agenticspec sim …` calls `agenticspec_sim::cli`, `agenticspec ui generate` calls `agenticspec_ui::generate` (with `--project`, `agenticspec_ui::generate_project`), `agenticspec ui panels` calls `agenticspec_views::run`, `agenticspec ui derive` calls `agenticspec_ui::derive_uis` (without `--root` on the compiled blocks, panels, their scripts and digest and port catalogue built into the binary, `operator::built_in_model`, `operator::built_in_ports` and the scripts of the embedded serve bundle, each parsed once per process; with `--root` on `ess specify compile` of its `blocks/` and its `ui/views/`; one composition per ingress with a `ui` export when the contract gives one to an ingress other than `operator`), `agenticspec ui build` calls `agenticspec_ui::operator` (`build_for` with `--ingress`), `agenticspec ui conform` runs `agenticspec_ui::conform` (against the fixture source of `agenticspec_ui::source` without `--source`; with `--architecture`, `conform::run_plane` against the plane of `agenticspec_ui::plane`, the contract derived through `agenticspec_sim::cli::contract_built_in` (the ports and KPI catalogue built into the binary) when `--contract` is absent, and against `ui serve` of it started in-process without `--source`, on the built-in fixtures or those below `--root`; `--origin` and `--header` into `conform::Options`, refused as `origin_invalid` and `header_invalid`; on a failed check every check on a line of standard output, or the report with `--json`, in `conform_reply`), `agenticspec ui serve` builds the operator UI with `api_base` `/ingress/operator` and the UI of every other ingress with a `ui` export with `api_base` `/ingress/<ingress>` (`operator::build_ingress`), and runs `agenticspec_ui::serve` (`Served::start_uis`) on `Fixtures::embedded`, with `--fixtures` on `Fixtures::from_dir`, or with `--sim` on `Fixtures::replay` of `agenticspec_sim::replay::run` (whose KPIs go to the Metrics page as the control UI's result; every plane answers from that one run); `ui panels` also writes or checks `ui/views/serve.fixtures.json` (`agenticspec_ui::serve::write_bundle`) |
| `agenticspec-sim` | `check.rs`, `whole.rs`, `nest.rs`, `requirements.rs`, `sim.rs`, `sim/swarm.rs`, `sim/record.rs`, `search.rs`, `profile.rs`, `cli.rs` | the library of `agenticspec sim check\|digest\|run\|render\|ladder` (`cli.rs`, no binary of its own); `sim render` of a file of format `agenticspec-render-model/1` writes that page (`agenticspec_render::page::render`); `sim check --json` prints the `agenticspec-check/1` document (`cli::check_document`, in `cli.rs` because the catalog digest leaves it out): an architecture document (`architectures/`) checked, measured as a whole (`whole.rs`: the KPIs `sim check` reports and every variant of a result carries), simulated with a behaviour profile (`profiles/`) and searched over its marked places (`experiments/`); the same seeds give the same `result.json` at any thread count. `check.rs` reads the part kinds and `carries` values from `blocks/9-composition/architecture.yaml` (compiled in), so the check and the block cannot differ. `nest.rs` resolves a part of block `architecture` (`ref`, `digest` of the canonical parts, links and border) and flattens it into parts named `<part>.<inner part>` before the check and the simulation. `requirements.rs` checks the `requirements:` key (`requirement_kpi_unknown`, `requirement_invalid`), gives each requirement its stage and decides it: at check (`sim check`, the bundle) and per variant in the search, which ranks requirements missed before fitness and stops `fit` when all are met |
| `agenticspec-sim` | `settings.rs` | the settings table `blocks/settings.yaml` (`agenticspec-settings/1`, compiled in, not an ESS input): per part kind the settings a part may carry, each with its type (number and integer with unit and range, boolean, enum, duration, string and string list, part, shares, notes), default, the contract ports it narrows (`narrows`) and a summary; an `enum:` names a block's own enum, whose variants are read from the block files (`unavailable::SOURCES`). `check.rs` refuses an undeclared setting (`setting_unknown`) and a value outside its type (`setting_invalid`); every part may carry `notes` (`NOTES`), a map with free keys and values the check takes as written and the simulator and the search (`marked.rs`) never read; `contract/derive.rs` copies the settings marked `narrows`; the block pages list them (`agenticspec-docs`, `blocks.rs`); the catalog digest reads `Table::canonical`. A new part kind gets an entry in the same change (a test holds the table against `PartKind`); a new setting the simulator reads is declared there first |
| `agenticspec-sim` | `scenario.rs`, `unavailable.rs`, `corpus.rs`, `events.rs`, `reader.rs`, `sim/script.rs` | `agenticspec sim scenario check\|run` and `sim corpus run` (`docs/scenarios.md`): scenarios (`agenticspec-scenario/1`) and task corpora (`agenticspec-corpus/1`), read by a typed reader that refuses an unknown key by its path. A run drives the simulator with a `Script` (arrivals, run outcomes, build times, replies) on the scenario's seed, and `events::TABLE` is the one map from the simulator's event kinds to block events and marks the eight an `on_event` trigger waits for; a test holds it against every kind the sources record and every event `blocks/` declares. `unavailable.rs` reads `unavailable` and `then.parts`, checks views, commands and errors (and the block events and refused commands a `then` names, by their full names) against the block files it includes (`SOURCES`, held against the root `ess-inputs.yaml` by a test) and plans the faults and refused inputs of a run. `scenario.rs` and `unavailable.rs` do not change an experiment's result and are left out of the catalog digest (`events.rs` is in it, for the trigger rows); a profile draws its tasks from a corpus with `load.corpus` |
| `agenticspec-sim` | `event_log.rs` | the event log of a recorded run (`sim run --events`, `events.jsonl`): each timeline kind written as the block event of its table `LOG` (or listed in `SILENT`), with part path, copy and subject key. Not an input of a catalog result (`SIM_EXCLUDED` in `crates/agenticspec-docs/src/results.rs`); a new timeline kind needs a row in `LOG` or `SILENT`, and `tests/cases/event_log.rs` checks each row against `blocks/` |
| `agenticspec-sim` | `replay.rs` | `agenticspec ui serve --sim <profile>`: the architecture run once at its built values (`cli::built_model`, shared with `sim trace`; seed 1, one day), its event log, and the state that log leaves in the views of the blocks built in (`rows::SOURCES`): the key field names the entity, the emitting command outcome's `creates`/`moves`/`updates` gives its state, a view keeps its `state ==`/`state !=` rows; returned as `agenticspec_ui::serve::Replay`, with the run's KPIs as a result for the Metrics page. Not an input of a catalog result (`SIM_EXCLUDED`); tests in `tests/cases/replay.rs` |
| `agenticspec-sim` | `rows.rs` | the declarations of `blocks/` built into the binary (`SOURCES`, a test holds it against the files) and the check of a row `{ entity, id, state, fields, not_recorded, evidence }` of a scenario's `given.rows`; left out of the catalog digest |
| `agenticspec-sim` | `contract/ports.rs`, `contract/protocol.rs` | `agenticspec sim ports`: the ports of the building blocks from their compiled ESS system, their protocols (regular expressions over command names), `blocks/ports.json` and its digest |
| `agenticspec-sim` | `contract/derive.rs`, `contract/predicate.rs` | `agenticspec sim contract`: the contract of an architecture document (`agenticspec-contract/1`), its check, the predicate language of routing invariants, and the table of the ports each part kind uses (`PART_KINDS`, the variants of `agentic.architecture.PartKind`; `sim check` refuses any other block) |
| `agenticspec-sim` | `contract/suite.rs` | `agenticspec sim contract --suite`: the blocks' conformance suite cut to the scenarios about a command of a port of the contract (`task contract:conform`, `scripts/contract-conform.sh`) |
| `agenticspec-sim` | `contract/trace.rs`, `sim/record.rs` | `agenticspec sim trace`: the port uses of one recorded run (`sim/record.rs`, kept only when a run records) as a trace (`agenticspec-trace/1`), and `agenticspec sim contract --trace`, the check of the routing invariants on it |
| `agenticspec-sim` | `contract/ingress.rs` | the `ingress:` key of an architecture document: its check (`ingress_*` rules, one refusing fixture each in `tests/cases/ingress/`) and each interface resolved into operations for the contract, from the surface of `blocks/ports.json` and `kpis/catalogue.yaml` |
| `agenticspec-docs` | `lib.rs`, `main.rs`, `packs.rs` | the generated docs pages of the blocks, architectures and KPI catalogue, and of the packs and fields (`packs.rs`, from the root `ess-inputs.yaml`) (`task docs:generate`, `--check`) |
| `agenticspec-sim` | `deploy/binding.rs`, `deploy/packing.rs`, `deploy/admit.rs`, `deploy/grants.rs`, `deploy/config.rs`, `deploy/bundle.rs` | `agenticspec packing default`, `agenticspec deploy bundle` and `agenticspec deploy check`: the binding (`agenticspec-binding/1`, its `refines:` chain), its `packing:` section and strategies, admission with its named refusals, the secrets and egress each part declares by name (`grants.rs`), what the target is asked to serve (ingress exports, check ports, volume mounts, local images, network isolation; `bundle.rs`), agent units (`kind: agent` with `shape`, a scheduled unit's cron `schedule` (the binding's, else the one its clock's `every` states exactly, `schedule_not_cron` otherwise) and `clock`, the `agent-runtime` runtime, each agent document with its exact bytes in `content_base64`, `agent_document_mismatch` at check), each nested document by its canonical digest with its exact bytes (`nested_document_mismatch`, `nested_document_missing` at check), the requirements with their state, the runtime image of each core and service unit (`parts.<p>.image`, `packing.units[].image`, else `runtimes.core` for the core only; `binding::unit_image`, refusals `unit_image_conflict`, `unit_image_missing` and `unit_runtime_mismatch` (`bundle::unit_runtimes`); tests in `tests/cases/deploy_images.rs` over `tests/cases/deploy/unit_image_missing.architecture.yaml`), the configuration of each unit (`parts.<p>.env`/`args`, `packing.units[].env`/`args`, merged into the unit's `config: {env, args}`, a `{secret: <name>}` value through the unit's secrets; `config.rs`, refusals `unit_config_conflict` and `unit_config_invalid`, `config::check_units` at check; tests in `tests/cases/deploy_config.rs`), the durable state of a runtime part (`durable_state: true`, a `<part>-state` volume claim, `binding::durable_state`, the outside writers it names in `state_writers` as the claim's `writers` (`binding::state_writers`; `check.rs` refuses them without `durable_state: true`), `bundle::STATELESS_KINDS`, refusal `durable_state_unplaceable`; tests in `tests/cases/deploy_state.rs` over `tests/cases/deploy/durable_state_unplaceable.architecture.yaml`), the access of each volume claim from who mounts it (`bundle::access`; tests in `tests/cases/deploy_access.rs` over `tests/cases/deploy/volume_access.architecture.yaml`), the deployment bundle (`agenticspec-deployment/1`) and its check when read back; `docs/deployment.md`. Left out of the catalog digest |
| `agenticspec-views` | `lib.rs`, `panels.rs`, `values.rs` | the library of `agenticspec ui panels`: from the compiled `blocks/`, one `ess-ui/1` widget per view (`panels.rs`, naming no block, view or command), the synthetic fixture rows and script payloads (`values.rs`), the part of the compiled blocks a derivation reads (`ui/views/blocks.model.json`, `model`), the digest of the inputs and `--check` (`lib.rs`) |
| `agenticspec-sim` | `deploy/fixtures.rs` | the fixture bundles of `bundles/` and `bundles/sources.sha256` (`agenticspec-docs --bundles`, `task bundles:generate`, `check:bundles`, `slow:bundles`) |
| `agenticspec-sim` | `deploy/ui.rs` | the UI units of a bundle: a binding's `ui:`, one row (`Ui`) per UI with its source (`Source`): the built-in `operator` and `control-plane` (`BUILT_IN`, units `operator-ui` and `control-plane-ui`, each `{port, bind, upstream: {unit, port, path}}` (`path` the `api_base` of its `config.json` and the unit's `upstream.path`, `/api/` when absent, `ui::path`), the archive built in-process by `agenticspec_ui::operator::build_ui`) and, under any other key, a UI of the consumer's own (`{source: {archive, digest} or {ess_ui}, export: <ingress>.<protocol>, upstream?, port?, bind?}`, unit `<name>` with `roles: [ui]`, its archive read beside the binding file, `Binding::ui_files`); the first UI asked serves its export (`serves_export`); claims and check ports from the binding by unit name; the archive carried base64 in the bundle's `files`; the refusals `ui_upstream_unknown`, `ui_upstream_path_invalid`, `ui_export_unknown`, `ui_archive_mismatch` (bundle and check), `ui_build_needs_toolchain` (an `ess-ui/1` source) and `ui_build_refused`; tests in `tests/cases/deploy_ui.rs`. The fixture bundle of a binding with a built-in UI also digests the UI's sources (`fixtures.rs`, `UI_SOURCES`), and of a UI of the consumer's own its archive |
| `agenticspec-sim` | `contract/ingress.rs` (defaults) | the default control plane: the ingresses `operator` and `metrics` every document has by name (`DEFAULTS`), `none`, the reserved path `control` of the implied audiences, `agenticspec ingress default` |
| `agenticspec-sim` | `contract/projection.rs`, `contract/projection/` (`stream.rs`, `cli.rs`, `tool.rs`, `agent.rs`, `metrics.rs`) | `agenticspec ingress project`: the projection of each export (`FILES`): per `http` export the `ess-http/1` protocol binding and the narrowed OpenAPI document (`projection.rs`, with the helpers the others share), per `event_stream` the stream list, per `cli` the `ess-cli/2` binding (written, not validated: `design/ess-repro/cli-service-forward/`), per `tool` the tool list, per `agent` the agent card, per `metrics_scrape` the metric list, each with its refusal (`projection_refused`); tests in `tests/cases/projection.rs`; the committed projections of the worked example are in `architectures/projections/workstation-ingress/` (`task projections:generate`, `task check:projections`) |
| `agenticspec-docs` | `contract.rs` | the "Contract" section of an architecture page and a catalog entry |
| `agenticspec-docs` | `tests/cache.rs` | the pages generated once per test run: one entry per catalogue under `target/tmp/docs-generated/<digest>/`, keyed by the size and modification time of the generator's inputs and of the test executable, and by `ess --version`, built under a file lock by the first test that needs it. A new input of the generator goes into `INPUT_DIRS` or `INPUT_FILES` there; a test generates afresh only where it proves something of a fresh generation |
| `agenticspec-render` | `page.rs`, `graph.rs`, `fmt.rs`, `collapse.rs`, `schema/render-model.schema.json` | the renderer, which names no agenticspec term (a test holds `page.rs`, `graph.rs`, `fmt.rs` and `collapse.rs` to that): `page::render(&Page)`, one HTML page from the render model `agenticspec-render-model/1` (nodes in groups, ports, edges, regions, marks that are refusals or information, panels; unknown fields refused), the graph at natural size with pan, zoom and fit. `graph.rs` lays the graph out in rows (a group's rank, else the longest chain of edges); a row of cards, or the cards of a frame, wider than `page::WRAP_WIDTH` (2400 px) wraps into rows in its order (`layered`, `place_lines`; tests in `tests/wrap/`, on `tests/fixtures/bands-109.json`, 109 nodes and 209 edges in bands); edges with one label into or out of one card share one label and a trunk; a region is a frame drawn around the cards of its nodes and the frames of its nested regions, an edge to a node in a region lands on that node's card, and an edge between two cards of one region that do not stand side by side runs on a lane below what the region holds; `groups` set the colour (a validated `Colour`) and icon (`ICONS` or 1-2 characters) of a group; an edge's stroke grows with `count`; a badge `on: panel` is a row of the table "Details" under the graph; a port row shows its `via`, and an `unresolved` port leaves its row as a dashed stub named by the refusal marks on it. `layout: bands` (`graph.rs`, `bands`) lays out each top-level region as one band in list order: each band is laid out as a page of its own and drawn at an offset (`Frame`), and edges between bands are routed through the gaps and beside the bands, on lanes the edges of a gap share (`bands`, `bundle`, `pack`, `routes`): runs that lie `LANE_CLEAR` (16 px) apart share a lane, edges with one label out of or into one card are one bundle with one label, an edge that passes the cards of its band leaves by the nearer side, and edges straight down one side through a gap with one label are one bundle; tests in `tests/lanes/`. A region `collapsed: true` (`collapse.rs`) is one card with the count of its nodes and edges merged per pair of visible ends; each state with one collapsible region open (and those around it) is laid out in full by `graph::render_in` and written as a `<template data-state data-up>` beside the graph, which `assets/page.js` swaps in (the card opens it, the `close` control of its frame or band closes it); tests in `tests/collapse/`. A panel `place: below` is drawn under the graph. Reading a page from JSON runs `Page::check`, which refuses an edge end that resolves to nothing (`edge_end_unknown`). A `collapsed` that is no boolean is refused when read (`region_collapsed_invalid`), a collapsed region with no node by `Page::check` (`region_collapsed_empty`). What a page of this crate draws beyond the model (subtitles, icons, properties, ranks, a part a link passes over, values at marked places) is the crate-private `graph::Decor`. The schema is written by hand; `tests/page/` checks it against the types both ways |
| `agenticspec-render` | `architecture/` (`mod.rs`, `map.rs`, `model.rs`, `result.rs`, `optim.rs`) | the architecture page through the renderer: `architecture::page(architecture, result)` with the tables of parts and links and, when given, the search result; `map.rs` maps parts to nodes (block layer as group and rank), `per` to regions, the border parts of a nested architecture (resolved beside its part as `inner`) to ports, links to edges and marked places to marks; `architecture::graph(architecture)`: the graph alone (SVG, size, parts with their properties, links) for agenticstudio to draw inline; `architecture::render_model` returns the model alone |
| `agenticspec-records` | `sq/architecture.rs` | the fact base of an architecture (`part`, `setting`, `link`, `marker`), compiled apart from the session relations |
| `agenticspec-patch` | `lib.rs`, `facts.rs`, `text.rs`, `diff.rs` | `agenticspec-patch/1`: the facts of an architecture document, the verbs `set` and `add`, atomic application checked by `agenticspec-sim`'s check, refusals by rule position and error name (`ErrorName`, each listed in `docs/studio.md`) |
| `agenticspec-ui` | `doc.rs`, `check.rs`, `filter.rs`, `emit.rs`, `runtime.ts`, `interface.rs`, `source.rs`, `conform.rs`, `wire.rs` | `agenticspec ui generate --path --out`: an `ess-ui/1` document to Vue pages, one component per widget (`widgets/<Widget>.vue`, the same bytes for the same widget in any document), a hash router with the navigation, a typed client and a runtime (live re-read coalesced or row patch, the scope client of a widget) over `ui/kit` components and host slots only, byte for byte the same each run. A construct it cannot draw is refused by its node path; the look is never generated. An action that sends a command shows only while the action set (R5) of its scope allows it. `filter.rs` parses and compiles a row predicate (`reads.filter`, a confirm's `show`) in the released ess-ui grammar. Golden tests in `tests/cases/` (`UPDATE_GOLDEN=1` rewrites them; `rows` holds filters, sort, groups and the other R108 constructs), per-construct tests in `tests/constructs/`. Beside it, the runtime interface (`ui/runtime.openapi.yaml`, `interface.rs`): the fixture source that answers it (`source.rs`), the conformance suite of `agenticspec ui conform` run against a source by URL (`conform.rs`: `Origin` and the option's headers on every write, a view answered `view_not_served` skipped, `Report::lines` for the text output), the scope rule with a row's `scope` field (`interface::in_scope_at`) and the small HTTP client and server both use (`wire.rs`) |
| `agenticspec-ui` | `derive/` (`mod.rs`, `model.rs`, `view_key.rs`, `write.rs`, `control.rs`, `ingress.rs`) | `agenticspec ui derive`: an architecture (its nested documents, its `view:` key), its contract and ports into one `ess-ui/1` document over the block panels, copying each widget of `ui/views/blocks.ui.yaml` byte for byte, with its fixtures (`<name>.fixtures/`), its layout (`<name>.layout.json`, `agenticspec-ui-layout/1`, read by the package of `ui/views`) and the digest of its inputs. `control.rs` is `--control-plane`: the pages across parts (Inbox, Activity, Operations, Metrics, Architecture, Events) and the overview tiles, selected by port class and the configuration/administration split, with the rules that never expose a configuration command, a command of report-issuing actors or machine traffic (`Rules::hidden`), and the predictions of a sim result of the same revision; written under the layout's `control` key. `ingress.rs` is the UI per ingress (`design/ingress.md` I10): when the contract gives a `ui` export to an ingress other than `operator` (`per_ingress`), `derive_all` writes `<name>.<ingress>.ui.yaml` per such ingress, each scoped to its interface (`scope`, `apply`: panels of its views, its commands as actions, its feeds), the layout and title naming the ingress; a contract whose only `ui` export is the operator's is not scoped. Tests in `tests/derive/` (`control.rs` for the control plane, `ingress.rs` per ingress) |
| `agenticspec-ui` | `operator.rs` | `agenticspec ui build`: the operator app of `ui/operator/built` embedded (`APP`, a test holds it against the directory) with the panels document, `blocks/ports.json` and `ui/views/blocks.model.json`, so a derivation needs neither `ess` nor `blocks/`; the layout of `derive::layout_with` as `views.json` (`build_with` takes the contract in memory, for `deploy bundle`; `build_ui` with `ControlInputs` builds the control-plane UI, `config.json` in `mode: control-plane`; `build_ingress` and `build_for` build the UI of one ingress with a `ui` export, `config.json` naming it) beside `contract.json`, `ports.json` and `config.json` (`agenticspec-operator-config/1`), and the reproducible ustar archive (`archive`, sorted entries, mode 0644, owner 0, time 0) with its sha256; tests in `crates/agenticspec/tests/ui/` |
| `agenticspec-ui` | `project.rs` | `agenticspec ui generate --project` (`generate_project`): the generated app under `src/generated/` with the sources of `ui/kit/src` embedded (`KIT`, a test holds it against the directory) under `kit/`, a host page with a placeholder per host slot, a vite config with the operator app's output options, an npm manifest pinning `PINS` at the versions of `ui/package.json`, the lock of exactly those (`lock`: the entries of `ui/package-lock.json` they reach by node's resolution, their flags computed again), and `pack.mjs`, the packer that writes the tar as `operator::archive` does. A change of `ui/kit/src` (a file added or removed) updates `KIT`; a change of `ui/package.json` or its lock needs nothing here. Tests in `tests/project.rs` (the kit, the pins, the lock and the build config held to their sources) and `crates/agenticspec/tests/ui/` (the command line); no npm runs in a test. `task check:operator` builds the project of the workstation with vite over `ui/node_modules` and holds `pack.mjs` to the bytes of `ui build --archive` |
| `agenticspec-ui` | `plane.rs`, `serve.rs` | `agenticspec ui serve <architecture> [--contract] [--port] [--fixtures | --sim]`: the default control plane (`plane.rs`: the parts and the reads of each part page from `views.json`, the views, commands and feeds of the contract's ingress the layout names, `operator` by default, each view's key and fields from the panels document) answered from fixtures on 127.0.0.1 below `/ingress/operator`, and the plane of every other ingress with a `ui` export below `/ingress/<ingress>` with its app at `/ingress/<ingress>/` (`Served::start_uis`, `root_of`) (`serve.rs`: R1 from the view fixtures, R2 from the channel scripts with the cursor, R4 recorded at `/fixture/commands` with the scripted outcomes, R5 allowing every command; the fixtures of `ui/views/fixtures/` and `ui/views/source/outcomes.json` are built into the binary as `ui/views/serve.fixtures.json` (`BUNDLE`, `bundle`, `write_bundle`; a test holds it against the files), or read from a `--fixtures` directory; with `--sim`, a `Replay`: rows by view with the places (`At`) their events happened, answered only at those scopes, and the run's events with their log `seq` as the cursor, each plane kept to its own views, channels and feeds), beside the operator app at `/`, whose page sets the session cookie, and the control-plane UI at `/control/` (its reads across parts are `Plane::across`, which `ui conform --architecture` checks at the architecture's scope); a request to another `Host` is refused. `Faults` breaks one obligation for the tests of `conform::run_plane` (`tests/serve/`); a row or event whose `scope` field names a scope of the architecture is answered only there and above (`interface::named_in_scope`), on every plane. A runtime's adapter that is not `ui serve` is `tests/serve/partial.rs` (views not served, a plain 404, same-origin writes with a proof header, rows at listed scopes), used by `tests/serve/adapter.rs` and, through `#[path]`, by `crates/agenticspec/tests/ui/conform.rs`; the command line's tests start one server per run (`crates/agenticspec/tests/ui/serve.rs`) |
| `agenticstudio` | `studio.rs`, `tools.rs`, `live.rs`, `routes.rs`, `app.rs`, `http.rs`, `main.rs` | `agenticstudio serve`: drafts, revisions, patches and applications, studio sessions and their tool calls in memory; the four tools (`tools.rs`: query, apply, check, simulate, each answer with a `summary`); the event channel of each draft (`live.rs`, server-sent events of `studio.draft.DraftRevised` and the answered or failed calls) and the source files of open drafts read again every 500 ms (a change on disk is a new revision); the views and commands of `studio/studio.ui.yaml` (`/api/views`, `/api/commands`, `app.rs`), the older `/api` routes, and the built app of `ui/studio` from `--www`. Its types are `src/generated/` (`ess generate types` from `studio/` and the architecture block, held by `task check:generated`), included as modules; its browser test is `e2e/` (`task check:studio`) |
| `agenticstudio-mcp` | `lib.rs`, `main.rs` | the agent channel: an MCP server over stdio (`agenticstudio-mcp --studio <url> [--draft <id> \| --open <source>]`) whose tools `query`, `apply`, `check` and `simulate` are calls with caller `Agent` in a studio session: every request it sends carries `X-Studio-Caller: Agent`, through `POST /api/sessions/<id>/calls` of a running `agenticstudio serve` (`Http`) or a studio in the same process (`Local`, which its tests use). Its tests replay the recorded exchange `tests/exchange.jsonl` |

Beside the crates, `ui/` holds:

- `agenticspec.ui.yaml`, the UI as an `ess-ui/1` document, with its `ess-ui-test/1` tests, checked by
  `task check:ui`. No UI is generated from it.
- `fixtures/`, the synthetic view rows every UI test reads, one row per line.
- `app/`, the live UI, written by hand: Vue 3, Vite and TypeScript. Its types are
  `app/src/generated/`, from `ess generate types` (held by `task check:generated`). Its client reads
  the views page by page and the SSE channels (`app/src/client.ts`).
- `app/src/testing/fixtures.ts`, which answers every view from `fixtures/` the way `serve`'s queries
  do. vitest uses it and so does Playwright (`app/e2e/harness.ts`), so no test starts a server.
- `kit/`, the design system: the tokens (`kit/src/tokens.css`, from the docs site's product theme,
  light and dark), the base styles and the `Kit*` components every generated page draws with; no
  colour, size or font is written outside the tokens. Its gallery (`kit/gallery/`) shows every
  component in both themes; `task check:ui-kit` (`scripts/check-ui-kit.sh`) runs its types, vitest,
  the gallery build and its browser test.
- `studio/`, the agenticstudio app: `src/generated/` is written by `agenticspec ui generate` from
  `studio/studio.ui.yaml` (`task ui:generate`, held by `task check:generated`), never edited;
  `src/slots/` holds the host slots (`DraftGraph`, `DraftPanels`, `PatchEditor`) and `src/hooks.ts` the command
  hooks. `task studio:build` builds it into `target/studio/www`; `task check:studio`
  (`scripts/check-studio.sh`) checks the document, its types and vitest, and runs
  `crates/agenticstudio/e2e`.
- `package.json`, one npm package for `kit/`, `studio/`, `views/`, `operator/` and the apps of `views/`; `app/` keeps its own.
- `views/`, generated by `agenticspec ui panels` and never edited by hand: `blocks.ui.yaml`, the
  block panels as an `ess-ui/1` document over `blocks/` (one widget per view, named as the view,
  and a gallery page per block), `fixtures/` (three rows per view, a script per block channel)
  and `blocks.sources.sha256`, the digest of the inputs, then `serve.fixtures.json`, the fixtures
  and `source/outcomes.json` in one compact file with the digest of its sources, which the binary
  embeds for `agenticspec ui serve`. `task check:ui` holds them. Beside them,
  `views/gallery/` is the gallery app of the panels: `src/generated/` is written by
  `agenticspec ui generate` from `blocks.ui.yaml` (held by `task check:generated`), never edited;
  `src/fixtures.ts` answers its reads from `fixtures/` in the browser. `task views:build` builds it
  into `target/views/gallery`; `task check:views` (`scripts/check-views.sh`) runs its types,
  vitest, the build and its browser test (`views/gallery/e2e`).
- `runtime.openapi.yaml`, the runtime interface the generated client calls (R1 read, R2 feed with a
  cursor, R3 placement views, R4 command, R5 action set, at `{root}/scopes/{scope}`); the
  `agenticspec-ui` crate includes it, and a test holds the client to its operations.
- `views/source/`, written by hand: `source.json`, the fixture source (principals by token,
  placements, parts, grants, the action class of each command, command answers), and `plan.json`,
  its conformance plan. Synthetic ids only; `agenticspec ui conform` and the gallery read them.
  `outcomes.json` (`agenticspec-command-outcomes/1`) scripts what `agenticspec ui serve` answers a
  command: per command, optionally per input, the accepted row or the refusal's row; a change of it
  runs `task ui:panels`, which writes it into `views/serve.fixtures.json`.
- `views/examples/`, written by `agenticspec ui derive` (`task ui:derive`) and never edited by
  hand: `workstation.ui.yaml` from `architectures/workstation.yaml` with
  `workstation.contract.yaml` (written by `agenticspec sim contract`) and `ports.json` (a copy of
  `blocks/ports.json`), both written by `task ui:derive` and held by `task check:ui`,
  `large-37.ui.yaml` from
  `crates/agenticspec-render/tests/fixtures/large-37.yaml`, the control-plane UIs
  `workstation.control-plane.ui.yaml` and `team-with-console.control-plane.ui.yaml`
  (`ui derive --control-plane`, the latter with `team-with-console.contract.yaml`), one UI per ingress with a `ui`
  export of `architectures/workstation-ingress.yaml`, `workstation-ingress.user.ui.yaml` and
  `workstation-ingress.operator.ui.yaml` (with `workstation-ingress.contract.yaml`), each with `<name>.fixtures/`,
  `<name>.layout.json` and `<name>.sources.sha256`; `task check:ui` compares them and holds them
  at 0 errors and 0 warnings. `host-slot.ui.yaml` (hand-written) places the custom element through
  a host slot. `views/examples/app/` shows both over the fixtures: `src/<name>/generated/` is
  written by `agenticspec ui generate` (held by `task check:generated`); `task check:examples`
  (`scripts/check-examples.sh`) runs its types, vitest, build and browser test
  (`views/examples/app/e2e`).
- `views/package.json`, `views/src/`: the Vue package `@agenticspec/views` (the panels of
  `views/gallery/src/generated/widgets`, `AgenticPartPage` and `AgenticOverview` drawn from a
  layout, `AgenticBlockPanels`, the panels' runtime re-exported) and `src/elements.ts`, the custom
  elements built from the same components (`task elements:build`, into `target/views/elements`).
  `views/hosts/` holds the plain and React test pages; `task check:elements`
  (`scripts/check-elements.sh`) runs the package's types and vitest, both builds and the browser
  test (`views/hosts/e2e`). agenticstudio imports the panels from here (`studio/src/slots/DraftPanels.vue`);
  no panel file is written under `ui/studio/`.

`task ui:build` builds the app into `target/ui/www`. Outside `task check:app`, vitest runs as
`slot --local npx vitest run --maxWorkers=2` from `ui/app`.

## UI changes (ui-iteration-loop)

- `ux/adapter.json` is the adapter of the loop, and `ux/scenarios/*/scenario.json` holds the five
  scenario cards and their truths.
- The fast loop runs in `task check:app` (`scripts/check-app.sh`): every scenario at the gating
  viewports 1440x900 and 1280x720 in both themes.
  - The screen-only user (`app/e2e/bot.ts`) plays each scenario.
  - The layout contract and axe-core run at every state (`app/e2e/layout.ts`).
  - Every scenario is done once more by keyboard alone.
  - One test per finding is in `app/e2e/findings.spec.ts`.
  - The ratchet compares with `ux/baseline.json` (`app/e2e/ratchet.spec.ts`). An improved metric
    is taken into the baseline in the same change with
    `UX_RATCHET=write npx playwright test --project=ratchet`. A worse one needs a decision id
    beside the value.
- A round is measured by `task ux:round -- R-nn` into `ux/rounds/R-nn/` (report.md, round.json,
  shots/, `ux/rounds/trend.md`), with the measured-only viewports too. Then:
  - two fresh review agents write `review-heuristics.md` and `review-report.md`;
  - `triage.md` picks at most 5 changes;
  - each change is built test first, with its `ux/findings/UXF-nnn.md` and the test that is its
    regression asset.
- Round screenshots come from the fixtures only, never from a real store.
- A closed S1 or S2 finding without an existing regression asset fails the ratchet project.
- The docs site has its own adapter, `ux/site/adapter.json`, with scenario cards in
  `ux/site/scenarios/`, findings `ux/site/findings/SUF-nnn.md`, the baseline
  `ux/site/baseline.json` and rounds `ux/site/rounds/S-nn/` (`task ux:site-round -- S-nn`). Its
  fast loop is `task check:site` (`scripts/check-site.sh`, specs in `website/e2e/`) on the built
  site, run as `slot task check:site`; only `task docs:generate` runs with `slot --here`. axe-core
  results are cached under `target/site-ux/axe/` by a digest of the built HTML.
- `website/e2e/graph.spec.ts` checks the rendered page of a large architecture on the 37-part fixture
  `crates/agenticspec-render/tests/fixtures/large-37.yaml`, which `scripts/check-site.sh` renders
  into `target/site-ux/large-37.html` with `agenticspec sim render`. The fixture is not under
  `architectures/`, so it is neither a docs page nor a catalog entry. It also opens and closes the
  collapsed regions of the render model page `crates/agenticspec-render/tests/fixtures/collapsed-regions.json`,
  rendered the same way into `target/site-ux/collapsed-regions.html`.

## Repository files

| path | what |
|---|---|
| `README.md`, `LICENSE`, `CHANGELOG.md` | for humans; Apache-2.0; the changelog in the Keep a Changelog 1.1.0 form, `## [Unreleased]` first. A change a user of the repository notices writes its line there |
| `scripts/check-generated.sh` | one line per generated directory, the drift check and the regeneration |
| `scripts/check-mutation.sh`, `check-mutation.test.sh`, `spec/mutation-known.txt`, `studio/mutation-known.txt` | `task check:mutation` (in the gate, about 35 s): the mutation audit of `spec/` and `studio/` on the interpreter (`ess verify conform mutate`, every class, `grant-add` included, authored scenarios run against each mutant; `--class` narrows it for the script's own test), passing only when the mutants not killed are exactly those the specification's `mutation-known.txt` lists, each with its reason. A survivor is answered by an authored scenario under `<dir>/scenarios/` or by making the rule observable, then its line is removed; a grant added to an actor is killed only by an authored scenario that sends the command as that actor and expects `refused: not_granted` (`<dir>/scenarios/grants/`). Each run prints the stillborn mutants, the unmutated sites by reason and the unavailable sites, without failing on them. `spec/` runs on a copy with Binary64 read as Decimal (the audit carries no Binary64). `check-mutation.test.sh` (in `task check:scripts`) plants two defects it must name. `blocks/` is not audited in the gate: one class takes longer than a pool job |
| `scripts/spec-diff.sh`, `spec-diff.test.sh` | `task spec:diff [-- <revision>]`, not in the gate: `ess verify diff --compatibility` of `blocks/`, `spec/` and `studio/` against a git revision (the newest release tag by default), every breaking or unknown change listed, exit 4 on one; `--self-test` (run by `task check:scripts`) proves a removed enum variant is breaking |
| `bundles/` | the fixture deployment bundles, `<architecture>.<binding>.yaml`, one per catalog architecture and test binding of `bindings/`, the agent documents they name (`agents/`), the bindings (`bindings/`: `local.yaml` is the base of one `local-<architecture>.yaml` per catalog architecture, which declares its egress, and `local-team-with-console.yaml`, the nested team-with-console under `local`; `operator-ui.yaml` refines `split.yaml` and asks for the operator UI, the control-plane UI and a UI of the consumer's own, `console`, from the fixture archive `console-ui.tar`, side by side), the digest of their sources (`sources.sha256`, held by `task check:bundles`) and the copy of the translator's accepted unit kinds and roles (`cloudspec-accepts.yaml`). A change of a bundle's sources runs `task bundles:generate` in the same change |
| `scripts/ui-build.sh` | `task ui:build` |
| `ui/operator/` | the generic operator app of `agenticspec ui build` (Vue over `ui/kit` and `/views`): it reads `config.json`, `views.json`, `contract.json` and `ports.json` beside it at start and calls the runtime interface at `config.json`'s `api_base`. Its browser test is `e2e/`, over one static server with a fixture adapter and over one `agenticspec ui serve` of the workstation (`e2e/serve.spec.ts`, the operator UI at `/` and the control-plane UI at `/control/`). `src/Control*.vue` and `src/control.ts` draw the pages across parts of a layout with a `control` key |
| `ui/operator/built/`, `ui/operator/built.sources.sha256` | the committed build of the app (`index.html`, `app.js`, `app.css`, fixed names), which the binary embeds, and the digest of its sources (the app without `e2e/`, `ui/kit/src`, `ui/views/src`, the generated panels, `ui/package.json`, `ui/package-lock.json`) and of the built files. Never edited: `task operator:build` (`scripts/operator-app.sh --write`) rebuilds both after a change of those sources; `task check:operator` compares the digest without node, `task slow:operator` builds afresh and compares bytes |
| `scripts/operator-app.sh`, `scripts/check-operator.sh` | the digest check, `--write` and `--slow` of the operator app; `task check:operator`: the digest, the app's types, `agenticspec ui build` of the workstation into `target/operator/workstation` and its browser test over one static server and fixture adapter |
| `scripts/check-app.sh` | `task check:app`: types, vitest, the build, and the Playwright gates and ratchet of `ux/` |
| `scripts/check-ui-kit.sh`, `scripts/check-studio.sh` | `task check:ui-kit` and `task check:studio` (`ui/kit`, `ui/studio`, `crates/agenticstudio/e2e`); `SCREENS=1` writes screenshots into `target/screens` |
| `studio/studio.ui.yaml` | the studio app as an `ess-ui/1` document over the `studio` model (`studio/components.yaml` serves it); its `unmapped` entries name what the document cannot say yet |
| `ux/` | the ui-iteration-loop: adapter, scenarios, `baseline.json`, `round.sh`, rounds and findings |
| `scripts/change-class.sh`, `change-class.test.sh` | the change classifier of `pages.yml` (planning, docs-site, code) and its cases, run by `task check:scripts`. `design/` and `research/` are planning; `website/` and `docs/` are the site. A new site source goes into its path list with a case |
| `scenarios/` | example scenarios and one corpus per architecture (`<architecture>/*.scenario.yaml`, `*.corpus.yaml`), run by `task check:scenarios` (in the gate); each names the files it reads with their sha256, so a change of the architecture or the profile updates the digests there |
| `website/` | the public documentation site at `https://beyond10x.github.io/agenticspec/` (Docusaurus on `@beyond10x/docs-system`, pinned by commit in `package.json`). Its pages are `docs/*.md`, the user documentation, read in place; `docusaurus.config.ts` gives each its route, title and header (`pages`). `product.json` is the landing page. A new page needs an entry in `pages` and in `sidebars.ts`. A Markdown link to a file the site does not publish fails the build, so link such files by GitHub URL. `task site` builds it |
| `.github/workflows/pages.yml` | `Documentation validation`: classifies the changed files with `scripts/change-class.sh`, builds the site, refuses a machine path in it, and on `main` uploads the artifact `b10x-project-site`. Its job `gate` runs `task check` for the class `code`, with `task`, `cargo-nextest` and `ess` downloaded at a pinned version and refused unless their SHA-256 is the one in the step |
| `.github/workflows/b10x-docs-site.yml` | publishes that artifact to GitHub Pages through `beyond10x/website`'s `project-site.yml`, pinned by commit, for bot-authored pushes to `main` |
| `.github/workflows/shared-gates.yml` | the organization's shared source gates (`beyond10x/gates`, pinned by commit) |
| `.github/workflows/release.yml` | `Release`, with `contents: read`. On a tag `<major>.<minor>.<patch>`: `release-check.sh` and `task check`, then one archive per target of `release-package.sh targets`, each built and its `agenticspec --version` checked on a native runner (Linux gnu on `ubuntu-22.04` and `ubuntu-22.04-arm`, musl on `ubuntu-22.04` and `ubuntu-24.04-arm`, static and run in Alpine, both macOS targets on `macos-latest`, the x86_64 one run under Rosetta 2, Windows on `windows-latest`), then `SHA256SUMS` and `notes.md` as the artifact `agenticspec-<version>-release`. On a pull request from `release/<version>` to `main`: `release-check.sh` on the workspace version, which the branch must name, and the same archives and artifact without the gate (the gate of `pages.yml` runs there), so the release pull request proves every target builds before the tag. It creates no release and reads no token (§ "Release"). Its job `Gate` installs the tools of the job `gate` of `pages.yml` at the same versions and digests; `release.test.sh` fails when they differ, or when the matrix is not the targets of `release-package.sh` on their runners |
| `.config/nextest.toml` | the limits of `task test`: the profile `forge` (GitHub Actions) ends a test at 60 s and the run at 120 s, the profile `default` (everywhere else) at 30 s and 75 s (§ "The test step equals the forge") |
| `.cargo/config.toml` | links the C runtime of `x86_64-pc-windows-msvc` statically, so the Windows binary needs no Visual C++ runtime. It sets nothing for any other target |
| `scripts/release-check.sh`, `release-package.sh`, `release-paths.sh`, `release-verify.sh`, `release.test.sh` | the scripts of a release: a release binary names no path of the machine that built it (`release-paths.sh`, run by each packaging job after the build maps the runner's home, cargo home and checkout with `--remap-path-prefix`); the tag, the workspace version and the changelog section agree; targets, archives (`.tar.gz`, a `.zip` for Windows; GNU tar or bsdtar, zip or 7z), `SHA256SUMS` and notes; a published release is what § "Release" makes. `release.test.sh` holds their cases |

## Local store

- `agenticspec sync` lifts each main transcript of the window whose files changed (size and
  modification time of the transcript and its `subagents/` files) and writes, in one
  transaction per session: the redacted commands (zstd), the `agenticspec.view` rows (events
  zstd), the session's KPIs and its check verdicts (`queries/session.sq`).
- `query`, `mine`, `procedure`, `kpi` and `analyze` sync first and replay the stored commands;
  `query --transcript` reads the file.
- The store is `$XDG_DATA_HOME/agenticspec/sessions.db` (or `--state-dir`), never inside this
  repository: it holds the harness's session titles. `SCHEMA_VERSION` in `store.rs` rises when
  the derived tables change meaning; a store of another version drops and fills them again and
  keeps its `wire_steps`. `WIRE_FORMAT` rises when a wire row's encoding changes; a store of
  another wire format is refused and left unchanged.
- `agenticspec serve` binds 127.0.0.1 only and answers as the one local `Viewer` (`ui/README.md`).

## Rules

- Raw transcripts and unredacted IR never enter this repository. Only redacted IR and aggregate
  numbers do. Test fixtures are synthetic.
- A finding about secrets or personal data names the class and the count, never the value.
- KPIs read the events the generated component published, never a transcript.
- An author or a decision the transcript does not record is `Inferred` or `NotRecorded`, never
  guessed as `Known`.
- `GatewayFacts`, `WireCall`, `Wire`, `Answer` and the signatures of `lift_call`,
  `Appender::spawn` and `AppenderHandle::append` are a contract with llm-gateway (its
  interaction-records epic). Change a name or a signature only after telling the gateway's
  maintainers, and with a new tag.
- The sessions that act for the owner are named at run time (`AGENTICSPEC_DELEGATES`), never in
  code or fixtures; a fixture uses a neutral name such as `operator`.
- `shellspec` (the sibling repository) will lift the input of an `Execute` tool call into its
  sub-commands.

## The test step equals the forge

The forge runs `task test` on a runner with 4 threads. On a machine that runs many more threads, a
cost that every test process pays once hides in the wall time: the local step took 51 s while the
forge stopped at its 120 s limit after 450 of 660 tests (PR #44; the settings table parsed every
block file in each process, fixed in 529ad0b). The forge also ran that setup 2 to 16 times slower
than the build pool did (`a_set_on_a_link_by_its_index` 7.8 s against 0.49 s), while the current
tree runs at about the same speed on both (731 tests: 184 s of summed test time on the forge, 194 s
in the pool).

- `task test` runs nextest with `--test-threads 4`, the forge's count, on every machine. It is a
  flag because a flag wins over `NEXTEST_TEST_THREADS`, which a build pool sets to the width of
  the job (one pool's run of the old tree took 25.6 s on about 30 threads), and that variable wins
  over `.config/nextest.toml`.
- `task test` takes the profile `forge` (60 s a test, 120 s the run) when `GITHUB_ACTIONS` is
  `true`, and the profile `default` (30 s a test, 75 s the run) everywhere else: below the
  80.8 s the old tree took in the pool at 4 threads, with room for a busy pool.
- Measured in the build pool at 4 threads: the tree of 529ad0b with its fix reverted took 80.8 s,
  and a 60 s limit stopped it after 479 of 731 tests; the tree after it takes 36 to 51 s.
- A run of the default profile over 75 s is a defect of the test budget, like a forge run over
  120 s: do not raise the limit or the thread count, find the cost that grew.

## Commits

Every commit and push is `b10x-bot[bot]`'s through `b10x-gates bot --repository
beyond10x/agenticspec -- <git command>`; every other GitHub write goes through `b10x-gates api`,
with the gates policy file of the primary checkout. Every commit has a body. A tag and a release are
publications and wait for the owner's order.

Work lands in batches, not one pull request per story: each story is built on its own branch in its
own worktree, the finished branches are merged into one branch `integration/<date>` based on
`main`, the batch's gate (`slot --timeout 30m task check`) runs once on that branch, and one pull
request opens for it. A story's branch is not pushed and gets no pull request of its own. Open pull
requests that overlap (the same `CHANGELOG.md`, `blocks/`, generated files) conflict with each other
after every merge and each costs a full CI run; one batch costs one.

## Release

A release is the workspace version at a bare tag (`0.1.0`): one archive of the `agenticspec`
binary per target, each with `LICENSE` and `README.md`, `SHA256SUMS`, and notes cut from
`CHANGELOG.md`. The simulator is its subcommand `agenticspec sim`; `agenticspec-docs` and
`agenticspec-cli-contract` are not shipped. The targets (`release-package.sh targets`) and their
archives:

| target | runner | archive |
|---|---|---|
| `x86_64-unknown-linux-gnu` | `ubuntu-22.04` | `agenticspec-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| `aarch64-unknown-linux-gnu` | `ubuntu-22.04-arm` | `agenticspec-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| `x86_64-unknown-linux-musl` | `ubuntu-22.04` | `agenticspec-<version>-x86_64-unknown-linux-musl.tar.gz` |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | `agenticspec-<version>-aarch64-unknown-linux-musl.tar.gz` |
| `aarch64-apple-darwin` | `macos-latest` | `agenticspec-<version>-aarch64-apple-darwin.tar.gz` |
| `x86_64-apple-darwin` | `macos-latest` (run under Rosetta 2) | `agenticspec-<version>-x86_64-apple-darwin.tar.gz` |
| `x86_64-pc-windows-msvc` | `windows-latest` | `agenticspec-<version>-x86_64-pc-windows-msvc.zip`, with `agenticspec.exe` |

Every binary prints its `--version` on its runner before it is packed. The tag workflow builds
and the bot publishes; a pull request from `release/<version>` builds the same archives first.
The musl archives need no glibc of the host; a consumer of the binary pins the x86_64 one
(organization rule for every Rust tool). The workflow refuses a musl binary that is not static
or whose `--version` is wrong in a plain Alpine container. The Windows binary links its C
runtime statically (`.cargo/config.toml`). Consumers of `agenticspec-records` pin a released tag.

Steps; `<version>` is greater than the newest tag on `origin/main`, and every git command of the
bot is `b10x-gates bot --repository beyond10x/agenticspec -- <git command>`:

1. Branch `release/<version>` from `main`, with one commit `release: agenticspec <version>`: the
   `version` of `[workspace.package]`, `Cargo.lock` as `cargo build` rewrites it, and in
   `CHANGELOG.md` the line `## [<version>] - <YYYY-MM-DD>` directly below `## [Unreleased]`.
   `sh scripts/release-check.sh <version>` passes and `slot task check` is green on it.
2. A bot pull request to `main`, merged by the bot with a merge commit when its checks are green,
   among them the `Package` job of `Release` for every target.
3. An annotated tag on that merge commit: `tag -a <version> -m "agenticspec <version>" <commit>`,
   then `push origin refs/tags/<version>`.
4. The tag starts `Release`. Wait for it in the background; read a red run with
   `gh run view <run> --repo beyond10x/agenticspec --log-failed`. A runner fault is re-run once
   (`b10x-gates gh --repository beyond10x/agenticspec -- run rerun <run>`); a fault of the tree
   leaves the tag as it is, without a release, and the next patch version is cut after the fix.
   A green run: `gh run download <run> --repo beyond10x/agenticspec -n
   agenticspec-<version>-release -D target/release-assets`, then `sha256sum -c SHA256SUMS` there.
5. `b10x-gates gh --repository beyond10x/agenticspec -- release create <version> --verify-tag
   --title "agenticspec <version>" --notes-file target/release-assets/notes.md` with the seven
   archives and `SHA256SUMS` of `target/release-assets/`.
6. `sh scripts/release-verify.sh <version>` exits 0; only then is the release reported as
   released.
