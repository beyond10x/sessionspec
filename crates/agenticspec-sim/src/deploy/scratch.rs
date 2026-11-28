//! The scratch directories a binding gives a unit (R118): a writable directory at a fixed path
//! that lives as long as one instance of the unit and holds nothing after a restart, distinct
//! from a volume, which a unit claims and keeps. On a unit `packing.units` lists
//! (`packing.units[].scratch`) or on a part of the unit (`parts.<p>.scratch`), each a list of
//! `{mount, size}`: `mount` an absolute path in normal form, `size` the most it may hold, text in
//! the form of a volume's `size` (`64Mi`). The bundle carries them per unit as
//! `scratch: [{mount, size}]`, from every source of the unit ([`super::config::sources`]) sorted
//! by mount, and left out when there are none.
//!
//! Refusals: `unit_scratch_invalid`, a `scratch` of the wrong form (a relative path among them)
//! or on a part in no unit; `unit_mount_collision`, a mount of the unit's own, a scratch directory
//! or a secret file (`secret_files` of [`super::config`], R120), at the mount of another mount of
//! its unit (a volume a part of the unit mounts, or another mount of its own), or one that holds
//! the other or lies inside it: a nested mount hides part of the outer one, so part of a directory
//! that should hold nothing after a restart would keep its files, part of a volume would lose
//! them, or a secret file would hide or be hidden. Paths compare by segment (`/var/lib` holds
//! `/var/lib/flow`, not `/var/lib/flowx`). Two volumes are not held against each other here. A
//! finding names the unit, the source and the volume, never a path.

use super::admit::Finding;
use super::packing::{Packing, Unit};
use serde_json::{Value, json};

/// The key of a unit's scratch directories, in a binding and in a bundle.
pub const KEY: &str = "scratch";

/// The keys of one scratch directory.
const ENTRY_KEYS: &[&str] = &["mount", "size"];

fn invalid(detail: String) -> Finding {
    Finding {
        refusal: "unit_scratch_invalid",
        detail,
    }
}

/// An absolute path in normal form: `/` and one or more segments, none empty, `.` or `..`, and
/// no `/` at its end.
pub fn is_path(s: &str) -> bool {
    s.strip_prefix('/')
        .is_some_and(|r| r.split('/').all(|x| !x.is_empty() && x != "." && x != ".."))
}

/// How a mount at `a` stands to one at `b`, by segment: the same, inside it, holding it, or
/// apart (`None`).
fn relation(a: &str, b: &str) -> Option<&'static str> {
    let seg = |s: &str| -> Vec<String> {
        s.split('/')
            .filter(|x| !x.is_empty())
            .map(str::to_owned)
            .collect()
    };
    let (a, b) = (seg(a), seg(b));
    if a == b {
        Some("is the mount of")
    } else if a.starts_with(&b) {
        Some("lies inside the mount of")
    } else if b.starts_with(&a) {
        Some("holds the mount of")
    } else {
        None
    }
}

/// The problems of one `scratch` value's form, at `at`, each a line; and the entries when it has
/// none, each `{mount, size?}` as the bundle carries it.
fn form(at: &str, v: &Value) -> Result<Vec<Value>, Vec<String>> {
    let Some(list) = v.as_array() else {
        return Err(vec![format!("{at}: scratch is a list of {{mount, size}}")]);
    };
    let mut out = Vec::new();
    let mut problems = Vec::new();
    for (i, e) in list.iter().enumerate() {
        let at = format!("{at}: scratch[{i}]");
        let Some(m) = e.as_object() else {
            problems.push(format!("{at} is {{mount, size}}"));
            continue;
        };
        for k in m.keys().filter(|k| !ENTRY_KEYS.contains(&k.as_str())) {
            problems.push(format!("{at}: unknown key `{k}`"));
        }
        match m.get("mount") {
            None => problems.push(format!("{at} needs a `mount:`, the path of the directory")),
            Some(x) if x.as_str().is_some_and(is_path) => {}
            Some(_) => problems.push(format!(
                "{at}: mount is an absolute path (`/` and segments, none empty, `.` or `..`, no `/` at the end)"
            )),
        }
        if let Some(x) = m.get("size")
            && !x.as_str().is_some_and(super::binding::is_text)
        {
            problems.push(format!(
                "{at}: size is text without spaces, as a volume's (`64Mi`)"
            ));
        }
        let mut entry = json!({ "mount": m.get("mount") });
        if let Some(s) = m.get("size") {
            entry["size"] = s.clone();
        }
        out.push(entry);
    }
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// The scratch directories of a unit from its sources, each with how a finding names it
/// (`scratch[<i>] of <source>`), in the order of the sources; or every finding of their form.
fn named(binding: &Value, unit: &Unit) -> Result<Vec<(String, Value)>, Vec<Finding>> {
    let mut out = Vec::new();
    let mut findings = Vec::new();
    for (at, _, entry) in super::config::sources(binding, unit) {
        let Some(v) = entry.get(KEY) else { continue };
        match form(&at, v) {
            Ok(list) => out.extend(
                list.into_iter()
                    .enumerate()
                    .map(|(i, e)| (format!("scratch[{i}] of {at}"), e)),
            ),
            Err(p) => findings.extend(p.into_iter().map(invalid)),
        }
    }
    if findings.is_empty() {
        Ok(out)
    } else {
        Err(findings)
    }
}

/// The `scratch` of a unit as the bundle carries it, sorted by mount; `None` when it has none,
/// or every finding of its form.
pub fn resolve(binding: &Value, unit: &Unit) -> Result<Option<Value>, Vec<Finding>> {
    let mut list: Vec<Value> = named(binding, unit)?.into_iter().map(|(_, e)| e).collect();
    list.sort_by(|a, b| a["mount"].as_str().cmp(&b["mount"].as_str()));
    Ok((!list.is_empty()).then_some(Value::Array(list)))
}

/// The `unit_mount_collision` findings of unit `unit`'s own mounts, its secret files and scratch
/// directories (each with how a finding names it), against each other and against the volume
/// mounts of the unit (each with how a finding names it).
fn collisions(unit: &str, own: &[(String, &str)], mounts: &[(String, &str)]) -> Vec<Finding> {
    let mut out = Vec::new();
    for (i, (name, at)) in own.iter().enumerate() {
        let others = own[i + 1..].iter().chain(mounts);
        for (other, m) in others {
            if let Some(how) = relation(at, m) {
                out.push(Finding {
                    refusal: "unit_mount_collision",
                    detail: format!("unit {unit}: {name} {how} {other}"),
                });
            }
        }
    }
    out
}

/// The mounts the parts of a unit give under `parts.<p>.volumes[]` of the binding, each named
/// `volume <name> (parts: <p>)`.
fn volume_mounts<'a>(binding: &'a Value, unit: &Unit) -> Vec<(String, &'a str)> {
    let mut out = Vec::new();
    for p in &unit.parts {
        for v in binding["parts"][p]["volumes"]
            .as_array()
            .into_iter()
            .flatten()
        {
            if let Some(m) = v["mount"].as_str() {
                out.push((
                    format!("volume {} (parts: {p})", v["name"].as_str().unwrap_or("")),
                    m,
                ));
            }
        }
    }
    out
}

/// Every finding of the scratch directories a binding gives the units of a packing: their form,
/// the collisions of each unit's own mounts (its secret files, then its scratch directories), and
/// `scratch` on a part in no unit.
pub fn findings(binding: &Value, packing: &Packing) -> Vec<Finding> {
    let mut out = Vec::new();
    for u in &packing.units {
        match named(binding, u) {
            Err(f) => out.extend(f),
            Ok(list) => {
                let mut own = super::config::file_mounts(binding, u);
                own.extend(
                    list.iter()
                        .map(|(n, e)| (n.clone(), e["mount"].as_str().unwrap_or(""))),
                );
                out.extend(collisions(&u.name, &own, &volume_mounts(binding, u)));
            }
        }
    }
    for (p, entry) in binding["parts"].as_object().into_iter().flatten() {
        if entry.get(KEY).is_some() && packing.unit_of(p).is_none() {
            out.push(invalid(format!(
                "parts: {p}: scratch is for a part of a unit, and {p} is in no unit"
            )));
        }
    }
    out
}

/// The findings of the units of a written bundle: a `scratch` of the wrong form (not empty when
/// present); a mount of the unit's own, a secret file it carries in `config.secret_files` or a
/// scratch directory, at or around another of them or a volume mount the unit carries
/// (`unit_mount_collision`); and a `scratch` that is not what the carried binding resolves for its
/// unit (none for a unit the packing does not have; `unit_scratch_invalid`). A unit whose binding
/// scratch is refused by [`findings`] is left to that finding.
pub fn check_units(bundle: &Value, packing: &Packing) -> Vec<Finding> {
    let binding = &bundle["binding"]["document"];
    let mut out = Vec::new();
    for u in bundle["units"].as_array().into_iter().flatten() {
        let name = u["name"].as_str().unwrap_or("");
        let found = u.get(KEY);
        let paths = |v: &Value, key: &str, at: &str| -> Vec<(String, String)> {
            v.as_array()
                .into_iter()
                .flatten()
                .enumerate()
                .filter_map(|(i, e)| {
                    e[key]
                        .as_str()
                        .filter(|p| is_path(p))
                        .map(|p| (format!("{at}[{i}]"), p.to_owned()))
                })
                .collect()
        };
        let mut own = paths(
            &u["config"][super::config::SECRET_FILES],
            "path",
            "config: secret_files",
        );
        let mut scratch_ok = true;
        if let Some(s) = found {
            let at = format!("unit {name}");
            match form(&at, s) {
                Ok(list) if list.is_empty() => {
                    out.push(invalid(format!(
                        "{at}: scratch is empty, and an empty scratch is left out"
                    )));
                    scratch_ok = false;
                }
                Ok(_) => own.extend(paths(s, "mount", "scratch")),
                Err(p) => {
                    out.extend(p.into_iter().map(invalid));
                    scratch_ok = false;
                }
            }
        }
        let mounts: Vec<(String, &str)> = u["claims"]["volumes"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| {
                v["mount"]
                    .as_str()
                    .map(|m| (format!("volume {}", v["name"].as_str().unwrap_or("")), m))
            })
            .collect();
        let own: Vec<(String, &str)> = own.iter().map(|(n, p)| (n.clone(), p.as_str())).collect();
        out.extend(collisions(name, &own, &mounts));
        if !scratch_ok {
            continue;
        }
        let expected = match packing.units.iter().find(|x| x.name == name) {
            Some(pu) => match resolve(binding, pu) {
                Ok(s) => s,
                Err(_) => continue,
            },
            None => None,
        };
        if found != expected.as_ref() {
            out.push(invalid(format!(
                "unit {name}: carries {}, and its binding resolves {}",
                if found.is_some() {
                    "scratch directories"
                } else {
                    "no scratch directory"
                },
                match &expected {
                    Some(_) if found.is_some() => "others",
                    Some(_) => "some",
                    None => "none",
                }
            )));
        }
    }
    out
}
