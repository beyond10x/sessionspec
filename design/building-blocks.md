# agenticspec: building blocks of agentic development (draft 1, 2026-11-27)

Status: design only. No block is specified yet. `design/design.md` describes what is built today:
the session IR, the store, the KPIs, the queries and the dashboard.

## What it is

- A specification, written in ESS, of what agentic development consists of.
- It declares building blocks: agent, model, gateway, harness, tool, channel, workspace, sandbox,
  runtime and the rest of the list below.
- Each block is a contract: the commands it accepts, the events it emits, the views it serves.
- An architecture is a document that composes blocks into one concrete setup.
- An architecture is checked against the specification before anything is built.
- An architecture is simulated, and the run is scored with KPIs.
- A search changes the architecture and keeps the variant with the better score.
- A runtime that implements a block proves it against the block's contract.

## Three levels, two sources of a run

```text
blocks ──────▶ architecture ──────▶ run ──────▶ KPIs
(spec, ESS)    (one document)       (events)    (numbers, gates)

                 simulated run ──┐
                                 ├──▶ one event vocabulary ──▶ the same KPIs
                 observed run  ──┘
```

- **Blocks**: types. What an agent is, what a sandbox is, what may call what.
- **Architecture**: an instance. Which agents, which channels, which sandbox, how many.
- **Run**: what happened over time, as commands, decisions and events.
- A **simulated run** comes from the specification, an architecture and a behaviour profile.
- An **observed run** comes from a real harness. This is what `agenticspec sync` lifts today.
- Both produce the same events, so one KPI definition scores both.
- The observed run checks the simulation: predicted KPI beside measured KPI, per architecture.

## What the existing tool becomes

- The session IR (`spec/domains/session.yaml`) is the run record of the blocks.
- `sync`, the store, `kpi`, `query`, `mine` and the dashboard stay. They evaluate a run.
- New: they evaluate a simulated run the same way as an observed one.
- New: the store's measured distributions feed the behaviour profile of a simulation.
- The nouns of the session IR are aligned with the block nouns, one change per noun, each with
  its `docs/` page.

## The blocks

One domain per block, one directory per layer. Dependencies point down only:
`scripts/check-layers.sh` reads `ess specify compile` output and refuses a dependency on a higher
layer, a cycle between domains of one layer, and a domain outside a layer directory. A dependency
is a type of another domain, the identifier of another domain's entity (an identifier in `shared`
counts for the domain whose entity it identifies), or a relation. Its negative fixtures must fail.

Each line: the block, what it is from first principles, and where the concept exists today.

### 0 foundation

- `shared`: identifiers of every entity; `Attribution` (known, inferred, not recorded); content
  as digest, length and class; usage; money; duration. From the session IR.

### 1 principals

- `principal`: who can want something and answer for it. A human, an organization, a service.
- `delegation`: one principal acts for another, with a chain that ends at a human.
  An operator agent acting for an owner is a delegation. From the session IR's `Delegate` and
  Commission's principal and authority context.

### 2 intelligence

- `model`: a model, its provider, an endpoint, a named route, a price. One inference is a request
  with usage (input, cache read, cache write, output, thinking). From llm's routes, prices and
  usage documents.
- `gateway`: the one place model calls pass. Routes, upstreams, admission, budgets, a record of
  every call. From llm-gateway.

### 3 capability

- `tool`: something an agent can call. Its provider (built in, a tool server, a shell, a
  connector, another agent), its contract, its category (read, search, edit, execute, network,
  spawn, message, plan, other), and whether its effect leaves the sandbox.
- `knowledge`: what an agent is given before it acts. Instructions, skills, memory, context
  files. Concepts an agent applies are declared here.
- `store`: data a part writes and another part reads, in a store of one of five kinds (record,
  artifact, key-value, semantic, graph) with a scope and a retention. Records hold digests, never
  bytes; an artifact is an immutable record addressed by its digest, with provenance; gets and
  queries are recorded. Design: `design/events-and-storage.md`.

### 4 agent

- `agent`: the definition. A role, instructions, a model per role, tool grants, budgets. A
  revision is immutable. From Loom's `AgentSpec` and Commission's `Agent` and `AgentRevision`.
- `harness`: the loop that turns a model into an agent. Context window, compaction, permission
  mode, hooks, turns, spawned sub-agents as a tree. Vendor harnesses and a harness driven from
  outside are kinds of this block. From the session IR and the layers of agent-stack.

### 5 work

- `work`: what an agent is asked to do. A goal, a task, an assignment that binds an agent
  revision to a task under a principal, a bounded run, a deliverable, evidence. From Commission.
- `process`: the procedure a task follows. Referenced by identifier only; Canon evaluates it.

### 6 governance

- `authority`: what may happen. A grant, a policy, the set of actions admissible now, a decision
  (allow, deny, approval required), who decided (a mode, a classifier, a hook, a delegate, a
  human), a budget. From Commission's frontier and admission, Loom's authority answer, and the
  session IR's decision and decider.

### 7 collaboration

- `channel`: how two parties exchange messages. Kinds: a prompt, an operator channel, a message
  between agents, agent-to-agent over a network, a chat adapter, a notification.
- `question`: a message that needs an answer. Lettered options, a default, a deadline, and what
  happens at the deadline. An approval is a question.
- The message sequences are protocols (`ess-protospec/1`): operator channel, agent message,
  question with a default. The a2a specification is the network form of agent message.

### 8 place

- `workspace`: the files an agent works on. A checkout, a scratch copy, a worktree. Sharing is
  exclusive, shared read-only or shared read-write. A lease names the holder. A change leaves a
  workspace as a proposal, then a merge. From Loom's scratch copy and proposal, weft's volume
  sharing, and the worktree tool's lease.
- `sandbox`: what an agent can reach. Isolation level, writable paths, egress rules, secret
  grants, an identity. Default deny: no rule, no reach. From weft's claims, secrets and identity,
  Substrate's confinement, and Loom's confinement profile.
- `runtime`: what runs a workload. A workload is a package, a shape and its claims; an agent is
  one package kind beside a service, a job and a model. An instance is one running copy in one
  place. The runtime's API starts, observes and stops instances. From weft's workloads and
  Loom's run.
- `resource`: what agents share and wait for. A pool with slots, a queue, a claim, a rate limit,
  a cache. A build pool and a provider's rate limit are both this block.

### 9 composition

- `architecture`: the composed setup. Parts (instances of blocks), links (who may call or
  message whom), a border (what a human reaches), revisions. No link, no reach. From weft's
  formation, component, link and border.
- `deployment`: an architecture revision installed on a runtime, and the runs started under it.
  An installation drains (takes no new runs, lets started ones end) and rolls back to the one it
  replaced. A run is queued, running, waiting, done, failed or cancelled, and keeps the revision
  and binding it was queued under. A run's identity is the caller's idempotency key.

### 10 evaluation

- `run`: the record of what happened. Today's session IR.
- `kpi`: a KPI as data. Name, unit, the events it reads, the block it belongs to, and its
  direction per profile (autonomous, pairing, review).
- `experiment`: an architecture, a load, a behaviour profile, faults, mutations, a search, and
  the result.

## A block is a contract

- Commands, events, views and errors in ESS (`ess/37`), as every domain is today.
- Surfaces over them, each its own ESS document:
  - the runtime API as `ess-http/1`;
  - the command line as `ess-cli/2`;
  - the channels as `ess-protospec/1`.
- Guards carry the rules. Examples:
  - a tool call with no grant is refused;
  - a message over a missing link is refused;
  - an egress to a host outside the sandbox's rules is refused;
  - a question past its deadline takes its default.
- Refusals are named errors, never a silent fallback.

## How another repository uses a block

What ESS offers today (`ess` 0.99.2 installed, checked 2026-11-27):

- No import between specifications. A specification's files stay inside its own directory.
- `ess-composition` pins a component of another compiled system by digest and checks that a
  local type has the shape of the imported one.
- `roles` in `ess-composition/4`: a local component plays a component of the other system.
  `implements` compares command inputs. Outputs, events and errors are not compared.
- `ess specify vendor` writes the copy and the entries that prove it. It is on ESS's main branch
  and in no release.

So the route is:

1. The consumer copies the block's types into its own domain (`ess specify vendor`).
2. Its composition document proves the copy matches, pinned to an agenticspec release digest.
3. Its component declares the role it plays: Loom plays `runtime` for agents, weft plays
   `runtime` and `sandbox` for any workload, llm-gateway plays `gateway`.
4. The block's conformance suite runs against the consumer's implementation.

No `ess-composition` document across two systems was found in the organization's repositories.
Loom mirrors Commission's types by hand and says so in comments.

### Example: the agent runtime's host ports

The agent runtime's SDK names 15 ports a host implements (its `docs/reference/sdk-stability.md`,
§ "Ports a host implements"). A port is a Rust trait called in process; a block is an ESS contract
on the wire. They meet at the command level: each port call is one command of a block, and a
conformance adapter drives the host through its ports.

- Contract ports, each the seam of one block:
  - `AuthorityProvider` → `authority` (decide a request; the port's Allow, ApprovalRequired,
    Deny, Undecided against the block's Allow, ApprovalRequired, Deny);
  - `SuspensionPort` → `work` (a run Running ⇄ Suspended) and `question` (the withheld approval);
  - `EffectPort` → `harness` tool call Allowed → Completed, under `sandbox`;
  - `ConnectorInvoker` → `tool` (provider kind Connector);
  - `CredentialResolver` → `sandbox` (secret grants by name, never a value);
  - `ObservationPort`, `EvidencePort` → `work` deliverable evidence and the `verification` block
    of `design/research-findings.md`;
  - `AgentExecutor` → `harness` (one session proposes for one frontier; usage → `model`);
  - `Governor` → `process` (an external evaluator) plus the frontier of admissible actions.
- Finer than the blocks: `ActionSelector` and `ArgumentGenerator` split one harness turn into
  selecting an action and generating its arguments. The `harness` block keeps one turn.
- Storage and time seams, not contracts: `CaseStore`, `FallibleCaseStore`, `WorkflowRunStore`,
  `EvaluationTime`. In ESS they are the storage and context ports of generated code.
- What the blocks lack that the ports have: the frontier (the set of actions admissible now, with
  those that need approval) and a trusted evaluation time. Both go into `authority` and `shared`.

## An architecture document

Format `agenticspec-architecture/1`. Example: one workstation, one owner, an operator agent,
several coding agents, a shared build pool (`architectures/workstation.yaml`, shortened).

```yaml
format: agenticspec-architecture/1
architecture: workstation
parts:
  owner:    { block: principal, kind: human }
  operator: { block: agent, harness: coding, acts_for: owner }
  worker:   { block: agent, harness: coding, count: { value: 12, vary: { min: 2, max: 24, step: 2 } } }
  gateway:  { block: gateway, rate_limit_per_minute: { value: 120, vary: [60, 120, 240] } }
  model:    { block: model, route: frontier }
  builds:   { block: pool, kind: build_slots, capacity: { value: 16, vary: [8, 16, 24, 32] }, job_limit: 10m }
  chat:     { block: channel, kind: operator }
  tree:     { block: workspace, sharing: exclusive, per: worker }
  box:      { block: sandbox, isolation: process, egress: [gateway], per: worker }
links:
  - { from: owner,    to: operator, carries: prompt, over: chat }
  - { from: operator, to: owner,    carries: question, over: chat }
  - { from: worker,   to: operator, carries: question }
  - { from: worker,   to: gateway,  carries: model_call }
  - { from: gateway,  to: model,    carries: model_call }
  - { from: worker,   to: builds,   carries: claim }
border: [operator]
policies:
  question_default_after: { value: 15m, vary: [5m, 15m, 60m] }
  operator_answers_alone: { value: false, vary: [false, true] }
```

- Every part names its `block`. Principals are parts of block `principal`.
- A duration is a string with a unit: `250ms`, `90s`, `15m`, `24h`, `1d`.
- `count` makes several instances of one part; `per` draws a part inside each instance of another.
- A reviewer is an agent part with `role: reviewer`. Workers reach it over a `review` link. Its
  strictness is `strictness: lenient | balanced | strict` (default `balanced`), a name in the
  profile, or its own `false_accept` and `false_reject`, which win over the name. A reviewer is an
  agent so that its reviews are model calls that the gateway, the price and the
  `agent_without_model_path` rule cover, like any other agent's. The templates
  `architectures/solo.yaml`, `parallel-unaware.yaml`, `handoff.yaml`, `author-reviewer.yaml`,
  `scope-partition.yaml`, `planner-executor.yaml`, `scheduled-loops.yaml`, `shared-board.yaml` and
  `orchestrator-workers.yaml` follow § "Topologies" of `design/research-findings.md`. Each one's
  summary says what the simulator does not model about it.
- Triggers, schedules and swarms. The names follow `agentic.trigger`, `agentic.channel` and
  `agentic.architecture`, written in lower case:
  - A `schedule` part has `every` (a duration) and `next_fire_at` (the first tick, default `0s`).
    The spec's `expression` and `timezone` are not read: the simulator knows a fixed interval only.
  - A `trigger` part has `kind: on_schedule` with `schedule: <schedule part>`, `kind: on_message`
    with `channel: <channel part>` (the channel its subscription is on), or `kind: on_event` with
    `event_name`.
  - A trigger's `policy` is `skip`, `queue` or `parallel` (default `skip`). `max_parallel` defaults
    to 1 and is above 1 only under `parallel`. `task_template` (any text) makes each scheduled
    firing a new task.
  - `takes: all` is the simulator's own setting, not the spec's: a run goes on with the next item of
    its source until the source is empty. The default is one item per run.
  - A link `carries: trigger` goes from a trigger part to the agent part it wakes. A schedule
    linked straight to an agent is a trigger with the defaults.
  - A channel part's `distribution` is `direct` (default), `topic`, `queue` or `board`. A principal
    with a `message` link to a topic, queue or board posts every arriving task there. A worker
    kind with a `message` link to one posts every task it finishes there.
  - A worker kind with an `order` link to another worker kind hands it every task it finishes.
  - An agent part's `coordination` is `orchestrator`, `board`, `mesh` or `none` (default).
    `start_stagger` (a duration) starts copy n at n times the stagger.
  - `partition: scope` is the simulator's own setting: each copy owns one scope.

Checking it:

- ESS has no instance document. It has commands, and an interpreter that runs them in memory.
- The architecture document is lowered to commands of the `architecture` domain, in a fixed
  order: declare the parts, declare the links, commit the revision.
- The interpreter runs them. The architecture is valid when every command is accepted.
- A refusal names the part and the rule: a link to a part that does not exist, a worker with a
  tool that needs egress its sandbox does not allow, a question with no party that may answer.
- weft checks a formation the same way: rows through commands, then `CommitRevision`.

Until the `architecture` domain exists, `agenticspec sim check <architecture>` applies the
structural rules directly. It exits 1 and prints one line per refusal. Each rule is also a gate
KPI, the count of its refusals:

| rule | refused when |
|---|---|
| `unknown_link_parts` | a link's `from`, `to` or `over` names no part |
| `unknown_blocks` | a part's `block` is not a variant of `agentic.architecture.PartKind` (the check reads the list from the block) |
| `unknown_carries` | a link's `carries` is not a variant of `agentic.architecture.Carries` |
| `per_without_part`, `acts_for_without_part`, `egress_without_part` | the field names no part |
| `agent_without_model_path` | an agent reaches no `model` part over `model_call` links (through a gateway or not) |
| `question_without_answerer` | a `question` link ends at a part that is not a principal or an agent; or, in an architecture that has a principal part, at an agent that acts for no principal and has no `question` link to a principal part |
| `review_without_reviewer` | a `review` link ends at a part that is not an agent with `role: reviewer` |
| `border_without_part` | `border` names no part |
| `trigger_source_not_trigger` | a `trigger` link starts at a part that is not a trigger or a schedule |
| `trigger_target_not_agent` | a `trigger` link ends at a part that is not an agent, a runtime or a process |
| `trigger_without_source` | the trigger's kind is missing or unknown; or an `on_schedule` trigger names no schedule part; or an `on_message` trigger names no topic, queue or board channel; or an `on_event` trigger names no `event_name`, or a `source` that is no part |
| `invalid_concurrency` | `policy` is not `skip`, `queue` or `parallel`; or `max_parallel` is not a whole number of at least 1; or it is above 1 under `skip` or `queue` |
| `invalid_schedule` | a schedule part has no positive `every` |
| `swarm_needs_agent` | `coordination` or `start_stagger` is on a part that is not an agent; or the coordination is unknown; or the stagger is not a duration |
| `unreachable_parts` | no link, `per`, `acts_for`, `egress` or `border` names the part, and it has no `per`; a trigger's `schedule`, `channel` and `source` count as names |
| `empty_architecture` | the document has no part; the refusal names the architecture |
| `self_reference` | a link goes from a part to itself, or a part's `per` or `acts_for` names the part itself |
| `not_a_channel` | a link's `over` names a part that is not a channel |
| `link_exists` | a link has the `from`, `to` and `carries` of an earlier link |
| `acts_for_not_principal` | an agent's `acts_for` names a part that is not a principal |
| `not_reached_by_principal` | in an architecture that has a principal part, a `border` part that no principal part links to and no `grant` link reaches |
| `gateway_without_model` | a gateway part has no `model_call` link to a model part |

The last seven rules are refusals of the architecture block
(`blocks/9-composition/architecture.yaml`) and carry the names of its errors.
`question_without_answerer` is its `QuestionWithoutAnswerer` and its `TargetCannotAnswer`, and its
`ActsForNeedsAgent` is `setting_unknown` (`blocks/settings.yaml` declares `acts_for` on agents
only).

`NotReachedByPrincipal` and `QuestionWithoutAnswerer` hold only in an architecture that has a
principal part. One with none is made to be nested (`architectures/console.yaml`): its border is
its interface, which the nesting architecture links to and through which its questions leave it.
A part counts as reached through a `grant` link, because an authority part holds a principal's
grant (the coding field's `auto-merger`). An agent that acts for no principal answers a question
when it forwards questions to a principal part, as the simulator lets it answer and forward one
hop. The block cannot test that the authority part a grant comes from is itself reached by a
principal (a test over rows holds one level of rows inside it), so the check counts any `grant`
link as well.

The trigger, schedule and swarm rules read the `value` of a marked place, so a document with
markers is checked as built.

A part of block `architecture` nests another document (`ref`, `digest`); the check resolves it
first (rules `nested_*`, not gate KPIs: a result is only computed for a document that resolves)
and then applies the rules above to the flattened document, where the nested parts are named
`<part>.<inner part>`. The digest is the one `agentic.architecture.Digest` describes: the
canonical form of the parts, links and border. The format: `docs/architectures/index.md`,
"Nesting an architecture".

## Simulation

What exists: weft's simulator (`tools/sim/src` in the weft repository, 6,080 lines of Rust).

- It runs commands on ESS's in-memory interpreter. It reads no store and is deterministic per
  seed.
- An experiment file has setup steps, traffic, faults, controllers, mutations and a search.
- It counts KPIs, checks invariants after every step, and writes `result.json` and one HTML page.
- Its search is `exhaustive` (every combination of the mutation values) or `fit` (hill climbing
  with restarts). Fitness is a weighted sum of KPIs. A variant with a non-zero gate KPI is out.

What agent work adds:

- The specification says which commands are legal. It does not say how many turns a task takes,
  how many tokens a turn costs, or how long a human takes to answer.
- A **behaviour profile** supplies these as distributions: turns per task, tool calls per turn by
  category, tokens per turn, tool duration, share of calls that need a decision, answer delay of
  a human.
- The profile is fitted from the store of observed runs.
- The simulator draws from the profile, sends the commands, and the interpreter accepts or
  refuses them. Time advances by the drawn durations.
- The output is the event stream of a run. The existing KPI code reads it.

Format `agenticspec-experiment/1` (`experiments/workstation-tuning.yaml`). The paths are relative
to the experiment file; the search space is the architecture's marked places, not a list in the
experiment.

```yaml
format: agenticspec-experiment/1
experiment: workstation-tuning
architecture: ../architectures/workstation.yaml
profile: ../fields/coding/profiles/coding.yaml
horizon: 24h
seeds: 8
seed: 1
faults:
  - { kind: principal_away, part: owner, from: 2h, for: 3h }
  - { kind: capacity_drop, part: builds, to: 4, from: 10h, for: 1h }
optimize: marked
search:
  mode: fit
  budget: 200
  restarts: 4
  weights: { tasks_correct: 10, false_completions_shipped: -5, owner_wait_share: -200,
             cost_usd: -0.05, queue_wait_p90_seconds: -0.01, backlog_wait_p90_seconds: -0.001,
             questions_defaulted: -1, operator_decided_owner_questions: -1 }
  gates: [unreachable_parts, agent_without_model_path, review_without_reviewer]
```

- The weights reward correct tasks, not raw `tasks_done`. With equal weights (+5 correct, -5 for a
  shipped false completion), a shipped task is worth less than nothing under the coding profile:
  about 45% of shipped tasks are correct. The search then cuts the workers to the minimum. The
  experiment counts a correct task as worth twice the damage of a shipped wrong one.
- `experiments/reviewer-tuning.yaml` searches the reviewer's strictness and the worker count of
  `architectures/author-reviewer.yaml` exhaustively.
- `shared-board-tuning`, `scheduled-loops-tuning`, `scope-partition-tuning` and
  `orchestrator-workers-tuning` search the marked places of their architectures. They use the same
  weights.

- `seeds` runs per variant; `seed` (default 0) derives their seeds and the search's restarts.
- `bursts: [{ at, tasks }]`: tasks that arrive at once, besides the Poisson stream, for example a
  backlog at `0s`.
- `search.limit` (default 256) bounds `exhaustive`; `budget` (default 24) counts the evaluations
  of `fit`, `restarts` (default 4) its random restarts. `gates` defaults to every gate KPI.
- A weight or a gate naming an unknown KPI is refused.
- `capacity_drop` lowers a pool's capacity or a gateway's rate to `to` in its window.

Format `agenticspec-profile/1` (`fields/coding/profiles/coding.yaml`):

- `roles.<harness>`: `task_turns`, `turn_seconds`, `tokens_per_turn` by class, `builds_per_task`,
  `build_seconds`, `questions_per_task`, `question_needs_owner` (a share).
- `roles.<harness>`, the failure dynamics. The defaults switch each one off:
  - `step_turns`: turns per step (default: the whole task is one step).
  - `step_pass_first` (default `step_pass_after_pass`), `step_pass_after_pass`,
    `step_pass_after_fail` (default 1 each).
  - `rework_turns`: rework turns after a failed step, and per round of a task that goes back
    (default 0, at least 1 per round).
  - `default_wrong`: share of defaulted questions that rest on a wrong assumption (default 0).
  - `rework_share`: share of the task's turns that a wrong assumption costs again (default 0).
  - `false_completion`: share of tasks that are not correct and are still reported done
    (default 1).
- `principals.<kind>`: `answer_seconds` and `away` windows (`from`, `for`, optional `every`).
- `agents.<part>`: `answer_seconds` of an agent part that answers questions.
- `review.turns`: model calls per review (default 1).
- `coordination`, what coordination costs. The defaults switch the race and the overlap off:
  - `board_read_seconds`: the delay before an idle copy reads a new board post (a distribution,
    default 0);
  - `claim_window_seconds`: how long a claim stays unseen by the other readers (default 0);
  - `mesh_read_calls`: model calls to read one mesh message (default 1);
  - `scope_overlap`: the chance that a task touches one given other scope (default 0).
- Roles `planner`, `executor` and `orchestrator` in `fields/coding/profiles/coding.yaml`. The planner takes one
  turn per task (2610.29870: under 2% of the executor's tokens). The executor has a higher pass rate
  after a failed step (0.212), so that its share of correct tasks rises by the ratio 65.0/50.2 of
  2610.29870. The orchestrator spends 3 calls per task, an estimate.
- `review.strictness.<name>`: `false_accept` and `false_reject`. Built in, and overridable:
  `lenient` 0.86 and 0.05, `balanced` 0.58 and 0.27, `strict` 0.30 and 0.495. Lenient false accept
  and strict false reject are the middles of the ranges of 2611.02207; the other values are
  estimates.
- `load.tasks_per_hour`: arrivals for the whole architecture. The older
  `load.tasks_per_hour_per_worker` still reads, as that rate times the worker instances. Giving both
  is refused.
- `prices.<route>` per token class, `per: 1M_tokens_usd`. `prices.agent_hour_usd` (default 0):
  the price of one agent instance alive for one hour, idle or not.
- `sources`: each value's paper (arXiv id), or `estimate`. The simulator does not read it.
- A distribution is a number, a duration, or `{ dist: lognormal, median, p90 }`,
  `{ dist: poisson, mean }`, `{ dist: exponential, mean }`, `{ dist: uniform, min, max }`.

### The simulator

`crates/agenticspec-sim`, a discrete-event simulation over simulated seconds. Nothing sleeps.

- Every agent part without `acts_for` and without `role: reviewer` is a worker kind, `count`
  instances. Tasks arrive for the whole architecture as one Poisson stream (`load.tasks_per_hour`)
  plus the experiment's `bursts`.
- Where an arrival goes:
  - to the shared channel a principal posts to, if any;
  - else to the copy that owns its scope, if a worker kind has `partition: scope`;
  - else to the one backlog.
- A free worker first takes a task that a reviewer sent its kind back. Otherwise it takes from
  its input:
  - the oldest arrival in the backlog, or its own scope's oldest task;
  - a task handed to it over an `order` link;
  - a waiting firing, if a trigger wakes it. A kind that a trigger wakes takes nothing else.
- Every agent instance costs `prices.agent_hour_usd` per hour from its start to the horizon, idle
  or not. So a worker that the load does not need costs money and adds nothing.
- A task is a drawn number of turns. Builds and questions are placed after drawn turns.
- Steps: the turns are grouped into steps of `step_turns`. The first step passes with
  `step_pass_first`, each later one with `step_pass_after_pass` or `step_pass_after_fail`,
  depending on the step before it. A failed step adds `rework_turns` after its last turn. The task
  is correct when its last step passed.
- Questions: a question that took its default rests on a wrong assumption with `default_wrong`.
  A wrong assumption adds `rework_share` of the task's turns at the end of the task. An answered
  question never does.
- Completion: at the end of its steps a worker reports the task done when it is correct. When it
  is not, the worker still reports it done with `false_completion`. Otherwise it knows the task
  failed and starts a round on it.
- A round is `rework_turns` turns (at least 1) and one more step. That step passes with
  `step_pass_after_pass` when the task is correct, else with `step_pass_after_fail`.
- Verification: a worker with a `review` link puts a task it reports done into its reviewer's
  queue and takes its next task. A free reviewer instance takes the oldest review. A review is
  `review.turns` model calls (at least 1) through the reviewer's gateway, priced at its model's
  route.
  - A correct task is rejected with `false_reject`; a wrong one is accepted with `false_accept`.
  - An accepted task ships. A rejected one goes back to the workers for a round.
  - Without a reviewer, every task reported done ships, false completions included.
- A turn is one model call. It goes through the worker's gateway, which admits at most
  `rate_limit_per_minute` calls in any 60 s window and queues the rest in order. The turn's
  tokens are priced at the route of the model the path ends at.
- A build claims a slot of the worker's pool, or queues in order. It is killed at `job_limit`.
- A question goes to the target of the worker's `question` link. An agent target takes its
  answer delay. A question that needs the owner (`question_needs_owner`) is then forwarded over
  that agent's `question` link to a principal, unless `operator_answers_alone`. The principal answers
  after its delay, counted from the end of any away window. A question unanswered after
  `question_default_after` takes its default.
- A worker waiting on a question, a slot or the rate limit does no work.
- Hand-on: when a task is done at a worker kind (after its review, if it has a reviewer), it
  ships. If the kind has a `message` link to a shared channel, it is posted there instead. If the
  kind has an `order` link to another worker kind, it is handed on. The next kind starts the task
  afresh with its own role: a planner's task is one turn, an executor's a whole task.
- Schedules and triggers:
  - A schedule ticks at `next_fire_at` and every `every` after it. Each tick fires the triggers
    on it. A post fires the triggers on its channel.
  - A trigger is at its limit when its started runs plus its firings that wait for a free copy
    reach `max_parallel`. At the limit, `skip` and `parallel` drop the firing (`firings_skipped`;
    the spec refuses a parallel one without a record), and `queue` holds it until a run of the
    trigger ends (`firings_queued`).
  - A firing within the limit waits for an idle, started copy of the kind it wakes; that wait
    and a queued firing's wait are its start wait.
  - A run takes one item: a new task from `task_template`, else the oldest backlog task (a
    schedule), or the oldest untaken post (a queue or board). A topic firing carries its own
    copy of the post. A run with nothing to take is empty: it makes one model call and ends.
  - A run ends when its copy reports its task done or hands it on. Under `takes: all` it goes on
    with the next item until the source is empty.
  - A run fails when it releases a task that is not correct, counted at the first such task.
  - When a run on a queue or board ends and untaken posts remain, the trigger fires again for the
    oldest one. A skipped post therefore waits, but is not lost.
- Channels:
  - On a queue, one trigger fires per post: the first one below its limit, else the first one.
  - On a board, every trigger on it fires. When a run starts on a board post, every other idle,
    started copy of the kinds those triggers wake reads the post with one model call
    (`coordination_calls`). Each reader reads after a delay drawn from `board_read_seconds`. The
    run's copy reads first, and its run starts after its read. A reader within
    `claim_window_seconds` of that read tries the same claim (`claim_conflicts`).
  - On a topic, every trigger fires with its own copy of the task. The first copy that ships
    counts; a later one is a duplicate run (`duplicate_runs`) and does not ship.
- Swarm coordination:
  - `orchestrator`: the turns of a kind that hands work to this swarm over an `order` link are
    its coordination calls.
  - `mesh`: when a copy starts a task, it tells every other started copy
    (`coordination_messages`). Each copy reads with `mesh_read_calls` model calls.
  - `board`: the race above, which the board channel causes.
  - `start_stagger`: copy n starts at n times the stagger. Before its start it takes no work,
    reads nothing and costs no agent time.
- Scope partition:
  - A task's scope is drawn uniformly among the `count` scopes. It touches one other scope with
    the chance 1 - (1 - `scope_overlap`)^(count - 1), so a finer partition crosses more often.
  - A task merges when its copy reports it done. The merge conflicts (`merge_conflicts`) when
    another merge touched one of its scopes after its work started.
  - Under `mesh`, the notice turns the conflict into a round of rework. Without it, the merge
    lands and the task is not correct.
- Model calls outside a task (board reads, mesh reads, empty runs) are priced and counted on
  their model and gateway. They wait on no rate limit and do not hold the reader.
- Draws come from streams of the run's seed: one for the arrivals, one per task (whichever worker
  or reviewer handles the task), one per task for its scopes, and one for coordination. Two
  variants of one seed therefore see the same tasks, and their difference comes from the
  architecture, not from other draws. The coordination stream is drawn in the order of events, so
  two variants that coordinate differently draw different coordination values.
- Events at or after the horizon are not run; time shares are clipped to the horizon.

KPIs of a run, averaged over the seeds, each with its kind:

- Quality:
  - tasks: `tasks_arrived`; `tasks_done` (shipped); `tasks_correct` (shipped and correct);
    `tasks_claimed_done` (reports of done, a task reported again after a rejection counts again);
  - waits: `backlog_wait_p50_seconds` and `backlog_wait_p90_seconds`, from arrival to a worker's
    start; a task still in the backlog at the horizon counts with its wait so far;
  - turns: `turns`; `rework_turns` (the part of them spent again after a failed step, a wrong
    assumption or a round); `steps`; `steps_failed`;
  - questions and reviews: `wrong_assumptions`, `reviews`, `reviews_rejected`, `review_calls`,
    `questions_asked`, `questions_forwarded`, `questions_defaulted`,
    `operator_decided_owner_questions`;
  - shares of worker instance time: `owner_wait_share`, `operator_wait_share`, `slot_wait_share`,
    `worker_busy_share`;
  - pools: `queue_wait_p50_seconds` and `queue_wait_p90_seconds` (every job, 0 for one that
    started at once), `jobs`, `jobs_killed`, `pool_utilization`, `rate_limited_seconds`;
  - money and tokens: `agent_time_cost_usd`; `cost_usd` (model calls of workers and reviewers plus
    agent time); `cost_per_task_usd`; `tokens_input`, `tokens_cache_read`, `tokens_cache_write`,
    `tokens_output`.
  - stages: `tasks_started`. A task that passes two worker kinds counts at each, and so do
    `tasks_claimed_done` and `turns`. Its backlog wait is counted at its first start only.
  - triggers: `firings`, `firings_started`, `firings_skipped`, `firings_queued`, `firings_empty`;
    `firing_skip_share` (skipped of fired); `firing_start_wait_p90_seconds` (from firing to the
    run's start; a firing still waiting at the horizon counts with its wait so far). The ids of
    `kpis/catalogue.yaml` are used where it has them.
  - channels and coordination: `posts`, `claim_conflicts`, `coordination_calls`,
    `coordination_messages`, `merge_conflicts`, `duplicate_runs`.
- Completeness: `task_completion`, tasks done of tasks arrived. Template tasks count as arrived;
  a topic's copies do not.
- Correctness: the gate KPIs of the check; `false_completions_shipped` (shipped tasks that are
  not correct); `firing_failure_share` (failed of started firings). `false_completions_shipped` is
  a gate candidate, not a default gate: under the coding profile no architecture without a perfect
  reviewer reaches 0.
- Per part:
  - workers: tasks started, claimed, done, correct and false; turns, rework turns; steps, and the
    four counts of the chain (`steps_after_pass`, `steps_passed_after_pass`, `steps_after_fail`,
    `steps_passed_after_fail`); cost; waits;
  - reviewers: reviews, rejected, calls, cost, busy share, queue wait;
  - every agent part: `agent_time_cost_usd`;
  - gateways (calls, limited calls and seconds), models (calls, cost, tokens), pools (jobs, killed,
    queue wait, utilization), principals and answering agents (questions received, answered,
    forwarded), channels (messages; `posts` on a shared channel);
  - triggers: the firing KPIs above, and `firings_failed`; schedules: `ticks`;
  - worker kinds that coordinate, are partitioned or orchestrate: `claim_conflicts`,
    `coordination_calls`, `coordination_messages`, `merge_conflicts`.

Search:

- `fit` evaluates the baseline, then every unseen neighbour of the current point (one marked
  place one step) as one batch, and moves to the best that improves. At a local optimum it
  restarts from a random unseen point. It stops at `budget` evaluations or when the restarts
  are spent.
- `exhaustive` evaluates the product of the ranges in one batch, refused above `limit`.
- Variants and seeds of a batch run concurrently (rayon). Results are gathered in order, so
  `result.json` is the same bytes for any `--threads`.

Commands and output:

- `agenticspec sim run <experiment> --out <dir> [--threads N]` writes `result.json`, format
  `agenticspec-sim-result/1`: `experiment`, `architecture` (as written, markers included),
  `profile`, `marked` (`path`, `value`, `range`), `baseline` and `best` (`params`, `kpis`,
  `per_part`, `fitness`, `feasible`; `best` is null when no variant is feasible), `variants`
  (`id`, `params`, `kpis`, `fitness`, `feasible`, in evaluation order), `trace` (`eval`, `variant`,
  `fitness`, `best_so_far`), and `timeline` (`t_seconds`, `part`, `kind`: at most 2,000 events of
  the best variant's first seed, evenly sampled; one turn is not an event, a rate-limited one is).
  A KPI is `{ value, kind }`. It also writes `index.html` from `agenticspec-render`.
- `agenticspec sim render <architecture> [--result result.json] --out page.html`.

### Optimization markers

- Any value in an architecture can carry `vary:`. Example: `capacity: { value: 16, vary: [8, 16, 32] }`
  or `count: { value: 12, vary: { min: 2, max: 24, step: 2 } }`.
- `value` is the architecture as built; `vary` is the search space at that place.
- `vary` is a list of values, or `{ min, max, step }` over numbers or durations. A `value` off the
  list joins it.
- A marked place is addressed by its path in the document: `parts.worker.count`,
  `policies.question_default_after`.
- `optimize: marked` searches only the marked places. `optimize: all` also searches every other
  numeric or duration value, over five steps from half to double (factor 2^(k/2)). Enum values
  get no default range yet.
- Variants are independent, so the search runs them concurrently: one thread per variant and seed.
- Search modes: `exhaustive` (every combination, refused above a limit) and `fit` (hill climbing
  over neighbour points with random restarts within a budget).
- A variant is feasible only when every gate KPI is 0. Fitness is the weighted sum of KPIs.
- Example files: `architectures/workstation.yaml`, `fields/coding/profiles/coding.yaml`,
  `experiments/workstation-tuning.yaml`.

### Rendering

- An architecture renders as a graph: parts as nodes grouped by layer, links as edges labelled by
  what they carry, the border marked, `per` parts drawn inside their owner, counts as badges.
- Marked places are drawn as controls with their range.
- An experiment renders on the same graph: the best variant's values at each marked place, KPIs per
  part, the search trace (fitness per evaluated variant), and the variants ranked.
- One self-contained HTML file per experiment, like `agenticspec analyze`, and a page of the docs
  site and the dashboard later.

What a simulation answers:

- waiting: on a human, on a pool, on a rate limit, on another agent;
- cost: tokens and money per task, by model and by delegation depth;
- reach: which part can reach which, and where a secret can travel;
- behaviour under a fault: a human away, a pool at zero, a channel that drops a message.

What it does not answer:

- whether the work an agent produced is correct. The simulator draws correctness from the
  profile's step chain, so it compares architectures under assumed failure rates. Only an observed
  run says what the rates are.
- what a topology does to the success rate. Solo, parallel and hand-off differ in 2611.00314 in
  how often the work succeeds; the simulator takes that rate from the profile, whatever the
  topology. A hand-off link is drawn, not simulated.
- what a rejection tells the worker. A round after a rejection passes with `step_pass_after_fail`,
  as if the worker learned nothing from the review.
- what a plan, an orchestrator or a board does to the success rate. A plan's gain is set in the
  `executor` role, not derived. An orchestrator and a board change cost and waiting only.
  2609.26781's gain from 1 to 128 agents on a board does not come out of a run.
- cron expressions, time zones, a schedule's restriction checks, `on_event` triggers (checked,
  never fired), how a partition is cut, conflicts inside one scope or without a partition,
  and a reader that is busy while it reads.
- Each architecture that is built is measured, and the measured KPIs are set beside the
  predicted ones. A profile whose predictions miss is refitted.

## Evaluation as a process

The loop that turns observed work into tasks, protocols, scores and new KPIs. Today only its first
step and a deterministic part of its third step exist.

1. **Lift**: a source (a harness transcript, a gateway's wire) becomes a session in the IR.
   Specified (`spec/domains/session.yaml`) and implemented (`agenticspec sync`).
2. **Extract tasks**: every concrete task in a session, with the turns that are its evidence, its
   goal, its outcome as observed (done, abandoned, corrected by the human), and who asked.
   Not specified, not implemented.
3. **Draft a protocol** for a task or a family of tasks: steps, evidence per step, outcomes, in
   Canon's `protocol/1`. Implemented deterministically for repeated tool-call runs
   (`agenticspec mine` → procedure → Canon protocol and agent-runtime bindings). An AI author that
   writes a protocol from a task's evidence: not implemented.
4. **Score**: a run of an agent or a whole architecture on a task, judged against its protocol:
   which steps hold, where the run fails, the KPIs of the run. Not implemented here; Canon
   evaluates a protocol over a case, and the `work`, `kpi` and `experiment` blocks hold the nouns.
5. **Explore outcomes**: the protocol of a task is a small specification, so ESS synthesis
   enumerates its outcomes; the simulator draws runs through them with the behaviour profile.
   Each outcome no observed run reached, and each one where agents fail most, is a candidate KPI.
   Not implemented.
6. **Grow the catalogue**: a candidate KPI that separates good runs from bad ones on observed
   sessions goes into `kpis/catalogue.yaml` with its source.

Specified next: a block `evaluation` (layer 10) with `Source`, `ExtractedTask` (evidence spans by
turn, goal, observed outcome), `DraftProtocol` (author principal, protocol digest, reviewed by),
`Case` (a task replayed under a protocol by an agent revision or an architecture revision),
`Score` (steps held, first failing step, KPI measurements) and `KpiCandidate`. The commands refuse
a protocol with no evidence span, a score of a case whose protocol is not reviewed, and a KPI
candidate with no separating measurement.

## KPIs per block

`docs/kpis.md` documents what is computed today: volume, cost and context size per model, cache
hits and busts, concurrency, autonomy (working, waiting on the human, prompts by author,
decisions by decider), and secrets and personal data by channel. Each moves under the block it
measures. A line marked `new` is not computed today.

- `principal`: seconds agents wait on the human; prompts by author; questions that took their
  default (new).
- `model`, `gateway`: cost and output tokens per model; cache hits and busts by cause; cost per
  task (new); time waiting on a rate limit (new).
- `harness`: context size (median, 90th percentile, largest); turns per task (new).
- `agent`: sub-agents started; figures for main agents and for sub-agents; tree depth (new).
- `tool`: error rate (new); duration (new).
- `authority`: decisions by decider and outcome; refusals per 1,000 tool calls (new).
- `channel`, `question`: messages per task (new); round trip of a question (new).
- `workspace`: time waiting on a lease (new); proposals merged of proposals made (new).
- `sandbox`: a secret in a prompt, in a tool result, in a tool argument that left the session;
  denied egress attempts (new).
- `resource`: queue wait (new); utilization (new); jobs killed at their limit (new).
- `architecture`: unreachable parts (gate, new); parts a secret can reach (new).
- `work`: tasks done per hour (new); time from task start to merge (new).

A KPI has a direction only under a profile. A gate KPI must be zero for a variant to count.

### Correct, complete, quality

Every block, and every domain of any system specified in ESS, can carry three kinds of criteria:

- **correct**: a check that passes or fails. A deliverable has evidence; a question past its
  deadline took its default; no secret left the sandbox.
- **complete**: a share of what was promised that exists. Tasks done of tasks assigned; outcomes
  of a command reached by the conformance suite; parts of an architecture with a link.
- **quality**: a number with a direction under a profile. Cost per task; time waiting on the
  human; queue wait.

The `kpi` domain stores each as a `Kpi` with its kind (`Correctness`, `Completeness`, `Quality`),
the block it belongs to and the events it reads. A correctness KPI is a gate by default.

## Repository layout (proposal)

- `spec/`: the tool's own system, as today (`agenticspec.session`, `.cli`, `.view`).
- `blocks/`: a second ESS system holding the blocks, one directory per layer.
- `blocks/protocols/`, `blocks/surfaces/`: the channel protocols and the runtime API.
- `architectures/`: architecture documents, each checked by the gate.
- `experiments/`: experiment files; fitted behaviour profiles are a field's, `fields/<field>/profiles/`.
- `crates/agenticspec-sim`: the simulator and the search, a library and a binary.

Two systems, because the tool's generated crates are synthesized from its system. weft's 23
domains take 451 s to synthesize as one system. The gate's budget is 120 s, so the blocks are
synthesized per domain, the suite is committed with a digest of its sources, and full
regeneration runs in the `slow` lane.

## What ESS lacks for this

Each is a story for the ESS planning store, in this order:

1. `ess specify vendor` in a release. Without it every consumer copies block types by hand.
2. A way to send a free command sequence to the interpreter from the CLI. Today the CLI reaches
   it only through a conformance suite. weft's simulator links the crate at a pinned revision.
3. `roles` compare command inputs only. A block's contract includes its events and errors.
4. No construct declares a KPI. An aggregate view (`group_by`, `aggregate`) gives count, sum,
   minimum, maximum and average over one entity. Not checked: whether a duration spent in a
   state can be written as one.
5. Synthesis in 0.99.2 ignores an input guard that an earlier branch of the same command
   checks, when a later branch tests that input against a related row
   (`when_related: {via: input.x, exists: false}`), and emits scenarios that the interpreter then
   rightly refuses. Reproduction: `design/ess-repro/synth-ignores-input-guard/` (3 of 7
   scenarios fail). `workspace.GrantLease` now reads the workspace through the lease instead.
6. `ess verify conform run --scenarios` reads only the `scenarios:` list of one `ess-inputs.yaml`,
   and synthesis of the blocks takes 30 to 60 s, so the gate keeps the suite in a cache keyed by
   the digest of its sources (`scripts/check-blocks-conform.sh`).

## Build order

Each step ends with something that runs. Step 1 is the first consumer gap: no architecture can
be written down and checked today.

1. Blocks of layers 0, 1, 4, 7 and 9 (`shared`, `principal`, `delegation`, `agent`, `harness`,
   `channel`, `question`, `architecture`) and `resource`. `ess specify validate` passes. The
   workstation architecture is accepted; three broken variants are refused with a named error.
2. The session IR emits block events. Observed runs of the workstation produce the question and
   queue KPIs. The existing KPIs do not change (checked against `research/`).
3. A behaviour profile fitted from the store. A simulated run of the workstation architecture.
   Predicted KPIs beside the observed ones for the same window.
4. Mutations and search. One report that ranks the variants.
5. `model`, `gateway`, `tool`, `authority`, `workspace`, `sandbox`, `runtime`, `work`. The
   composition documents in which Loom, weft and llm-gateway declare the blocks they play.

## Open, with the default taken

- Q1. One ESS system or two in this repository. Default: two (`spec/` and `blocks/`).
- Q2. Where the simulator lives. Taken: `crates/agenticspec-sim`, a discrete-event simulation of
  the architecture driven by the behaviour profile, with the search format of weft's simulator.
  The ESS interpreter checks the architecture's commands; weft's engine is not linked.
- Q3. How Loom and weft adopt a block. Default: they keep their domains and add a composition
  document that proves the match. Replacing their domains is not planned.
- Q4. Where a KPI is defined. Default: as data in the `kpi` domain, computed by the existing
  code, and as an aggregate view where ESS can express it.
- Q5. The system name of the blocks and their domain prefix. Default: system `agentic`,
  domains `agentic.<block>`.
