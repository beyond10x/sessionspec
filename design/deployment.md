# Deployment: from an agentic architecture to a running target

Status: approved, every open point (Q1-Q17) at its recommended option; nothing built. Decisions
K1-K19. Builds on `design/architecture-runtime.md`, `design/ingress.md` (I1-I12),
`design/control-plane.md` (C1-C8), `design/view-layer.md` (D1-D11) and
`design/events-and-storage.md` (E1-E10) without repeating them. Heads read: weft `main` at
`4138723`, cited `weft:<path>:<line>`; Loom `main` at `b4b5de28c`, cited `loom:<path>:<line>`;
cloudspec `main` at `ee8ec21`, cited `cloudspec:<path>` with the row of `docs/boundary.md`.

## The question, and the verdict

An agentic architecture is more than its agents: it has a control plane (a web service), stores,
a gateway, lease authorities, scheduled runs and endpoints. How does it reach a running target?

**Verdict: agenticspec writes one deployment bundle (`agenticspec-deployment/1`, K6); a cloudspec
inbound translator turns it into a cloudspec architecture; weft runs that architecture, and every
other target (a Helm chart, a Compose file, ...) is generated from it.** cloudspec is the one
common point: every source is translated into it, every target is generated from it, and weft runs
only cloudspec specifications. The translator lives in cloudspec (epic `translation`, being
drafted), not in weft and not here. agenticspec keeps what only it knows: packing, the contract,
admission of agentic semantics, and the bundle. It writes no file of a cloudspec or target format.
Until the chain exists, only the in-process runtime (`design/architecture-runtime.md` § "The
smallest first runtime") runs an architecture: the local test.

## What weft, Loom and cloudspec say today

| source | says | where |
|---|---|---|
| weft | a compute cloud a team deploys itself; specification first, no code | `weft:README.md:8-9` |
| weft | other formats of the ecosystem enter only as deployable workloads, through a translator of its `deployments` domain: ESS system, `loom.agent/1`, Compose, Helm, Kubernetes, later ModelSpec | `weft:AGENTS.md:62-67`; source kinds `weft:ess/6-delivery/deployments.yaml:26-34` (no agenticspec kind) |
| weft | the translator model: one `Translator` per source kind and version; source digest + translator version -> revision + `CoverageReport`; each object mapped, mapped with loss or refused, and a refusal stops the deploy; deterministic byte for byte; a translator cannot grant more than the source declares | `weft:docs/domain-sketch.md:354-366`; the grant rule is not yet held by its specification (`weft:ess/6-delivery/deployments.yaml:385-387`, "UNMAPPED") |
| cloudspec | `deployments` moves to cloudspec whole: `Translator`, `DeploymentSource`, `Translation`, `CoverageEntry`, `Plan`, `Release`, `ReleaseStep`, `Drift` | `cloudspec:docs/boundary.md` row 19 |
| weft | an agent document becomes an agent runtime instance with its tools, model access and sandbox; a spec kind never runs as it is; a `Runtime` runs packages of some kinds under a minimum isolation | `weft:docs/domain-sketch.md:346-347`; `weft:ess/4-applications/workloads.yaml:359-363`, `:1299-1302` (`SpecKindNotRunnable`) |
| weft | every package is a content-addressed, signed artifact the node verifies before start | `weft:docs/domain-sketch.md:222-223` |
| weft | instance health `unknown`, `starting`, `ready`, `not-ready`, `backing-off`; an observation comes only from the node agent's `Record*` call; releases `planned` to `failed`, plan, simulation, drift | `weft:ess/4-applications/workloads.yaml:224-231`; `weft:docs/domain-sketch.md:609-610`, `:369-387`, `:622-634` |
| weft | logs, metrics and traces; a `TelemetrySink` forwards to an outside receiver (OTLP, syslog, a bucket) | `weft:docs/domain-sketch.md:635-660` |
| weft | `local` lowering: one process is control plane and worker | `weft:docs/domain-sketch.md:713` |
| weft, nothing yet | agenticspec; a feed an outside reader follows for one formation (the API and CLI surface directories named in `weft:AGENTS.md:19-20` are empty); a workload's own records (an agent's runs) handed back; a source label on observations | none |
| Loom | runs an agent from a `loom.agent/1` document: `b10x-loom agent check` (refuses before anything runs), `agent serve` (long-running), `agent run` (one turn and exit, "as a scheduled job does", one line in `record.jsonl`) | `loom:README.md:262`, `:285`, `:310-315` |
| Loom | the image `ghcr.io/beyond10x/loom-agent:<version>`; the A2A listener's readiness and liveness probes; on Kubernetes the check runs first and the resource reads `Pending`, `Running`, `Degraded`, `Refused`, `Completed` or `Failed` | `loom:README.md:396`, `:727`, `:418-425` |
| Loom | the run events of a run, as a generated contract | `loom:docs/contracts/run-events-v1.json` |

## K1 Words, defined once

| word | means here | why this word |
|---|---|---|
| **pack**, packing | decide which parts share a unit | new; "placement" is taken twice: the entity-to-part `Placement` (`design/view-layer.md` M45) and cloudspec's target kind and scheduling decision (cloudspec `docs/boundary.md` rows 2, 12) |
| **unit** | one thing a target starts and watches: an agent host, a service, a job, a scheduled job, a store | neutral; becomes one `agentic.runtime.Workload` (`blocks/8-place/runtime.yaml`) |
| **bundle** (noun and verb) | write the `agenticspec-deployment/1` document of one deployment (K6) | new. *package* is taken twice: weft's package axis (`weft:docs/domain-sketch.md` § 4 "Package") and, beside *pack*, it reads as the same step. weft also has a package kind `bundle`, a directory of code (`weft:docs/domain-sketch.md:226`); a deployment bundle is never a package, only a translator's source, and is written with its format name wherever both could meet |
| **translate**, translation | the cloudspec inbound translator turns a bundle into a cloudspec architecture and a coverage report, with a digest | the word of the translator model (`weft:docs/domain-sketch.md:354-366`), which moves to cloudspec (row 19); agenticspec runs no translator |
| **realize**, realization | a cloudspec architecture goes onto one target: weft runs it, other targets are generated from it | cloudspec `epic:realization`; ESS `ess specify realization` |
| **project**, projection | typed input to files of one format, never applied | ESS `ess generate project`: "never apply them"; inside cloudspec's realization only |
| **deploy** | bundle, translate, realize and start, ending in a running deployment | the user's word for the whole step |

Not used: *package* (above), *synthesize* (ESS suites, code), *materialize* (control-plane C1),
*compile*, *distribute*, *lower* (weft's word for deploying itself onto a host or cluster).

## The chain

```
architecture (portable)          architectures/<name>.yaml           revision digest
  + contract (derived)           agenticspec-contract/1              contract digest
  + binding with packing         agenticspec-binding/1, packing:     binding digest
  -> admit (agenticspec)         plan with units, or every refusal   K5, K14
  -> bundle (agenticspec)        agenticspec-deployment/1            bundle digest
  -> translate (cloudspec)       cloudspec architecture + coverage   translation digest
  -> realize: weft runs it; Helm, Compose and other targets are generated from it
  -> running: the target reports units and agent runs; the core unit records them (K15)
```

What exists: binding and admission as concepts with an open format
(`design/architecture-runtime.md` § "Three documents", § "Admission"); the runtime block
(`blocks/8-place/runtime.yaml`: `WorkloadKind` `Agent, Service, Job, Model, Snippet`, `Shape`,
`StartInstance` to `ReportFailed`) with nothing that defines workloads from an architecture; a
simulator that runs all parts in one world (`crates/agenticspec-sim/src/sim.rs:1-6`); cloud
blocks to be (cloudspec `docs/boundary.md` rows 9-22) and no cloudspec format yet (scope of
cloudspec `epic:simulation-and-search`).

## K2 Packing lives in the binding, never in the architecture

A `packing:` section of `agenticspec-binding/1`: the architecture stays portable
(`design/architecture-runtime.md:8-10`), the packing differs per target as the binding does.
Absent, it is the default (K4). `agenticspec packing default <architecture>` prints the resolved
packing to paste and change, as `agenticspec ingress default` does (`design/control-plane.md` C1).

## K3 Unit kinds

| unit kind | lives | workload kind | holds |
|---|---|---|---|
| `core` | long, exactly one | `Service` | the event log, the blocks' inner behaviour, the adapter serving every ingress (D7, I10) and with it the control plane |
| `agent` | long, replicas | `Agent` | agent parts, their `per` parts (workspace, sandbox), channels inside |
| `service` | long, addressable | `Service` | gateway, pool lease authority, tool server, a bus |
| `job` | until done, started on demand | `Job` | an agent part started by a spawn or an `on_event` reaction |
| `scheduled` | until done, started by a clock | `Job` | an agent part whose only input is an `on_schedule` trigger |
| `store` | long, keeps state | none | a store part (E4) bound to a storage service |
| `model` | long | `Model` | a self-hosted endpoint (`model.serve`) |
| `external` | not started | none | principals, hosted model routes, external stores and tools |

A unit names its least isolation (the highest of its sandboxes); its form is the target's (K8).

## K4 Strategies, and the default

| strategy | units | for |
|---|---|---|
| `single` (default) | everything in `core`, `external` parts outside | the local test, day one, small setups |
| `by_kind` | one unit per part kind | fewer units with separate scaling of agents |
| `by_part` | one unit per part; `count: n` becomes n replicas | independent scaling and failure |
| `groups` | named lists of parts, the rest in `core` | anything in between |

Per part, `schedule: job` or `shape: job` (an agent woken only by `on_schedule` becomes `scheduled`; `schedule: <cron>` makes it `scheduled` with that rule, K18; the trigger's
`policy` and `max_parallel` become the overlap rule) and `external` override the strategy. `per`
parts follow the part they belong to. A `count` marked `vary` is packed at its built value (`sim.rs`).

## K5 Admission of a packing

`admit` (`design/architecture-runtime.md` § "Admission") also admits the packing: the plan lists
the units, their part instances and the links that cross units; a refusal lists every finding:

| refusal | when |
|---|---|
| `part_unpacked`, `part_packed_twice` | a part instance is in no unit, or in two; `external` counts as packed |
| `per_split` | a `per: X` part is not in the unit of its X instance |
| `shared_state_split` | a pool, a gateway's rate limit, an architecture-scope store or an exclusive workspace is reached from two units and bound in memory |
| `link_without_transport` | a link crosses units and the binding names no transport for its carry |
| `isolation_unmet` | a sandbox's isolation exceeds what the target gives a unit and the binding names no sandbox inside it |
| `network_isolation_unmet` | a part the binding marks `network: isolated` on a target whose `network_policy` is `not_enforced` (an absent policy is unknown and refuses nothing) |
| `scheduled_with_inputs` | a `scheduled` unit holds a part with an input other than its trigger, or neither a clock nor a cron rule wakes it |
| `schedule_not_cron` | a `scheduled` unit without a cron rule of the binding whose clock's `every` no cron rule states exactly (K18); in a written bundle, also a unit whose `schedule` is not that rule |
| `core_count` | not exactly one `core` |
| `unit_not_hosted` | a unit kind or a role the copy of cloudspec's accepted list (K16) does not hold |
| `image_unpinned` | with `target.images: local`, a runtime, a part's `image` or a listed unit's `image` with neither `tag` nor `digest` (with `registry`, the default, one without its repository digest is refused as a binding of the wrong form) |
| `unit_image_conflict` | parts of one core or service unit name different images (`parts.<p>.image`) and the unit names none (`packing.units[].image`); K19 |
| `unit_image_missing` | a core or service unit holds a runtime part (block `runtime`) and has no image of its own, of its parts or, for the core unit only, `runtimes.core`; names the unit and its runtime parts; K19 |
| `unit_runtime_mismatch` | a core or service unit's `code.runtime` names an image that is not in the bundle's `runtimes`, or differs from the image the carried binding resolves for that unit (`binding::unit_image`; none gives the role `core-runtime`); names the unit, the image found and the image expected. At bundle time, two images of one key (one digest, two names) leave the second uncarried; in a written bundle (`deploy check`), also a unit or `runtimes` entry changed after writing. A unit refused `unit_image_conflict`, `unit_image_missing` or `image_unpinned` is left to that refusal; K19 |
| `durable_state_unplaceable` | a runtime part with `durable_state: true` (K7, R88) in a `job`, `scheduled` or `model` unit, which runs until done or serves a model and holds no volume; or whose state volume's name, `<part>-state`, is the name of a part of the document |
| `volume_mount_missing` | a volume a unit claims (a workspace's, or a runtime part's durable state) that no part of the unit mounts (`parts.<p>.volumes[].mount`, matched by name), or mounts at a relative path; there is no default path |
| `unit_config_conflict` | two sources of one unit (its `packing.units` entry, its parts) give one environment variable two different values, or two parts of one unit both give `args`; K20 |
| `unit_config_invalid` | a unit's `env`, `args` or `secret_files` of the wrong form (a name that is not `[A-Z_][A-Z0-9_]*`, a value that is neither a scalar nor `{secret: <name>}`, `args` not a list of strings, a secret file that is not `{secret, key?, path}` with an absolute `path`, on a part in no unit; K22); in a written bundle, also a `config` of the wrong form or not the one the carried binding resolves; K20 |
| `unit_scratch_invalid` | a unit's `scratch` of the wrong form (not a list of `{mount, size}`, a `mount` missing or not an absolute path in normal form, a `size` that is not text, another key, on a part in no unit); in a written bundle, also a `scratch` of the wrong form or not the one the carried binding resolves; K21 |
| `unit_mount_collision` | a mount of a unit's own, a scratch directory (K21) or a secret file (K22), at the mount of another mount of the unit (a volume a part of it mounts under `parts.<p>.volumes[]`, another mount of its own), holding it or inside it, by path segment; in a written bundle, also against the volume mounts the unit carries; K21 |
| `ui_upstream_unknown` | a UI unit (K17) reads the adapter of a unit (`ui.<ui>.upstream.unit`) the packing does not have; in a written bundle, a UI unit's `upstream.unit` that is no other unit of the bundle |
| `ui_upstream_path_invalid` | a UI unit's (K17) base path of its upstream's adapter (`ui.<ui>.upstream.path`, text) does not start with `/`; in a written bundle, a UI unit's `upstream.path` that is not text starting with `/`. A `path` that is no text is a binding of the wrong form |
| `nested_document_missing` | in a written bundle (`deploy check`) only: a part of block `architecture`, at any depth, pins a digest the bundle's `nested` carries no document of |
| `nested_document_mismatch` | in a written bundle (`deploy check`) only: a carried nested document whose `content_base64` is not its `sha256`, does not parse to its `document`, or whose canonical digest is not its key |
| `requirement_kpi_unknown`, `requirement_invalid` | a requirement (K12) names a KPI no stage knows, or is not `{kpi, at_least \| at_most \| equals, part?, window?}` with one number; `sim check` refuses both as `architecture_refused` |
| `ui_archive_mismatch` | at bundle time, the archive a UI of the consumer's own names (`ui.<name>.source.archive`) is not its `digest`; in a written bundle (`deploy check`), a UI unit's archive is not in `files`, or its SHA-256 is not the unit's `code.digest` |
| `ui_export_unknown` | a UI serves an export (`ui.<name>.export`; `operator.ui` for the built-in UIs) that no ingress of the architecture has |
| `ui_build_needs_toolchain` | a UI of the consumer's own names an `ess-ui/1` document (`ui.<name>.source.ess_ui`): its generated pages need node and vite, which the binary does not run; the detail names the archive form |

## K6 The bundle: `agenticspec-deployment/1`

`agenticspec deploy bundle <architecture> --binding <file> --out <file>` admits (K14) and, only
when admitted, writes the bundle; a refusal writes nothing and lists every finding. The `deploy`
group holds the commands on the deploy side, beside `agenticspec packing default` and
`agenticspec ingress default`; the deploy itself is cloudspec's and the target's. One YAML file
with every member inline, each document keyed by its digest (Q2). Same inputs, same bytes.

| member | holds | digest |
|---|---|---|
| `format`, `generator` | `agenticspec-deployment/1`; the agenticspec version | inside the bundle digest |
| `architecture` | the released document as written, its name | revision digest: SHA-256 of the canonical JSON of `parts`, `links`, `border` (`agentic.architecture.Digest`); and the digest of its bytes |
| `nested` | every document the architecture references, the nesting chain resolved: per document its name, the parts that refer to it (dotted), `document` parsed and `content_base64`, its exact bytes, with their `sha256`. `deploy check` resolves the architecture from these by the pins, recomputes each digest (`nested_document_mismatch`) and refuses a pin with no document (`nested_document_missing`); units, packing and contract read the architecture flattened | its canonical digest, the pin `crates/agenticspec-sim/src/nest.rs` checks |
| `contract` | `agenticspec-contract/1`: ports, routing invariants, multiplicities, ingress | `Contract.digest` |
| `binding` | `agenticspec-binding/1`, its `refines:` chain resolved into one document. Its `target` says what the target gives: `isolation`, `images` (`registry` or `local`), `network_policy` (`enforced` or `not_enforced`; absent, unknown), which a part marked `parts.<p>.network: isolated` needs enforced (`network_isolation_unmet`) | binding digest |
| `packing` | the resolved packing, the default applied (K4); its `external` a list of `{part, kind, reach}`: the part, its block, and where the target reaches it, `{address: <text>}` or `{name: <text>}` from the binding (`packing.external` entries `<part>` or `{part, reach}` beside `units:`, `packing.parts.<p>.reach` beside a strategy), `{name: <part>}` when it gives none. Any part may be external | packing digest |
| `units` | one entry per unit with the fields of K7 | inside the bundle digest |
| `ingress` | one entry per export of each ingress of the contract: `ingress`, `protocol`, `audience`, `authenticated`; `unit` and `part` that serve it (the binding's `part`, else `control`, the control plane in the core unit; the UI unit for the export a UI serves, K17); `bind` (`loopback` or `all`, `loopback` when the binding names none); `port` (an integer) and `address` when the binding gives them, from `ingress.<name>.exports.<protocol>`. Without a port the entry has no entry point on the target, and nothing is refused | operations inside the contract digest; unit, part, port, bind and address in the binding digest |
| `requirements` | the `requirements:` of K12, each as written with the stage it is decided at (`check`, `evaluation`, `operation`, `cloud`), its state (`met`, `missed`, `undecided`) and the value where known; checked like `sim check` does, a refusal a finding of K14 | inside the bundle digest |
| `agents` | each `loom.agent/1` document the binding names: `document`, parsed, and `content_base64`, the file's exact bytes (K18) | the SHA-256 of those bytes (Q12) |
| `runtimes` | each runtime artifact the binding pins: the core runtime image (`core`); an agent runtime image (`agent-runtime`, K18) only where the binding pins one (Q8); then each image a core or service unit runs of its own (K19), named by the unit, unless one of those carries it already. With `target.images: registry` (the default) by OCI repository digest, `<image>@sha256:<digest>`; with `local`, what the binding gave, `{image, tag, digest}` with a tag, a digest (the image's config id) or both | the image digest, or `<image>:<tag>` for a local image without one |
| `files` | only with a UI unit (K17): each file a unit names, by file name: `{digest, size, base64}`, the bytes inline | its `digest`, the unit's `code.digest` |
| `digest` | the bundle digest: SHA-256 of the canonical JSON of the document without this member | |

The bundle carries no secret value, no host, no provider and no target name: it is the input of
every target alike.

## K7 Per unit, and abstract roles

Each entry of `units`:

| field | holds |
|---|---|
| `name`, `kind` | the unit's name; its kind (K3), `agent` for the packing's `agent`, `job` and `scheduled` units |
| `shape` | an agent unit's: `service`, `job` or `scheduled` (K18) |
| `parts` | the part instances, by path |
| `replicas` | from `count`; `scheduled` and `job` units have none |
| `code` | the artifact that runs, by digest. An `agent`, `job` or `scheduled` unit stays one agent: its agent document (instructions, tools, model route, authority, harness role) by digest and the role `agent-runtime`, never flattened into a process command; a core or service unit names its image (K19) of `runtimes` as `runtime`, or the role `core-runtime` without one; a UI unit (K17) serves static files, `{kind: static, archive, digest}` |
| `claims` | CPU, memory, accelerators: the unit's `Shape` (`blocks/8-place/runtime.yaml`), plus an accelerator for `model`; `volumes`, one per workspace (Q16) and one per runtime part with durable state (below), each with its access (below), size and `mount`, the absolute path from the binding's `parts.<p>.volumes[].mount` of a part of the unit, matched by name (no default path; `volume_mount_missing`) |
| `isolation` | the least isolation level required, the highest of its sandboxes |
| `checks` | `startup`, `ready`, `alive`, each with its kind and path or command, and `port` (an integer) where the binding gives one (`checks.<unit>.<check>.port`). An agent unit with `expose.a2a` uses Loom's readiness and liveness probes (`loom:README.md:727`); without it, `b10x-loom agent check` is its startup command |
| `schedule` | for `scheduled`: the cron rule the binding gives (K18) |
| `clock` | for `scheduled`: the trigger and schedule that wake it, its `policy` and `max_parallel` as the overlap rule |
| `upstream` | for a UI unit (K17): `{unit, port, path, secret?}`, the unit whose adapter the page reads, the `api_base` of its `config.json` and the name of the credential it presents there |
| `config` | the non-secret configuration the binding gives the unit (K20): `env`, variable names sorted, each value text or `{secret: <name>}`, `args`, and `secret_files` (K22), `{path, secret, key?}` sorted by `path`; absent when it gives none |
| `scratch` | the scratch directories the binding gives the unit (K21): `{mount, size}` each, sorted by `mount`; no claim, absent when it gives none |
| `secrets` | every secret by name; never a value |
| `egress` | every destination by name: another unit's link with transport and protocol, or an `external` dependency. Each entry carries `name`, the string the target resolves: an external part's reach name (or its address, when the reach is one), the target unit's name for a crossing, the declared name for a named destination; and `address` when the binding resolves one (a named destination's `address`, a CIDR block or `<host>:<port>` or `{namespace, labels}`; an external part's reach address). A named destination the binding does not resolve carries `unresolved: true`. Nothing undeclared is granted (`weft:docs/domain-sketch.md:364-365`) |
| `roles` | the abstract roles the unit needs (below) |
| `ports` | the contract ports its parts use, by their derived names (`agenticspec sim contract`, `blocks/ports.json`), with the settings that narrow them; cloudspec holds them unchanged, and a target matches a runtime against them, more than one runtime per role being possible |
| `source` | bundle digest, unit name, part paths: carried into every object translated from the unit and every observation of it (C3) |

**Durable state of a runtime part (R88).** A workspace is files at a base revision (Checkout,
ScratchCopy, Worktree); a service's own durable state, the records it appends and reads back after
a restart, is none of those. A runtime part declares it with the setting `durable_state: true`
(`blocks/settings.yaml`, default false). Its unit claims one more volume, named `<part>-state`,
its access by the rule below, `per_instance` when the part is `per` another, its `size` from the binding's
`parts.<part>.size` and its `mount` from `parts.<p>.volumes[]` of a part of the unit naming
`<part>-state`, as a workspace's volume takes them; without an absolute mount it is refused
`volume_mount_missing`. A `job` or `scheduled` unit runs until done and a `model` unit serves a
model the target runs: none of them holds such a volume (`durable_state_unplaceable`). An external
runtime part is not started by the target, and its state is not the bundle's.

**Outside writers of durable state (R109).** A service may share its state directory with other
programs on the same host (a backup, an indexer). Modelled as a workspace with
`sharing: shared_read_write`, it would make the contract ask for `workspace.lease` and
`workspace.proposal`, which nothing serves. So the runtime part names them instead:
`state_writers: [<name>, ...]`, beside `durable_state: true`. A sibling setting, not a map in place
of `true`: the settings table has one type per setting, `durable_state` stays a boolean, its
meaning and every bundle without writers are unchanged, and the architecture page shows the list as
the part's property `state writers`. The new type is `string` with `list: true` (a non-empty list
of non-empty texts); the writers are not parts of the architecture, so `part` does not fit. The
volume claim is the one of `durable_state: true`, its access by the rule below (the writers are
outside the bundle's units), with `writers` added. `state_writers` without `durable_state: true` is
`setting_invalid`. The contract does not carry the writers: no port is narrowed by them.

**Volume access follows from who mounts it (R93).** A workspace's `sharing` says how its holders
use it; a claim's access says how many pods attach it, and a single-node storage provisioner
refuses `read_write_many`. So the access is derived from the mounts, the same data that gives the
`mount` and `per_instance`: the parts naming the volume under `parts.<p>.volumes[]` of the binding,
by unit, and the claiming unit's `replicas`. A `shared_read_only` workspace is `read_only_many`
(the bundle's only read-only access, for one mounting copy as for many). Any other claim is
`read_write_once` when no part of another unit names the volume and the claim is `per_instance` or
the unit has `replicas: 1`; otherwise `read_write_many`. A `job` or `scheduled` unit has no
`replicas` and its runs may overlap, so a claim there that is not per instance is
`read_write_many`. The same rule covers a runtime part's `<part>-state`. Links to a workspace do
not count as mounts: a part that reaches it over a link without naming it under `volumes` does
not attach the claim. `deploy check` does not re-derive the access; it checks the mounts
(`volume_mount_missing`).

**Abstract roles.** The bundle names roles, never a provider: a store's kind (record, key-value,
semantic, graph; E4), a model's kind and its route alias, a capability the unit needs from the
platform (to start jobs in its scope, for a `job` unit; Q7). The binding stays the place where the
agent side is concrete (a harness, an agent document, a model route); the translator binds each
role on its target, and its coverage report says how each was bound. agenticspec checks that every
port is bound to a role, an implementation or a fixture; whether a target can serve a role is the
translator's and the target's.

## K8 The target is cloudspec's

- **Always a namespace-like scope: yes.** Every target has one scope holding exactly one
  deployment's units, names and state, deleted with it: a state directory on this host, a project
  name on a container host, a cluster namespace, a scope of a compute cloud; cloudspec's `scope`
  (cloudspec `docs/boundary.md` D1 A). Several architectures share a scope only nested in one.
- **Targets.** weft runs the cloudspec architecture (on this host through its `local` lowering,
  `weft:docs/domain-sketch.md:713`); a container host and a cluster namespace get files generated
  from the same architecture (Compose, Helm). Each declares through cloudspec the unit kinds,
  roles and isolation it hosts and refuses the rest by name.
- **An agent stays an agent workload, and the target picks its runtime.** An `agent`, `job` or
  `scheduled` unit reaches cloudspec as an agent workload (package kind `Agent` over its agent
  document), not as a generic process. The target binds the role `agent-runtime`: weft runs the
  unit as a Loom agent (its deployment source kind `LoomAgent`, `weft:ess/6-delivery/deployments.yaml:34`;
  Loom owns the agent document format, `weft:docs/domain-sketch.md:725-726`), as a workload
  process that runs Loom over the agent document: `agent serve` for a long-running unit, `agent run` for one turn
  (`loom:README.md:285`, `:310`). The target embeds no Loom. One scope and one network carry
  everything. Loom serves no web service, so it is never a whole target: `core` and the control
  plane need a service. Agents that already run elsewhere are bound `external` (Q11).

## K9 Boundary with cloudspec

| concern | agenticspec | cloudspec | target (weft, generated files) |
|---|---|---|---|
| which parts share a unit, unit kind, replicas from `count` | packing (K2-K4) | none | none |
| contract, ports, routing invariants, agentic admission | yes (K14) | none | none |
| the bundle and its digests | writes it (K6) | reads it | none |
| translation, its coverage, grants within the source | a copy of the accepted kinds and roles (K16) | the inbound translator (epic `translation`) | none |
| roles to providers, files of each format | none | yes | runs what it is given |
| network, entry points, certificates, network policy | the reach list and exports | endpoint, route, certificates, limits | serves them |
| secrets; quota, placement, isolation | names; `Shape` per unit | custody and delivery; infrastructure, scheduling | quota, policy, placement (K14) |
| rollout, drift, health | records what is reported (K10, K15) | `deployments` (plan, release, drift; row 19) | observes and reports |

agenticspec keeps no target-specific code; cloudspec and the targets keep no agentic semantics.

## K10 Records and the control plane

- New domain `agentic.deployment` (layer 9): `Deployment` (revision; contract, binding, packing
  and bundle digests; the translation digest once translated; target and scope by value, as ESS
  has no relation across systems, cloudspec `docs/boundary.md` E3), states `Requested`,
  `Refused`, `Bundled`, `Translated`, `Realizing`, `Running`, `Degraded`, `Stopped`, `Failed`;
  `Unit` (name, kind, part instances, replicas, its workload, its target object by value, read
  from the coverage report). `RequestDeployment` is refused as `NotReleased` (except to the
  in-process runtime), `NotAdmitted` (with the list), `CoverageRefused` (the translator's refused
  objects), `TargetRefused` (the target's named violations), `ScopeTaken`.
- Health: what the target reports (K15) becomes each unit's `ReportRunning`, `ReportExited`,
  `ReportFailed` (`workload.run`). Operations (C3) gains a `Units` view (desired and ready
  replicas, last failure, parts), the Overview a tile; a row reaches its unit through its part
  (`Placement`, M45). Conformance: a scenario that passes in process passes on the deployment.

## K11 Local test, then promote

The local test runs the released revision in process, with the scenario's fixtures or a local
binding. Promotion is a new deployment of the same revision with another binding and packing, so
a new bundle; a target binding `refines:` a base binding, as an ESS server binding refines a
protocol binding (`design/ingress.md` § "What ESS already offers"). `Deployment` names the one it
was promoted from, so both are compared on the same KPIs.

| changes | must not change |
|---|---|
| target and scope, packing, endpoint addresses, authenticators, secret names, transports, store kinds, the bundle digest | revision digest, contract digest, requirements (K12), scenarios and their expectations |

## K12 Requirements as KPIs with a target

A top-level `requirements:` key of the architecture, `- { kpi: cache_hit_share, at_least: 0.7 }`,
optionally per part and window; portable, outside the nesting digest like `ingress:` (I8). The
check refuses an unknown KPI id. A requirement is decided when its KPI can be read: a design KPI
(`story:architecture-kpis`: `agents_total`, `capacity_ratio`) at check, a simulated one at
evaluation, a computed one (`cache_hit_share`, `cost_usd`) in operation. The search treats
requirements as constraints before weights and stops when all are met. The Metrics page (C3)
shows each with its target, met or missed, predicted beside observed. A cloud KPI (node count,
cost per hour) is named by its cloudspec id and evaluated by cloudspec. The bundle carries the
requirements; the translator keeps them with the revision and no target enforces them yet.

As built (`crates/agenticspec-sim/src/requirements.rs`):

- Entry: `{kpi, at_least | at_most | equals, part?, window?}`, exactly one number as target;
  anything else is `requirement_invalid`, an id no stage knows `requirement_kpi_unknown`. A cloud
  KPI is written `cloudspec:<id>`. A design KPI takes no `part`.
- Stages: `check` for the KPIs of `whole.rs` and the gate counts; `evaluation` for what the
  simulator measures or the catalogue marks `simulated`; `operation` for the rest of the
  catalogue; `cloud` for `cloudspec:<id>`.
- A missed requirement is a finding the check reports (`sim check` prints each with its state),
  never a refusal: K14 admits on a known KPI and a well-formed target, the Metrics page shows met
  or missed, and no target enforces a requirement yet. The same holds in the bundle.
- The search ranks a feasible variant by requirements missed, then by fitness; `fit` stops after
  the round in which a variant first meets every requirement it can decide. A target is never
  searched, also under `optimize: all`.

## K13 Translation, in cloudspec

The inbound translator follows the translator model (`weft:docs/domain-sketch.md:354-366`), which
moves to cloudspec with `deployments` (row 19): registered for `agenticspec-deployment/1`, bundle
digest + translator version to a cloudspec architecture and a coverage report, deterministic,
granting nothing the bundle does not declare. What agenticspec asks it to map:

| agentic | cloudspec construct (cloudspec `docs/boundary.md` row) |
|---|---|
| `agent` unit | package kind `Agent` over the agent document, its runtime bound by role (on weft: Loom); shape `Service` with the unit's replicas (row 10) |
| `job` unit | an `agent` unit of shape `job` (K18): `Job`, started at run time (`agent run`); needs the role "start jobs in the scope" |
| `scheduled` unit | an `agent` unit of shape `scheduled` (K18): `Schedule` of a `Job` on the unit's `schedule` (cron), overlap from the trigger's `policy` (row 10) |
| `service` unit, `core` unit | `Service`; `core` runs the core runtime image (Q4) |
| `store` unit | a managed-service binding (row 21) or a volume claim (row 6), by role |
| store kind | cloudspec's managed-service contracts (its layer 1, epic `service-contracts`): `record` -> relational record store, `key-value` -> key-value store, `semantic` -> semantic store, `artifact` -> artifact store, `graph` -> graph store |
| `model` unit | package kind `Model` with an accelerator claim (rows 10, 11) |
| `external` part | an egress link to a named destination (cloudspec E4: not modelled yet) |
| link across units | link with protocol; the unit's `egress` its allow-list |
| export `http`, `event_stream`, `ui`, `tool`, `agent` (I7) | an edge route on `core`, a certificate when public (rows 9, 18); service exposure when internal |
| export `metrics_scrape` | observability: series named by KPI id (row 16) |
| secret names | secret grants by name (row 15); values never |
| requirements | kept with the revision, not enforced |

Not mapped, served by `core` and checked by agenticspec: routing invariants beyond the network
allow-list (every model call inside a gateway call), grants, questions, decisions, lease
semantics, `on_message` and `on_event` triggers, spawned sub-agents, principal ports.

## K14 Who admits what

| step | admits | refused as |
|---|---|---|
| agenticspec, before hand-over (`deploy bundle`) | `agenticspec sim check` passes; the contract derives and is valid; every nested architecture is pinned and its canonical digest matches; every port is bound to a role, an implementation or a fixture; the packing (K5) and the ingress (I9) are admitted; every requirement names a known KPI and a well-formed target; every agent document is present and matches its digest; every UI unit's upstream names a unit and its archive matches its digest (K17); every unit kind and role is in the accepted list (K16) | `NotAdmitted`, every finding |
| cloudspec's translator | the bundle digest; full coverage: every unit, link, export and role mapped, mapped with loss, or refused by name, and a refusal stops; grants within what the bundle declares | `CoverageRefused`, the report |
| the target (weft) | quota; placement feasibility (isolation level, accelerators); the account's secret and egress policy; each agent document served by Loom, which checks it before it runs (`b10x-loom agent check`, `loom:README.md:262`) | `TargetRefused`, the named violations (weft's plan and simulation, `weft:docs/domain-sketch.md:378-384`) |

## K15 What comes back

- From the translator: the coverage report, each object with its source (C3), and the translation
  digest. agenticspec fills each `Unit`'s target object from it.
- From the target, through cloudspec's observation contract (C5), per unit: the release state,
  desired and ready instances, instance health (`weft:ess/4-applications/workloads.yaml:224-231`),
  exits and failures with their reason, drift; each observation labelled with its source.
- Agent runs: per turn of a Loom agent its record (one line of `record.jsonl` in the state
  directory, `loom:README.md:310-315`; run events, `loom:docs/contracts/run-events-v1.json`), as a
  resumable feed per instance; `agent-run-lift` lifts them into block events.
- KPI series of the `metrics_scrape` exports.
- The reader is the `core` unit: it follows its own deployment's feeds with its workload identity
  and records them through `agentic.runtime.Host` (Q5). weft says nothing yet about such a feed,
  about handing back a workload's own records, or about a source label on observations.

## K16 The equality test sits with cloudspec

- agenticspec commits fixture bundles, `bundles/<architecture>.<binding>.yaml`, one per catalog
  architecture with a test binding, and `bundles/sources.sha256` (architecture, nested documents,
  binding, agent documents, generator version). `task check` compares the digests; the `slow` lane
  writes them afresh and compares bytes.
- cloudspec knows an agenticspec bundle only as a deployment artifact, as it knows a Compose
  file: its translator story (`translate-in-agenticspec`) takes copies of these fixtures as its
  sample inputs, with no lock on an agenticspec commit, and tests that each translates with full
  coverage, every loss named.
- agenticspec holds a copy of cloudspec's accepted unit kinds and roles, `bundles/cloudspec-accepts.yaml`,
  with the cloudspec commit it came from. `deploy bundle`, admission (`unit_not_hosted`) and a test
  fail when a bundle would carry a kind or role not in it.
- A change of the bundle's meaning raises the format to `/2`; cloudspec registers a translator per
  format version.

## K17 A UI is a unit of its own

Settled with the translator, names exact. A binding asks for a UI under `ui:`:
`ui: {operator: {port, bind, upstream: {unit, port, path, secret}}}`, every key optional. Without `ui:` the
bundle is the bundle it was (K16's fixtures keep their bytes); with it, the bundle stays `/1`: the
additions are a unit, the optional member `files` and the unit of one ingress entry.

- One unit per UI, named by its role: `{name: operator-ui, kind: service, roles: [operator-ui],
  parts: [], code: {kind: static, archive: operator-ui.tar, digest: sha256:<hex>}, upstream,
  claims, checks, ...}`. `deploy bundle` builds the archive in-process with the library of
  `agenticspec ui build`, from the architecture and its contract; it is the same bytes as
  `ui build --archive` writes for them. The archive is carried inline, base64, in `files` under
  its file name, as every other member is inline.
- The UI unit serves its export: the `ingress` entry of the operator ingress's `ui` export names
  `unit: operator-ui` and `part: control`, with the port and bind of the binding's export
  (`ui.operator.port` and `bind` when the export gives none; both given, they must agree). The
  other exports of the ingress stay with the adapter.
- `upstream: {unit, port, path}` is the runtime unit whose adapter (the runtime UI interface,
  `ui/runtime.openapi.yaml`) the page reads: the binding's `upstream.unit`, else the unit that
  serves the operator ingress's `http` export; its port, the binding's, else that export's when it
  is served by the same unit; `path`, the binding's `upstream.path` (`/api/` when absent), the
  `api_base` of a built-in UI's `config.json` (the archive of a UI of the consumer's own is
  carried as it is). A binding without `path` gives the bundle it gave before.
- `upstream.secret` (R128) is the name of the credential the UI presents to its upstream, the
  `bearer` credential of the runtime UI interface, never a value. A part of the upstream unit
  declares it (`parts.<part>.secrets`), as a secret of any unit is declared; the UI unit carries it
  in `upstream.secret` and in `secrets`, so the client and its upstream share one declared secret.
  A name that is none is `secret_value`; one no part of the upstream unit declares, or that the
  UI unit's `secrets` does not list, is `secret_undeclared`, at bundle and at check. A binding
  without it gives the bundle it gave before. How a target mounts the secret into the static
  server is the target's.
- Its claims and check ports come from the binding as for any other unit, by the unit's name:
  `claims.operator-ui` (500 millicores, 512 MiB and no accelerator when absent) and
  `checks.operator-ui.<ready|alive>.port` (the export's port when absent).
- Refusals: an upstream that names no unit (`ui_upstream_unknown`, at bundle and check); an
  upstream path that does not start with `/` (`ui_upstream_path_invalid`, at bundle and check);
  `deploy check` hashes the carried archive against the unit's digest (`ui_archive_mismatch`);
  the role is in the accepted list (K16, `unit_not_hosted`); a UI that does not build is
  `ui_build_refused`.
- A UI is one row (`crates/agenticspec-sim/src/deploy/ui.rs`, `Ui`): its key under `ui:`, its
  unit's name and role, the ingress and export it serves and the source of its files. Two sources
  are built in (`BUILT_IN`): `operator` (`operator-ui`) and `control-plane` (`control-plane-ui`,
  `ui build --control-plane`), both serving `operator.ui`, on the same code path as any other row.
- A UI of the consumer's own (R84) is any other key: `ui.<name>: {source, export, upstream?,
  port?, bind?}`. `source` is `{archive: <file>, digest: sha256:<hex>}`, a static archive the
  consumer built, read beside the binding file that names it and checked against `digest`
  (`ui_archive_mismatch`), or `{ess_ui: <file>}`, an `ess-ui/1` document, refused with
  `ui_build_needs_toolchain`: `ui generate` writes Vue components that need node and vite to
  build, and the binary does not run them (the operator app is prebuilt and renders a derived
  layout, not an arbitrary `ess-ui/1` document). `export` is `<ingress>.<protocol>`
  (`ui_export_unknown` when no ingress has it). Its unit is `{name: <name>, kind: service, roles:
  [ui], code: {kind: static, archive: <name>.tar, digest}, ...}`, the archive carried byte for
  byte in `files`, with the upstream, checks, claims and refusals of the built-in UIs. The first UI
  that serves an export serves it, the built-in ones first, then the others by key; the accepted
  list holds the role `ui`.

## K18 Agent units, as the translator reads them

Settled with the translator, names exact; the bundle stays `/1`.

- **The agent runtime.** A binding may pin it as `runtimes.agent-runtime: {image, tag, digest}`,
  named by the role it plays, in the shape of the core runtime and under the same `target.images`
  rules: from a registry by its repository digest (a tag is refused as a binding of the wrong
  form), from the local store by tag, config id or both (neither: `image_unpinned`). The bundle's
  `runtimes` carry it, and each agent unit's `code.runtime` names it beside `code.role:
  agent-runtime`. Absent, nothing changes: the target binds the role (Q8 A). The earlier name
  `runtimes.agent` is refused with the new one.
- **The agent document's bytes.** `agents.<digest>` carries `content_base64`, the exact bytes of
  the file the binding names, beside `document`, the same file parsed. `<digest>` is the SHA-256 of
  those bytes; it was already computed over the file's bytes, never over re-serialized YAML, so no
  digest changed. `deploy check` decodes, hashes and parses them again: bytes that are missing, not
  the digest, or not the `document` are refused as `agent_document_mismatch` (check only).
- **Shape.** The packing's `agent`, `job` and `scheduled` units are written as `kind: agent` with
  `shape: service`, `job` or `scheduled`; the packing document keeps its own kinds. A scheduled
  unit carries `schedule`, the cron rule (five fields) the binding gives with
  `packing.parts.<p>.schedule: <cron>`, or a listed unit's `schedule:`; a part given one is packed
  as a scheduled unit, and the rule counts as its clock (`scheduled_with_inputs` asks for a clock
  or a rule). Its clock in the architecture, the former `schedule` object, is `clock`.
  `packing.parts.<p>.shape: job` is another spelling of `schedule: job`.
- **Cron from `every`.** A scheduled unit whose binding gives no cron rule carries the one that
  states the `every` of its clock's schedule exactly: N minutes dividing 60 `*/N * * * *`, N hours
  dividing 24 `0 */N * * *`, one day `0 0 * * *`, one week `0 0 * * 0`; `next_fire_at` is not
  carried into the rule, it stays in `clock`. Any other interval (`90m`, `7h`, `45s`) is refused
  as `schedule_not_cron`, naming the unit, the part, the interval and the schedule. A rule the
  binding gives wins, whatever the `every`. The packing document keeps only the binding's rules.
  `deploy check` derives the same rule and refuses a unit that carries another, or none where one
  is due, as `schedule_not_cron`.

## K19 A runtime image per unit

An architecture may hold several runtime parts that are different programs, so each core and
service unit may run an image of its own. Settled with the consumer (R83), names exact; the bundle
stays `/1`.

- **Binding.** `parts.<p>.image` or, on a unit listed under `packing.units`, its `image`, each
  `{image, tag?, digest}` under the `target.images` rules of `runtimes.core` (registry: repository
  digest, no tag; local: tag, config id or both, `image_unpinned` with neither). Only a part of a
  core or service unit, or such a listed unit, may name one; anywhere else it is a binding of the
  wrong form.
- **Which image.** The unit's own wins over its parts'. Parts of one unit that name different
  images, with none from the unit, are refused as `unit_image_conflict` (the finding names the
  parts, never the images). Else the parts' image; else, for the core unit only, `runtimes.core`.
  `runtimes.core` is never a default for a service unit.
- **None.** A core or service unit that holds a runtime part (block `runtime`) and has no image is
  refused as `unit_image_missing`, naming the unit and its runtime parts. One without a runtime
  part keeps the role `core-runtime`, which the target binds, as before.
- **Bundle.** The unit's `code.runtime` names its image by the reference of `runtimes.core`
  (`<image>@<digest>`; local `<image>:<tag>`, else the config id). `runtimes` carries every image
  used, keyed as `runtimes.core` is: the binding's runtimes by their names, then each unit's own by
  the unit's name, unless an entry of the same key is there already.
- **Check.** `deploy check` reads the binding the bundle carries and applies the same rules, and
  compares each core and service unit's `code.runtime` with the image that binding resolves for
  it and with the references `runtimes` carry (`unit_runtime_mismatch`, naming the unit, the image
  found and the image expected). `deploy bundle` refuses the same on the bundle it built: an image
  that shares a key with another under a different name is not carried.

## K20 Configuration of a unit

A unit needs non-secret values its program reads, such as the Host names a server allows (R87). A
binding could give none: a part and a runtime took no key for it. The bundle stays `/1`; a bundle
without configuration keeps its bytes.

- **Binding.** `parts.<p>.env: {NAME: value}` and `parts.<p>.args: [string]`, and the same `env`
  and `args` on a unit `packing.units` lists. `NAME` is `[A-Z_][A-Z0-9_]*`; a value is text, a
  number or a boolean taken as its text, or `{secret: <name>}`.
- **Secrets.** `{secret: <name>}` is a reference through the secrets of the unit
  (`deploy/grants.rs`): the name is a secret name (`secret_value`) that a part of the unit
  declares (`secret_undeclared`). The bundle carries the reference, never a value.
- **Bundle.** Per unit `config: {env, args}`, merged in one order: the unit's entry, then its
  parts by name. `env` keys sorted; `args` the unit's, then those of the one part that gives any.
  Each member is left out when empty, and `config` when both are.
- **Refusals.** `unit_config_conflict`: two sources of one unit give one variable two different
  values, or two parts of one unit both give `args`. `unit_config_invalid`: a name that is no
  variable name, a value of another kind (a list, a mapping other than `{secret}`, null), an `env`
  that is no mapping, `args` that are not a list of strings, `env` or `args` on a part in no unit.
  A finding names the unit, the source and the variable, never a value.
- **Check.** `deploy check` reads each unit's `config` back: one of the wrong form, or one that is
  not what the carried binding resolves for its unit (none for a unit the packing does not have),
  is `unit_config_invalid`; a `{secret}` in it is checked like the unit's other secrets
  (`deploy/config.rs`).

## K21 Scratch directories of a unit

A service's program may write run files under a fixed path while its configuration and secret
files are mounted read-only (R118). A volume does not fit: a workspace is files at a base revision
with a claim the target keeps, and `durable_state` keeps what it holds across restarts. What the
program needs lives as long as one instance and holds nothing after a restart, the target's
empty directory per pod. The bundle stays `/1`; a unit without one keeps its bytes.

- **Binding.** `scratch: [{mount, size?}]` on a unit `packing.units` lists and on a part under
  `parts:`, the two places of K19 and K20, so it works under a strategy as under listed units.
  `mount`, not `path`, as a volume's mount is named (`parts.<p>.volumes[].mount`); `size` is text
  like a volume's `size` (`64Mi`), so a translator reads both in one form. A list of mappings, so a
  key can be added (a medium) without a new form.
- **Bundle.** Per unit `scratch: [{mount, size}]`, every source of the unit (its entry, then its
  parts by name) sorted by `mount`; not under `claims.volumes`, whose entries a translator turns
  into claims with an access. Absent when empty.
- **Refusals.** `unit_scratch_invalid`: the wrong form; `mount` must be `/` and segments, none
  empty, `.` or `..`, no `/` at the end, so mounts compare as written. `unit_mount_collision`: a
  scratch directory at the mount of a volume a part of the unit mounts or of another scratch
  directory, or one holding the other or inside it, by segment. Nesting is refused as well as
  equality: a target mounts the inner directory over the outer one, so part of a scratch directory
  would keep its files (a volume inside it) or part of a volume would lose them (a scratch
  directory inside it), and the target's order of mounts decides which. Two volumes of one unit at
  one path are not refused by this code: a volume's mount is checked by `volume_mount_missing`
  only. A finding names the unit, the source and the volume, never a path.
- **Check.** `deploy check` applies the binding's rules to the carried binding, and reads each
  unit's `scratch` back: its form, its collisions with the volume mounts the unit carries, and
  whether it is what the carried binding resolves (`deploy/scratch.rs`).

## K22 A secret as a read-only file of a unit

A program may read a secret from a file at a fixed path rather than from an environment variable
(R120): a key file, a certificate. K20 gave a secret only as `env: {NAME: {secret: <name>}}`. The
bundle stays `/1`; a unit without secret files keeps its bytes.

- **Binding.** `secret_files: [{secret: <name>, key?: <key>, path: <absolute path>}]`, on a part
  and on a unit `packing.units` lists, the two places of `env` and `args`. `secret` is the
  secret's name, as in `{secret: <name>}`; `key` the one key of the secret the file holds
  (`[-._a-zA-Z0-9]+`), all of it when absent; `path` in the normal form of a scratch directory's
  `mount` (K21).
- **Secrets.** The name goes through the same check as an `env` reference
  (`deploy/grants.rs`): a secret name (`secret_value`) that a part of the unit declares
  (`secret_undeclared`). The bundle carries name, key and path, never a value.
- **Bundle.** In the unit's `config`, beside `env` and `args`, so a translator reads every input a
  unit's program takes from the binding in one place: `secret_files: [{path, secret, key?}]`,
  every source's, sorted by `path`; left out when empty.
- **Refusals.** `unit_config_invalid`: an entry of the wrong form, a relative path among them, or
  `secret_files` on a part in no unit. Two entries of one path, and a secret file at, around or
  inside a volume mount or a scratch directory of the unit, are `unit_mount_collision`: a secret
  file is mounted like a directory, so it is one of the unit's own mounts and held by the rule of
  K21, not a code of its own.
- **Check.** `deploy check` reads `config.secret_files` back with the rest of `config` (its form,
  the binding's resolution), checks each `secret` like the unit's other secrets, and holds each
  path against the unit's carried volume mounts, scratch directories and other secret files.

## Risks

cloudspec has no format, blocks, translator or realization yet (its epics are draft, `translation`
is being drafted) and weft has no code (`weft:README.md:8-9`): no remote target runs before both
move, and the remote path waits on cloudspec. weft's own text still registers translators per
source kind in its `deployments` domain (`weft:AGENTS.md:62-67`). Without the source field kept
through translation and realization, an observation maps to no unit or part. Two KPI catalogues
drift where a KPI is in both. The accepted list (K16) goes stale between two copies; the
vendored fixtures catch it on the cloudspec side. agenticspec simulates agent behaviour and
cloudspec infrastructure load (Q15).

## Requests to cloudspec

Filed in beyond10x/cloudspec under `epic:translation` (pull request 2); each request names its
story there. Nothing is written in weft; weft gets what it needs from the cloudspec side. weft's
own epic `agent-workloads` runs a vendored fixture bundle from a hand-written formation until the
translator exists (`story:agent-architecture-run`), and frames `agenticspec sim render` output by
its `data-part` attribute per part (`story:agenticspec-render-contract`; the attribute exists). The agent workload kind is cloudspec `story:agent-workload-kind` (C2 depends on it); weft binds `agent-runtime` to Loom in `story:agent-loom-runtime`. weft accepts Q16 A and Q17 A.

- C1 (`story:architecture-format-constructs`) An architecture format with K13's constructs:
  component; package (an agent document with a runtime by digest, an ESS system, an image); shape
  (service, job, schedule with an overlap rule); replicas; volume per replica; link with protocol;
  endpoint and route; storage kinds; secrets by name; isolation level; checks.
- C2 (`story:translate-in-agenticspec`) The inbound translator of `agenticspec-deployment/1`, with
  K13's mapping, the coverage outcomes, role binding reported per role, and the grant rule.
- C3 (`story:source-field`) A source field on every object, kept into target objects and
  observations.
- C4 (`story:accepted-kinds-and-roles`) A published list of accepted unit kinds and roles with
  refusal names, and `scope` (D1 A) as the target boundary.
- C5 (`story:observation-contract`) An observation contract a target fulfils: per unit release
  state, instances, health, failures, drift; a resumable feed per instance of a workload's own
  records (Loom's run records). weft fulfils it.
- C6 (`story:external-dependencies-and-kpi-ids`) External dependencies with egress routes
  (cloudspec E4); KPI ids qualified by system, series labelled with the source.
- C7 (`story:agenticspec-fixture-bundles`) The vendored fixture bundles with their lock and the
  coverage test (K16).

Gaps cloudspec names, open on both sides: how a unit's `isolation` maps to weft's level names
(Q17). Disk and volume claims per unit (Q16) are in the bundle: `claims.volumes`, each with its
access, size and mount.

No request for a realization on this host: the in-process runtime is the local test, and weft's
`local` lowering runs a deployment on this host.

## Open points

| # | question | options | recommended |
|---|---|---|---|
| Q1 | the word for producing the bundle | A: bundle. B: package. C: seal | A |
| Q2 | the bundle's form | A: one YAML file, members inline, keyed by digest. B: a directory, a manifest naming content-addressed files. C: an archive | A: a translator keys a source by the digest of its bytes (`weft:docs/domain-sketch.md:357`); one file needs no archive rules |
| Q3 | the command | A: `agenticspec deploy bundle`. B: a top-level `agenticspec bundle` | A |
| Q4 | the code of the `core` unit | A: a core runtime image by digest in `runtimes`. B: the blocks as an ESS system run by Spindle, plus the adapter | A: what an ESS system translates to is open in weft (`weft:docs/domain-sketch.md:735-736`) |
| Q5 | who reads the target's reports | A: the `core` unit, its own deployment's feeds, with its workload identity. B: the deployer's agenticspec polls | A |
| Q6 | how agent runs come back | A: the target forwards Loom's records as a feed per instance (C5); agenticspec lifts them. B: each agent posts them to `core` over a link | A: Loom stays unaware of the blocks |
| Q7 | `job` units started at run time | A: `core` starts jobs through the target, under the role "start jobs in the scope". B: jobs declared ahead and woken by a trigger | A |
| Q8 | how an agent unit's runtime is chosen | A: by the role `agent-runtime`, bound by the target (weft: Loom); a binding may pin an image by digest. B: the bundle always pins the Loom image by digest | A: the agent stays an agent workload through cloudspec; where a digest is pinned the node verifies it before start (`weft:docs/domain-sketch.md:222-223`) |
| Q9 | where the packing lives | A: section of the binding. B: its own document | A |
| Q10 | default strategy | A: `single` everywhere. B: `by_part` on remote targets | A |
| Q11 | where agent units run | A: on the same target as Loom runtime processes. B: Loom elsewhere, the agent parts bound `external` | A |
| Q12 | who writes the agent document of an agent unit | A: the binding names it by digest, the bundle carries it. B: agenticspec generates it | A |
| Q13 | a target always a scope | A: yes, one deployment per scope. B: several deployments share one | A |
| Q14 | requirements | A: `requirements:` key of the architecture. B: in the experiment | A |
| Q15 | simulation coupling | A: one way, agent run to cloud load. B: co-simulation | A |
| Q16 | disk and volume per unit | A: `claims` gains `volumes`, each with size and access mode, from the `store` role. B: only `store` units claim disk | A: a `job` unit's workspace needs disk too |
| Q17 | isolation names | A: agenticspec keeps its own levels, and cloudspec's accepted list (C4) maps them to each target's names. B: use weft's level names | A: the bundle names no target |

## Proposed stories

In this repository, under a new epic `architecture-deployment` (the one
`epic:architecture-contract` defers to); requests to cloudspec are C1-C7 and the load hand-off of
Q15, not stories here:

1. **packing-format**: the `packing:` section, unit kinds, strategies, `agenticspec packing default`.
2. **packing-admission**: K5's refusals, the plan's units; extends `ingress-admission` (ingress 6).
3. **deployment-domain**: `agentic.deployment`, its states, refusals, views and scenarios (K10).
4. **deployment-bundle**: `agenticspec deploy bundle`, the format, its digests, per-unit fields
   and roles (K6, K7, K14).
5. **bundle-fixtures**: the fixture bundles, `bundles/sources.sha256`, the copy of cloudspec's
   accepted list and its check (K16); after 4.
6. **requirements-kpis**: the `requirements:` key and its stages; extends `story:architecture-kpis`.
7. **control-plane-units**: `Units` view, requirement status; extends `control-plane-pages`.
8. **first-runtime**: the in-process runtime of `design/architecture-runtime.md`: the local test.
9. **agent-run-lift**: Loom's run records lifted into block events (K15).

Every story that runs tests keeps each test within 60 s and its step within 120 s. A job in the
build pool ends within 10 minutes.
