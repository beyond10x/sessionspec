//! Admission of a packing: the plan (the units with the roles each needs and the links that
//! cross units), or every refusal, each by name. The accepted unit kinds and roles come from the
//! copy of cloudspec's accepted list (`bundles/cloudspec-accepts.yaml`, [`Accepts`]).

use super::binding::{isolation_rank, target_isolation};
use super::packing::{Packing, Unit, is_clock, schedule_of};
use crate::check::value;
use crate::contract::derive::link_name;
use crate::doc::parse_yaml;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

/// The refusals of admission, in the order they are listed.
pub const REFUSALS: &[&str] = &[
    "part_unpacked",
    "part_packed_twice",
    "per_split",
    "shared_state_split",
    "link_without_transport",
    "isolation_unmet",
    "network_isolation_unmet",
    "scheduled_with_inputs",
    "schedule_not_cron",
    "core_count",
    "unit_not_hosted",
    "agent_document_missing",
    "agent_document_digest",
    "image_unpinned",
    "unit_image_conflict",
    "unit_image_missing",
    "unit_runtime_mismatch",
    "durable_state_unplaceable",
    "volume_mount_missing",
    "unit_config_conflict",
    "unit_config_invalid",
    "unit_scratch_invalid",
    "unit_mount_collision",
    "secret_value",
    "secret_undeclared",
    "egress_undeclared",
    "address_in_architecture",
    "ui_upstream_unknown",
    "ui_upstream_path_invalid",
    "ui_export_unknown",
    "ui_archive_mismatch",
    "ui_build_needs_toolchain",
];

/// The transport of the link from a clock into the scheduled unit it starts: the target's
/// schedule, which needs no transport in the binding.
pub const SCHEDULE_TRANSPORT: &str = "schedule";

/// One refusal of admission: its name and what it is about.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub refusal: &'static str,
    pub detail: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.refusal, self.detail)
    }
}

fn finding(refusal: &'static str, detail: impl Into<String>) -> Finding {
    Finding {
        refusal,
        detail: detail.into(),
    }
}

/// Sort findings by the order of [`REFUSALS`], then by detail.
pub fn sort(findings: &mut [Finding]) {
    let rank = |r: &str| REFUSALS.iter().position(|x| *x == r).unwrap_or(usize::MAX);
    findings.sort_by(|a, b| {
        rank(a.refusal)
            .cmp(&rank(b.refusal))
            .then_with(|| a.detail.cmp(&b.detail))
    });
}

pub const ACCEPTS_FORMAT: &str = "agenticspec-cloudspec-accepts/1";

/// The copy of cloudspec's accepted unit kinds and roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accepts {
    /// The cloudspec commit the list was copied from; `None` while cloudspec publishes none.
    pub cloudspec_commit: Option<String>,
    pub unit_kinds: BTreeSet<String>,
    /// Role names; a role written `<name>:<argument>` is accepted by its name.
    pub roles: BTreeSet<String>,
}

impl Accepts {
    pub fn parse(text: &str) -> Result<Accepts, String> {
        let v = parse_yaml(text)?;
        if v["format"] != ACCEPTS_FORMAT {
            return Err(format!("not {ACCEPTS_FORMAT}"));
        }
        let set = |k: &str| -> Result<BTreeSet<String>, String> {
            v[k].as_array()
                .ok_or(format!("no `{k}`"))?
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(str::to_owned)
                        .ok_or(format!("`{k}` lists names"))
                })
                .collect()
        };
        Ok(Accepts {
            cloudspec_commit: v["cloudspec_commit"].as_str().map(str::to_owned),
            unit_kinds: set("unit_kinds")?,
            roles: set("roles")?,
        })
    }

    pub fn load(path: &Path) -> Result<Accepts, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Accepts::parse(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    fn holds_role(&self, role: &str) -> bool {
        self.roles.contains(role.split(':').next().unwrap_or(role))
    }
}

/// A link whose two parts are in two units, with the transport the binding names for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crossing {
    pub link: String,
    pub from_unit: String,
    pub to_unit: String,
    pub carries: String,
    pub transport: String,
}

/// One unit of the plan and the abstract roles it needs from its target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedUnit {
    pub unit: Unit,
    pub roles: Vec<String>,
}

/// An admitted packing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub units: Vec<PlannedUnit>,
    pub external: Vec<String>,
    pub crossings: Vec<Crossing>,
}

fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn links(arch: &Value) -> Vec<&Value> {
    arch["links"].as_array().into_iter().flatten().collect()
}

/// A part whose state must live in one place: a pool, a gateway with a rate limit, an
/// exclusive workspace, a store.
fn stateful(part: &Value) -> Option<&'static str> {
    match text(&part["block"]) {
        "pool" => Some("a pool"),
        "gateway" if !value(&part["rate_limit_per_minute"]).is_null() => {
            Some("a gateway's rate limit")
        }
        "workspace" if value(&part["sharing"]).as_str() == Some("exclusive") => {
            Some("an exclusive workspace")
        }
        "store" => Some("a store"),
        _ => None,
    }
}

/// The roles a unit needs: `agent-runtime` for an agent, job or scheduled unit;
/// `core-runtime` for a core or service unit the binding gives no image
/// ([`super::binding::unit_image`]); `start-jobs` for the core when a job unit exists;
/// `model-server` and `store` for those kinds; `model-route:<route>` for each hosted model route
/// a part of the unit calls.
fn roles(arch: &Value, binding: &Value, packing: &Packing, unit: &Unit) -> Vec<String> {
    let mut out = BTreeSet::new();
    let imaged = super::binding::unit_image(binding, unit).image().is_some();
    match unit.kind.as_str() {
        "agent" | "job" | "scheduled" => {
            out.insert("agent-runtime".to_owned());
        }
        "core" | "service" if !imaged => {
            out.insert("core-runtime".to_owned());
        }
        "model" => {
            out.insert("model-server".to_owned());
        }
        "store" => {
            out.insert("store".to_owned());
        }
        _ => {}
    }
    if unit.kind == "core" && packing.units.iter().any(|u| u.kind == "job") {
        out.insert("start-jobs".to_owned());
    }
    for l in links(arch) {
        let (from, to) = (text(&l["from"]), text(&l["to"]));
        if text(&l["carries"]) == "model_call"
            && unit.parts.iter().any(|p| p == from)
            && packing.external.iter().any(|e| e == to)
            && arch["parts"][to]["block"] == "model"
        {
            let route = value(&arch["parts"][to]["route"]).as_str().unwrap_or(to);
            out.insert(format!("model-route:{route}"));
        }
    }
    out.into_iter().collect()
}

/// Admit a resolved packing of an architecture under a binding: the plan, or every finding of
/// the refusals of [`REFUSALS`] up to `unit_not_hosted`, sorted.
pub fn admit(
    arch: &Value,
    binding: &Value,
    packing: &Packing,
    accepts: &Accepts,
) -> Result<Plan, Vec<Finding>> {
    let empty = serde_json::Map::new();
    let parts = arch["parts"].as_object().unwrap_or(&empty);
    let mut out = Vec::new();

    // Each part instance in exactly one unit; `external` counts as packed.
    let mut places: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for u in &packing.units {
        for p in &u.parts {
            places
                .entry(p)
                .or_default()
                .push(format!("unit {}", u.name));
        }
    }
    for e in &packing.external {
        places.entry(e).or_default().push("external".into());
    }
    for name in parts.keys() {
        match places.get(name.as_str()).map(Vec::as_slice) {
            None | Some([]) => out.push(finding(
                "part_unpacked",
                format!("part {name} is in no unit and not external"),
            )),
            Some([_]) => {}
            Some(many) => out.push(finding(
                "part_packed_twice",
                format!("part {name} is in {}", many.join(" and ")),
            )),
        }
    }
    let unit_of = |p: &str| packing.unit_of(p).map(|u| u.name.as_str());
    let is_external = |p: &str| packing.external.iter().any(|e| e == p);

    // A `per: X` part sits with its X.
    for (name, part) in parts {
        if let Some(x) = part["per"].as_str() {
            let (a, b) = (unit_of(name), unit_of(x));
            if (a.is_some() && b.is_some() && a != b) || (a.is_some() && is_external(x)) {
                out.push(finding(
                    "per_split",
                    format!(
                        "part {name} is per {x}, and {name} is in unit {} while {x} is {}",
                        a.unwrap_or(""),
                        b.map_or("external".to_owned(), |b| format!("in unit {b}"))
                    ),
                ));
            }
        }
    }

    // State reached from two units and bound in memory.
    for (name, part) in parts {
        let Some(what) = stateful(part) else { continue };
        let mut reached: BTreeSet<&str> = BTreeSet::new();
        for l in links(arch) {
            let (from, to) = (text(&l["from"]), text(&l["to"]));
            let other = if from == name {
                to
            } else if to == name {
                from
            } else {
                continue;
            };
            if let Some(u) = unit_of(other) {
                reached.insert(u);
            }
        }
        for (other, p) in parts {
            if p["per"].as_str() == Some(name)
                && let Some(u) = unit_of(other)
            {
                reached.insert(u);
            }
        }
        let state = binding["parts"][name]["state"].as_str().unwrap_or("memory");
        if reached.len() >= 2 && state == "memory" {
            out.push(finding(
                "shared_state_split",
                format!(
                    "part {name} ({what}) is reached from units {} and bound in memory (bind it `state: service`, or pack its users into one unit)",
                    reached.into_iter().collect::<Vec<_>>().join(", ")
                ),
            ));
        }
    }

    // Links that cross units, each with a transport.
    let mut crossings = Vec::new();
    for l in links(arch) {
        let (from, to) = (text(&l["from"]), text(&l["to"]));
        let (Some(a), Some(b)) = (unit_of(from), unit_of(to)) else {
            continue;
        };
        if a == b {
            continue;
        }
        let carries = text(&l["carries"]);
        // The clock of a scheduled unit starts it through the target's schedule, not a link.
        let scheduled = packing.unit_of(to).is_some_and(|u| u.kind == "scheduled")
            && carries == "trigger"
            && is_clock(arch, from);
        let transport = if scheduled {
            Some(SCHEDULE_TRANSPORT)
        } else {
            binding["transports"][carries].as_str()
        };
        match transport {
            Some(t) => crossings.push(Crossing {
                link: link_name(l),
                from_unit: a.to_owned(),
                to_unit: b.to_owned(),
                carries: carries.to_owned(),
                transport: t.to_owned(),
            }),
            None => out.push(finding(
                "link_without_transport",
                format!(
                    "link `{}` crosses from unit {a} to unit {b}, and the binding names no transport for `{carries}`",
                    link_name(l)
                ),
            )),
        }
    }

    // A sandbox isolates no more than the target gives a unit, unless one runs inside it.
    let given = target_isolation(binding);
    for (name, part) in parts {
        if part["block"] != "sandbox" || is_external(name) {
            continue;
        }
        let level = value(&part["isolation"]).as_str().unwrap_or("none");
        if isolation_rank(level) > isolation_rank(given)
            && binding["parts"][name]["inside"].as_str().is_none()
        {
            out.push(finding(
                "isolation_unmet",
                format!(
                    "sandbox {name} needs isolation {level}, the target gives a unit {given}, and the binding names no sandbox `inside:` it"
                ),
            ));
        }
    }

    // A scheduled unit is woken by its clock and takes no other input. A cron rule the binding
    // gives it is a clock of the target's.
    for u in packing.units.iter().filter(|u| u.kind == "scheduled") {
        let mut woken = u.schedule.is_some();
        for l in links(arch) {
            let to = text(&l["to"]);
            if !u.parts.iter().any(|p| p == to) {
                continue;
            }
            if text(&l["carries"]) == "trigger" && is_clock(arch, text(&l["from"])) {
                woken = true;
            } else {
                out.push(finding(
                    "scheduled_with_inputs",
                    format!(
                        "scheduled unit {} takes the input of link `{}`, not only its clock",
                        u.name,
                        link_name(l)
                    ),
                ));
            }
        }
        if !woken {
            out.push(finding(
                "scheduled_with_inputs",
                format!(
                    "scheduled unit {} holds no part a schedule or an on_schedule trigger wakes",
                    u.name
                ),
            ));
        }
        // Without a cron rule of the binding, the clock's `every` is the unit's rule, when one
        // states it exactly.
        if let Err(detail) = schedule_of(arch, u) {
            out.push(finding("schedule_not_cron", detail));
        }
    }

    let cores = packing.units.iter().filter(|u| u.kind == "core").count();
    if cores != 1 {
        out.push(finding(
            "core_count",
            format!("the packing has {cores} core units, not exactly one"),
        ));
    }

    let mut planned = Vec::new();
    for u in &packing.units {
        if !accepts.unit_kinds.contains(&u.kind) {
            out.push(finding(
                "unit_not_hosted",
                format!(
                    "unit {}: kind {} is not in the accepted list",
                    u.name, u.kind
                ),
            ));
        }
        let rs = roles(arch, binding, packing, u);
        for r in &rs {
            if !accepts.holds_role(r) {
                out.push(finding(
                    "unit_not_hosted",
                    format!("unit {}: role {r} is not in the accepted list", u.name),
                ));
            }
        }
        planned.push(PlannedUnit {
            unit: u.clone(),
            roles: rs,
        });
    }

    if !out.is_empty() {
        sort(&mut out);
        return Err(out);
    }
    crossings.sort_by(|a, b| a.link.cmp(&b.link));
    Ok(Plan {
        units: planned,
        external: packing.external.clone(),
        crossings,
    })
}
