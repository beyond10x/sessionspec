//! The packing of an architecture: which parts share a unit, one thing a target starts and
//! watches. The `packing:` section of a binding names a strategy (`single`, the default when the
//! section is absent, `by_kind`, `by_part` or `groups`) with per-part overrides, or lists the
//! units themselves (`units:`, the resolved form [`Packing::text`] prints). Any part may be
//! external, with the `reach` the target finds it by. [`resolve`] turns either into the units;
//! admission (`crate::deploy::admit`) then checks them.

use crate::check::value;
use crate::contract::derive::{flow, scalar};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// The strategies of the `packing:` section; the first is the default.
pub const STRATEGIES: &[&str] = &["single", "by_kind", "by_part", "groups"];

/// The kinds of unit. `external` parts are not started and are listed beside the units.
pub const UNIT_KINDS: &[&str] = &[
    "core",
    "agent",
    "service",
    "job",
    "scheduled",
    "store",
    "model",
];

/// One unit: its name, kind, the parts it holds by path, and how many copies run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub name: String,
    pub kind: String,
    pub parts: Vec<String>,
    /// The copies that run; `None` for a `job` or `scheduled` unit, which runs on demand.
    pub replicas: Option<u64>,
    /// The cron rule that starts a `scheduled` unit, when the binding gives one
    /// (`packing.parts.<p>.schedule: <cron>`, or `schedule:` of a listed unit). The bundle's
    /// unit carries this one, else the one its clock's `every` states ([`schedule_of`]).
    pub schedule: Option<String>,
}

/// A cron rule: five fields separated by spaces (minute, hour, day of month, month, day of
/// week), each of digits, letters, `*`, `/`, `,` and `-`.
pub fn is_cron(s: &str) -> bool {
    let fields: Vec<&str> = s.split_whitespace().collect();
    fields.len() == 5
        && fields.iter().all(|f| {
            f.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '*' | '/' | ',' | '-'))
        })
}

/// The cron rule that states an interval of `seconds` exactly: a number of minutes that divides
/// 60 (`*/N * * * *`), a number of hours that divides 24 (`0 */N * * *`), one day (`0 0 * * *`)
/// or one week (`0 0 * * 0`). `None` for any other interval (`90m`, `7h`, `45s`).
pub fn cron_of(seconds: f64) -> Option<String> {
    if seconds <= 0.0 || seconds.fract() != 0.0 {
        return None;
    }
    let s = seconds as u64;
    if !s.is_multiple_of(60) {
        return None;
    }
    let m = s / 60;
    if m < 60 && 60_u64.is_multiple_of(m) {
        return Some(format!("*/{m} * * * *"));
    }
    if !m.is_multiple_of(60) {
        return None;
    }
    match m / 60 {
        24 => Some("0 0 * * *".into()),
        168 => Some("0 0 * * 0".into()),
        h if h < 24 && 24_u64.is_multiple_of(h) => Some(format!("0 */{h} * * *")),
        _ => None,
    }
}

/// The link that wakes one of `parts` from a clock: the first `trigger` link into one of them
/// from a schedule part or an `on_schedule` trigger.
pub fn clock_link<'a>(arch: &'a Value, parts: &[String]) -> Option<&'a Value> {
    arch["links"].as_array()?.iter().find(|l| {
        text(&l["carries"]) == "trigger"
            && parts.iter().any(|p| p == text(&l["to"]))
            && is_clock(arch, text(&l["from"]))
    })
}

/// The schedule part a clock part reads its interval from: the part itself, or the `schedule`
/// an `on_schedule` trigger names.
pub fn schedule_part<'a>(arch: &'a Value, clock: &'a str) -> &'a str {
    let p = &arch["parts"][clock];
    if p["block"] == "trigger" {
        value(&p["schedule"]).as_str().unwrap_or("")
    } else {
        clock
    }
}

/// The cron rule a scheduled unit carries: the binding's, else the one that states the `every`
/// of the schedule that wakes it exactly ([`cron_of`]); `None` without either. Refused, with
/// what `schedule_not_cron` says, when that `every` is no exact cron rule.
pub fn schedule_of(arch: &Value, unit: &Unit) -> Result<Option<String>, String> {
    if unit.schedule.is_some() || unit.kind != "scheduled" {
        return Ok(unit.schedule.clone());
    }
    let Some(l) = clock_link(arch, &unit.parts) else {
        return Ok(None);
    };
    let (to, from) = (text(&l["to"]), text(&l["from"]));
    let sched = schedule_part(arch, from);
    let every = value(&arch["parts"][sched]["every"]);
    if every.is_null() {
        return Ok(None);
    }
    let secs = crate::doc::num(every);
    if let Some(cron) = secs.and_then(cron_of) {
        return Ok(Some(cron));
    }
    let interval = match (every.as_str(), secs) {
        (Some(t), _) => t.to_owned(),
        (None, Some(s)) => crate::doc::format_duration(s),
        (None, None) => every.to_string(),
    };
    let by = if from == sched {
        format!("schedule {sched}")
    } else {
        format!("trigger {from} of schedule {sched}")
    };
    Err(format!(
        "unit {}: part {to} is woken every {interval} (by {by}), and a cron rule states only N minutes dividing 60, N hours dividing 24, one day or one week (give it one with `packing.parts.{to}.schedule: <cron>`)",
        unit.name
    ))
}

/// A resolved packing: the units, then the parts no target starts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packing {
    /// The strategy it was resolved from, `units` when the section listed the units.
    pub strategy: String,
    pub units: Vec<Unit>,
    pub external: Vec<String>,
    /// Where the target reaches an external part, per part whose binding names it: `{address}` or
    /// `{name}`. A part without one is reached by its own name.
    pub reach: BTreeMap<String, Value>,
}

fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

/// The built value of a part's `count`, at least 0, 1 when absent.
pub fn count(part: &Value) -> u64 {
    value(&part["count"])
        .as_f64()
        .map_or(1, |c| c.max(0.0) as u64)
}

/// Whether part `name` is woken by a clock: a `trigger` link into it from a schedule part or
/// from a trigger part of kind `on_schedule`.
pub fn woken_by_schedule(arch: &Value, name: &str) -> bool {
    arch["links"].as_array().into_iter().flatten().any(|l| {
        text(&l["to"]) == name
            && text(&l["carries"]) == "trigger"
            && is_clock(arch, text(&l["from"]))
    })
}

/// A schedule part, or a trigger part of kind `on_schedule`.
pub fn is_clock(arch: &Value, name: &str) -> bool {
    let p = &arch["parts"][name];
    match text(&p["block"]) {
        "schedule" => true,
        "trigger" => value(&p["kind"]).as_str() == Some("on_schedule"),
        _ => false,
    }
}

/// The part a `per` chain ends at: the part itself when it names no `per`.
fn owner<'a>(parts: &'a Map<String, Value>, name: &'a str) -> &'a str {
    let mut seen = BTreeSet::new();
    let mut at = name;
    while let Some(next) = parts.get(at).and_then(|p| p["per"].as_str()) {
        if !seen.insert(at) || !parts.contains_key(next) {
            break;
        }
        at = next;
    }
    at
}

/// Where a part goes before any strategy: outside, into the core, or a unit of its own kind.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Place {
    External,
    Core,
    Own(&'static str),
}

/// The per-part overrides of the section, `parts: { <name>: { external, schedule, shape } }`.
struct Overrides<'a>(&'a Value);

impl Overrides<'_> {
    fn external(&self, name: &str) -> Option<bool> {
        self.0[name]["external"].as_bool()
    }
    /// The cron rule the binding gives the part (`schedule: <cron>`).
    fn cron(&self, name: &str) -> Option<&str> {
        self.0[name]["schedule"].as_str().filter(|s| is_cron(s))
    }
    /// Whether the part runs until done: `schedule: job`, `shape: job` or a cron rule.
    fn job(&self, name: &str) -> bool {
        self.0[name]["schedule"] == "job"
            || self.0[name]["shape"] == "job"
            || self.cron(name).is_some()
    }
}

fn place(arch: &Value, name: &str, part: &Value, o: &Overrides) -> Place {
    if o.external(name) == Some(true) {
        return Place::External;
    }
    match text(&part["block"]) {
        "principal" => Place::External,
        "model" if o.external(name) == Some(false) => Place::Own("model"),
        "model" => Place::External,
        "agent" if o.job(name) => {
            if o.cron(name).is_some() || woken_by_schedule(arch, name) {
                Place::Own("scheduled")
            } else {
                Place::Own("job")
            }
        }
        "agent" => Place::Own("agent"),
        "gateway" | "pool" | "tool" | "runtime" => Place::Own("service"),
        "workspace" if part["per"].is_null() => Place::Own("store"),
        _ => Place::Core,
    }
}

/// Whether a unit of this kind runs as copies (`replicas`) rather than on demand.
fn replicated(kind: &str) -> bool {
    !matches!(kind, "job" | "scheduled")
}

/// Resolve the `packing:` section of a binding (`Null` when absent) for one architecture
/// document. Refused, with every problem of the section's form, when it names an unknown key,
/// strategy, kind or part, or two units of one name. A part placed twice or nowhere is not a
/// problem of the form: admission names it.
pub fn resolve(arch: &Value, section: &Value) -> Result<Packing, Vec<String>> {
    let empty = Map::new();
    let parts = arch["parts"].as_object().unwrap_or(&empty);
    let mut problems = Vec::new();
    let section = match section {
        Value::Null => &Value::Object(Map::new()),
        Value::Object(_) => section,
        _ => return Err(vec!["packing: is a mapping".into()]),
    };
    for k in section.as_object().into_iter().flatten().map(|(k, _)| k) {
        if !["strategy", "groups", "parts", "units", "external"].contains(&k.as_str()) {
            problems.push(format!("packing: unknown key `{k}`"));
        }
    }
    let known = |name: &str, at: &str, problems: &mut Vec<String>| {
        if !parts.contains_key(name) {
            problems.push(format!(
                "packing: {at}: `{name}` is not a part of the document"
            ));
        }
    };
    if !section["units"].is_null() {
        for k in ["strategy", "groups", "parts"] {
            if !section[k].is_null() {
                problems.push(format!(
                    "packing: `{k}:` and `units:` do not go together (`units:` lists every unit)"
                ));
            }
        }
        let units = explicit(section, &known, &mut problems);
        let mut external = Vec::new();
        let mut reach = BTreeMap::new();
        match &section["external"] {
            Value::Null => {}
            Value::Array(list) => {
                for (i, e) in list.iter().enumerate() {
                    let Some(part) = external_entry(e, i, parts, &mut reach, &mut problems) else {
                        continue;
                    };
                    known(&part, "external", &mut problems);
                    external.push(part);
                }
            }
            _ => problems.push("packing: `external:` is a list".into()),
        }
        check_names(&units, &mut problems);
        if !problems.is_empty() {
            return Err(problems);
        }
        return Ok(Packing {
            strategy: "units".into(),
            units,
            external,
            reach,
        });
    }
    if !section["external"].is_null() {
        problems.push(
            "packing: `external:` goes with `units:` (with a strategy, mark a part `external: true` under `parts:`)"
                .into(),
        );
    }
    let strategy = match &section["strategy"] {
        Value::Null => STRATEGIES[0],
        Value::String(s) if STRATEGIES.contains(&s.as_str()) => STRATEGIES
            .iter()
            .find(|x| **x == s)
            .copied()
            .unwrap_or("single"),
        other => {
            problems.push(format!(
                "packing: strategy `{}` is not one of {}",
                text(other),
                STRATEGIES.join(", ")
            ));
            "single"
        }
    };
    let overrides = &section["parts"];
    let mut reach = BTreeMap::new();
    if let Some(o) = overrides.as_object() {
        for (name, entry) in o {
            known(name, "parts", &mut problems);
            for (k, v) in entry.as_object().into_iter().flatten() {
                if k == "reach" {
                    match reach_problem(v) {
                        Some(p) => problems.push(format!("packing: parts: {name}: {p}")),
                        None => {
                            reach.insert(name.clone(), v.clone());
                        }
                    }
                    continue;
                }
                let ok = match k.as_str() {
                    "external" => v.is_boolean(),
                    "schedule" => v == "job" || v.as_str().is_some_and(is_cron),
                    "shape" => v == "job",
                    _ => false,
                };
                if !ok {
                    problems.push(format!(
                        "packing: parts: {name}: `{k}` is not `external: true|false`, `schedule: job|<cron>`, `shape: job` or `reach: {{address|name}}`"
                    ));
                }
            }
            if parts.get(name).is_some_and(|p| p["block"] != "agent") {
                for k in ["schedule", "shape"] {
                    if let Some(v) = entry[k].as_str() {
                        problems.push(format!(
                            "packing: parts: {name}: `{k}: {v}` is for an agent part"
                        ));
                    }
                }
            }
        }
    } else if !overrides.is_null() {
        problems.push("packing: `parts:` is a mapping".into());
    }
    let groups: Vec<(String, Vec<String>)> = match &section["groups"] {
        Value::Null => Vec::new(),
        Value::Object(g) => g.iter().map(|(k, v)| (k.clone(), names(v))).collect(),
        _ => {
            problems.push("packing: `groups:` is a mapping from a unit name to its parts".into());
            Vec::new()
        }
    };
    if strategy != "groups" && !groups.is_empty() {
        problems.push("packing: `groups:` goes with `strategy: groups`".into());
    }
    for (g, list) in &groups {
        for p in list {
            known(p, &format!("groups: {g}"), &mut problems);
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let o = Overrides(overrides);
    let units = by_strategy(arch, parts, strategy, &groups, &o);
    let mut external: Vec<String> = parts
        .iter()
        .filter(|(name, part)| {
            let root = owner(parts, name);
            place(arch, root, &parts[root], &o) == Place::External
                || place(arch, name, part, &o) == Place::External
        })
        .map(|(n, _)| n.clone())
        .collect();
    external.sort();
    for name in reach.keys() {
        if !external.contains(name) {
            problems.push(format!(
                "packing: parts: {name}: `reach` is for an external part, and {name} is in a unit"
            ));
        }
    }
    check_names(&units, &mut problems);
    if !problems.is_empty() {
        return Err(problems);
    }
    Ok(Packing {
        strategy: strategy.into(),
        units,
        external,
        reach,
    })
}

/// Why a `reach` is of the wrong form, or `None`: `{address: <text>}` or `{name: <text>}`, one
/// of the two.
fn reach_problem(v: &Value) -> Option<String> {
    let ok = v.as_object().is_some_and(|m| {
        m.len() == 1
            && m.iter().all(|(k, x)| {
                matches!(k.as_str(), "address" | "name")
                    && x.as_str().is_some_and(super::binding::is_text)
            })
    });
    (!ok).then(|| "`reach` is {address: <text>} or {name: <text>}".to_owned())
}

/// One entry of `external:` under `units:`: a part's name, or `{part, reach}` (the resolved
/// form also writes the part's `kind`, its block, which must then be the part's). The part, or
/// `None` with the problem pushed.
fn external_entry(
    e: &Value,
    i: usize,
    parts: &Map<String, Value>,
    reach: &mut BTreeMap<String, Value>,
    problems: &mut Vec<String>,
) -> Option<String> {
    match e {
        Value::String(p) => Some(p.clone()),
        Value::Object(m) => {
            let Some(part) = m.get("part").and_then(Value::as_str) else {
                problems.push(format!(
                    "packing: external[{i}] is a part's name or {{part, reach}}"
                ));
                return None;
            };
            for (k, x) in m {
                match k.as_str() {
                    "part" => {}
                    "reach" => match reach_problem(x) {
                        Some(p) => problems.push(format!("packing: external: {part}: {p}")),
                        None => {
                            reach.insert(part.to_owned(), x.clone());
                        }
                    },
                    "kind" => {
                        if parts.get(part).is_some_and(|p| &p["block"] != x) {
                            problems.push(format!(
                                "packing: external: {part}: kind is the part's block, `{}`",
                                text(&parts[part]["block"])
                            ));
                        }
                    }
                    other => {
                        problems.push(format!("packing: external: {part}: unknown key `{other}`"))
                    }
                }
            }
            Some(part.to_owned())
        }
        _ => {
            problems.push(format!(
                "packing: external[{i}] is a part's name or {{part, reach}}"
            ));
            None
        }
    }
}

fn names(v: &Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(|x| x.as_str().map(str::to_owned))
        .collect()
}

fn check_names(units: &[Unit], problems: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for u in units {
        if !seen.insert(u.name.as_str()) {
            problems.push(format!("packing: two units are named `{}`", u.name));
        }
    }
}

/// The units listed under `units:`. A listed unit's `image:`, `env:`, `args:`, `secret_files:` and
/// `scratch:` are the binding's, checked and read by [`crate::deploy::binding`],
/// [`crate::deploy::config`] and [`crate::deploy::scratch`]; its `image:` is read through
/// [`crate::deploy::binding::unit_image`].
fn explicit(
    section: &Value,
    known: &dyn Fn(&str, &str, &mut Vec<String>),
    problems: &mut Vec<String>,
) -> Vec<Unit> {
    let Some(list) = section["units"].as_array() else {
        problems.push("packing: `units:` is a list".into());
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, u) in list.iter().enumerate() {
        let Some(m) = u.as_object() else {
            problems.push(format!("packing: units[{i}] is a mapping"));
            continue;
        };
        for k in m.keys() {
            if ![
                "name",
                "kind",
                "parts",
                "replicas",
                "schedule",
                "image",
                "env",
                "args",
                "scratch",
                "secret_files",
            ]
            .contains(&k.as_str())
            {
                problems.push(format!("packing: units[{i}]: unknown key `{k}`"));
            }
        }
        let name = text(&u["name"]).to_owned();
        if name.is_empty() {
            problems.push(format!("packing: units[{i}] needs a `name:`"));
        }
        let kind = text(&u["kind"]).to_owned();
        if !UNIT_KINDS.contains(&kind.as_str()) {
            problems.push(format!(
                "packing: unit `{name}`: kind `{kind}` is not one of {}",
                UNIT_KINDS.join(", ")
            ));
        }
        let parts = names(&u["parts"]);
        for p in &parts {
            known(p, &format!("unit {name}"), problems);
        }
        let replicas = match &u["replicas"] {
            Value::Null => replicated(&kind).then_some(1),
            v => match v.as_u64() {
                Some(n) if n > 0 && replicated(&kind) => Some(n),
                Some(_) if !replicated(&kind) => {
                    problems.push(format!(
                        "packing: unit `{name}`: a {kind} unit runs on demand and has no replicas"
                    ));
                    None
                }
                _ => {
                    problems.push(format!(
                        "packing: unit `{name}`: replicas is a whole number above 0"
                    ));
                    None
                }
            },
        };
        let schedule = match &u["schedule"] {
            Value::Null => None,
            Value::String(s) if is_cron(s) && kind == "scheduled" => Some(s.clone()),
            _ => {
                problems.push(format!(
                    "packing: unit `{name}`: schedule is a cron rule of five fields, on a scheduled unit"
                ));
                None
            }
        };
        out.push(Unit {
            name,
            kind,
            parts,
            replicas,
            schedule,
        });
    }
    out
}

/// The units of a strategy. The core comes first and always exists; the other units follow in
/// name order, each with its parts in name order.
fn by_strategy(
    arch: &Value,
    parts: &Map<String, Value>,
    strategy: &str,
    groups: &[(String, Vec<String>)],
    o: &Overrides,
) -> Vec<Unit> {
    let mut units: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    let mut core: Vec<String> = Vec::new();
    let listed: BTreeMap<&str, Vec<&str>> = {
        let mut m: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for (g, list) in groups {
            for p in list {
                m.entry(p.as_str()).or_default().push(g.as_str());
            }
        }
        m
    };
    let mut put = |unit: &str, kind: &str, part: &str, core: &mut Vec<String>| {
        if unit == "core" && kind == "core" {
            core.push(part.to_owned());
        } else {
            let e = units
                .entry(unit.to_owned())
                .or_insert_with(|| (kind.to_owned(), Vec::new()));
            e.1.push(part.to_owned());
        }
    };
    for (name, part) in parts {
        let mut placed = false;
        if strategy == "groups"
            && let Some(gs) = listed.get(name.as_str())
        {
            for g in gs {
                put(g, "", name, &mut core);
            }
            placed = true;
        }
        let root = owner(parts, name);
        let rp = place(arch, root, &parts[root], o);
        let own = place(arch, name, part, o);
        // A part listed in a group and outside too is in both: admission names it packed twice.
        if placed || own == Place::External || rp == Place::External {
            continue;
        }
        // An unlisted `per` part follows the part its chain ends at.
        let (target, kind) = if root != name {
            if strategy == "groups"
                && let Some(gs) = listed.get(root)
            {
                for g in gs {
                    put(g, "", name, &mut core);
                }
                continue;
            }
            (root, rp)
        } else {
            (name.as_str(), own)
        };
        match kind {
            Place::External => {}
            Place::Core => put("core", "core", name, &mut core),
            Place::Own(k) => {
                let unit = match (strategy, k) {
                    (_, "job" | "scheduled") => target.to_owned(),
                    ("single" | "groups", _) => "core".to_owned(),
                    ("by_kind", _) => text(&parts[target]["block"]).to_owned(),
                    _ => target.to_owned(),
                };
                if unit == "core" {
                    put("core", "core", name, &mut core);
                } else {
                    put(&unit, k, name, &mut core);
                }
            }
        }
    }
    let mut out = vec![Unit {
        name: "core".into(),
        kind: "core".into(),
        parts: sorted(core),
        replicas: Some(1),
        schedule: None,
    }];
    for (name, (kind, list)) in units {
        let list = sorted(list);
        let kind = if kind.is_empty() {
            group_kind(arch, parts, &list, o)
        } else {
            kind
        };
        let roots: Vec<&String> = list.iter().filter(|p| parts[*p]["per"].is_null()).collect();
        let replicas = replicated(&kind).then(|| match roots.as_slice() {
            [one] => count(&parts[*one]).max(1),
            _ => 1,
        });
        let schedule = (kind == "scheduled")
            .then(|| list.iter().find_map(|p| o.cron(p)).map(str::to_owned))
            .flatten();
        out.push(Unit {
            name,
            kind,
            parts: list,
            replicas,
            schedule,
        });
    }
    out
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}

/// The kind of a named group: the kind of its one `job` or `scheduled` agent, else `agent` when
/// it holds an agent, `model` when it holds a served model, `store` when it holds only stores,
/// `service` otherwise.
fn group_kind(arch: &Value, parts: &Map<String, Value>, list: &[String], o: &Overrides) -> String {
    let places: Vec<Place> = list
        .iter()
        .filter(|p| parts[*p]["per"].is_null())
        .map(|p| place(arch, p, &parts[p], o))
        .collect();
    let has = |k: &'static str| places.contains(&Place::Own(k));
    let kind = if places.len() == 1 && (has("job") || has("scheduled")) {
        if has("job") { "job" } else { "scheduled" }
    } else if has("agent") || has("job") || has("scheduled") {
        "agent"
    } else if has("model") {
        "model"
    } else if !places.is_empty() && places.iter().all(|p| *p == Place::Own("store")) {
        "store"
    } else {
        "service"
    };
    kind.into()
}

impl Packing {
    /// Where the target reaches an external part: the binding's `reach`, else `{name: <part>}`.
    pub fn reach_of(&self, part: &str) -> Value {
        self.reach
            .get(part)
            .cloned()
            .unwrap_or_else(|| json!({ "name": part }))
    }

    /// The packing as data: `units` (name, kind, parts, replicas when it runs as copies) and
    /// `external`, each `{part, kind, reach}` with the part's block as its kind. Its digest is
    /// the packing digest of a bundle.
    pub fn to_value(&self, arch: &Value) -> Value {
        let units: Vec<Value> = self
            .units
            .iter()
            .map(|u| {
                let mut m = json!({ "name": u.name, "kind": u.kind, "parts": u.parts });
                if let Some(r) = u.replicas {
                    m["replicas"] = json!(r);
                }
                if let Some(s) = &u.schedule {
                    m["schedule"] = json!(s);
                }
                m
            })
            .collect();
        let external: Vec<Value> = self
            .external
            .iter()
            .map(|p| {
                json!({
                    "part": p,
                    "kind": text(&arch["parts"][p]["block"]),
                    "reach": self.reach_of(p),
                })
            })
            .collect();
        json!({ "units": units, "external": external })
    }

    /// The `packing:` section in its resolved form, to paste into a binding and change: the
    /// same bytes for the same packing.
    pub fn text(&self) -> String {
        let list = |v: &[String]| flow(&json!(v));
        let mut s = String::from("packing:\n  units:\n");
        for u in &self.units {
            s.push_str(&format!("    - name: {}\n", scalar(&u.name)));
            s.push_str(&format!("      kind: {}\n", scalar(&u.kind)));
            s.push_str(&format!("      parts: {}\n", list(&u.parts)));
            if let Some(r) = u.replicas {
                s.push_str(&format!("      replicas: {r}\n"));
            }
            if let Some(c) = &u.schedule {
                s.push_str(&format!("      schedule: {}\n", scalar(c)));
            }
        }
        let external: Vec<Value> = self
            .external
            .iter()
            .map(|p| match self.reach.get(p) {
                Some(r) => json!({ "part": p, "reach": r }),
                None => json!(p),
            })
            .collect();
        s.push_str(&format!("  external: {}\n", flow(&json!(external))));
        s
    }

    /// The unit that holds a part, when exactly one does.
    pub fn unit_of(&self, part: &str) -> Option<&Unit> {
        let mut found = self
            .units
            .iter()
            .filter(|u| u.parts.iter().any(|p| p == part));
        match (found.next(), found.next()) {
            (Some(u), None) if !self.external.iter().any(|e| e == part) => Some(u),
            _ => None,
        }
    }
}
