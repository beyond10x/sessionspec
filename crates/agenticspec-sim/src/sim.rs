//! A discrete-event simulation of one resolved architecture under a behaviour profile.
//!
//! Time is simulated seconds; nothing sleeps. Tasks arrive for the whole architecture and wait in
//! one backlog for a free worker. Every draw about a task comes from that task's own stream of the
//! run's seed, whichever worker or reviewer handles it, so a run is the same on every host and two
//! variants see the same tasks.

use crate::check::{block, is_reviewer, reach};
use crate::corpus::Class;
use crate::doc::num;
use crate::profile::{
    Coordination as CoordProfile, Dist, Load, Profile, Role, Strictness, TOKEN_CLASSES,
};
use crate::rng::Rng;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};

mod record;
mod script;
mod swarm;

pub use record::{PortUse, RECORDED};
pub use script::{Arrival, RunOutcome, Script, Stamp, TaskRecord, TaskScript};
use swarm::{Coordination, Distribution, Input, Output, Schedule, TrigAcc, Trigger};

#[derive(Debug, Clone, PartialEq)]
pub enum FaultKind {
    /// The principal answers nothing in the window.
    PrincipalAway,
    /// A pool's capacity or a gateway's rate falls to `to` in the window.
    CapacityDrop { to: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fault {
    pub kind: FaultKind,
    pub part: String,
    pub from: f64,
    pub until: f64,
}

pub fn parse_faults(list: &Value) -> Result<Vec<Fault>, String> {
    let mut out = Vec::new();
    for (i, f) in list.as_array().into_iter().flatten().enumerate() {
        let at = |k: &str| {
            f.get(k)
                .and_then(num)
                .ok_or_else(|| format!("faults.{i}: needs `{k}`"))
        };
        let part = f["part"]
            .as_str()
            .ok_or_else(|| format!("faults.{i}: needs `part`"))?
            .to_string();
        let from = at("from")?;
        let until = from + at("for")?;
        let kind = match f["kind"].as_str() {
            Some("principal_away") => FaultKind::PrincipalAway,
            Some("capacity_drop") => FaultKind::CapacityDrop { to: at("to")? },
            other => {
                return Err(format!(
                    "faults.{i}: kind {other:?}, expected principal_away or capacity_drop"
                ));
            }
        };
        out.push(Fault {
            kind,
            part,
            from,
            until,
        });
    }
    Ok(out)
}

/// An experiment's `bursts`: `{ at, tasks }`, tasks that arrive at once.
pub fn parse_bursts(list: &Value) -> Result<Vec<(f64, usize)>, String> {
    let mut out = Vec::new();
    for (i, b) in list.as_array().into_iter().flatten().enumerate() {
        let at = b
            .get("at")
            .and_then(num)
            .filter(|t| *t >= 0.0)
            .ok_or_else(|| format!("bursts.{i}: needs `at`, a duration"))?;
        let tasks = b["tasks"]
            .as_u64()
            .ok_or_else(|| format!("bursts.{i}: needs `tasks`, a whole number"))?;
        out.push((at, tasks as usize));
    }
    Ok(out)
}

/// The windows `(from, until, to)` of capacity drops on `part`.
fn drops(faults: &[Fault], part: &str) -> Vec<(f64, f64, f64)> {
    faults
        .iter()
        .filter(|f| f.part == part)
        .filter_map(|f| match f.kind {
            FaultKind::CapacityDrop { to } => Some((f.from, f.until, to)),
            _ => None,
        })
        .collect()
}

fn level_at(base: f64, drops: &[(f64, f64, f64)], t: f64) -> f64 {
    drops
        .iter()
        .filter(|(a, b, _)| *a <= t && t < *b)
        .fold(base, |acc, (_, _, to)| acc.min(*to))
}

#[derive(Debug, Clone)]
struct Gateway {
    part: String,
    rate: f64,
    drops: Vec<(f64, f64, f64)>,
}

#[derive(Debug, Clone)]
struct Pool {
    part: String,
    capacity: f64,
    job_limit: f64,
    drops: Vec<(f64, f64, f64)>,
}

#[derive(Debug, Clone)]
struct Principal {
    part: String,
    answer: Option<Dist>,
    away: Vec<(f64, f64)>,
}

impl Principal {
    fn available_from(&self, mut t: f64) -> f64 {
        loop {
            let moved = self
                .away
                .iter()
                .find(|(a, b)| *a <= t && t < *b)
                .map(|(_, b)| *b);
            match moved {
                Some(b) => t = b,
                None => return t,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Delegate {
    part: String,
    answer: Dist,
    forward_to: Option<usize>,
    channel: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Target {
    Principal(usize),
    Delegate(usize),
}

/// One model a part reaches over `model_call` links: the gateway on the way to it, the price of
/// its route, and the running total of the shares up to it.
#[derive(Debug, Clone)]
struct Route {
    model: usize,
    gateway: Option<usize>,
    price: [f64; 4],
    upto: f64,
}

/// What every kind that works turns has (an agent, a reviewer, a runtime that executes runs):
/// its instances, its behaviour and the models it calls.
#[derive(Debug, Clone)]
struct Exec {
    part: String,
    count: usize,
    role: Role,
    task_turns: Dist,
    turn_seconds: Dist,
    /// The models it reaches with a share above 0; empty when its turns call no model.
    routes: Vec<Route>,
}

impl Exec {
    /// The model of one call: the only one, or one drawn by share. Draws from `rng` only when
    /// there are several, so a part with one model sees the same stream as before.
    fn route(&self, rng: &mut Rng) -> Option<&Route> {
        match self.routes.len() {
            0 => None,
            1 => self.routes.first(),
            _ => {
                let total = self.routes.last().map_or(0.0, |r| r.upto);
                let u = rng.unit() * total;
                self.routes
                    .iter()
                    .find(|r| u < r.upto)
                    .or(self.routes.last())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct WorkerKind {
    exec: Exec,
    question: Option<Target>,
    pool: Option<usize>,
    /// The reviewer kind its `review` link ends at.
    review: Option<usize>,
    /// Where its copies take work from.
    input: Input,
    /// Where a task goes when it is done here.
    output: Output,
    coordination: Coordination,
    /// Seconds between the starts of two copies.
    stagger: f64,
    /// Each copy owns one scope; tasks are routed to the owner of their scope.
    partition: bool,
    /// Its turns hand work to a swarm with `coordination: orchestrator`.
    orchestrates: bool,
    /// The triggers that wake it.
    triggers: Vec<usize>,
    /// A runtime or process part that executes runs: its steps are its turns, and it is no agent
    /// alive.
    runtime: bool,
}

#[derive(Debug, Clone)]
struct ReviewerKind {
    exec: Exec,
    turns: Dist,
    strictness: Strictness,
    stagger: f64,
}

/// One architecture made ready to simulate.
#[derive(Debug, Clone)]
pub struct Model {
    horizon: f64,
    workers: Vec<WorkerKind>,
    reviewers: Vec<ReviewerKind>,
    gateways: Vec<Gateway>,
    pools: Vec<Pool>,
    principals: Vec<Principal>,
    delegates: Vec<Delegate>,
    models: Vec<String>,
    channels: Vec<String>,
    /// The distribution of each channel, in the order of `channels`.
    distribution: Vec<Distribution>,
    schedules: Vec<Schedule>,
    triggers: Vec<Trigger>,
    /// The triggers that fire on an event of the simulation.
    on_event: Vec<usize>,
    /// The shared channel arrivals are posted to, when a principal posts to one, and that
    /// principal part.
    intake: Option<(usize, String)>,
    /// The worker kind whose scope partition routes the arrivals.
    partition: Option<usize>,
    /// Tasks that arrive at once, `(at, tasks)`.
    bursts: Vec<(f64, usize)>,
    coord: CoordProfile,
    parts: Vec<String>,
    /// Every agent part, its instance count and its start stagger, for the price of being alive.
    alive: Vec<(String, usize, f64)>,
    default_after: f64,
    answers_alone: bool,
    load_per_second: f64,
    agent_hour_usd: f64,
    /// The profile's corpus: each arrival draws one entry by weight.
    corpus: Vec<(f64, Class)>,
    /// Inputs and fixtures of a scripted run, with parts resolved.
    script: Option<Resolved>,
}

/// A [`Script`] with its parts resolved to worker kinds and principals.
#[derive(Debug, Clone)]
struct Resolved {
    arrivals: Vec<Arrival>,
    runs: Vec<(Option<usize>, TaskScript)>,
    builds: Vec<f64>,
    replies: Vec<(usize, f64)>,
}

fn index_by_block(arch: &Value, b: &str) -> Vec<String> {
    arch["parts"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, p)| p["block"] == b)
        .map(|(n, _)| n.clone())
        .collect()
}

fn position(list: &[String], name: &str) -> Option<usize> {
    list.iter().position(|n| n == name)
}

fn number_or(v: &Value, default: f64) -> f64 {
    num(v).unwrap_or(default)
}

/// The settings the simulator reads from a part, per block. Every read of a part's setting in
/// this module and its submodules goes through [`setting`], which asserts in a debug build that
/// the part's block is listed here with the key; a test holds this list against the settings
/// table (`blocks/settings.yaml`) and the sources against [`setting`]. Two reads go through
/// `check` and are listed by hand: an agent's `role` (`check::is_reviewer`) and a channel's
/// `distribution` (`check::distribution`).
pub const PART_READS: &[(&str, &[&str])] = &[
    (
        "agent",
        &[
            "count",
            "role",
            "model_share",
            "start_stagger",
            "coordination",
            "partition",
            "acts_for",
            "strictness",
            "false_accept",
            "false_reject",
        ],
    ),
    ("runtime", &["count", "role", "model_share"]),
    ("process", &["count", "role", "model_share", "steps"]),
    ("principal", &["kind"]),
    ("gateway", &["rate_limit_per_minute"]),
    ("pool", &["capacity", "job_limit"]),
    ("model", &["route"]),
    ("channel", &["distribution"]),
    (
        "trigger",
        &[
            "kind",
            "schedule",
            "channel",
            "event_name",
            "source",
            "policy",
            "max_parallel",
            "task_template",
            "takes",
        ],
    ),
    ("schedule", &["every", "next_fire_at"]),
];

/// The keys [`PART_READS`] lists for `block`.
pub fn part_reads(block: &str) -> &'static [&'static str] {
    PART_READS
        .iter()
        .find(|(b, _)| *b == block)
        .map_or(&[], |(_, keys)| keys)
}

/// Setting `key` of part `p`, `Null` when it has none: the one way the simulator reads a part.
fn setting<'a>(p: &'a Value, key: &str) -> &'a Value {
    debug_assert!(
        part_reads(p["block"].as_str().unwrap_or("")).contains(&key),
        "the simulator reads `{key}` from a part of block {} and PART_READS does not list it",
        p["block"]
    );
    &p[key]
}

fn count_of(p: &Value) -> usize {
    num(setting(p, "count")).unwrap_or(1.0).max(0.0).round() as usize
}

/// The strictness of reviewer part `name`: its own `false_accept` and `false_reject`, else the
/// profile's entry named by `strictness` (default `balanced`).
fn strictness_of(name: &str, p: &Value, profile: &Profile) -> Result<Strictness, String> {
    let label = setting(p, "strictness").as_str().unwrap_or("balanced");
    let named = profile.review.strictness.get(label).copied();
    let base = match named {
        Some(s) => s,
        None if setting(p, "false_accept").is_number()
            && setting(p, "false_reject").is_number() =>
        {
            Strictness {
                false_accept: 0.0,
                false_reject: 0.0,
            }
        }
        None => {
            let known: Vec<&str> = profile
                .review
                .strictness
                .keys()
                .map(String::as_str)
                .collect();
            return Err(format!(
                "part `{name}`: strictness `{label}` is not in review.strictness ({})",
                known.join(", ")
            ));
        }
    };
    let rate = |k: &str, d: f64| setting(p, k).as_f64().map_or(d, |x| x.clamp(0.0, 1.0));
    Ok(Strictness {
        false_accept: rate("false_accept", base.false_accept),
        false_reject: rate("false_reject", base.false_reject),
    })
}

impl Model {
    pub fn build(
        arch: &Value,
        profile: &Profile,
        horizon: f64,
        faults: &[Fault],
        bursts: &[(f64, usize)],
    ) -> Result<Model, String> {
        if let Some(r) = &profile.corpus_ref
            && profile.corpus.is_empty()
        {
            return Err(format!(
                "load.corpus `{r}` is not read: a corpus is read relative to the profile file"
            ));
        }
        let part = |n: &str| &arch["parts"][n];
        let links: Vec<Value> = arch["links"].as_array().cloned().unwrap_or_default();
        let parts: Vec<String> = arch["parts"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(n, _)| n.clone())
            .collect();
        for f in faults {
            let want = match f.kind {
                FaultKind::PrincipalAway => &["principal"][..],
                FaultKind::CapacityDrop { .. } => &["pool", "gateway"][..],
            };
            match block(arch, &f.part) {
                Some(b) if want.contains(&b) => {}
                _ => {
                    return Err(format!(
                        "fault on `{}`: expected a part of block {}",
                        f.part,
                        want.join(" or ")
                    ));
                }
            }
        }

        let gateway_names = index_by_block(arch, "gateway");
        let gateways = gateway_names
            .iter()
            .map(|n| Gateway {
                part: n.clone(),
                rate: number_or(setting(part(n), "rate_limit_per_minute"), f64::INFINITY),
                drops: drops(faults, n),
            })
            .collect();
        let pool_names = index_by_block(arch, "pool");
        let pools = pool_names
            .iter()
            .map(|n| Pool {
                part: n.clone(),
                capacity: number_or(setting(part(n), "capacity"), f64::INFINITY),
                job_limit: number_or(setting(part(n), "job_limit"), f64::INFINITY),
                drops: drops(faults, n),
            })
            .collect();
        let principal_names = index_by_block(arch, "principal");
        let mut principals = Vec::new();
        for n in &principal_names {
            let kind = setting(part(n), "kind").as_str().unwrap_or("human");
            let prof = profile.principals.get(kind);
            let mut away = Vec::new();
            for w in prof.map(|p| p.away.as_slice()).unwrap_or(&[]) {
                let mut start = w.from;
                while start < horizon {
                    away.push((start, start + w.length));
                    match w.every {
                        Some(e) => start += e,
                        None => break,
                    }
                }
            }
            for f in faults
                .iter()
                .filter(|f| &f.part == n && f.kind == FaultKind::PrincipalAway)
            {
                away.push((f.from, f.until));
            }
            away.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
            principals.push(Principal {
                part: n.clone(),
                answer: prof.map(|p| p.answer_seconds.clone()),
                away,
            });
        }
        let models = index_by_block(arch, "model");
        let channels = index_by_block(arch, "channel");
        let question_link = |from: &str| {
            links.iter().find(|l| {
                l["from"] == from
                    && l["carries"] == "question"
                    && block(arch, l["to"].as_str().unwrap_or("")).is_some()
            })
        };
        // The profile role of a part is named by its `role`; an agent without one is `coding`.
        // `harness` names the kind of harness and picks no role.
        let role_of = |name: &str, p: &Value| -> Result<(String, Role), String> {
            let label = match crate::check::value(setting(p, "role")).as_str() {
                Some(l) => l,
                None if p["block"] == "agent" => "coding",
                None => {
                    return Err(format!(
                        "part `{name}` executes runs and names no `role`, a role of the profile"
                    ));
                }
            };
            let role = profile
                .roles
                .get(label)
                .ok_or_else(|| format!("profile has no role `{label}` for part `{name}`"))?
                .clone();
            Ok((label.to_string(), role))
        };
        let routes = |name: &str, p: &Value| -> Result<Vec<Route>, String> {
            let path = reach(arch, name, "model_call");
            let reached: Vec<usize> = path.iter().filter_map(|r| position(&models, r)).collect();
            let shares: Vec<f64> = match crate::check::value(setting(p, "model_share")) {
                Value::Null => vec![1.0; reached.len()],
                Value::Object(o) => {
                    if let Some(k) = o
                        .keys()
                        .find(|k| !reached.iter().any(|m| &models[*m] == *k))
                    {
                        return Err(format!(
                            "part `{name}`: model_share names `{k}`, not a model part it reaches over model_call"
                        ));
                    }
                    reached
                        .iter()
                        .map(|m| match o.get(&models[*m]) {
                            None => Ok(0.0),
                            Some(v) => num(crate::check::value(v))
                                .filter(|x| *x >= 0.0)
                                .ok_or_else(|| {
                                    format!(
                                        "part `{name}`: model_share.{}: expected a number of at least 0",
                                        models[*m]
                                    )
                                }),
                        })
                        .collect::<Result<_, _>>()?
                }
                _ => {
                    return Err(format!(
                        "part `{name}`: model_share maps the model parts it reaches to shares"
                    ));
                }
            };
            if !reached.is_empty() && shares.iter().sum::<f64>() <= 0.0 {
                return Err(format!(
                    "part `{name}`: model_share gives every model it reaches a share of 0"
                ));
            }
            let mut out = Vec::new();
            let mut upto = 0.0;
            for (&m, share) in reached.iter().zip(shares) {
                if share <= 0.0 {
                    continue;
                }
                upto += share;
                let route = setting(part(&models[m]), "route")
                    .as_str()
                    .ok_or_else(|| format!("model part `{}` names no route", models[m]))?;
                let price = profile.prices.get(route).copied().ok_or_else(|| {
                    let priced: Vec<&str> = profile.prices.keys().map(String::as_str).collect();
                    format!(
                        "part `{}`: route `{route}` has no price in the profile (priced: {})",
                        models[m],
                        priced.join(", ")
                    )
                })?;
                let gateway = path
                    .iter()
                    .filter_map(|r| position(&gateway_names, r))
                    .find(|g| reach(arch, &gateway_names[*g], "model_call").contains(&models[m]));
                out.push(Route {
                    model: m,
                    gateway,
                    price,
                    upto,
                });
            }
            Ok(out)
        };
        let exec = |name: &str, p: &Value, works_tasks: bool| -> Result<Exec, String> {
            let (label, role) = role_of(name, p)?;
            let need = |d: &Option<Dist>, key: &str| {
                d.clone()
                    .ok_or_else(|| format!("part `{name}`: profile role `{label}` has no {key}"))
            };
            let turn_seconds = need(&role.turn_seconds, "turn_seconds")?;
            let task_turns = if works_tasks {
                need(&role.task_turns, "task_turns")?
            } else {
                Dist::Fixed(1.0)
            };
            Ok(Exec {
                part: name.to_string(),
                count: count_of(p),
                role,
                task_turns,
                turn_seconds,
                routes: routes(name, p)?,
            })
        };

        let mut reviewers: Vec<ReviewerKind> = Vec::new();
        let mut reviewer_names: Vec<String> = Vec::new();
        let mut alive = Vec::new();
        for (name, p) in arch["parts"].as_object().into_iter().flatten() {
            if p["block"] != "agent" {
                continue;
            }
            alive.push((name.clone(), count_of(p), swarm::stagger_of(p)));
            if is_reviewer(arch, name) {
                reviewers.push(ReviewerKind {
                    exec: exec(name, p, false)?,
                    turns: profile.review.turns.clone(),
                    strictness: strictness_of(name, p, profile)?,
                    stagger: swarm::stagger_of(p),
                });
                reviewer_names.push(name.clone());
            }
        }

        // A runtime or process part executes runs when it is given them: a `run` or `order` link,
        // or a trigger that wakes it.
        let executes = |name: &str| {
            links.iter().any(|l| {
                l["to"] == name
                    && matches!(l["carries"].as_str(), Some("run" | "order" | "trigger"))
            })
        };
        let mut delegates: Vec<Delegate> = Vec::new();
        let mut workers = Vec::new();
        for (name, p) in arch["parts"].as_object().into_iter().flatten() {
            // A process part executes runs like a runtime: each run takes the steps of its
            // program, each a turn of its role.
            let process = p["block"] == "process";
            let runtime = p["block"] == "runtime" || process;
            let agent = p["block"] == "agent"
                && setting(p, "acts_for").is_null()
                && !is_reviewer(arch, name);
            if !(agent || runtime && executes(name)) {
                continue;
            }
            let pool = links
                .iter()
                .filter(|l| l["from"] == name.as_str() && l["carries"] == "claim")
                .find_map(|l| position(&pool_names, l["to"].as_str().unwrap_or("")));
            let review = links
                .iter()
                .filter(|l| l["from"] == name.as_str() && l["carries"] == "review")
                .find_map(|l| position(&reviewer_names, l["to"].as_str().unwrap_or("")));
            let question = match question_link(name).and_then(|l| l["to"].as_str()) {
                None => None,
                Some(to) if block(arch, to) == Some("principal") => {
                    position(&principal_names, to).map(Target::Principal)
                }
                Some(to) if block(arch, to) == Some("agent") => {
                    let d = match delegates.iter().position(|d| d.part == to) {
                        Some(d) => d,
                        None => {
                            let fwd = question_link(to).filter(|l| {
                                block(arch, l["to"].as_str().unwrap_or("")) == Some("principal")
                            });
                            // The agent a question link ends at answers it, with the answer
                            // delay of its own role, whatever the part is called.
                            let (label, role) = role_of(to, part(to))?;
                            let answer = role.answer_seconds.ok_or_else(|| {
                                format!(
                                    "part `{to}` answers questions; profile role `{label}` has no answer_seconds"
                                )
                            })?;
                            delegates.push(Delegate {
                                part: to.to_string(),
                                answer,
                                forward_to: fwd.and_then(|l| {
                                    position(&principal_names, l["to"].as_str().unwrap_or(""))
                                }),
                                channel: fwd.and_then(|l| {
                                    position(&channels, l["over"].as_str().unwrap_or(""))
                                }),
                            });
                            delegates.len() - 1
                        }
                    };
                    Some(Target::Delegate(d))
                }
                Some(_) => None,
            };
            let mut e = exec(name, p, !process)?;
            if process {
                let steps = num(crate::check::value(setting(p, "steps"))).unwrap_or(1.0);
                e.task_turns = Dist::Fixed(steps.round().max(1.0));
            }
            workers.push(WorkerKind {
                exec: e,
                question,
                pool,
                review,
                input: Input::Backlog,
                output: Output::Ship,
                // How copies coordinate, stagger and partition is an agent's setting; the copies
                // of a runtime or process part take the defaults.
                coordination: if agent {
                    swarm::coordination_of(name, p)?
                } else {
                    Coordination::None
                },
                stagger: if agent { swarm::stagger_of(p) } else { 0.0 },
                partition: agent && swarm::partition_of(name, p)?,
                orchestrates: false,
                triggers: Vec::new(),
                runtime,
            });
        }
        let distribution = channels
            .iter()
            .map(|c| swarm::distribution_of(arch, c))
            .collect::<Result<Vec<_>, _>>()?;
        let schedules = swarm::schedules(arch);
        let triggers = swarm::wire(arch, &mut workers, &channels, &distribution, &schedules)?;
        let intake = swarm::intake(arch, &channels, &distribution);
        let partition = workers
            .iter()
            .position(|w| w.partition && w.input == Input::Backlog);
        let on_event = triggers
            .iter()
            .enumerate()
            .filter(|(_, t)| matches!(t.source, swarm::Source::Event(_)))
            .map(|(i, _)| i)
            .collect();
        let asked = delegates
            .iter()
            .filter_map(|d| d.forward_to)
            .chain(workers.iter().filter_map(|w| match w.question {
                Some(Target::Principal(p)) => Some(p),
                _ => None,
            }));
        for p in asked {
            if principals[p].answer.is_none() {
                return Err(format!(
                    "profile has no principals entry for the kind of part `{}`",
                    principals[p].part
                ));
            }
        }
        let per_hour = match profile.load {
            Load::Total(t) => t,
            Load::PerWorker(w) => {
                w * workers
                    .iter()
                    .filter(|k| !k.runtime)
                    .map(|k| k.exec.count as f64)
                    .sum::<f64>()
            }
        };
        let policies = &arch["policies"];
        Ok(Model {
            horizon,
            workers,
            reviewers,
            gateways,
            pools,
            principals,
            delegates,
            models,
            channels,
            distribution,
            schedules,
            triggers,
            on_event,
            intake,
            partition,
            bursts: bursts.to_vec(),
            coord: profile.coordination.clone(),
            parts,
            alive,
            default_after: number_or(&policies["question_default_after"], f64::INFINITY),
            answers_alone: policies["operator_answers_alone"]
                .as_bool()
                .unwrap_or(false),
            load_per_second: per_hour / 3600.0,
            agent_hour_usd: profile.agent_hour_usd,
            corpus: profile.corpus.clone(),
            script: None,
        })
    }

    /// The model run by `script` instead of the profile's load: only its arrivals make tasks,
    /// and its fixtures decide outcomes, build times and replies. Refuses a fixture whose part is
    /// not a worker (runs) or a principal (replies), and an arrival at a part that does not exist.
    pub fn with_script(mut self, script: Script) -> Result<Model, String> {
        let mut runs = Vec::new();
        for (i, (part, s)) in script.runs.into_iter().enumerate() {
            let kind = match part {
                None => None,
                Some(p) => Some(
                    self.workers
                        .iter()
                        .position(|w| w.exec.part == p)
                        .ok_or_else(|| format!("runs.{i}: part `{p}` is no worker"))?,
                ),
            };
            runs.push((kind, s));
        }
        let mut replies = Vec::new();
        for (i, (part, after)) in script.replies.into_iter().enumerate() {
            let p = self
                .principals
                .iter()
                .position(|pr| pr.part == part)
                .ok_or_else(|| format!("replies.{i}: part `{part}` is no principal"))?;
            replies.push((p, after));
        }
        for (i, a) in script.arrivals.iter().enumerate() {
            if !self.parts.contains(&a.part) {
                return Err(format!("arrivals.{i}: part `{}` does not exist", a.part));
            }
        }
        self.load_per_second = 0.0;
        self.bursts.clear();
        self.corpus.clear();
        self.script = Some(Resolved {
            arrivals: script.arrivals,
            runs,
            builds: script.builds,
            replies,
        });
        Ok(self)
    }
}

/// What one run measured.
#[derive(Debug, Clone, Default)]
pub struct RunOut {
    pub kpis: BTreeMap<String, f64>,
    pub per_part: BTreeMap<String, BTreeMap<String, f64>>,
    /// Ordered by time.
    pub timeline: Vec<Stamp>,
    /// Each task of a scripted run, by its number; empty otherwise.
    pub tasks: Vec<TaskRecord>,
    /// The port uses of a recorded run, in the order they were made; empty otherwise.
    pub uses: Vec<PortUse>,
}

#[derive(Debug, Clone, Copy)]
enum Ev {
    Arrive,
    StepDone(usize),
    BuildDone(usize),
    PoolCheck(usize),
    /// A schedule ticks.
    Tick(usize),
    /// Tasks arrive at once.
    Burst(usize),
    /// A copy held back by its start stagger starts.
    Wake,
    /// A copy that read a board starts the entry it claimed.
    Begin(usize),
    /// An empty run ends.
    EmptyDone(usize),
    /// An event a trigger waits for occurred.
    Fire(usize),
    /// A task of the script arrives.
    Scripted(usize),
}

struct Item {
    t: f64,
    seq: u64,
    ev: Ev,
}

impl PartialEq for Item {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == Ordering::Equal
    }
}
impl Eq for Item {}
impl PartialOrd for Item {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Item {
    /// Reversed, so the heap pops the earliest event; ties go by scheduling order.
    fn cmp(&self, o: &Self) -> Ordering {
        o.t.total_cmp(&self.t).then(o.seq.cmp(&self.seq))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Turn,
    /// A turn spent again: after a failed step, a wrong assumption, a rejection.
    Rework,
    Build,
    Question,
    /// A model call of a reviewer.
    Review,
}

#[derive(Debug, Clone, Copy)]
enum Answerer {
    Principal(usize),
    Delegate(usize),
}

#[derive(Debug, Clone, Copy)]
struct Outcome {
    defaulted: bool,
    answerer: Answerer,
    channel: Option<usize>,
    /// The reply of the script the question took.
    reply: Option<usize>,
    /// The question's number.
    question: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Who {
    Worker(usize),
    Reviewer(usize),
}

struct Inst {
    who: Who,
    task: Option<usize>,
    steps: Vec<Step>,
    at: usize,
    pending: Option<Outcome>,
    killed: bool,
    /// Its number among the copies of its part.
    copy: usize,
    /// When it starts, after its part's start stagger.
    start_at: f64,
    /// The trigger whose run it works, if a firing started it.
    run: Option<usize>,
    /// The run released a task that is not correct, and goes on with the next.
    run_failed: bool,
    /// Busy without a task: reading a board or doing an empty run.
    hold: bool,
    /// The entry it claimed on a board, started when its read ends.
    begin: Option<usize>,
    /// Tasks routed to the scope it owns.
    own: VecDeque<usize>,
    /// The number of its current build's claim.
    claim: usize,
    /// The number of the firing whose run it works.
    firing: usize,
}

/// The next number of each simulated entity that has no index of its own.
#[derive(Debug, Default)]
struct Ids {
    question: usize,
    claim: usize,
    firing: usize,
    post: usize,
    deliverable: usize,
}

fn next(counter: &mut usize) -> usize {
    *counter += 1;
    *counter - 1
}

struct Task {
    arrived: f64,
    rng: Rng,
    base_turns: usize,
    correct: bool,
    /// The worker kind that last reported it done.
    kind: usize,
    /// The task it is a copy of (a topic delivers one post to each subscriber); itself otherwise.
    origin: usize,
    /// Its scope and the other scope it touches, in a scope partition.
    scope: Option<(usize, Option<usize>)>,
    /// When the work that its next merge carries started.
    work_started: f64,
    /// A worker started it once; its backlog wait is counted.
    waited: bool,
    /// What is decided about it instead of drawn.
    script: Option<TaskScript>,
    /// The arrival of the script it came from.
    arrival: Option<usize>,
    /// The run fixture of the script its first run took.
    fixture: Option<usize>,
    /// The copy of the worker kind that last reported it done.
    by: usize,
    /// The number of the deliverable last submitted for it.
    deliverable: usize,
}

#[derive(Default, Clone)]
struct KindAcc {
    started: f64,
    claimed: f64,
    done: f64,
    correct: f64,
    false_shipped: f64,
    turns: f64,
    rework_turns: f64,
    /// Turns that called a model.
    model_calls: f64,
    cost: f64,
    asked: f64,
    wrong_assumptions: f64,
    steps: f64,
    steps_failed: f64,
    after_pass: f64,
    passed_after_pass: f64,
    after_fail: f64,
    passed_after_fail: f64,
    owner_wait: f64,
    operator_wait: f64,
    slot_wait: f64,
    rate_wait: f64,
    busy: f64,
    claim_conflicts: f64,
    coordination_calls: f64,
    coordination_messages: f64,
    merge_conflicts: f64,
}

#[derive(Default, Clone)]
struct ReviewerAcc {
    queue: VecDeque<(usize, f64)>,
    waits: Vec<f64>,
    reviews: f64,
    rejected: f64,
    calls: f64,
    cost: f64,
    rate_wait: f64,
    busy: f64,
}

#[derive(Default, Clone)]
struct GatewayAcc {
    starts: VecDeque<f64>,
    last: f64,
    calls: f64,
    limited_calls: f64,
    limited_seconds: f64,
}

#[derive(Default, Clone)]
struct PoolAcc {
    running: usize,
    queue: VecDeque<(usize, f64)>,
    waits: Vec<f64>,
    jobs: f64,
    killed: f64,
    busy: f64,
}

#[derive(Default, Clone)]
struct ModelAcc {
    calls: f64,
    cost: f64,
    tokens: [f64; 4],
}

#[derive(Default, Clone)]
struct AnswerAcc {
    received: f64,
    answered: f64,
    forwarded: f64,
    decided_owner: f64,
}

/// The stream index of the arrivals; a task's stream is its number.
const ARRIVALS: u64 = u64::MAX - 1;
/// The stream index of the draws of coordination: board reads, mesh reads, empty runs.
const COORDINATION: u64 = u64::MAX - 2;
/// The stream index of the draws of corpus entries.
const CORPUS: u64 = u64::MAX - 3;
/// Mixed into the seed for the scope streams, one per task.
const SCOPES: u64 = 0x5C0B_E5C0_BE5C_0BE5;

struct Sim<'a> {
    m: &'a Model,
    seed: u64,
    heap: BinaryHeap<Item>,
    seq: u64,
    arrivals: Rng,
    coord: Rng,
    tasks: Vec<Task>,
    /// Tasks that arrived, a topic's copies not counted.
    arrived: f64,
    backlog: VecDeque<usize>,
    /// Tasks a reviewer sent back, by the worker kind that takes them before other work.
    returned: Vec<VecDeque<usize>>,
    /// Tasks handed to a worker kind over an `order` link.
    handed: Vec<VecDeque<usize>>,
    /// Untaken posts of each queue or board channel.
    posts: Vec<VecDeque<usize>>,
    posted: Vec<f64>,
    trig: Vec<TrigAcc>,
    ticks: Vec<f64>,
    /// Per worker kind and scope, when a merge last touched the scope.
    last_merge: Vec<Vec<f64>>,
    /// Per task origin, whether one of its copies shipped.
    shipped: Vec<bool>,
    duplicates: f64,
    /// The instances of each worker kind, by copy number.
    copies: Vec<Vec<usize>>,
    backlog_waits: Vec<f64>,
    insts: Vec<Inst>,
    kinds: Vec<KindAcc>,
    reviewers: Vec<ReviewerAcc>,
    gateways: Vec<GatewayAcc>,
    pools: Vec<PoolAcc>,
    models: Vec<ModelAcc>,
    principals: Vec<AnswerAcc>,
    delegates: Vec<AnswerAcc>,
    channels: Vec<f64>,
    defaulted: f64,
    record: bool,
    timeline: Vec<Stamp>,
    ids: Ids,
    /// The draws of corpus entries.
    corpus: Rng,
    /// Run fixtures not yet taken: by worker kind, and for any kind.
    runs_left: Vec<VecDeque<usize>>,
    runs_any: VecDeque<usize>,
    /// Replies not yet taken, by principal.
    replies_left: Vec<VecDeque<usize>>,
    /// Build times of the script taken so far.
    builds_taken: usize,
    uses: Vec<PortUse>,
}

/// The part of `[a, b)` inside the horizon `[0, h)`.
fn span(a: f64, b: f64, h: f64) -> f64 {
    (b.min(h) - a.min(h)).max(0.0)
}

fn percentile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = (q * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

fn ratio(a: f64, b: f64) -> f64 {
    if b > 0.0 { a / b } else { 0.0 }
}

fn whole(d: &Dist, rng: &mut Rng, least: f64) -> usize {
    d.sample(rng).round().max(least) as usize
}

impl<'a> Sim<'a> {
    fn at(&mut self, t: f64, ev: Ev) {
        if t.is_finite() && t < self.m.horizon {
            self.seq += 1;
            self.heap.push(Item {
                t,
                seq: self.seq,
                ev,
            });
        }
    }

    /// Records an event of the simulation, and fires the `on_event` triggers that wait for it
    /// once the current event is handled. `subject` and `copy` are kept for the event log only
    /// (`crate::event_log`).
    fn rec(
        &mut self,
        t: f64,
        part: &str,
        kind: &'static str,
        subject: Option<usize>,
        copy: Option<usize>,
    ) {
        self.rec_task(t, part, kind, None, subject, copy);
    }

    /// [`Sim::rec`] of an event about `task`, the task a scripted run records it for.
    fn rec_task(
        &mut self,
        t: f64,
        part: &str,
        kind: &'static str,
        task: Option<usize>,
        subject: Option<usize>,
        copy: Option<usize>,
    ) {
        if self.record && t < self.m.horizon {
            self.timeline.push(Stamp {
                t,
                part: part.to_string(),
                kind,
                task,
                reply: None,
                subject,
                copy,
            });
        }
        let m = self.m;
        for &tr in &m.on_event {
            if m.triggers[tr].fires_on(kind, part) {
                self.at(t, Ev::Fire(tr));
            }
        }
    }

    /// An answered or defaulted question of instance `i`, with the reply of the script it took.
    #[allow(clippy::too_many_arguments)]
    fn rec_reply(
        &mut self,
        t: f64,
        part: &str,
        kind: &'static str,
        i: usize,
        reply: Option<usize>,
        subject: Option<usize>,
        copy: Option<usize>,
    ) {
        let task = self.insts[i].task;
        let before = self.timeline.len();
        self.rec_task(t, part, kind, task, subject, copy);
        if self.timeline.len() > before
            && let Some(s) = self.timeline.last_mut()
        {
            s.reply = reply;
        }
    }

    /// Seconds the next build of instance `i` of worker kind `kind` takes: the script's next
    /// build time, else a draw.
    fn build_time(&mut self, kind: usize, i: usize) -> f64 {
        let m = self.m;
        if let Some(d) = m
            .script
            .as_ref()
            .and_then(|s| s.builds.get(self.builds_taken))
        {
            self.builds_taken += 1;
            return d.max(0.0);
        }
        m.workers[kind]
            .exec
            .role
            .build_seconds
            .sample(self.rng(i))
            .max(0.0)
    }

    /// The next run fixture for a run of worker kind `kind`: its own first, then any kind's.
    fn next_run(&mut self, kind: usize) -> Option<usize> {
        self.m.script.as_ref()?;
        self.runs_left[kind]
            .pop_front()
            .or_else(|| self.runs_any.pop_front())
    }

    /// How long principal `p` takes to answer the question of instance `i`: the next reply of
    /// the script (never, with none left), else a draw.
    fn reply_delay(&mut self, p: usize, i: usize) -> (f64, Option<usize>) {
        if let Some(sc) = &self.m.script {
            return match self.replies_left[p].pop_front() {
                Some(r) => (sc.replies[r].1.max(0.0), Some(r)),
                None => (f64::INFINITY, None),
            };
        }
        let d = self.m.principals[p]
            .answer
            .as_ref()
            .map_or(f64::INFINITY, |d| d.sample(self.rng(i)).max(0.0));
        (d, None)
    }

    fn exec(&self, who: Who) -> &'a Exec {
        let m = self.m;
        match who {
            Who::Worker(k) => &m.workers[k].exec,
            Who::Reviewer(r) => &m.reviewers[r].exec,
        }
    }

    fn rng(&mut self, i: usize) -> &mut Rng {
        let task = self.insts[i]
            .task
            .expect("an instance at work holds a task");
        &mut self.tasks[task].rng
    }

    fn admit(&mut self, g: usize, now: f64) -> f64 {
        let gw = &self.m.gateways[g];
        let acc = &mut self.gateways[g];
        if gw.rate.is_infinite() && gw.drops.is_empty() {
            return now;
        }
        let mut c = now.max(acc.last);
        loop {
            // The rate is calls per minute and may be a fraction: `n` calls in each window of
            // `w` seconds. 120 is 120 calls a minute, 0.5 one call every 120 s, 1.5 one every 40 s.
            let rate = level_at(gw.rate, &gw.drops, c);
            let n = rate.floor().max(1.0);
            let w = if rate.is_finite() && rate > 0.0 {
                60.0 * n / rate
            } else {
                60.0
            };
            while acc.starts.front().is_some_and(|s| *s + w.max(60.0) <= c) {
                acc.starts.pop_front();
            }
            if rate <= 0.0 {
                let next = gw
                    .drops
                    .iter()
                    .filter(|(a, b, _)| *a <= c && c < *b)
                    .map(|(_, b, _)| *b)
                    .fold(f64::INFINITY, f64::min);
                if next.is_infinite() {
                    return f64::INFINITY;
                }
                c = next;
                continue;
            }
            if rate.is_infinite() {
                break;
            }
            let within = acc.starts.iter().rev().take_while(|s| **s + w > c).count();
            if (within as f64) < n {
                break;
            }
            c = acc.starts[acc.starts.len() - n as usize] + w;
        }
        acc.starts.push_back(c);
        acc.last = c;
        c
    }

    /// Counts one step of a task's chain after `prev` (`None` for the first step).
    fn count_step(&mut self, kind: usize, prev: Option<bool>, pass: bool) {
        let k = &mut self.kinds[kind];
        k.steps += 1.0;
        if !pass {
            k.steps_failed += 1.0;
        }
        match prev {
            Some(true) => {
                k.after_pass += 1.0;
                if pass {
                    k.passed_after_pass += 1.0;
                }
            }
            Some(false) => {
                k.after_fail += 1.0;
                if pass {
                    k.passed_after_fail += 1.0;
                }
            }
            None => {}
        }
    }

    /// Whether instance `i` has started and holds no work.
    fn idle(&self, i: usize, now: f64) -> bool {
        let x = &self.insts[i];
        x.task.is_none() && !x.hold && x.start_at <= now
    }

    /// Gives each free worker its next work: a task a reviewer sent back to its kind, else what
    /// its input offers (the oldest arrival, its own scope's oldest task, a handed task or a
    /// waiting firing).
    fn assign(&mut self, now: f64) {
        for i in 0..self.insts.len() {
            let Who::Worker(k) = self.insts[i].who else {
                continue;
            };
            if !self.idle(i, now) {
                continue;
            }
            if let Some(id) = self.returned[k].pop_front() {
                self.start_rework(i, id, now);
                continue;
            }
            let w = &self.m.workers[k];
            let next = match w.input {
                Input::Woken => {
                    self.take_firing(i, k, now);
                    continue;
                }
                Input::Handed => self.handed[k].pop_front(),
                Input::Backlog if w.partition && self.m.partition == Some(k) => {
                    self.insts[i].own.pop_front()
                }
                Input::Backlog => self.backlog.pop_front(),
            };
            if let Some(id) = next {
                self.start_task(i, id, now);
            }
        }
    }

    fn start_task(&mut self, i: usize, id: usize, now: f64) {
        let m = self.m;
        let Who::Worker(kind) = self.insts[i].who else {
            unreachable!("only workers start tasks")
        };
        let w = &m.workers[kind];
        let r = &w.exec.role;
        self.insts[i].task = Some(id);
        self.coordinate_start(i, kind, id, now);
        if self.tasks[id].script.is_none()
            && self.tasks[id].fixture.is_none()
            && let Some(f) = self.next_run(kind)
        {
            self.tasks[id].fixture = Some(f);
            self.tasks[id].script = m.script.as_ref().map(|s| s.runs[f].1.clone());
        }
        let task = &mut self.tasks[id];
        if !task.waited {
            task.waited = true;
            self.backlog_waits.push(now - task.arrived);
        }
        task.work_started = now;
        // A script decides the first run only: its outcome is taken when that run reports.
        let s = task
            .script
            .clone()
            .filter(|s| s.outcome.is_some())
            .unwrap_or_default();
        let rng = &mut task.rng;
        let turns = s
            .turns
            .map_or_else(|| whole(&w.exec.task_turns, rng, 1.0), |n| n.max(1));
        let builds = s
            .builds
            .unwrap_or_else(|| whole(&r.builds_per_task, rng, 0.0));
        let ask_first =
            w.question.is_some() && s.outcome == Some(RunOutcome::Question) && s.question_first;
        let questions = match (w.question.is_some(), s.outcome) {
            (false, _) => 0,
            (true, Some(RunOutcome::Question)) => usize::from(!s.question_first),
            (true, Some(_)) => 0,
            (true, None) => whole(&r.questions_per_task, rng, 0.0),
        };
        let mut after: Vec<(usize, Step)> = Vec::with_capacity(builds + questions);
        for _ in 0..builds {
            after.push((rng.below(turns), Step::Build));
        }
        for _ in 0..questions {
            after.push((rng.below(turns), Step::Question));
        }
        after.sort_by_key(|(pos, step)| (*pos, *step == Step::Question));
        // The chain of steps over the turns: each step passes with a chance that depends on the
        // step before it, and a failed step adds rework turns after its last turn.
        let mut rework_after = vec![0usize; turns];
        let mut outcomes: Vec<(Option<bool>, bool)> = Vec::new();
        let mut prev: Option<bool> = None;
        let mut t = 0;
        if let Some(o) = s.outcome {
            prev = Some(matches!(o, RunOutcome::Correct | RunOutcome::Question));
            t = turns;
        }
        while t < turns {
            let size = r.step_turns.as_ref().map_or(turns, |d| whole(d, rng, 1.0));
            let end = (t + size).min(turns);
            let p = match prev {
                None => r.step_pass_first,
                Some(true) => r.step_pass_after_pass,
                Some(false) => r.step_pass_after_fail,
            };
            let pass = rng.chance(p);
            if !pass {
                rework_after[end - 1] += whole(&r.rework_turns, rng, 0.0);
            }
            outcomes.push((prev, pass));
            prev = Some(pass);
            t = end;
        }
        task.base_turns = turns;
        task.correct = prev.unwrap_or(true);
        let mut steps = Vec::with_capacity(turns + after.len() + 1);
        if ask_first {
            steps.push(Step::Question);
        }
        let mut extra = after.iter().peekable();
        for (t, rework) in rework_after.iter().enumerate() {
            steps.push(Step::Turn);
            steps.extend(std::iter::repeat_n(Step::Rework, *rework));
            while let Some((_, s)) = extra.next_if(|(pos, _)| *pos == t) {
                steps.push(*s);
            }
        }
        for (prev, pass) in outcomes {
            self.count_step(kind, prev, pass);
        }
        self.kinds[kind].started += 1.0;
        self.insts[i].steps = steps;
        self.insts[i].at = 0;
        self.rec_task(
            now,
            &w.exec.part,
            "task_started",
            Some(id),
            None,
            Some(self.insts[i].copy),
        );
        self.advance(i, now);
    }

    /// One more round on a task that is not done: rework turns, then a step whose chance depends
    /// on whether the task is correct now.
    fn start_rework(&mut self, i: usize, id: usize, now: f64) {
        let m = self.m;
        let Who::Worker(kind) = self.insts[i].who else {
            unreachable!("only workers rework tasks")
        };
        let w = &m.workers[kind];
        let r = &w.exec.role;
        self.insts[i].task = Some(id);
        let task = &mut self.tasks[id];
        task.work_started = now;
        let n = whole(&r.rework_turns, &mut task.rng, 1.0);
        let before = task.correct;
        let p = if before {
            r.step_pass_after_pass
        } else {
            r.step_pass_after_fail
        };
        let pass = task.rng.chance(p);
        task.correct = pass;
        self.count_step(kind, Some(before), pass);
        self.insts[i].steps = vec![Step::Rework; n];
        self.insts[i].at = 0;
        self.rec_task(
            now,
            &w.exec.part,
            "task_reworked",
            Some(id),
            None,
            Some(self.insts[i].copy),
        );
        self.advance(i, now);
    }

    fn advance(&mut self, i: usize, now: f64) {
        let m = self.m;
        let h = m.horizon;
        let who = self.insts[i].who;
        let Some(step) = self.insts[i].steps.get(self.insts[i].at).copied() else {
            return match who {
                Who::Worker(k) => self.work_done(i, k, now),
                Who::Reviewer(r) => self.review_done(i, r, now),
            };
        };
        match step {
            Step::Turn | Step::Rework | Step::Review => {
                let e = self.exec(who);
                let route = e.route(self.rng(i));
                let gateway = route.and_then(|r| r.gateway);
                let start = match gateway {
                    Some(g) => self.admit(g, now),
                    None => now,
                };
                let waited = if start > now {
                    span(now, start, h)
                } else {
                    0.0
                };
                if start > now
                    && let Some(g) = gateway
                {
                    self.gateways[g].limited_seconds += waited;
                    self.gateways[g].limited_calls += 1.0;
                    self.rec_task(
                        now,
                        &m.gateways[g].part,
                        "rate_limited",
                        self.insts[i].task,
                        None,
                        None,
                    );
                }
                let rng = self.rng(i);
                let dur = e.turn_seconds.sample(rng).max(0.0);
                let mut tokens = [0.0; 4];
                for (c, d) in e.role.tokens_per_turn.iter().enumerate() {
                    tokens[c] = d.sample(rng).max(0.0).round();
                }
                let cost: f64 = route.map_or(0.0, |r| {
                    tokens.iter().zip(&r.price).map(|(t, p)| t * p).sum()
                });
                let ran = start < h;
                let busy = span(start, start + dur, h);
                match who {
                    Who::Worker(k) => {
                        let acc = &mut self.kinds[k];
                        acc.rate_wait += waited;
                        acc.busy += busy;
                        if ran {
                            acc.turns += 1.0;
                            acc.cost += cost;
                            if route.is_some() {
                                acc.model_calls += 1.0;
                            }
                            if step == Step::Rework {
                                acc.rework_turns += 1.0;
                            }
                            if m.workers[k].orchestrates {
                                acc.coordination_calls += 1.0;
                            }
                        }
                    }
                    Who::Reviewer(r) => {
                        let acc = &mut self.reviewers[r];
                        acc.rate_wait += waited;
                        acc.busy += busy;
                        if ran {
                            acc.calls += 1.0;
                            acc.cost += cost;
                        }
                    }
                }
                if ran {
                    if let Some(g) = gateway {
                        self.gateways[g].calls += 1.0;
                    }
                    if let Some(r) = route {
                        let copy = self.insts[i].copy;
                        self.model_call(&e.part, copy, (r.model, r.gateway), start, start + dur);
                        let ma = &mut self.models[r.model];
                        ma.calls += 1.0;
                        ma.cost += cost;
                        for (sum, t) in ma.tokens.iter_mut().zip(tokens) {
                            *sum += t;
                        }
                    }
                }
                self.at(start + dur, Ev::StepDone(i));
            }
            Step::Build => {
                let Who::Worker(kind) = who else {
                    unreachable!("only workers build")
                };
                match m.workers[kind].pool {
                    Some(p) => {
                        self.insts[i].claim = next(&mut self.ids.claim);
                        let cap = level_at(m.pools[p].capacity, &m.pools[p].drops, now);
                        if (self.pools[p].running as f64) < cap && self.pools[p].queue.is_empty() {
                            self.pools[p].waits.push(0.0);
                            self.start_build(p, i, now, now);
                        } else {
                            self.pools[p].queue.push_back((i, now));
                            let claim = Some(self.insts[i].claim);
                            self.rec_task(
                                now,
                                &m.pools[p].part,
                                "build_queued",
                                self.insts[i].task,
                                claim,
                                None,
                            );
                        }
                    }
                    None => {
                        let dur = self.build_time(kind, i);
                        self.kinds[kind].busy += span(now, now + dur, h);
                        self.at(now + dur, Ev::StepDone(i));
                    }
                }
            }
            Step::Question => self.ask(i, now),
        }
    }

    /// A worker finished its steps: it reports the task done, or, when it knows the task is not
    /// correct, works on it again.
    fn work_done(&mut self, i: usize, kind: usize, now: f64) {
        let m = self.m;
        let w = &m.workers[kind];
        let id = self.insts[i].task.expect("a worker at work holds a task");
        let task = &mut self.tasks[id];
        let scripted = task.script.as_mut().and_then(|s| s.outcome.take());
        let claims = match scripted {
            Some(RunOutcome::Wrong) => false,
            Some(RunOutcome::FalseCompletion) => true,
            Some(RunOutcome::Correct | RunOutcome::Question) => task.correct,
            None => task.correct || task.rng.chance(w.exec.role.false_completion),
        };
        if claims {
            task.kind = kind;
            task.by = self.insts[i].copy;
        } else {
            self.rec_task(
                now,
                &w.exec.part,
                "task_failed",
                Some(id),
                None,
                Some(self.insts[i].copy),
            );
            return self.start_rework(i, id, now);
        }
        if w.partition && !self.merge(kind, id, now) {
            return self.start_rework(i, id, now);
        }
        self.kinds[kind].claimed += 1.0;
        self.insts[i].task = None;
        self.end_run(i, Some(id), now);
        match w.review {
            Some(r) => {
                let d = next(&mut self.ids.deliverable);
                self.tasks[id].deliverable = d;
                self.rec_task(
                    now,
                    &w.exec.part,
                    "task_claimed",
                    Some(id),
                    Some(d),
                    Some(self.insts[i].copy),
                );
                self.reviewers[r].queue.push_back((id, now));
                self.dispatch_reviews(r, now);
            }
            None => self.release(id, now),
        }
        self.assign(now);
    }

    /// A task leaves the worker kind that reported it done (after its review, if any): it ships,
    /// or goes on to the next kind over an `order` link or a shared channel.
    fn release(&mut self, id: usize, now: f64) {
        match self.m.workers[self.tasks[id].kind].output {
            Output::Ship => self.ship(id, now),
            Output::Hand(k) => {
                self.handed[k].push_back(id);
                self.rec_task(
                    now,
                    &self.m.workers[k].exec.part,
                    "task_handed",
                    Some(id),
                    None,
                    None,
                );
                self.assign(now);
            }
            Output::Post(c) => {
                let task = &self.tasks[id];
                let m = self.m;
                let by = (m.workers[task.kind].exec.part.as_str(), task.by);
                self.post(c, id, now, Some(by))
            }
        }
    }

    fn ship(&mut self, id: usize, now: f64) {
        let kind = self.tasks[id].kind;
        let origin = self.tasks[id].origin;
        if self.shipped[origin] {
            self.duplicates += 1.0;
            self.rec_task(
                now,
                &self.m.workers[kind].exec.part,
                "duplicate_dropped",
                Some(id),
                Some(id),
                None,
            );
            return;
        }
        self.shipped[origin] = true;
        let k = &mut self.kinds[kind];
        k.done += 1.0;
        if self.tasks[id].correct {
            k.correct += 1.0;
        } else {
            k.false_shipped += 1.0;
        }
        let part = &self.m.workers[kind].exec.part;
        self.rec_task(now, part, "task_done", Some(id), Some(id), None);
    }

    fn dispatch_reviews(&mut self, r: usize, now: f64) {
        let m = self.m;
        for i in 0..self.insts.len() {
            if self.reviewers[r].queue.is_empty() {
                return;
            }
            if self.insts[i].who != Who::Reviewer(r) || !self.idle(i, now) {
                continue;
            }
            let (id, queued) = self.reviewers[r].queue.pop_front().expect("not empty");
            self.reviewers[r].waits.push(now - queued);
            let n = whole(&m.reviewers[r].turns, &mut self.tasks[id].rng, 1.0);
            self.insts[i].task = Some(id);
            self.insts[i].steps = vec![Step::Review; n];
            self.insts[i].at = 0;
            self.rec_task(
                now,
                &m.reviewers[r].exec.part,
                "review_started",
                Some(id),
                None,
                Some(self.insts[i].copy),
            );
            self.advance(i, now);
        }
    }

    /// A reviewer's verdict: a correct task is rejected with the false-reject rate, a wrong one
    /// accepted with the false-accept rate. A rejected task goes back to the workers.
    fn review_done(&mut self, i: usize, r: usize, now: f64) {
        let m = self.m;
        let s = m.reviewers[r].strictness;
        let id = self.insts[i]
            .task
            .take()
            .expect("a reviewer at work holds a task");
        let task = &mut self.tasks[id];
        let accept = if task.correct {
            !task.rng.chance(s.false_reject)
        } else {
            task.rng.chance(s.false_accept)
        };
        self.reviewers[r].reviews += 1.0;
        if accept {
            self.release(id, now);
        } else {
            self.reviewers[r].rejected += 1.0;
            self.rec_task(
                now,
                &m.reviewers[r].exec.part,
                "task_rejected",
                Some(id),
                Some(self.tasks[id].deliverable),
                Some(self.insts[i].copy),
            );
            let kind = self.tasks[id].kind;
            self.returned[kind].push_back(id);
        }
        self.dispatch_reviews(r, now);
        if !accept {
            self.assign(now);
        }
    }

    /// Starts the build of instance `i` in pool `p`, which it asked for at `requested`.
    fn start_build(&mut self, p: usize, i: usize, requested: f64, now: f64) {
        let m = self.m;
        let h = m.horizon;
        let Who::Worker(kind) = self.insts[i].who else {
            unreachable!("only workers build")
        };
        let dur = self.build_time(kind, i);
        let limit = m.pools[p].job_limit;
        let run = dur.min(limit);
        self.insts[i].killed = dur > limit;
        let pa = &mut self.pools[p];
        pa.running += 1;
        pa.jobs += 1.0;
        pa.busy += span(now, now + run, h);
        self.kinds[kind].busy += span(now, now + run, h);
        let copy = self.insts[i].copy;
        let claim = PortUse::new(
            "resource.claim",
            &m.workers[kind].exec.part,
            copy,
            requested,
            now + run,
        );
        self.port_use(claim.to(&m.pools[p].part));
        let claim = Some(self.insts[i].claim);
        self.rec_task(
            now,
            &m.pools[p].part,
            "build_started",
            self.insts[i].task,
            claim,
            None,
        );
        self.at(now + run, Ev::BuildDone(i));
    }

    fn dispatch(&mut self, p: usize, now: f64) {
        let m = self.m;
        loop {
            let cap = level_at(m.pools[p].capacity, &m.pools[p].drops, now);
            if (self.pools[p].running as f64) >= cap {
                return;
            }
            let Some((j, queued)) = self.pools[p].queue.pop_front() else {
                return;
            };
            self.pools[p].waits.push(now - queued);
            if let Who::Worker(kind) = self.insts[j].who {
                self.kinds[kind].slot_wait += span(queued, now, m.horizon);
            }
            self.start_build(p, j, queued, now);
        }
    }

    fn ask(&mut self, i: usize, now: f64) {
        let m = self.m;
        let h = m.horizon;
        let Who::Worker(kind) = self.insts[i].who else {
            unreachable!("only workers ask")
        };
        let w = &m.workers[kind];
        let Some(target) = w.question else {
            self.insts[i].at += 1;
            return self.advance(i, now);
        };
        self.kinds[kind].asked += 1.0;
        let question = next(&mut self.ids.question);
        self.rec_task(
            now,
            &w.exec.part,
            "question_asked",
            self.insts[i].task,
            Some(question),
            Some(self.insts[i].copy),
        );
        let task = self.insts[i].task.expect("an asking instance holds a task");
        let scripted = self.tasks[task]
            .script
            .as_ref()
            .filter(|s| s.outcome.is_some())
            .and_then(|s| s.needs_owner);
        let needs_owner = match scripted {
            Some(b) => b,
            None => self.rng(i).chance(w.exec.role.question_needs_owner),
        };
        let deadline = now + m.default_after;
        let mut channel = None;
        let mut reply = None;
        let (answer_at, forwarded_at, answerer) = match target {
            Target::Principal(p) => {
                self.principals[p].received += 1.0;
                let pr = &m.principals[p];
                let avail = pr.available_from(now);
                let (delay, r) = self.reply_delay(p, i);
                reply = r;
                (avail + delay, Some(now), Answerer::Principal(p))
            }
            Target::Delegate(d) => {
                self.delegates[d].received += 1.0;
                let dl = &m.delegates[d];
                let triage = now + dl.answer.sample(self.rng(i)).max(0.0);
                match dl.forward_to {
                    Some(p) if needs_owner && !m.answers_alone && triage < deadline => {
                        if triage < h {
                            self.delegates[d].forwarded += 1.0;
                            self.principals[p].received += 1.0;
                            if let Some(c) = dl.channel {
                                self.channels[c] += 1.0;
                            }
                        }
                        self.rec_task(
                            triage,
                            &dl.part,
                            "question_forwarded",
                            self.insts[i].task,
                            None,
                            None,
                        );
                        channel = dl.channel;
                        let pr = &m.principals[p];
                        let avail = pr.available_from(triage);
                        let (delay, r) = self.reply_delay(p, i);
                        reply = r;
                        (avail + delay, Some(triage), Answerer::Principal(p))
                    }
                    _ => {
                        if needs_owner && triage < deadline && triage < h {
                            self.delegates[d].decided_owner += 1.0;
                        }
                        (triage, None, Answerer::Delegate(d))
                    }
                }
            }
        };
        let resolve = answer_at.min(deadline);
        self.record_question(i, target, answerer, forwarded_at, channel, now, resolve);
        let k = &mut self.kinds[kind];
        match forwarded_at {
            Some(f) => {
                k.operator_wait += span(now, f.min(resolve), h);
                k.owner_wait += span(f, resolve, h);
            }
            None => k.operator_wait += span(now, resolve, h),
        }
        self.insts[i].pending = Some(Outcome {
            defaulted: answer_at > deadline,
            answerer,
            channel,
            reply,
            question,
        });
        self.at(resolve, Ev::StepDone(i));
    }

    fn step_done(&mut self, i: usize, now: f64) {
        let m = self.m;
        if let Some(o) = self.insts[i].pending.take() {
            if o.defaulted {
                self.defaulted += 1.0;
                let Who::Worker(kind) = self.insts[i].who else {
                    unreachable!("only workers ask")
                };
                let role = &m.workers[kind].exec.role;
                let copy = Some(self.insts[i].copy);
                self.rec_reply(
                    now,
                    &m.workers[kind].exec.part,
                    "question_defaulted",
                    i,
                    o.reply,
                    Some(o.question),
                    copy,
                );
                // A default rests on an assumption; a wrong one costs a share of the task again.
                if self.rng(i).chance(role.default_wrong) {
                    self.kinds[kind].wrong_assumptions += 1.0;
                    let id = self.insts[i].task.expect("a worker at work holds a task");
                    let n = (role.rework_share * self.tasks[id].base_turns as f64).round() as usize;
                    self.insts[i]
                        .steps
                        .extend(std::iter::repeat_n(Step::Rework, n));
                }
            } else {
                match o.answerer {
                    Answerer::Principal(p) => {
                        self.principals[p].answered += 1.0;
                        if let Some(c) = o.channel {
                            self.channels[c] += 1.0;
                            self.record_answer(i, p, c, now);
                        }
                        self.rec_reply(
                            now,
                            &m.principals[p].part,
                            "question_answered",
                            i,
                            o.reply,
                            Some(o.question),
                            None,
                        );
                    }
                    Answerer::Delegate(d) => {
                        self.delegates[d].answered += 1.0;
                        self.rec_task(
                            now,
                            &m.delegates[d].part,
                            "question_answered",
                            self.insts[i].task,
                            Some(o.question),
                            None,
                        );
                    }
                }
            }
        }
        self.insts[i].at += 1;
        self.advance(i, now);
    }
}

/// Simulates `model` once under `seed`; with `record`, keeps the timeline.
pub fn simulate(m: &Model, seed: u64, record: bool) -> RunOut {
    let h = m.horizon;
    let mut insts = Vec::new();
    let mut copies = vec![Vec::new(); m.workers.len()];
    let kinds = m
        .workers
        .iter()
        .enumerate()
        .map(|(k, w)| (Who::Worker(k), w.exec.count, w.stagger))
        .chain(
            m.reviewers
                .iter()
                .enumerate()
                .map(|(r, rk)| (Who::Reviewer(r), rk.exec.count, rk.stagger)),
        );
    let mut starts = Vec::new();
    for (who, count, stagger) in kinds {
        for copy in 0..count {
            let start_at = copy as f64 * stagger;
            if let Who::Worker(k) = who {
                copies[k].push(insts.len());
            }
            if start_at > 0.0 {
                starts.push(start_at);
            }
            insts.push(Inst {
                who,
                task: None,
                steps: Vec::new(),
                at: 0,
                pending: None,
                killed: false,
                copy,
                start_at,
                run: None,
                run_failed: false,
                hold: false,
                begin: None,
                own: VecDeque::new(),
                claim: 0,
                firing: 0,
            });
        }
    }
    let mut s = Sim {
        m,
        seed,
        heap: BinaryHeap::new(),
        seq: 0,
        arrivals: Rng::stream(seed, ARRIVALS),
        coord: Rng::stream(seed, COORDINATION),
        tasks: Vec::new(),
        arrived: 0.0,
        backlog: VecDeque::new(),
        returned: vec![VecDeque::new(); m.workers.len()],
        handed: vec![VecDeque::new(); m.workers.len()],
        posts: vec![VecDeque::new(); m.channels.len()],
        posted: vec![0.0; m.channels.len()],
        trig: vec![TrigAcc::default(); m.triggers.len()],
        ticks: vec![0.0; m.schedules.len()],
        last_merge: m
            .workers
            .iter()
            .map(|w| vec![f64::NEG_INFINITY; if w.partition { w.exec.count } else { 0 }])
            .collect(),
        shipped: Vec::new(),
        duplicates: 0.0,
        copies,
        backlog_waits: Vec::new(),
        insts,
        kinds: vec![KindAcc::default(); m.workers.len()],
        reviewers: vec![ReviewerAcc::default(); m.reviewers.len()],
        gateways: vec![GatewayAcc::default(); m.gateways.len()],
        pools: vec![PoolAcc::default(); m.pools.len()],
        models: vec![ModelAcc::default(); m.models.len()],
        principals: vec![AnswerAcc::default(); m.principals.len()],
        delegates: vec![AnswerAcc::default(); m.delegates.len()],
        channels: vec![0.0; m.channels.len()],
        defaulted: 0.0,
        record,
        timeline: Vec::new(),
        ids: Ids::default(),
        corpus: Rng::stream(seed, CORPUS),
        runs_left: vec![VecDeque::new(); m.workers.len()],
        runs_any: VecDeque::new(),
        replies_left: vec![VecDeque::new(); m.principals.len()],
        builds_taken: 0,
        uses: Vec::new(),
    };
    if let Some(sc) = &m.script {
        for (f, (kind, _)) in sc.runs.iter().enumerate() {
            match kind {
                Some(k) => s.runs_left[*k].push_back(f),
                None => s.runs_any.push_back(f),
            }
        }
        for (r, (p, _)) in sc.replies.iter().enumerate() {
            s.replies_left[*p].push_back(r);
        }
        for (a, arrival) in sc.arrivals.iter().enumerate() {
            s.at(arrival.at, Ev::Scripted(a));
        }
    }
    let first = s.arrivals.exponential(m.load_per_second);
    s.at(first, Ev::Arrive);
    for &(at, n) in &m.bursts {
        s.at(at, Ev::Burst(n));
    }
    for (i, sc) in m.schedules.iter().enumerate() {
        s.at(sc.first, Ev::Tick(i));
    }
    starts.sort_by(f64::total_cmp);
    starts.dedup();
    for t in starts {
        s.at(t, Ev::Wake);
    }
    for (p, pool) in m.pools.iter().enumerate() {
        for &(from, until, _) in &pool.drops {
            s.rec(from, &pool.part, "capacity_dropped", None, None);
            s.rec(until, &pool.part, "capacity_restored", None, None);
            s.at(until, Ev::PoolCheck(p));
        }
    }
    for g in &m.gateways {
        for &(from, until, _) in &g.drops {
            s.rec(from, &g.part, "capacity_dropped", None, None);
            s.rec(until, &g.part, "capacity_restored", None, None);
        }
    }
    for p in &m.principals {
        for &(from, until) in &p.away {
            s.rec(from, &p.part, "principal_away", None, None);
            s.rec(until, &p.part, "principal_back", None, None);
        }
    }

    while let Some(Item { t, ev, .. }) = s.heap.pop() {
        match ev {
            Ev::Arrive => {
                s.arrive(t);
                let next = t + s.arrivals.exponential(m.load_per_second);
                s.at(next, Ev::Arrive);
                s.assign(t);
            }
            Ev::Burst(n) => {
                for _ in 0..n {
                    s.arrive(t);
                }
                s.assign(t);
            }
            Ev::Scripted(a) => {
                s.arrive_scripted(a, t);
                s.assign(t);
            }
            Ev::Tick(sc) => s.tick(sc, t),
            Ev::Wake => {
                s.assign(t);
                for r in 0..m.reviewers.len() {
                    s.dispatch_reviews(r, t);
                }
            }
            Ev::Begin(i) => {
                s.insts[i].hold = false;
                let id = s.insts[i].begin.take().expect("a read claims an entry");
                s.start_task(i, id, t);
            }
            Ev::EmptyDone(i) => {
                s.insts[i].hold = false;
                s.end_run(i, None, t);
                s.assign(t);
            }
            Ev::Fire(tr) => s.fire_on_event(tr, t),
            Ev::StepDone(i) => s.step_done(i, t),
            Ev::BuildDone(i) => {
                let Who::Worker(kind) = s.insts[i].who else {
                    unreachable!("only workers build")
                };
                let p = m.workers[kind].pool.expect("a build in a pool");
                s.pools[p].running -= 1;
                if s.insts[i].killed {
                    s.pools[p].killed += 1.0;
                    s.rec_task(
                        t,
                        &m.pools[p].part,
                        "build_killed",
                        s.insts[i].task,
                        Some(s.insts[i].claim),
                        None,
                    );
                } else {
                    s.rec_task(
                        t,
                        &m.pools[p].part,
                        "build_done",
                        s.insts[i].task,
                        Some(s.insts[i].claim),
                        None,
                    );
                }
                s.dispatch(p, t);
                s.step_done(i, t);
            }
            Ev::PoolCheck(p) => s.dispatch(p, t),
        }
    }
    for p in 0..m.pools.len() {
        let queue: Vec<(usize, f64)> = s.pools[p].queue.iter().copied().collect();
        for (j, queued) in queue {
            if let Who::Worker(kind) = s.insts[j].who {
                s.kinds[kind].slot_wait += span(queued, h, h);
            }
        }
    }
    // Work still waiting at the horizon counts with its wait so far.
    let waiting: Vec<usize> = s
        .backlog
        .iter()
        .chain(s.handed.iter().flatten())
        .chain(s.posts.iter().flatten())
        .chain(s.insts.iter().flat_map(|x| x.own.iter()))
        .copied()
        .filter(|id| !s.tasks[*id].waited)
        .collect();
    for id in waiting {
        s.backlog_waits.push(h - s.tasks[id].arrived);
    }
    for a in &mut s.trig {
        let left: Vec<f64> = a
            .waiting
            .iter()
            .chain(a.queued.iter())
            .map(|(fired, _, _)| h - fired)
            .collect();
        a.waits.extend(left);
    }
    for r in &mut s.reviewers {
        let left: Vec<f64> = r.queue.iter().map(|(_, q)| h - q).collect();
        r.waits.extend(left);
    }
    finish(s)
}

fn finish(s: Sim<'_>) -> RunOut {
    let m = s.m;
    let h = m.horizon;
    let mut per_part: BTreeMap<String, BTreeMap<String, f64>> = m
        .parts
        .iter()
        .map(|p| (p.clone(), BTreeMap::new()))
        .collect();
    let mut put = |part: &str, name: &str, v: f64| {
        per_part
            .entry(part.to_string())
            .or_default()
            .insert(name.to_string(), v);
    };
    let mut instances = 0.0;
    let mut sum = KindAcc::default();
    for (w, k) in m.workers.iter().zip(&s.kinds) {
        let part = &w.exec.part;
        let time = w.exec.count as f64 * h;
        instances += w.exec.count as f64;
        put(part, "tasks_started", k.started);
        put(part, "tasks_claimed_done", k.claimed);
        put(part, "tasks_done", k.done);
        put(part, "tasks_correct", k.correct);
        put(part, "false_completions_shipped", k.false_shipped);
        put(part, "turns", k.turns);
        put(part, "rework_turns", k.rework_turns);
        put(part, "steps", k.steps);
        put(part, "steps_failed", k.steps_failed);
        put(part, "steps_after_pass", k.after_pass);
        put(part, "steps_passed_after_pass", k.passed_after_pass);
        put(part, "steps_after_fail", k.after_fail);
        put(part, "steps_passed_after_fail", k.passed_after_fail);
        put(part, "wrong_assumptions", k.wrong_assumptions);
        put(part, "cost_usd", k.cost);
        put(part, "questions_asked", k.asked);
        put(part, "owner_wait_share", ratio(k.owner_wait, time));
        put(part, "operator_wait_share", ratio(k.operator_wait, time));
        put(part, "slot_wait_share", ratio(k.slot_wait, time));
        put(part, "busy_share", ratio(k.busy, time));
        put(part, "rate_limited_seconds", k.rate_wait);
        if w.runtime {
            put(part, "run_steps", k.turns);
            put(part, "model_calls", k.model_calls);
            put(part, "person_steps", k.asked);
        }
        if w.coordination != Coordination::None || w.partition || w.orchestrates {
            put(part, "claim_conflicts", k.claim_conflicts);
            put(part, "coordination_calls", k.coordination_calls);
            put(part, "coordination_messages", k.coordination_messages);
            put(part, "merge_conflicts", k.merge_conflicts);
        }
        sum.claim_conflicts += k.claim_conflicts;
        sum.coordination_calls += k.coordination_calls;
        sum.coordination_messages += k.coordination_messages;
        sum.merge_conflicts += k.merge_conflicts;
        sum.started += k.started;
        sum.claimed += k.claimed;
        sum.done += k.done;
        sum.correct += k.correct;
        sum.false_shipped += k.false_shipped;
        sum.turns += k.turns;
        sum.rework_turns += k.rework_turns;
        sum.steps += k.steps;
        sum.steps_failed += k.steps_failed;
        sum.wrong_assumptions += k.wrong_assumptions;
        sum.cost += k.cost;
        sum.asked += k.asked;
        sum.owner_wait += k.owner_wait;
        sum.operator_wait += k.operator_wait;
        sum.slot_wait += k.slot_wait;
        sum.busy += k.busy;
        sum.rate_wait += k.rate_wait;
    }
    let (mut reviews, mut rejected, mut review_calls, mut review_cost) = (0.0, 0.0, 0.0, 0.0);
    for (r, a) in m.reviewers.iter().zip(&s.reviewers) {
        let part = &r.exec.part;
        let mut waits = a.waits.clone();
        waits.sort_by(f64::total_cmp);
        put(part, "reviews", a.reviews);
        put(part, "reviews_rejected", a.rejected);
        put(part, "review_calls", a.calls);
        put(part, "cost_usd", a.cost);
        put(part, "busy_share", ratio(a.busy, r.exec.count as f64 * h));
        put(part, "rate_limited_seconds", a.rate_wait);
        put(part, "queue_wait_p50_seconds", percentile(&waits, 0.5));
        put(part, "queue_wait_p90_seconds", percentile(&waits, 0.9));
        reviews += a.reviews;
        rejected += a.rejected;
        review_calls += a.calls;
        review_cost += a.cost;
        sum.rate_wait += a.rate_wait;
    }
    let mut agent_time_cost = 0.0;
    for (part, count, stagger) in &m.alive {
        let seconds: f64 = (0..*count)
            .map(|copy| (h - copy as f64 * stagger).max(0.0))
            .sum();
        let c = seconds / 3600.0 * m.agent_hour_usd;
        put(part, "agent_time_cost_usd", c);
        agent_time_cost += c;
    }
    for (g, a) in m.gateways.iter().zip(&s.gateways) {
        put(&g.part, "calls", a.calls);
        put(&g.part, "calls_limited", a.limited_calls);
        put(&g.part, "rate_limited_seconds", a.limited_seconds);
    }
    let mut tokens = [0.0; 4];
    for (name, a) in m.models.iter().zip(&s.models) {
        put(name, "calls", a.calls);
        put(name, "cost_usd", a.cost);
        for (c, class) in TOKEN_CLASSES.iter().enumerate() {
            put(name, &format!("tokens_{class}"), a.tokens[c]);
            tokens[c] += a.tokens[c];
        }
    }
    let mut all_waits = Vec::new();
    let (mut jobs, mut killed, mut busy, mut capacity) = (0.0, 0.0, 0.0, 0.0);
    for (p, a) in m.pools.iter().zip(&s.pools) {
        let mut waits = a.waits.clone();
        waits.sort_by(f64::total_cmp);
        let cap = slot_seconds(p, h);
        put(&p.part, "jobs", a.jobs);
        put(&p.part, "jobs_killed", a.killed);
        put(&p.part, "queue_wait_p50_seconds", percentile(&waits, 0.5));
        put(&p.part, "queue_wait_p90_seconds", percentile(&waits, 0.9));
        put(&p.part, "utilization", ratio(a.busy, cap));
        all_waits.extend(waits);
        jobs += a.jobs;
        killed += a.killed;
        busy += a.busy;
        capacity += cap;
    }
    all_waits.sort_by(f64::total_cmp);
    let mut decided = 0.0;
    let mut forwarded = 0.0;
    for (p, a) in m.principals.iter().zip(&s.principals) {
        put(&p.part, "questions_received", a.received);
        put(&p.part, "questions_answered", a.answered);
        put(&p.part, "away_seconds", away_seconds(&p.away, h));
    }
    for (d, a) in m.delegates.iter().zip(&s.delegates) {
        put(&d.part, "questions_received", a.received);
        put(&d.part, "questions_answered", a.answered);
        put(&d.part, "questions_forwarded", a.forwarded);
        put(&d.part, "operator_decided_owner_questions", a.decided_owner);
        decided += a.decided_owner;
        forwarded += a.forwarded;
    }
    for (c, n) in m.channels.iter().zip(&s.channels) {
        put(c, "messages", *n);
    }
    for (i, c) in m.channels.iter().enumerate() {
        if m.distribution[i] != Distribution::Direct {
            put(c, "posts", s.posted[i]);
        }
    }
    for (sc, n) in m.schedules.iter().zip(&s.ticks) {
        put(&sc.part, "ticks", *n);
    }
    let mut f = TrigAcc::default();
    for (tr, a) in m.triggers.iter().zip(&s.trig) {
        let mut waits = a.waits.clone();
        waits.sort_by(f64::total_cmp);
        put(&tr.part, "firings", a.fired);
        put(&tr.part, "firings_started", a.started);
        put(&tr.part, "firings_skipped", a.skipped);
        put(&tr.part, "firings_queued", a.queued_n);
        put(&tr.part, "firings_empty", a.empty);
        put(&tr.part, "firings_failed", a.failed);
        put(&tr.part, "firing_skip_share", ratio(a.skipped, a.fired));
        put(
            &tr.part,
            "firing_start_wait_p90_seconds",
            percentile(&waits, 0.9),
        );
        put(&tr.part, "firing_failure_share", ratio(a.failed, a.started));
        f.fired += a.fired;
        f.started += a.started;
        f.skipped += a.skipped;
        f.queued_n += a.queued_n;
        f.empty += a.empty;
        f.failed += a.failed;
        f.waits.extend(waits);
    }
    f.waits.sort_by(f64::total_cmp);
    let posts: f64 = s.posted.iter().sum();
    let mut backlog_waits = s.backlog_waits;
    backlog_waits.sort_by(f64::total_cmp);
    let arrived = s.arrived;
    let time = instances * h;
    let cost = sum.cost + review_cost + agent_time_cost;
    let mut k = BTreeMap::new();
    let mut set = |name: &str, v: f64| {
        k.insert(name.to_string(), if v.is_finite() { v } else { 0.0 });
    };
    set("tasks_arrived", arrived);
    set("tasks_done", sum.done);
    set("tasks_correct", sum.correct);
    set("tasks_claimed_done", sum.claimed);
    set("false_completions_shipped", sum.false_shipped);
    set(
        "task_completion",
        if arrived > 0.0 {
            sum.done / arrived
        } else {
            1.0
        },
    );
    set("backlog_wait_p50_seconds", percentile(&backlog_waits, 0.5));
    set("backlog_wait_p90_seconds", percentile(&backlog_waits, 0.9));
    set("turns", sum.turns);
    set("rework_turns", sum.rework_turns);
    set("steps", sum.steps);
    set("steps_failed", sum.steps_failed);
    set("wrong_assumptions", sum.wrong_assumptions);
    set("reviews", reviews);
    set("reviews_rejected", rejected);
    set("review_calls", review_calls);
    set("questions_asked", sum.asked);
    set("questions_forwarded", forwarded);
    set("questions_defaulted", s.defaulted);
    set("operator_decided_owner_questions", decided);
    set("owner_wait_share", ratio(sum.owner_wait, time));
    set("operator_wait_share", ratio(sum.operator_wait, time));
    set("slot_wait_share", ratio(sum.slot_wait, time));
    set("worker_busy_share", ratio(sum.busy, time));
    set("queue_wait_p50_seconds", percentile(&all_waits, 0.5));
    set("queue_wait_p90_seconds", percentile(&all_waits, 0.9));
    set("jobs", jobs);
    set("jobs_killed", killed);
    set("pool_utilization", ratio(busy, capacity));
    set("rate_limited_seconds", sum.rate_wait);
    set("agent_time_cost_usd", agent_time_cost);
    set("cost_usd", cost);
    set("cost_per_task_usd", ratio(cost, sum.done));
    for (c, class) in TOKEN_CLASSES.iter().enumerate() {
        set(&format!("tokens_{class}"), tokens[c]);
    }
    set("tasks_started", sum.started);
    set("firings", f.fired);
    set("firings_started", f.started);
    set("firings_skipped", f.skipped);
    set("firings_queued", f.queued_n);
    set("firings_empty", f.empty);
    set("firing_skip_share", ratio(f.skipped, f.fired));
    set("firing_start_wait_p90_seconds", percentile(&f.waits, 0.9));
    set("firing_failure_share", ratio(f.failed, f.started));
    set("posts", posts);
    set("claim_conflicts", sum.claim_conflicts);
    set("coordination_calls", sum.coordination_calls);
    set("coordination_messages", sum.coordination_messages);
    set("merge_conflicts", sum.merge_conflicts);
    set("duplicate_runs", s.duplicates);
    let mut timeline = s.timeline;
    timeline.sort_by(|a, b| a.t.total_cmp(&b.t));
    let tasks = if s.record && m.script.is_some() {
        s.tasks
            .iter()
            .map(|t| TaskRecord {
                arrival: t.arrival,
                fixture: t.fixture,
                correct: t.correct,
            })
            .collect()
    } else {
        Vec::new()
    };
    RunOut {
        kpis: k,
        per_part,
        timeline,
        tasks,
        uses: s.uses,
    }
}

/// The global KPIs a run reports, without the gate KPIs of the check.
pub const RUN_KPIS: &[&str] = &[
    "tasks_arrived",
    "tasks_done",
    "tasks_correct",
    "tasks_claimed_done",
    "false_completions_shipped",
    "task_completion",
    "backlog_wait_p50_seconds",
    "backlog_wait_p90_seconds",
    "turns",
    "rework_turns",
    "steps",
    "steps_failed",
    "wrong_assumptions",
    "reviews",
    "reviews_rejected",
    "review_calls",
    "questions_asked",
    "questions_forwarded",
    "questions_defaulted",
    "operator_decided_owner_questions",
    "owner_wait_share",
    "operator_wait_share",
    "slot_wait_share",
    "worker_busy_share",
    "queue_wait_p50_seconds",
    "queue_wait_p90_seconds",
    "jobs",
    "jobs_killed",
    "pool_utilization",
    "rate_limited_seconds",
    "agent_time_cost_usd",
    "cost_usd",
    "cost_per_task_usd",
    "tokens_input",
    "tokens_cache_read",
    "tokens_cache_write",
    "tokens_output",
    "tasks_started",
    "firings",
    "firings_started",
    "firings_skipped",
    "firings_queued",
    "firings_empty",
    "firing_skip_share",
    "firing_start_wait_p90_seconds",
    "firing_failure_share",
    "posts",
    "claim_conflicts",
    "coordination_calls",
    "coordination_messages",
    "merge_conflicts",
    "duplicate_runs",
];

/// Slot-seconds a pool offers over the horizon, its drops included.
fn slot_seconds(p: &Pool, h: f64) -> f64 {
    if p.capacity.is_infinite() {
        return 0.0;
    }
    let mut cuts = vec![0.0, h];
    for &(a, b, _) in &p.drops {
        cuts.extend([a.clamp(0.0, h), b.clamp(0.0, h)]);
    }
    cuts.sort_by(f64::total_cmp);
    cuts.windows(2)
        .map(|w| (w[1] - w[0]) * level_at(p.capacity, &p.drops, w[0]).max(0.0))
        .sum()
}

fn away_seconds(windows: &[(f64, f64)], h: f64) -> f64 {
    let mut total = 0.0;
    let mut end = 0.0f64;
    for &(a, b) in windows {
        let a = a.max(end);
        if b > a {
            total += span(a, b, h);
            end = b;
        }
    }
    total
}
