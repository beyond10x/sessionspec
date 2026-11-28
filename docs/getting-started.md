# Getting started

## Install

Each [release](https://github.com/beyond10x/agenticspec/releases) carries the binary for Linux
(x86_64 and aarch64, static or for glibc), macOS (Apple silicon and Intel) and Windows (x86_64),
one archive per platform, with `SHA256SUMS` to check it against.

With a Rust toolchain, install the binary from the repository instead:

```console
cargo install --locked --git https://github.com/beyond10x/agenticspec agenticspec
```

## Fill the store

```console
agenticspec sync
```

This reads every Claude Code session with a file changed in the last 72 hours and writes it to the
store. It prints how many sessions it read, lifted and skipped. The first run reads everything in
the window. Later runs only read what changed.

Choose another window or another transcript directory:

```console
agenticspec sync --since-hours 168
agenticspec sync --root /path/to/projects
```

You rarely need to run `sync` yourself. `analyze`, `kpi`, `query`, `mine`, `procedure` and `serve`
sync first.

## Open the dashboard

```console
agenticspec serve
```

Open http://127.0.0.1:7420/. The server listens on 127.0.0.1 only and syncs every 5 seconds, so
running sessions show up while they work. Use `--port` to pick another port.

The live UI is a web app that is built from a checkout of the repository with `task ui:build`,
which needs Node.js 22 and npm.
`serve` finds it in `target/ui/www` beside the binary, in the directory named by
`AGENTICSPEC_UI_DIR`, or in `ui/` inside the state directory. Without it, `serve` still answers the
data endpoints and says the UI is not built.

## Write a dashboard file

```console
agenticspec analyze --out sessions.html
```

This writes one self-contained HTML file: the KPIs of the window and each session. Without `--out`
it writes `agenticspec-dashboard.html` beside the store, so a run inside a checkout leaves nothing
there. The file contains your session titles, so keep it out of repositories you publish.

## Where your data lives

| what | where |
|---|---|
| transcripts read | `~/.claude/projects`, or `--root` |
| the store | `$XDG_DATA_HOME/agenticspec/sessions.db`, else `~/.local/share/agenticspec/sessions.db` |
| another store | `--state-dir <dir>` on any command keeps the store in `<dir>/sessions.db` |
| the dashboard file | `agenticspec-dashboard.html` beside the store, unless `analyze --out` names another |

A new version of agenticspec may change what the store holds. It then empties the store and fills
it again on the next sync.

A store from before the name agenticspec, in `~/.local/share/agentspec/` or
`~/.local/share/sessionspec/`, is moved to `~/.local/share/agenticspec/` the first time
agenticspec opens it.

## Common options

`sync`, `serve`, `analyze`, `kpi`, `query`, `mine` and `procedure` take these:

| option | effect |
|---|---|
| `--since-hours <n>` | the window: from `n` hours ago until now, 72 by default |
| `--root <dir>` | the Claude Code projects directory |
| `--state-dir <dir>` | the directory of the store |
| `--output json` | print the result as JSON |

## Sessions that act for you

Some setups let one agent session type into another on your behalf. Name those sessions so their
messages count as yours:

```console
export AGENTICSPEC_DELEGATES=operator,night-shift
```

A message from a named session counts as a delegate's prompt, not a peer session's. A message that
says it was sent on someone's order also counts as a delegate's.

The older names `AGENTSPEC_DELEGATES` and `SESSIONSPEC_DELEGATES` are still read when
`AGENTICSPEC_DELEGATES` is not set.

## Check an architecture

`agenticspec sim check <architecture.yaml>` checks an architecture document against the building
blocks. It exits 0 and prints the architecture with its counts of parts and links, the KPIs of the
architecture as a whole, the parts it cannot lose (`single_points`), the architectures it nests and
the count of its ingresses, or exits 1 and names each refusal on standard error.

The rules are the refusals of the architecture block that a document can write, with two left out
for now: a border part that no principal part links to, and a question to an agent that acts for
no principal (`design/building-blocks.md`, "An architecture document", lists the rules and why).

Every key of a part other than `block` is a setting, and the part's block declares which settings
it takes and of what type: the "Settings" section of each block page lists them with type, unit and
default. The check refuses a setting the block does not declare (`setting_unknown`) and a value
outside its type or range (`setting_invalid`, for example `capacity: many` on a pool), each naming
the part, the setting and what was expected. A setting whose values are a block's own enum takes
the word (`sharing: shared_read_only`) or the variant as the block writes it (`SharedReadOnly`);
both are one value, and what the simulator, the contract and the deployment bundle write carries
the word. A `model_share` names only model parts the part reaches over `model_call`, and gives at
least one of them a share above 0.

What no setting of the block types goes into the part's `notes`, a map with free keys and values
that every part may carry. The check takes any map there as written, and the simulator, the search
and the contract never read it. A channel's file, path or audience, or the model and use of an
agent, are notes, for example:

```yaml
parts:
  inbox:
    block: channel
    kind: screen
    notes: { path: /inbox, audience: owner }
  writer:
    block: agent
    notes: { model: small, use: drafts }
```

The typed settings include a channel of kind `screen` (a page of a user interface), a pool of kind
`queue` with its `order`, a model's `hosting` and `billing` (where its price comes from), a spend
cap on an authority or a runtime (`per_day_usd`, `per_month_usd`, and `route`, what happens at the
cap), and a question part's `default` and `default_after`.

With `--json` it prints one JSON document on standard output, also when it refuses the
architecture, for a program to read. The exit code stays the same. A file it cannot read prints
no document.

```console
$ agenticspec sim check --json broken.yaml
{"architecture":"broken","counts":{"acts_for_not_principal":0,…,"unknown_link_parts":1,"unreachable_parts":0},"format":"agenticspec-check/1","links":1,"parts":1,"valid":false,"violations":[{"gate":true,"link":0,"message":"link 0 names a part that does not exist","part":"nobody","rule":"unknown_link_parts","subject":"you->nobody:prompt"}]}
```

| field | what |
|---|---|
| `format` | `agenticspec-check/1` |
| `architecture`, `parts`, `links` | the architecture's name and its counts of parts and links |
| `valid` | `true` when no violation is of a gate rule |
| `violations` | every violation, sorted by rule, subject, part, link and message |
| `counts` | the number of violations of every rule of the check, 0 included |
| `kpis`, `single_points`, `nested`, `ingresses` | only when the architecture is accepted: what `sim check` prints without `--json` |

Each violation has:

| field | what |
|---|---|
| `rule` | the rule, for example `unknown_link_parts` |
| `gate` | `true` when the rule refuses the architecture, `false` when it only warns; every rule of the check is a gate |
| `subject` | the part, or the link as `<from>-><to>:<carries>` |
| `part` | the part the violation names; for a link, the end at fault |
| `link` | for a link, its index in `links`, counted from 0 |
| `message` | what is wrong, in words |

Keys are sorted, so the same document gives the same bytes on every run.
