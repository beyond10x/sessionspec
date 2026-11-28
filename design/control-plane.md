# The default control plane: what every architecture brings for its operator

Status: design for approval, nothing built. Decisions are numbered (C1-C8), each open point (P1-P9)
names the option recommended. It builds on `design/ingress.md` (I1-I12) and `design/view-layer.md`
(D1-D11) and repeats neither. Line numbers refer to this branch unless a pull request is named.

An architecture that is admitted and running has to be watched and steered: open questions answered,
approvals given, a gateway drained, runs and costs read. Today each consumer authors that surface:
an `ingress:` key per audience (`design/ingress.md:341-392`), a `view:` key (`design/view-layer.md`
D4), or an app of its own. A **control plane** is the set of ingresses, interfaces, panels,
administration commands and feeds an operator needs to run one architecture. This document makes
one the default: derived from the architecture, usable with nothing authored, replaceable piece by
piece.

## What exists

| piece | where | gap for a default |
|---|---|---|
| ingress, interface, export | `design/ingress.md` I1-I3, I7 | "An architecture without `ingress:` declares none; nothing is implicit" (`:221-222`); an audience must be a principal part of the document (I3), and most documents have no operator principal |
| configuration and administration split | `design/ingress.md` I4 (`:133-158`) | none; the default reads it |
| generated panels and composition per architecture | `design/view-layer.md` D2, D3, D8 | pages per level and per part only; no page across parts (all open questions, all gateways) |
| one runtime, one adapter, R1-R5 | `design/view-layer.md` D7; per ingress `design/ingress.md` I10 | none |
| event log, cursor, `CursorExpired` | `design/events-and-storage.md` E3 (`:75-92`) | none |
| a consumer's control plane as a nested architecture | pull request #21: `architectures/console.yaml` (a steward agent on the border, a helper, a gateway, a model), nested in `team-with-console.yaml` by `ref` and `digest` | nesting keeps only `parts`, `links` and `border` (`nest.rs:4-13`, `DIGESTED`), policies of the nested document do not apply (`docs/architectures/index.md:77` there), and a nested document's ingress is not served (`design/ingress.md` Q6) |

The console of that consumer is the agents that operate (parts). The surface they and the owner
use (ingress and view) is what nesting cannot carry. The default control plane is that surface.

## C1 The concept: a derived default, materialized on request (option c)

Three forms were weighed:

- **(a) implicit only.** Derived from every architecture, never written down. Usable at once, but a
  consumer cannot see what it got, review it, or start its own from it.
- **(b) a standard nested architecture from a catalog**, included by `ref` and `digest`. Refused:
  nesting flattens a document into parts and links and drops every other key (`nest.rs:10-13`), so
  ingresses and `view:` would not survive it; including it adds parts to every document, which
  changes its digest, its simulator result and its catalog entry; and a catalog document cannot
  know the parts of the document that includes it, while everything a control plane shows is a
  function of that document's contract.
- **(c) both: derived, and materialized as a document a consumer can copy and change.** Taken.

The default is two ingresses, written in the `ingress:` form of `design/ingress.md` I1-I2 with three
new rule values (C2). `agenticspec ingress default <architecture>` prints them; pasting the output
into the document gives the same contract byte for byte. The copy holds rules, not lists, so it
still picks up a new view, command or KPI of the blocks on regeneration. The pin is the generated
`ess-ui/1` composition with its `sources.sha256` (`design/view-layer.md` D1), not the copy.

```yaml
ingress:
  operator:
    audience: control.operator
    interface:
      scope: all
      commands: { principal: all, plays: administration }
      views: true
      feeds: true
    exports: [{ protocol: http }, { protocol: event_stream }, { protocol: ui }, { protocol: tool }]
  metrics:
    audience: control.monitor
    authenticated: false
    interface: { scope: all, metrics: computed }
    exports: [{ protocol: metrics_scrape }]
```

## C2 What the derivation adds to the ingress design

- **Defaults by name.** An architecture has the ingresses `operator` and `metrics` above unless it
  declares an ingress of that name (which replaces it) or sets it to `none`. `ingress: none` removes
  both. This replaces "nothing is implicit" of `design/ingress.md:221-222`; every other rule of I8
  stays.
- **Implied audiences.** `control.operator` (`kind: human`) and `control.monitor` (`kind: service`)
  are principal parts of the contract only, under the reserved path `control`. They are not in the
  document, so no digest (`nest.rs` `DIGESTED`, the catalog's `sources.sha256`) and no simulator
  result changes (`design/ingress.md` I11: arrivals per ingress only with `load.ingress`). The check
  refuses a part named `control`; none of the documents in `architectures/` has one. The runtime
  binding binds them to identities like any principal part (I3); a one-person setup binds
  `control.operator` to the owner's identity.
- **Three rule values of I2** (`design/ingress.md:97-109`): `principal: all`, the principal ports of
  every principal part in scope, each answered under the grant that lets the caller act for the
  asked principal (I5 layer 3); `plays: administration`, every ESS actor of the scope that I4 lets
  an audience play; `metrics: computed`, every KPI of a block in scope whose status is `computed` or
  `both`, so the default is admissible on any runtime (`design/ingress.md:248-250`).
- **The I9 check `principal_port_without_ingress`** is met by the default `operator`, since it
  carries every principal port. It bites again once the default is opted out or narrowed.

## C3 What the default shows

Pages are selected by port class and by the I4 split, never by view name (C7). The level and part
pages are D3's; the four pages across parts are new.

| page | content, scoped to the architecture and its nested levels | actions |
|---|---|---|
| Overview | D3 overview (parts table, reach table, one-level graph) plus tiles: open count of each Inbox view, rows per state badge of each Operations view, the default KPIs | none |
| Parts, per part | D3 part pages and instance lists, nested pages recursively | D6 inputs into the border, on the border part's page |
| Inbox | views over the entities of the contract's principal ports (questions, decisions) | `principal: all` as D6 row actions (answer, approve, reject) |
| Activity | views over the entities that outbound and clock ports open (sessions, turns, tool calls, inferences, gateway calls, claims, firings, runs), newest first, in collapsed frames | none |
| Operations | views over the entities configuration creates (I4: gateways, upstreams, endpoints, pools, runtimes, sandboxes), so their state badges are the health of what the deployer set up; `Placements` | `plays: administration` as row actions on the entity each command moves |
| Metrics | the `metrics: computed` KPIs as metric widgets, per level and part; when a simulator result of the same revision exists, its predicted value beside each (`design/architecture-runtime.md:131-134`) | none |
| Architecture | `agentic.architecture` views: revisions, contract, ports, ingresses (read-only) | none |
| Events | the level's channel (D5) as a live log | none |

Never exposed, by rule: configuration commands (I4, the document through a revision is their only
source); commands of an actor whose `may:` holds a port report (`design/ingress.md:153-158`);
machine traffic (`model_call`, `tool_call`, `claim`, I7). Hiding an action stays a convenience; the
grant decides (I5).

## C4 From the default to a consumer's own

| step | how | what stays generated |
|---|---|---|
| use as is | nothing; the binding binds `control.operator` | everything |
| re-skin | its values for the `--kit-*` tokens | everything |
| relabel, order, hide, group | the `view:` key (D4) | everything; `hide` is presentation only |
| narrow | declare `operator` with `except`, a part list as `scope`, fewer exports | rule resolution, projections, panels, pages |
| rename or split audiences | its own principal parts (`admin`, `oncall`) and an ingress each, as `design/ingress.md:351-392` | the same, per ingress |
| place panels in its own app | the views package or custom elements with `source` set to the `operator` ingress (`design/view-layer.md` § "Who provides what", I10 `{root}/ingress/operator/...`) | panels, typed client, R1-R5 |
| replace the app | its own `ess-ui/1` document through host slots; drop the `ui` export, keep `http` and `event_stream` | wire, projections, panels |
| keep a nested console | the operating agents stay a nested document (pull request #21); its parts appear as a nested section of the default (D3); its surface is the enclosing document's ingress (Q6 A) | everything of the surface |
| opt out | `operator: none`, `metrics: none` | nothing; I9 holds again for principal ports |

## C5 Running it

- **One runtime.** The default ingresses are exports of the one adapter that serves every ingress
  (D7, I10). No second runtime, no view that exists only for the control plane.
- **Admission floor.** `operator` over `http`, `event_stream` and `ui` is required, with an
  authenticator for `control.operator` (I6); a local runtime may bind a loopback authenticator, as
  `agenticspec serve` binds 127.0.0.1 and answers as one viewer (`ui/README.md`). The default's
  `tool` and `metrics_scrape` exports are served when the environment has a server and are listed
  as `unserved` in the plan otherwise, not refused. A headless test runtime says
  `control_plane: unserved` in its binding, so the absence is explicit; fixtures already answer
  principal ports there (`design/ingress.md:251-253`). Authored ingresses keep every I9 refusal.
- **Day one, no runtime.** `agenticspec ui serve <architecture>` derives the default and answers
  it on 127.0.0.1: R1 from the block fixtures of `story:block-panels`, R2 from channel scripts, R4
  by a fixture that records the command and returns a scripted outcome, R5 with every action allowed
  (`design/view-layer.md:179-182`, `design/architecture-runtime.md:136-141`). With `--sim
  <profile>` it replays a simulator run instead: the simulator writes its timeline as block events
  with subject keys and part paths (it names 8 block events today, `EVENTS` in
  `crates/agenticspec-sim/src/sim/swarm.rs` in pull request #21), so the Inbox, Activity
  and Metrics pages move as the run does.

Worked example, `team-with-console.yaml` with nothing added: Overview lists `console` (nested,
first), `owner`, `worker` (3), `gateway`, `model`; Inbox answers the steward's questions to `owner`
(`principal.answer`); Operations shows `gateway` and `console.gateway` with the three administration
commands of `GatewayOperator` (`blocks/2-intelligence/gateway.yaml:146-154`, `OpenGateway` and
`AttachUpstream` drop out by I4); Metrics shows 9 KPIs (principal 2, model 4, agent 2, harness 1
through the agent parts' domains).

## C6 Relation to agenticstudio

The studio edits a draft of an architecture (`studio/system.yaml`); the control plane runs an
admitted one. They share the shell and the panels, not the app: the views package gains the app
frame (navigation, scope switcher, theme tokens, R5 wiring, the `embed` shell) as one component,
and both apps mount it. The studio's draft view previews the draft's default control plane through
`agenticspec ui serve --sim`, with no copy of any page. One app for both would put draft edits and
commands against a running runtime under one sign-in and one action set.

## C7 Field neutrality

The derivation names no block, view, command or KPI. It reads port classes (principal, outbound,
clock), the I4 split, the contract's block-to-domain table (`story:contract-derivation`) and the
KPI catalogues by block and status. It names only `agentic.architecture` entities (D10), the
reserved path `control` and the enum of the principal kinds, `agentic.shared.PrincipalKind`
(`blocks/0-foundation/shared.yaml`), whose variants it reads from the compiled blocks, so every
kind of principal gets an Inbox. A field's or pack's blocks (`design/fields.md` § "Two
layers", § "Packs") get Inbox, Activity and Operations rows by the same rules, and a field
catalogue's `computed` KPIs join `metrics: computed`, under the one-model limit of D10 (O8).

## C8 Simulator and docs

Simulator results stay byte for byte: the implied audiences link to nothing and no profile names
`load.ingress` for them. Each architecture page in `docs/` shows the default's resolved operations
beside its contract (`story:contract-docs`); no control-plane UI is built per catalog entry. This
repository keeps the default control planes of `workstation` and `team-with-console` as golden
examples under `ui/views/examples/` (O9).

## Open points

| # | question | options | recommended |
|---|---|---|---|
| P1 | the form of the concept | A: derived default, printed as `ingress:` rules a consumer can paste (C1). B: implicit only. C: a catalog nested architecture | A |
| P2 | defaults beside authored ingresses | A: by name: `operator` and `metrics` exist unless declared or `none`. B: any `ingress:` key removes both. C: always added, never replaced | A |
| P3 | the default audience | A: implied parts `control.operator` and `control.monitor`, contract only. B: the document's human principal parts (mixes user and operator). C: none; an audience must be authored | A |
| P4 | admission of the default | A: floor `http`, `event_stream`, `ui`; `tool` and `metrics_scrape` listed `unserved` when absent; `control_plane: unserved` in a test binding. B: every default export required. C: nothing required | A |
| P5 | principal ports in the default | A: `principal: all`, the grant decides for whom the operator answers. B: read-only Inbox; answering needs an authored ingress | A |
| P6 | the default metrics | A: `computed` and `both` only; simulator predictions beside them on the Metrics page. B: every KPI in scope | A |
| P7 | the day-one source | A: `agenticspec ui serve` on block fixtures or a simulator run. B: block fixtures only | A |
| P8 | the studio and the control plane | A: one shell component, two apps. B: one app with two modes. C: separate shells | A |
| P9 | a nested console bringing its own surface | A: not served when nested (ingress Q6 A); the enclosing default covers its parts. B: re-export by name with a pin of its own | A |

## Proposed stories

New:

1. **control-plane-default**: the defaults by name, `none`, the reserved path `control` and its
   implied audiences, the rule values `principal: all`, `plays: administration`,
   `metrics: computed`, and `agenticspec ingress default`. Extends the proposed `ingress-format` and
   `interface-derivation` (`design/ingress.md` stories 1 and 4) instead of repeating them.
2. **control-plane-pages**: the Inbox, Activity, Operations, Metrics, Architecture and Events
   pages and the overview tiles, selected by port class and the I4 split; the golden examples of
   C8. Extends `story:architecture-ui` (S4) and the proposed `view-layer-per-ingress`.
3. **ui-serve-fixtures**: `agenticspec ui serve <architecture>` answering R1, R2, R4 and R5 from
   block fixtures on 127.0.0.1. Extends `story:runtime-ui-interface` (S6, "against fixtures first").
4. **sim-event-log**: the simulator writes its run as block events with subject keys and part
   paths, replayed by `ui serve --sim`; every `result.json` stays byte for byte.

Extensions of existing or proposed stories, no new story:

5. `ingress-admission` (`design/ingress.md` story 6): the floor, `unserved` exports, the binding's
   `control_plane: unserved`, a loopback authenticator for a local runtime.
6. `story:views-package` (S8): the shared shell component; the studio's draft view mounts it and
   previews the default control plane.
7. `story:contract-docs`: each architecture page lists the default's resolved operations.

Every story that runs tests keeps each test within 60 s and its step within 120 s. A job in the
build pool ends within 10 minutes.
