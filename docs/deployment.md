# Deploying an architecture

An architecture document says what a system is, in block terms, and names no host, product or
target. To run it somewhere, three more things are needed: a **binding** says which concrete thing
plays each part on one target and how the parts are **packed** into units; **admission** checks the
architecture, its contract, the binding and the packing together; and the **deployment bundle** is
the one file a translator turns into its target's own architecture. agenticspec writes the bundle;
translating it and running it are the target's work.

```sh
agenticspec packing default architectures/workstation.yaml                  # the default packing
agenticspec deploy bundle architectures/workstation.yaml \
  --binding bundles/bindings/split.yaml --out workstation.bundle.yaml       # admit, then write
agenticspec deploy check workstation.bundle.yaml                            # admit a written bundle again
```

## The binding

A binding (`agenticspec-binding/1`) is a YAML file for one architecture and one target. Every key
but `format` and `binding` is optional.

| key | holds |
|---|---|
| `format` | `agenticspec-binding/1` |
| `binding` | the binding's name |
| `architecture` | the name of the architecture it is for; any architecture when absent |
| `refines` | a base binding file, relative to this one. The base's keys are read first and this file's keys go over them, mapping by mapping |
| `target` | what the target gives each unit: `isolation`, one of `none`, `process`, `container`, `user_kernel`, `micro_vm` (`container` when absent); `images`, where it takes runtime images from, `registry` (the default) or `local` (its own image store); `network_policy`, `enforced` or `not_enforced` (unknown when absent) |
| `parts` | per part: `agent` (the agent document that plays it, a path relative to the binding file), `digest` (that document's `sha256:`, checked), `state` (`memory`, the default, or `service`: where a part with shared state keeps it), `inside` (a sandbox that runs inside the unit), `secrets` (the secret names it needs, never values), `egress` (the destinations it reaches, below), `size` (of a workspace's volume, or of the durable state volume of a runtime part), `volumes` (where the part mounts a workspace's volume or a runtime part's durable state: `[{name: <workspace>, mount: <absolute path>}]`, or `name: <runtime part>-state`), `network: isolated` (the part may reach only what its unit's egress lists), `image` (the runtime image of its core or service unit,below), `env`, `args` and `secret_files` (configuration of its unit, see [configuration of a unit](#configuration-of-a-unit)), `scratch` (writable directories of its unit that hold nothing after a restart, see [scratch directories of a unit](#scratch-directories-of-a-unit)) |
| `transports` | per carried value (`model_call`, `order`, `question`, `claim`, ...), the transport a link that crosses units uses |
| `runtimes` | runtime images: `core` for the core unit, `agent-runtime` for the agent units (the role it plays; without it the target binds the role, and the older name `agent` is refused). Each has `image`, the image name without tag or digest (`registry.example/agenticspec-core`). From a registry it is pinned by `digest` (`sha256:` and 64 hex digits, the repository digest), and a runtime without either, or with a `tag`, is refused. From the target's local store (`target: {images: local}`) it is pinned by `tag`, by `digest` (the image's config id) or both. A core or service unit may run an image of its own, in the same form: `image` of the unit as `packing.units` lists it, else `parts.<part>.image` of its parts, which must all name the same one (`unit_image_conflict`), else, for the core unit only, `runtimes.core`. A service unit with none names the role `core-runtime`; a core or service unit that holds a runtime part (block `runtime`) and has none is refused (`unit_image_missing`) |
| `claims` | per unit name: `cpu_millis`, `memory_mib`, `accelerators` (500, 512 and 0 when absent; a model unit takes one accelerator). The operator UI's unit is `operator-ui`, the control-plane UI's `control-plane-ui` |
| `checks` | per unit name, the `port` of its `startup`, `ready` or `alive` check: `{core: {ready: {port: 8080}, alive: {port: 8080}}}`. A unit has only the checks of its kind (below); the UI units, `operator-ui` and `control-plane-ui`, have `ready` and `alive` |
| `ingress` | per ingress of the contract and per protocol it exports, where it listens: `{operator: {exports: {http: {port: 8080, bind: all}}}}`. `port` (1 to 65535), `bind` (`loopback`, the default, or `all`), `address` (the address it is reached at, text) and `part` (a part of a unit that serves it, when not the control plane of the core) |
| `ui` | the UIs to deploy as units of their own: `{operator: {port, bind, upstream: {unit, port, path, secret}}, control-plane: {port, bind, upstream: {unit, port, path, secret}}}`, every key optional (`ui: {operator: {}}`). See [the operator UI in a bundle](#the-operator-ui-in-a-bundle) and [the control-plane UI](#the-control-plane-ui) |
| `packing` | the packing section below |

A binding never changes the architecture: promoting a revision to another target is a new binding
and a new bundle of the same architecture.

## Secrets and egress

A unit is granted only what its parts declare, each by name. Under `parts:`, `secrets` lists the
secret names a part needs, and `egress` lists the destinations it reaches: a part of the
architecture (an external part, or a part in another unit), or a name the target resolves later
(a host, a registry). An egress entry is the destination's name, or
`{to: <name>, secret: <secret name>, address: <address>}`: `secret` when its link authenticates
with a secret, which a part of the same unit must declare; `address`, for a destination outside
the architecture, where the target finds it: a CIDR block (`10.0.0.0/8`) or `<host>:<port>`, or
`{namespace: <text>, labels: {<text>: <text>}}`. An external part is not given an address here:
its address is its `reach` (see [packing](#packing)).

```yaml
parts:
  gateway:
    secrets: [model-api-key]
    egress: [{to: model, secret: model-api-key}]
  operator:
    secrets: [chat-token]
    egress: [gateway, owner, worker, {to: chat-relay, secret: chat-token, address: "chat-relay.example:443"}]
```

Every link that leaves a unit needs its destination in the `egress` of the part it starts at, also
under the default packing: a link from a part of the core to an external part, such as a gateway
calling a hosted model, is egress. A binding that names no architecture declares no egress, so it
fits only an architecture whose links stay inside one unit; give each architecture a binding that
`refines:` it and declares the egress.

A secret is a name, never a value. A **secret name** is text of lower-case letters, digits and
hyphens that starts with a letter (`^[a-z][a-z0-9-]*$`), at most 63 characters, with no word
between hyphens longer than 20 characters (a key or a token is one long run, a name is words).
Admission refuses as `secret_value`:

- a secret in `secrets`, or an egress entry's `secret`, that is no secret name: a mapping, a
  number, capitals, `=`, `:`, `/`, a space, a long run of characters;
- a `secrets` that is not a list;
- a key `value`, `values`, `data`, `string_data`, `literal`, `password` or `token` at the top of
  the binding, in `target`, a part's entry, an egress entry, a runtime or a claim.

A finding says where the value is, never what it is.

## Configuration of a unit

A binding can give a unit non-secret configuration: environment variables (`env`) and arguments
(`args`), on a part under `parts:` or on a unit that `packing.units` lists. An environment
variable's name is `[A-Z_][A-Z0-9_]*`. Its value is text; a number or a boolean is taken as its
text (`8080`, `false`). A value written `{secret: <name>}` refers to a secret: the name follows the
rules of a secret name, a part of the same unit must declare it in `secrets`, and the bundle
carries the reference, never a value. `args` is a list of strings.

```yaml
parts:
  gateway:
    secrets: [model-api-key]
    env:
      ALLOWED_HOSTS: "gateway.agents.svc,localhost"
      MODEL_CREDENTIAL: {secret: model-api-key}
    args: [--log-format, json]
```

A secret can also reach the program as a read-only file: `secret_files`, a list of
`{secret: <name>, key: <key>, path: <absolute path>}`. `secret` is the secret's name, which a part
of the same unit must declare in `secrets`, as for an `env` reference. `key`, optional, is the one
key of the secret the file holds (`[-._a-zA-Z0-9]+`); without it the target writes the secret as
it keeps it. `path` is where the program reads the file: an absolute path in the same form as a
[scratch directory](#scratch-directories-of-a-unit)'s `mount`. The bundle carries the name and the
path, never a value.

```yaml
parts:
  gateway:
    secrets: [model-api-key, tls]
    secret_files:
      - {secret: model-api-key, path: /etc/gateway/key}
      - {secret: tls, key: tls.crt, path: /etc/gateway/tls.crt}
```

The bundle carries each unit's configuration as `config: {env, args, secret_files}`, merged from
the unit's entry under `packing.units` first and then from its parts in name order. `env` keys are
sorted. `args` are the unit's, then the args of the one part that gives any. `secret_files` are
every source's, sorted by `path`. An empty `env`, `args` or `secret_files` is left out, and a unit
with none of them carries no `config`. Admission refuses:

- as `unit_config_conflict`: two sources of one unit that give one variable two different values,
  and two parts of one unit that both give `args`;
- as `unit_config_invalid`: a name that is not an environment variable name, a value that is
  neither text, a number, a boolean nor `{secret: <name>}`, an `env` that is not a mapping, `args`
  that are not a list of strings, a `secret_files` that is not a list of `{secret, key, path}`, an
  entry without `secret` or `path`, a `path` that is relative or otherwise not such a path, a `key`
  that is not a key, another key, and `env`, `args` or `secret_files` on a part that is in no unit;
- as `secret_undeclared`, a `{secret: <name>}` or a secret file's `secret` that no part of the unit
  declares; as `secret_value`, one whose name is not a secret name;
- as `unit_mount_collision`, a secret file at the path of another secret file of the unit, or at,
  around or inside a volume mount or a scratch directory of the unit (the secret file's path is a
  mount of the unit, held like a [scratch directory](#scratch-directories-of-a-unit)'s).

A finding names the unit, the part and the variable or entry, never a value or a path.

## Scratch directories of a unit

A program may need a writable directory at a fixed path, for run files, sockets or a cache,
while everything else it reads is mounted read-only. Such a directory lives as long as one
instance of the unit: it starts empty and holds nothing after a restart. It is not a volume: the
unit claims nothing for it, and nothing in it is kept. Give it with `scratch`, a list of
`{mount, size}`, on a unit that `packing.units` lists or on a part under `parts:`:

```yaml
packing:
  units:
    - name: flow
      kind: service
      parts: [flow]
      scratch: [{mount: /run/flow, size: 64Mi}]
```

`mount` is an absolute path: `/` and one or more segments, none of them empty, `.` or `..`, and no
`/` at the end. `size`, optional, is the most the directory may hold, text in the form of a
volume's size (`64Mi`). The bundle carries the directories of each unit as `scratch`, from its
entry under `packing.units` and from its parts, sorted by `mount`; a unit without any carries no
`scratch`. Admission refuses:

- as `unit_mount_collision`: a scratch directory at the mount of another directory of its unit
  (a volume a part of the unit mounts under `volumes:`, another scratch directory, or the path of
  a [secret file](#configuration-of-a-unit)), or one that holds the other or lies inside it. A directory mounted inside another hides part of it, so part
  of a scratch directory would keep its files, or part of a volume would lose them. Paths compare
  by segment: `/var/lib` holds `/var/lib/flow`, and `/var/lib/flowx` is apart from both;
- as `unit_scratch_invalid`: a `scratch` that is not a list of `{mount, size}`, a relative or
  other `mount` that is not such a path, a missing `mount`, a `size` that is not text, another key,
  and `scratch` on a part that is in no unit.

A finding names the unit, the source (`scratch[0] of packing: unit flow`) and the volume, never a
path.

## Packing

A **unit** is one thing a target starts and watches. Each part of the architecture is in exactly
one unit, or is `external`: not started by the target at all.

| unit kind | lives | holds |
|---|---|---|
| `core` | long, exactly one | the event log, the inner behaviour of the blocks, the control plane and the parts no other unit takes |
| `agent` | long, replicas | agent parts, with the parts that are `per` one of them (workspace, sandbox) |
| `service` | long, addressable | a gateway, a pool, a tool, a runtime |
| `job` | until done, started on demand | an agent part marked `schedule: job` (or `shape: job`) that no clock wakes |
| `scheduled` | until done, started by a clock | an agent part marked `schedule: job` that a schedule or an `on_schedule` trigger wakes, or given a cron rule (`schedule: <cron>`) |
| `store` | long, keeps state | a shared workspace |
| `model` | long | a model part served by the target itself (marked `external: false`) |

Principals and model parts are `external` unless marked otherwise; any part may be marked
`external`. The target reaches an external part by its `reach`: `{name: <text>}`, a name it
resolves, or `{address: <text>}`; by the part's own name when the binding gives none.

The `packing:` section names a **strategy**, or lists the units:

| strategy | units |
|---|---|
| `single` (the default, also when the section is absent) | every part in `core`, external parts outside |
| `by_kind` | one unit per block that is not held by the core: all agents in one unit, each kind of service in one |
| `by_part` | one unit per part; a part's `count` becomes the unit's replicas |
| `groups` | the named lists of `groups:` are units; the rest is in `core` |

Per part, `parts: { <part>: { schedule: job } }` (or `shape: job`) puts an agent into a `job` or
`scheduled` unit of its own; `schedule: <cron>`, a cron rule of five fields (`"0 2 * * *"`), puts it
into a `scheduled` unit that carries the rule, which counts as its clock. A scheduled unit without
a rule of the binding carries the one that states its schedule's `every` exactly: N minutes
dividing 60 (`*/N * * * *`), N hours dividing 24 (`0 */N * * *`), one day (`0 0 * * *`) or one week
(`0 0 * * 0`); any other interval (`90m`, `7h`, `45s`) is refused as `schedule_not_cron`, and
`next_fire_at` stays in the unit's `clock`. `external: true` or `external: false` overrides where a part goes, and `reach` says where
an external part is reached. A part that is `per` another goes with it. A `count` marked for optimization is packed at its built value.

```yaml
packing:
  strategy: groups
  groups:
    edge: [gateway, builds]
  parts:
    sweeper: {schedule: job}
```

`agenticspec packing default <architecture> [--strategy <strategy>]` prints the section in its
**resolved** form, every unit listed, to paste into a binding and change:

```yaml
packing:
  units:
    - name: core
      kind: core
      parts: [box, builds, chat, gateway, operator, tree, worker]
      replicas: 1
  external: [model, owner]
```

In the resolved form, `units:` lists every unit (`name`, `kind`, `parts`, `replicas` for a unit
that runs as copies, `schedule` for a scheduled unit with a cron rule, `image` for a core or service unit that runs an image of its own, `env`, `args` and `secret_files` for its [configuration](#configuration-of-a-unit), `scratch` for its [scratch directories](#scratch-directories-of-a-unit)) and `external:` the parts outside, each a part's name or `{part: <name>, reach:
{...}}`; `strategy`, `groups` and `parts` do not go with it. A section of the wrong form (an unknown key, strategy, kind or part, two units of one
name) is refused with every problem as `binding_invalid`.

## Admission

`agenticspec deploy bundle` writes a bundle only for an architecture it admits: `agenticspec sim
check` accepts it, its contract derives and is valid, the binding is of the right form, and the
packing passes the rules below. A refusal exits 1 with `not_admitted`, names every finding in the
order of the table, and writes nothing.

| refusal | when |
|---|---|
| `part_unpacked` | a part is in no unit and not external |
| `part_packed_twice` | a part is in two units, or in a unit and external |
| `per_split` | a part that is `per` another is not in that part's unit |
| `shared_state_split` | a pool, a gateway with a rate limit or an exclusive workspace is reached from two units and its binding keeps its state in memory |
| `link_without_transport` | a link crosses two units and the binding names no transport for what it carries. A clock starting a `scheduled` unit needs none: the target's schedule starts it |
| `isolation_unmet` | a sandbox asks for more isolation than the target gives a unit, and the binding names no sandbox `inside:` it |
| `network_isolation_unmet` | a part is marked `network: isolated` and the target says `network_policy: not_enforced` |
| `scheduled_with_inputs` | a `scheduled` unit takes an input other than its clock, or neither a clock nor a cron rule of the binding wakes it |
| `schedule_not_cron` | a `scheduled` unit without a cron rule of the binding whose schedule has an `every` no cron rule states exactly (`90m`, `7h`, `45s`); the finding names the unit, the part, the interval and the schedule. Give the part a rule (`packing.parts.<part>.schedule: <cron>`) |
| `core_count` | the packing has not exactly one `core` unit |
| `unit_not_hosted` | a unit's kind or one of its roles is not in the accepted list |
| `agent_document_missing` | an agent, job or scheduled unit holds an agent part the binding names no readable agent document for |
| `agent_document_digest` | an agent document is not the digest the binding pins |
| `image_unpinned` | a runtime, a part's `image` or a unit's `image` from the target's local store names neither a `tag` nor a `digest` |
| `unit_image_conflict` | parts of one core or service unit name different images under `parts.<part>.image`, and the unit names none under `packing.units` |
| `unit_image_missing` | a core or service unit holds a runtime part (block `runtime`) and has no image: neither its own, its parts', nor, for the core unit, `runtimes.core`. The finding names the unit and its runtime parts |
| `unit_runtime_mismatch` | a core or service unit's `code.runtime` names an image the bundle's `runtimes` do not carry, or is not the image the binding resolves for the unit (none for a unit that names the role `core-runtime`). At bundle time this is two images under one digest with different names: the bundle carries one entry per digest, so give them one name. The finding names the unit, the image found and the image expected |
| `durable_state_unplaceable` | a runtime part with `durable_state: true` is in a `job`, `scheduled` or `model` unit, which runs until done or serves a model and holds no volume (place it in a core, service, agent or store unit), or a part of the document has the name of its state volume, `<part>-state` |
| `volume_mount_missing` | a unit claims a workspace's volume or a runtime part's durable state volume and no part of the unit mounts it, or mounts it at a path that is not absolute; there is no default path |
| `unit_config_conflict` | two sources of one unit (its entry under `packing.units`, its parts) give one environment variable two different values, or two parts of one unit both give `args` (see [configuration of a unit](#configuration-of-a-unit)) |
| `unit_config_invalid` | an `env`, `args` or `secret_files` of the wrong form: a name that is not `[A-Z_][A-Z0-9_]*`, a value that is neither text, a number, a boolean nor `{secret: <name>}`, `args` that are not a list of strings, a secret file that is not `{secret, key, path}` with an absolute `path` (a relative path among them), `env`, `args` or `secret_files` on a part in no unit; in a written bundle, also a unit's `config` of the wrong form or not the one its binding resolves |
| `unit_scratch_invalid` | a `scratch` of the wrong form: not a list of `{mount, size}`, a `mount` that is missing or not an absolute path in normal form (a relative path among them), a `size` that is not text, another key, `scratch` on a part in no unit; in a written bundle, also a unit's `scratch` of the wrong form or not the one its binding resolves (see [scratch directories of a unit](#scratch-directories-of-a-unit)) |
| `unit_mount_collision` | a scratch directory or a secret file of a unit is at the mount of another of the unit's mounts (a volume a part of it mounts, another scratch directory, another secret file), holds it or lies inside it; paths compare by segment |
| `secret_value` | something that looks like a secret's value stands where a secret name belongs (see [secrets and egress](#secrets-and-egress)) |
| `secret_undeclared` | an egress entry, an `env` reference or a secret file, or in a written bundle a unit, names a secret that no part of its unit declares |
| `egress_undeclared` | a link leaves its unit, for an external part or a part of another unit, and the part it starts at does not declare that destination in its `egress` |
| `address_in_architecture` | the architecture names an address: a key `address`, `host`, `hostname` or `url` anywhere in it (see [the bundle](#the-bundle)) |
| `ui_upstream_unknown` | the UI the binding asks for reads the adapter of a unit (`ui.<ui>.upstream.unit`) the packing does not have |
| `ui_upstream_path_invalid` | the path a UI reads its upstream's adapter at (`ui.<ui>.upstream.path`) does not start with `/` |
| `requirement_kpi_unknown` | a requirement names a KPI that is in neither the KPI catalogue nor the simulator and is not a cloud KPI (`cloudspec:<id>`) |
| `requirement_invalid` | a requirement of the wrong form: no `kpi`, not exactly one number of `at_least`, `at_most`, `equals`, a `part` the architecture does not have or on a KPI of the architecture as a whole, a `window` that is no duration, an unknown key |
| `ui_export_unknown` | a UI of your own serves an export (`ui.<name>.export`, `<ingress>.<protocol>`) that no ingress of the architecture has |
| `ui_archive_mismatch` | the archive a UI of your own names (`ui.<name>.source.archive`) is not the SHA-256 its `digest` gives |
| `ui_build_needs_toolchain` | a UI of your own names an `ess-ui/1` document (`ui.<name>.source.ess_ui`): its pages need node and vite to build, which the binary does not run; name the archive you built from it instead |

The **accepted list** (`bundles/cloudspec-accepts.yaml`, `agenticspec-cloudspec-accepts/1`) is
the copy of the unit kinds and roles the translator accepts, with the commit it was copied from
(`cloudspec_commit`, empty until the translator publishes its list). `--accepts <file>` reads
another.

## The bundle

`agenticspec deploy bundle <architecture> --binding <file> [--out <file>] [--ports <file>]
[--accepts <file>] [--kpis <file>]` writes one YAML file, `agenticspec-deployment/1`, with every
member inline, or prints it (`jq -r .bundle`) without `--out`. Its contract has the ingresses
resolved, the default `metrics` ingress against the KPI catalogue (`kpis/catalogue.yaml` unless
`--kpis` names another). The same inputs give the same bytes. A digest is `sha256:` and the
SHA-256 of the canonical JSON of what it covers (keys sorted, no space).

| member | holds |
|---|---|
| `format`, `generator` | `agenticspec-deployment/1`; the agenticspec version that wrote it |
| `architecture` | the document, its name, its revision digest (of its `parts`, `links` and `border`) and the digest of its bytes (`sha256`) |
| `nested` | every document the architecture nests, and every document those nest, keyed by its canonical digest, the pin its part writes (`agenticspec sim digest`): its `architecture` name, the `parts` that refer to it (dotted from the outermost document, `console`, `console.vault`), `sha256`, the digest of its file's bytes, `content_base64`, those exact bytes in standard base64, and `document`, the same document parsed. Empty when the architecture nests none. The units, the packing and the contract read the architecture flattened (`console.steward`) |
| `contract` | the contract (`agenticspec-contract/1`, see [the contract](contract.md)) and its digest |
| `binding` | the binding with its `refines:` chain resolved, its name and digest |
| `packing` | the resolved packing, its strategy and digest; its `external` lists each part outside as `{part, kind, reach}`, `kind` the part's block and `reach` the binding's or `{name: <part>}` |
| `units` | one entry per unit, below |
| `ingress` | one entry per export of each ingress of the contract: `ingress`, `protocol`, `audience`, `authenticated`, the `unit` and `part` that serve it (`control`, the control plane in the core unit, unless the binding names a part), `bind` (`loopback` unless the binding says `all`), and `port` and `address` when the binding gives them. An entry without `port` has no entry point on the target |
| `requirements` | each entry of the architecture's `requirements:` as written, with its `stage` (`check`, `evaluation`, `operation` or `cloud`), its `state` and, when decided, the `value` it was decided on: a KPI of the architecture as a whole or a gate count is `met` or `missed` here, every other one `undecided`. A missed requirement does not refuse the bundle. Empty when it has none. See [requirements](architectures/index.md#requirements) |
| `agents` | each agent document the binding names, keyed by the digest of its file's bytes: the parts it plays, `content_base64`, those exact bytes in standard base64, and `document`, the same document parsed |
| `runtimes` | each runtime image the binding pins under `runtimes`, then each image a core or service unit runs of its own that is not one of those. From a registry: keyed by its digest, with its name (the runtime's, or the unit's for its own image) and its image reference `<image>@sha256:<digest>`. From the target's local store: what the binding gave, its name, `image`, and `tag`, `digest` or both, keyed by the digest, else by `<image>:<tag>` |
| `files` | only when the binding asks for a UI: each file a unit names, by its file name, with its `digest`, its `size` in bytes and its bytes in `base64`. A UI unit's archive is carried here |
| `digest` | the bundle digest, of every member but this one |

Each unit:

| field | holds |
|---|---|
| `name`, `kind`, `parts` | the unit, its kind and its parts. The packing's `agent`, `job` and `scheduled` units are all of kind `agent` here, told apart by `shape` |
| `shape` | an agent unit only: `service` (long-running, the packing's `agent`), `job` (until done, started on demand) or `scheduled` (until done, started by a clock) |
| `replicas` | the copies that run; absent for a `job` or `scheduled` unit |
| `code` | what runs. An agent, job or scheduled unit stays an agent: the role `agent-runtime` and its agent documents by digest, never a process command, and the pinned agent runtime as `<image>@sha256:<digest>` when the binding pins one (`runtimes.agent-runtime`) (from a local store `<image>:<tag>`, else its config id). A core or service unit runs its image as `runtime`, by the same reference: its own, its parts', or for the core unit the pinned core runtime (`runtimes.core`); a unit without one names the role `core-runtime`. A UI unit serves static files: `{kind: static, archive, digest}`, the file name of its archive in `files` and its `sha256:` |
| `claims` | `cpu_millis`, `memory_mib`, `accelerators`, and `volumes`: one per workspace and one per runtime part with durable state ([below](#durable-state)), with its access ([below](#volume-access)), whether there is one per copy of its part, the programs outside the runtime that also write a durable state (`writers`, when its part names any), its size when the binding gives one, and its `mount`, the absolute path a part of the unit mounts it at |
| `isolation` | the highest isolation its sandboxes ask for, in agenticspec's own levels |
| `checks` | how the target sees it start and stay up: an agent, job or scheduled unit has a `startup` check (its agent runtime checks its agent document before it runs); the core and service units answer `/ready` and `/alive` over HTTP; a model unit has a `ready` check. Each check carries the `port` the binding gives it |
| `schedule` | a scheduled unit's cron rule: the one the binding gives (`packing.parts.<part>.schedule`), else the one that states its schedule's `every` exactly ([Packing](#packing)); absent when it has neither |
| `clock` | a scheduled unit's clock in the architecture: the trigger and schedule that wake it, `every`, `next_fire_at`, and the trigger's `policy` and `max_parallel` as the rule for overlapping runs |
| `upstream` | a UI unit only: the unit whose adapter the page reads (`unit`), its `port` when known, and `path`, the `api_base` of the UI's `config.json` (the binding's `ui.<ui>.upstream.path`, else `/api/`), and `secret`, the name of the credential it presents there (the binding's `ui.<ui>.upstream.secret`, when given; the unit lists it in `secrets` too) |
| `config` | the configuration the binding gives the unit, when it gives any: `env`, its environment variables with their keys sorted, each value text or `{secret: <name>}`, `args`, a list of strings, and `secret_files`, the secrets it reads as read-only files, `{path, secret, key}` each by the secret's name, sorted by `path`; merged from the unit's entry under `packing.units`, then its parts by name (see [configuration of a unit](#configuration-of-a-unit)) |
| `scratch` | the scratch directories the binding gives the unit, when it gives any: `{mount, size}` each, `size` when given, sorted by `mount`; writable, empty when an instance starts, and claimed as no volume (see [scratch directories of a unit](#scratch-directories-of-a-unit)) |
| `secrets` | the secret names its parts declare, never a value |
| `egress` | every destination its parts declare, each with the part it starts at (`from`), the secret it authenticates with, when declared (`secret`), and `name`, what the target resolves: a link to another unit, with its transport, named by that unit; a link to an external part (`external: true`), named by its reach (its name, or its address, which is then also `address`); or a destination outside the architecture (`named: true`, no link), named by its own name, with the `address` the binding gives it, else `unresolved: true`. Nothing that is not listed is granted |
| `roles` | what the unit needs from its target, never a provider: `agent-runtime`, `core-runtime` (a core or service unit without an image), `start-jobs` (the core, when a job unit exists), `model-server`, `store`, `model-route:<route>` for each hosted model route it calls, `operator-ui` for the operator UI's unit, `control-plane-ui` for the control-plane UI's unit |
| `ports` | the contract ports its parts use, by their derived names, with their settings and instances |
| `source` | the unit and its parts, carried into what the translator makes of it |

### Volume access

A volume's access follows from who mounts it, not from the workspace's `sharing` alone. A part
mounts a volume when the binding names it under that part's `volumes:`; the unit that holds the
workspace (or the runtime part with durable state) carries the claim.

| access | when |
|---|---|
| `read_only_many` | the workspace is `sharing: shared_read_only`, however many units or copies mount it |
| `read_write_once` | every part that mounts the volume is in the unit that claims it, and the claim is one per copy (`per_instance`) or the unit runs one copy (`replicas: 1`), whatever the sharing among those parts |
| `read_write_many` | a part of another unit mounts it too, or the claim is not one per copy and the unit runs more than one copy; a `job` or `scheduled` unit starts its runs on demand and counts as more than one |

A workspace with `sharing: shared_read_write` that only the parts of one single-copy unit write
is `read_write_once`, which a single-node storage provisioner can serve.

### Durable state

A workspace is files at a base revision. A service that keeps state of its own, records it
writes and reads back after a restart, marks its runtime part `durable_state: true` (default
`false`):

```yaml
parts:
  flow: { block: runtime, role: workflow, durable_state: true }
```

Its unit then claims one more volume, `flow-state`, one per copy when the part is `per` another,
its access by the same rule ([above](#volume-access)): `read_write_once` in a unit of one copy. The binding gives it a mount and, optionally, a size, as for a workspace:

```yaml
parts:
  flow: {size: 2Gi, volumes: [{name: flow-state, mount: /var/lib/flow}]}
```

Without an absolute mount it is refused `volume_mount_missing`. A `job` or `scheduled` unit runs
until done and a `model` unit serves a model, so a runtime part with durable state in one of them
is refused `durable_state_unplaceable`; place it in a core, service, agent or store unit. A runtime
part marked `external` is not started by the target, and the bundle claims nothing for it.

When programs outside the runtime write the same state too, such as a backup job or an indexer on
the same host, name them with `state_writers`, a non-empty list of texts, beside
`durable_state: true`:

```yaml
parts:
  flow: { block: runtime, role: workflow, durable_state: true, state_writers: [backup, indexer] }
```

The state is still the runtime's one volume, with the same access, and no workspace part, so the
contract asks for no `workspace.lease` or `workspace.proposal` port. The claim names the writers,
and the architecture page lists them with the part:

```yaml
volumes:
  - {name: flow-state, access: read_write_once, per_instance: false, writers: [backup, indexer], mount: /var/lib/flow}
```

`agenticspec sim check` refuses `state_writers` that is not a non-empty list of texts, or that
stands without `durable_state: true` (`setting_invalid`). Without `state_writers` the claim has no
`writers` key.

The bundle carries no secret value, host, provider or target name. It copies the architecture as
written, so an address in the architecture would land in the bundle: a key `address`, `host`,
`hostname` or `url` anywhere in the architecture document is refused as `address_in_architecture`,
by its path. Where a part is reached is the target's, written in the binding: a destination a part
reaches outside the architecture is a name in its `egress` that the target resolves, and the
address of what the system exports goes into the binding with the export, never into the
architecture.

## Checking a bundle

`agenticspec deploy check <bundle> [--accepts <file>]` admits a written bundle again from what it
carries, for a reader that did not write it. It refuses with `bundle_invalid` a bundle that is not
`agenticspec-deployment/1`, lacks a member, whose digests do not hold (the bundle's, the
architecture's revision, the contract's, the binding's, the packing's) or whose binding or packing
is of the wrong form (a check, a claim or an ingress export the packing or the contract does not
have among them). It resolves a nested architecture from the documents the bundle carries, each
found by the digest its part pins: it refuses a reference whose digest the bundle carries no
document of (`nested_document_missing`), and a carried document whose `content_base64` does not
decode to bytes whose SHA-256 is its `sha256`, which parse to its `document` and whose canonical
digest is its key (`nested_document_mismatch`). Otherwise it admits the architecture, flattened,
under the binding and packing the bundle carries, checks that each agent unit carries the agent documents of its agent parts at the digests
the binding pins and that each agent document's `content_base64` decodes to bytes whose SHA-256 is
its key and which parse to its `document` (`agent_document_mismatch`), and checks the secrets and egress of the binding and of every unit: a unit's
secret or an egress entry's secret that no part of the unit declares, an egress entry whose part
does not declare its destination, and a secret value, each by its name. It also refuses what the
target cannot be asked to serve: a volume without an absolute mount (in the binding or in a unit),
a unit's `config` of the wrong form or not the configuration its binding resolves
(`unit_config_invalid`; a `{secret: <name>}` or a secret file's `secret` in it is checked like any
secret of the unit), a unit's `scratch` of the wrong form or not the scratch directories its
binding resolves (`unit_scratch_invalid`), a scratch directory or secret file at, around or inside
a volume mount the unit carries or another of its scratch directories and secret files
(`unit_mount_collision`),
a local runtime image with neither tag nor digest, a core or service unit whose parts name different images or that holds a runtime part without an image (`unit_image_conflict`, `unit_image_missing`), a core or service unit whose `code.runtime` is not the image its binding resolves or is not carried in `runtimes` (`unit_runtime_mismatch`), a part isolated on the network of a target
that enforces no network policy, and a scheduled unit whose `schedule` is not the rule of its
binding or of its schedule's `every` (`schedule_not_cron`). Of a UI unit it hashes the archive the bundle carries and refuses
an archive that is missing or whose SHA-256 is not the unit's `digest` (`ui_archive_mismatch`),
an `upstream` that names no other unit of the bundle (`ui_upstream_unknown`), and an `upstream.path`
that does not start with `/` (`ui_upstream_path_invalid`), and an `upstream.secret` that is no name (`secret_value`), that
its unit's `secrets` does not list or that no part of its upstream unit declares (`secret_undeclared`). A refusal exits
1 with `not_admitted` and names every finding.

```sh
agenticspec deploy check workstation.bundle.yaml
```

## The operator UI

`agenticspec ui build <architecture> --contract <file> --out <dir> [--api-base <path>]
[--archive]` writes the operator UI of an architecture as static files, from the binary alone: no
node and no app of your own. The directory holds:

| file | holds |
|---|---|
| `index.html`, `app.js`, `app.css` | the operator app, built into the binary, the same for every architecture |
| `views.json` | the layout of the UI, the pages and panels `agenticspec ui derive` derives from the architecture and its contract |
| `contract.json` | the contract, as JSON |
| `ports.json` | the port catalogue the binary carries |
| `config.json` | the architecture, the title, and `api_base`: where the runtime's adapter answers the runtime interface (`/api/` unless `--api-base` names another path or URL); for the UI of an ingress (see "One UI per ingress"), `ingress` |

The app reads these files when it starts and calls the adapter at
`<api_base>/scopes/<architecture>/<part>`. Serve the directory as it is; `config.json` may be
edited where it is served. The contract must come from the same ports as the binary
(`agenticspec sim contract` of the same version); another is refused. The directory must be new,
empty or written by an earlier `ui build`, which is replaced. The same inputs give the same bytes.

`--archive` also writes `<dir>.tar`, the same files with sorted entries, mode 0644, owner 0 and
time 0, and prints its SHA-256 as `archive_sha256`, the digest a bundle carries for the archive.

```sh
agenticspec sim contract workstation.yaml --out workstation.contract.yaml
agenticspec ui build workstation.yaml --contract workstation.contract.yaml --out ui --archive
```

### One UI per ingress

An operator UI is the `ui` export of an ingress. When the contract gives a `ui` export to an
ingress other than `operator` (`architectures/workstation-ingress.yaml` gives one to `user` and
one to `operator`), each such ingress has a UI of its own, scoped to its interface: its pages show
only the panels of the interface's views (a part page with none left is dropped), its actions are
the interface's commands, and its channels carry only the interface's feeds. A command of the
interface that no link or principal port places becomes a row action on each panel of its domain
whose row key is one of its inputs. An architecture whose only `ui` export is the operator
ingress's keeps its one operator UI, byte for byte.

- `agenticspec ui derive --out <dir>/<name>.ui.yaml` writes one composition per such ingress,
  `<dir>/<name>.<ingress>.ui.yaml`, each with its fixtures, layout and digest, and lists them
  under `ingresses`.
- `agenticspec ui build ... --ingress <name>` writes the UI of that ingress (`operator` by
  default); `config.json` names the ingress and its title says it. An ingress without a `ui`
  export is refused with `document_refused`. Point `--api-base` at the ingress's own root,
  `/ingress/<name>`.
- `agenticspec ui serve` serves each such UI at `/ingress/<name>/`, its runtime interface below
  `/ingress/<name>/scopes/`, R5 listing only the commands of that ingress. The operator UI is
  at `/` as well, and the control-plane UI, scoped to the operator ingress, at `/control/`.

```sh
agenticspec sim contract workstation-ingress.yaml --out wi.contract.yaml
agenticspec ui build workstation-ingress.yaml --contract wi.contract.yaml --ingress user \
  --api-base /ingress/user --out user-ui
```

### The operator UI in a bundle

A binding with `ui: {operator: {...}}` makes `deploy bundle` add the operator UI as a unit of its
own. It builds the UI in-process with the library `ui build` uses, from the architecture and its
contract, and the archive is the same bytes as `ui build --archive` writes for them (with
`--api-base` set to `upstream.path`, `/api/` when absent). Without `ui:` the bundle is the same as before.

| key under `ui.operator` | holds |
|---|---|
| `port`, `bind` | where the UI's export listens when the binding's `ingress.operator.exports.ui` gives none. Given in both places, they must agree |
| `upstream.unit` | the unit whose adapter (the runtime interface, `ui/runtime.openapi.yaml`) the page reads; the unit that serves the operator ingress's `http` export (the core) when absent |
| `upstream.port` | its port; the `http` export's port when absent and that export is served by the same unit |
| `upstream.path` | the path the page reads that adapter at, starting with `/`: the `api_base` of the UI's `config.json` and the unit's `upstream.path` (`/api/` when absent). A UI of your own carries its archive as it is; only its unit's `upstream.path` says the path |
| `upstream.secret` | the name of the credential the UI presents to its upstream as the `bearer` credential of the runtime interface, never a value: a name (`^[a-z][a-z0-9-]*$`, words of at most 20 characters, else `secret_value`) that a part of the upstream unit declares in `parts.<part>.secrets` (else `secret_undeclared`, naming the UI and the unit). The UI unit carries it in `upstream.secret` and in `secrets`, so the client and its upstream share one declared secret. How a target mounts it into the static server is the target's. Absent, the unit is as before |

The bundle then has:

- a unit `operator-ui`, `kind: service`, `roles: [operator-ui]`, no parts, `code: {kind: static,
  archive: operator-ui.tar, digest: sha256:<hex>}`, `upstream: {unit, port, path, secret?}`, and
  `ready` and `alive` checks over HTTP at `/` on the export's port, or on the port of
  `checks.operator-ui.<check>.port`, and the claims of `claims.operator-ui` (500 millicores and
  512 MiB when absent);
- the archive under `files: {operator-ui.tar: {digest, size, base64}}`, inline like every other
  member;
- the `ingress` entry of the operator ingress's `ui` export naming `unit: operator-ui` and `part:
  control`; its other exports stay where they were.

`deploy bundle` refuses, with `binding_invalid`, a `ui:` of the wrong form, a packing that has a
unit named `operator-ui`, and a `part` on the `ui` export; with `not_admitted`, a contract whose
operator ingress exports no `ui` (`ui_export_unknown`), an upstream that names no unit of the
packing (`ui_upstream_unknown`), an upstream path that does not start with `/`
(`ui_upstream_path_invalid`), a role `operator-ui` the accepted list does not hold
(`unit_not_hosted`), and a UI that does not build (`ui_build_refused`, for example a contract
from other ports than the binary's). `bundles/bindings/operator-ui.yaml` is the worked example.

### The control-plane UI

`agenticspec ui build ... --control-plane [--prediction <result.json>]` writes the control-plane
UI: the same app and files, with pages across parts beside the overviews and part pages, and
`config.json` in `mode: control-plane`. `ui derive --control-plane` derives the same layout,
by the same code path. The pages are chosen by port class and by the split of entry commands into
configuration and administration, read from the contract and the port catalogue, never by a view
name:

| page | shows |
|---|---|
| Inbox | per principal part (any kind of principal: `human`, `organization`, `service` or `agent`), the views over the entity each of its principal ports opens, with the port's answers as row actions |
| Activity | the views over the entities the other ports the parts use open, across parts, newest first |
| Operations | per part, the views over the entities a configuration command of its blocks creates or an administration command moves, with the administration commands of the operator ingress as row actions; then the placements |
| Metrics | the KPIs of the `metrics` ingress, each with the simulator's prediction from `--prediction` (a result of the same revision; another is refused) |
| Architecture | the views of the architecture block, read-only |
| Events | the top level's channel as a live log |

The top overview gets tiles: the open rows of each Inbox view, the rows per state of each
Operations view, and each KPI. A page never offers a configuration command, a command that only
actors holding the report of a non-principal port may issue, or machine traffic (the commands of
a port that a `model_call`, `tool_call` or `claim` link stands for), whatever the operator ingress
lists. `ui serve` serves the control-plane UI at `/control/` beside the operator UI, and
`ui conform --architecture` checks its reads across parts at the architecture's scope.

A binding with `ui: {control-plane: {port, bind, upstream}}` adds the unit `control-plane-ui`:
`roles: [control-plane-ui]`, `code: {kind: static, archive: control-plane-ui.tar, digest}`, the
archive under `files`, the same keys, defaults and refusals as `ui.operator`. Both UIs can run
side by side: the operator UI then serves the operator ingress's `ui` export, and the
control-plane UI listens on `ui.control-plane.port` (its checks on that port) and reads the same
upstream. Asked alone, the control-plane UI serves the export. `bundles/bindings/operator-ui.yaml`
asks for both.

### A UI of your own

Any other key under `ui:` is a UI you built yourself, carried as a unit of the bundle beside the
operator and control-plane UIs. The key is the unit's name: lower-case letters, digits and `-`,
starting with a letter.

```yaml
ui:
  console:
    source: {archive: console-ui.tar, digest: "sha256:<hex>"}
    export: operator.ui
    port: 8091
```

| key under `ui.<name>` | holds |
|---|---|
| `source` | `{archive: <file>, digest: sha256:<hex>}`: a tar of the static files you built, read from the directory of the binding file that names it, and the SHA-256 of the file, which `deploy bundle` checks. Or `{ess_ui: <file>}`, your `ess-ui/1` document, which is refused (below) |
| `export` | `<ingress>.<protocol>`: the export of an ingress of the architecture the UI serves |
| `port`, `bind`, `upstream` | as for `ui.operator` |

The bundle then has a unit `<name>`, `kind: service`, `roles: [ui]`, `code: {kind: static,
archive: <name>.tar, digest}`, your archive byte for byte under `files: {<name>.tar: {digest,
size, base64}}`, and the `upstream`, checks and claims of a UI unit. The first UI that serves an
export serves it, `operator` and `control-plane` first and then the others by name; a UI beside
it listens on `ui.<name>.port`. The accepted list must hold the role `ui`.

`{ess_ui: <file>}` is refused with `ui_build_needs_toolchain`: the pages `agenticspec ui generate`
writes from an `ess-ui/1` document are Vue components that need node and vite to build, and the
binary does not run them. Build the app on a host with node (below) and name its archive as
`{archive, digest}`. Besides the refusals of `ui.operator`, `deploy bundle` refuses, with
`binding_invalid`, a `ui.<name>` without `source` or `export` or of the wrong form, and a name that
is a unit of the packing or of another UI; with `not_admitted`, an export no ingress has
(`ui_export_unknown`) and an archive that is not its digest (`ui_archive_mismatch`). The `console`
UI of `bundles/bindings/operator-ui.yaml` is the worked example.

#### Building a UI of your own from an ess-ui/1 document

`agenticspec ui generate --path <document> --out <dir> --project` writes a project that builds the
document's app on its own. On a host with node (20.19 or later, or 22.12 or later), the one
command is:

```sh
agenticspec ui generate --path console.ui.yaml --out console-ui --project
cd console-ui && npm ci && npm run build
```

The build writes `dist/` (`index.html`, `app.js`, `app.css`) and `dist.tar`, packed as `ui build
--archive` packs (sorted entries, mode 0644, owner 0, time 0), and prints the archive's sha256 as
`archive_sha256`. The same project gives the same archive. Name it in the binding as `source:
{archive: dist.tar, digest: "sha256:<hex>"}`, with the path relative to the binding file.

| in the project | holds |
|---|---|
| `src/generated/` | the app, the files `ui generate` writes without `--project` |
| `index.html`, `src/main.ts` | the host page; each host slot of the document shows a placeholder naming it until you pass a component of your own |
| `kit/` | the sources of the components the pages draw with, the kit the operator UI of `ui build` is built from |
| `package.json`, `package-lock.json` | the versions of vue, vite and its vue plugin the operator UI is built with, and the lock of exactly those |
| `vite.config.ts`, `pack.mjs` | the build, with the operator UI's output options, and the packer |

Generating again replaces the directory, `node_modules/` and `dist/` included.

### Generating the pages: `agenticspec ui generate`

`agenticspec ui generate --path <document>.ui.yaml --out <dir>` writes the Vue 3 app of an
`ess-ui/1` document into `<dir>`, replacing what was there: one page component per page, one
component per widget, the router, the typed client of the reads, commands and channels, the host
slots and the runtime, all drawn with the components of `ui/kit`. The same document gives the
same bytes. Build the app with vite, pack its output as a tar and name it as `{archive, digest}`.

It draws, besides pages, page kinds, shells, navigation, themes, channels and widgets:

- collections as a table, a list or cards (`style: cards` is drawn as the kit's list), their row
  actions, a sort (`sort: {by, dir}`, sorted in the browser), groups (`group_by`, `group_order`,
  `show_empty_groups`);
- records, forms (text, number, toggle, choice and host-slot inputs, a result), filter bars with
  choices and a search, metrics (`aggregate`, `field`, `match`, as a section or in the header);
- a read's row predicate (`reads.filter`) in the grammar of ess-ui/1: `==`, `!=`, `in [..]`,
  `contains`, `not`, `and`, `or` and parentheses over `row`, `state`, `params`, `shell` and widget
  `args`, so two panels of one view with different filters are two sections; a poll interval
  (`reads.refresh: 30s`);
- the header's title, `title_from`, filters, live channels, actions, metrics and help
  (`{text, link}`); a page's `actor`, which is checked against the model and not drawn;
- actions that send a command (shown only while the action set of their scope allows it), go
  to a page or copy a value, `as: button | link | icon`, `visible`, and a confirm in its short
  form `{title, confirm_label, show}`, asked in place.

A construct it does not draw is refused, the whole document with it: the command exits with
`document_refused` and prints each refusal as `<node path>: <message>`, the node path naming map
keys and list entries by their name (`pages/tickets/sections/open/reads/filter`). The messages
take one of these forms, `(...)` being the reader's own words:

| form | message |
|---|---|
| property | ``unknown property `<key>`: this generator does not draw it (...)`` |
| value | ``` `<value>` is not a value this generator draws here (...) ``` |
| component | ``` `<name>` is no component this generator draws (it draws collection, record, form, filter_bar, choice, metric) and no widget of the document ``` |
| misplaced | ``` `<key>` is no property of a <component> this generator draws ``` |

What released `ess-ui/1` (ESS 0.99.2) has and this generator does not draw yet:

| construct | not drawn | refused with |
|---|---|---|
| document | `types`, `tokens`, `placement_defaults` | property |
| shell | `preload`, `guards`, `overlays`, `source` | property |
| shell region | `props`, `visible`; `kind: overlay_outlet`, `assistant`, `account_menu` | property; value |
| navigation | `search`, `restrictions`; dynamic entries (`pages: {from_view, ...}`) | property; ``invalid type: map, expected a sequence`` |
| navigation entry | `roles_today` | property |
| page | `aliases`, `switch_to`, `visible`, `overlays`, `profile`, `source`, `unmapped` | property |
| page kind | `extends`, `overlays` | property |
| page layout | a grid row of more than one area; more than two columns | ``this generator stacks the areas of a one-column grid: each row names one area``; ``this generator lays out at most two columns, a wide and a narrow one`` |
| section, any node | `load`, `depends_on`, `visible`, `state`, `children`, `profile`, `degrades`, `unmapped` | property |
| section states | `refreshing`, `forbidden`, `stale` | property |
| header | `total`, `badge`, `switch`; a header metric that is no metric | property; ``a header metric is a metric here`` |
| collection | `selection`, `bulk_actions`, `expand`, `item`, `reorder`; `actions`; `style: tree`; `sort.allowed`; `sort.mode: server`; columns chosen by the user (`columns: {binds, all}`) | property; misplaced; value; property; value; ``invalid type: map, expected a sequence`` |
| record | `groups`, `tabs`, `item` | property |
| form | `loads`, `groups`, `tabs`, `parts`, `record`, `draft`, `save`, `variant_by`, `endpoint`, `submit.closes`, `submit.auto`; `actions` | property; misplaced |
| form input | `as:` `badge`, `list`, `timestamp`, `date`, `time`, `percent`, `json` | ``a form input `as: <as>` is not one this generator draws`` |
| choice | `multiple`, `creatable`, `visible`, `note`; `style` (`dropdown`, `tags`, `radio`, ...); options naming an enum type | property; value; ``` `<name>` names an enum type, which this generator does not resolve; list the options or read them from a view ``` |
| filter bar | `window`, `inputs`, `reset` | property |
| metric | `from`, `window`, `unit`, `scale`; `format: duration`, `percent`, `bytes`; a metric without `reads` | property; value; ``a metric reads a view here`` |
| composites | `confirm`, `chart`, `board`, `graph_editor`, `rich_text`, `references` as a node's `component` | component |
| primitives | `text`, `badge`, `icon`, `button`, `link`, `input`, `toggle`, `image`, `divider` (a node's `primitive`) | property |
| overlays | every overlay (`overlays` of a page, page kind or shell) | property |
| widget | `arrange: row`, `grid`; a use's `visible`, `body` | value; property |
| field | `unit`, `scale`, `visible`, `binds`, `label_from`, `suggests`, `note`, `help`; `as:` `secret`, `tags`, `date_range`, `duration`, `file`, `rich_text`, `map`, `cron`, `color`, `audio`, `link`, `bytes`; `as: choice` on a column or record field | property; value; ``a column or record field `as: choice` is not drawn here; it is a form input`` |
| host slot | `commands` | misplaced |
| reads | `pager`, `debounce`, `cache`, `endpoint`, `derived`; `paging: server`, `client`, `cursor`, `append`; `refresh` as an expression | property; value; ``` `<refresh>` is no poll interval this generator reads: a duration such as 30s, 2m or 1h (an expression is not drawn) ``` |
| action | `opens`, `export`, `upload`, `sets`, `choice`, `loads`, `checked`, `optimistic`, `bulk`, `endpoint`; `as: toggle`, `choice`, `menu_item`; a confirm naming an overlay; a confirm on an action that sends no command | property; value; ``` `<name>` names an overlay; this generator draws no overlay, only the short form {title, confirm_label, show} ```; ``a confirm stands on an action that sends a command here`` |
| channel | `sends`, `buffer`, `fields`, `transport_today`, `source`, `unmapped`; `direction: both` | property; value |
| live | `when_paged_away`, `max_rows`; `effect: patch_row`, `insert_top`, `remove_row`, `replace`; an `only_if` other than `matches(params)` | property; value; ``` expression `<e>` is not one this generator compiles here (matches(params)) ``` |
| state | `scope`, `sensitive`, `ttl`, `clear_on`, `fallback`, `endpoint`; `class: view_cache`, `channel_buffer`, `credential`; `store: server_session`, `server`; `store: session_storage` | property; value; value; ``this generator keeps state in memory, the URL or local storage`` |
| theme | any token a theme overrides (the look is the kit's) | property |
| expressions | outside a filter or a confirm's `show`: anything but `state.<name>`, `params.<name>`, `shell.<name>`, `row.<field>`, `args.<name>` in a widget, `actor.may(<command>)` or a literal | ``` expression `<e>` is not one this generator compiles (...) ``` |

A column's `sortable: true` is accepted and draws no sort control; the sort a collection declares
is the one it shows.

### Before a runtime: the UI on fixtures

`agenticspec ui serve <architecture> [--contract <file>] [--port <n>] [--sim <profile>]` serves the same operator
app on 127.0.0.1 (port 7430 by default) over the architecture's default control plane, answered
from the block fixtures built into the binary: rows from the panels' fixtures, the feed from the
channel scripts, each command recorded and answered with a scripted outcome, every action allowed.
It needs no checkout: a released binary serves from any directory, given the architecture's path.
`--fixtures <dir>` answers from your own fixtures instead. `--sim <profile>` answers a simulator run
of the architecture instead: the views hold the state its events left, the feed replays its event
log in order, and the Metrics page shows its KPIs. Open the printed URL to see the operator
UI with no runtime; the runtime interface is below `/ingress/operator`. The UI of every other
ingress with a `ui` export is at `/ingress/<ingress>/`, its interface below that root; with `--sim`
it answers from the same run, kept to what that ingress exports. `ui/README.md` § "Served
on fixtures" lists its paths and the layout of a fixtures directory.

### Proving the runtime answers the interface

A runtime proves that its adapter answers what the operator UI calls (`ui/runtime.openapi.yaml`)
by running `agenticspec ui conform` against itself, with the architecture it admitted and a token
its authenticator accepts:

```sh
AGENTICSPEC_UI_TOKEN=<token> agenticspec ui conform --architecture workstation.yaml \
  --contract workstation.contract.yaml --source http://127.0.0.1:<port>/ingress/operator
```

`--source` is the root of the adapter, the `api_base` of the UI, with the scopes below
`<root>/scopes/`. The command derives the default control plane as `ui build` does and checks: each
view of each part's page answers rows of the view's declared shape (R1, and the R3 views at the
architecture), the level's channel resumes from a cursor without repeating an event (R2), the
action set answers at the architecture and at each part (R5), a command it allows is not refused
by the authority evaluator and a command it withholds is refused (R4), and a request without
credentials gets 401. Commands are posted with the input `{}`, which the model refuses, so nothing
changes. It exits non-zero on a failed check, prints every check on a line of its own with its
status (`passed`, `failed` or `skipped`), named by the request
(`failed R1 agentic.gateway.Gateways at workstation/gateway: …`), and names the failed ones again
in the refusal. `--json` prints the report instead: every check with its status and detail, also
when one failed. Without `--source` it checks `ui serve` of the architecture, started for the run.

Each write carries `Origin`, the origin of `--source`, as a page served from the adapter sends it,
so an adapter that accepts only same-origin writes carries out the command. `--origin <url>` sends
another origin. `--header 'Name: value'`, repeatable, adds a header to every write, such as a CSRF
token:

```sh
AGENTICSPEC_UI_TOKEN=<token> agenticspec ui conform --architecture workstation.yaml \
  --source http://127.0.0.1:<port>/ingress/operator --header 'X-CSRF-Token: <token>'
```

An adapter that does not serve a view answers its read with 404 and
`{"error": "view_not_served", "view": "<view>"}`. The check of that view is skipped, by its name;
any other 404 fails. A row or change signal that names no entity can carry a top-level `scope`, the
scope id it belongs to (`<architecture>`, `<architecture>/<part>` or `<architecture>/<part>/<copy>`).
It then shows at that scope and at the scopes above it, whatever entity ids the row holds, and the
check fails when it comes back at another scope.

## Fixture bundles

`bundles/` holds a bundle of each catalog architecture under each test binding of
`bundles/bindings/` (`bundles/<architecture>.<binding>.yaml`), with the agent documents they name
and `bundles/sources.sha256`, one digest of the sources of each bundle. A binding that names no
architecture is bundled with every catalog architecture no binding of the same name names:
`local.yaml` is the base of each `local-<architecture>.yaml`, which declares the egress of that
architecture's parts. `task bundles:generate`
writes them, `task check:bundles` (in the gate) fails when their sources changed, and `task
slow:bundles` builds them afresh and compares bytes.
