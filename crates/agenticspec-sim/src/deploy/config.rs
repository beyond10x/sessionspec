//! The non-secret configuration a binding gives a unit: environment variables and arguments, on
//! a part (`parts.<p>.env`, `parts.<p>.args`) or on a unit `packing.units` lists
//! (`packing.units[].env`, `packing.units[].args`). The bundle carries them per unit as
//! `config: {env, args}`, merged from the unit's entry and then its parts by name; `env` keys
//! sorted, each member left out when empty, `config` left out when both are.
//!
//! An environment variable's name is `[A-Z_][A-Z0-9_]*`; its value is text (a number or a boolean
//! is taken as its text) or `{secret: <name>}`, a reference to a secret a part of the unit
//! declares, which goes through the secrets of [`super::grants`] and never becomes plain text.
//! `args` is a list of strings. Refusals: `unit_config_invalid`, a value of the wrong form, and
//! `unit_config_conflict`, two sources of one unit that give one variable two different values,
//! or two parts of one unit that both give `args`. A finding names the unit, the source and the
//! variable, never a value.
//!
//! `secret_files` (R120) gives a secret as a read-only file of the unit: a list of
//! `{secret: <name>, key: <key>, path: <absolute path>}`, `key` the one key of the secret the file
//! holds (all of them when absent). The bundle carries them in `config` beside `env` and `args`,
//! from every source of the unit sorted by `path`, by the secret's name, never a value. The name
//! goes through the secrets of [`super::grants`]; the path is a mount of the unit, which
//! [`super::scratch`] holds against its other mounts (`unit_mount_collision`). An entry of the
//! wrong form, a relative path among them, is `unit_config_invalid`.

use super::admit::Finding;
use super::packing::{Packing, Unit};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;

/// The keys of a unit's `config` in a bundle, and of the configuration on a part or a listed unit.
pub const KEYS: &[&str] = &["env", "args", SECRET_FILES];

/// The key of the secrets a unit is given as read-only files (R120).
pub const SECRET_FILES: &str = "secret_files";

/// The keys of one secret file.
const FILE_KEYS: &[&str] = &["secret", "key", "path"];

/// A key of a secret: `[-._a-zA-Z0-9]+`.
pub fn is_secret_key(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_'))
}

/// The problems of one `secret_files` value's form at `at`, a line each; or its entries as the
/// bundle carries them, `{path, secret, key?}`. Whether a secret is a name and declared is
/// [`super::grants`]'s.
fn file_form(at: &str, v: &Value) -> Result<Vec<Value>, Vec<String>> {
    let Some(list) = v.as_array() else {
        return Err(vec![format!(
            "{at}: secret_files is a list of {{secret, key, path}}"
        )]);
    };
    let mut out = Vec::new();
    let mut problems = Vec::new();
    for (i, e) in list.iter().enumerate() {
        let at = format!("{at}: secret_files[{i}]");
        let Some(m) = e.as_object() else {
            problems.push(format!("{at} is {{secret, key, path}}"));
            continue;
        };
        for k in m.keys().filter(|k| !FILE_KEYS.contains(&k.as_str())) {
            problems.push(format!("{at}: unknown key `{k}`"));
        }
        if !m.contains_key("secret") {
            problems.push(format!("{at} needs a `secret:`, the name of the secret"));
        }
        match m.get("path") {
            None => problems.push(format!("{at} needs a `path:`, where the file is read")),
            Some(p) if p.as_str().is_some_and(super::scratch::is_path) => {}
            Some(_) => problems.push(format!(
                "{at}: path is an absolute path (`/` and segments, none empty, `.` or `..`, no `/` at the end)"
            )),
        }
        if let Some(k) = m.get("key")
            && !k.as_str().is_some_and(is_secret_key)
        {
            problems.push(format!(
                "{at}: key is a key of the secret ([-._a-zA-Z0-9]+)"
            ));
        }
        let mut entry = json!({ "path": m.get("path"), "secret": m.get("secret") });
        if let Some(k) = m.get("key") {
            entry["key"] = k.clone();
        }
        out.push(entry);
    }
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// The secret files a unit's sources give, each with how a finding names it
/// (`secret_files[<i>] of <source>`) and its path; an entry whose path is no absolute path is left
/// out, a source whose `secret_files` is no list gives none (both are [`resolve`]'s findings).
pub fn file_mounts<'a>(binding: &'a Value, unit: &Unit) -> Vec<(String, &'a str)> {
    let mut out = Vec::new();
    for (at, _, entry) in sources(binding, unit) {
        for (i, e) in entry[SECRET_FILES]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
        {
            if let Some(p) = e["path"].as_str().filter(|p| super::scratch::is_path(p)) {
                out.push((format!("secret_files[{i}] of {at}"), p));
            }
        }
    }
    out
}

/// The key of a value that refers to a secret: `{secret: <name>}`.
pub const SECRET: &str = "secret";

/// An environment variable's name: `[A-Z_][A-Z0-9_]*`.
pub fn is_env_name(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_uppercase() || c == '_')
        && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn invalid(detail: String) -> Finding {
    Finding {
        refusal: "unit_config_invalid",
        detail,
    }
}

fn conflict(detail: String) -> Finding {
    Finding {
        refusal: "unit_config_conflict",
        detail,
    }
}

/// One value of `env` as the bundle carries it: text, a number or a boolean as text, or
/// `{secret: <s>}` as it is; `None` for any other value. Whether `<s>` is a secret name is
/// [`super::grants`]'s (`secret_value`).
pub fn env_value(v: &Value) -> Option<Value> {
    match v {
        Value::String(s) => Some(json!(s)),
        Value::Number(n) => Some(json!(n.to_string())),
        Value::Bool(b) => Some(json!(b.to_string())),
        Value::Object(m) if m.len() == 1 && m.contains_key(SECRET) => Some(v.clone()),
        _ => None,
    }
}

/// The secret a value of `env` refers to, `{secret: <s>}` and no other key, as written.
pub fn secret_ref(v: &Value) -> Option<&Value> {
    v.as_object()
        .filter(|m| m.len() == 1)
        .and_then(|m| m.get(SECRET))
}

/// The entry of a unit as `packing.units` lists it, by name.
fn listed<'a>(binding: &'a Value, unit: &str) -> Option<&'a Value> {
    binding["packing"]["units"]
        .as_array()?
        .iter()
        .find(|u| u["name"] == unit)
}

/// The sources of a unit's configuration, in order: its entry under `packing.units`, then each of
/// its parts by name; each with how a finding names it and whether it is a part.
pub fn sources<'a>(binding: &'a Value, unit: &Unit) -> Vec<(String, bool, &'a Value)> {
    let mut out = Vec::new();
    if let Some(e) = listed(binding, &unit.name) {
        out.push((format!("packing: unit {}", unit.name), false, e));
    }
    let mut parts: Vec<&String> = unit.parts.iter().collect();
    parts.sort();
    for p in parts {
        let e = &binding["parts"][p.as_str()];
        if e.is_object() {
            out.push((format!("parts: {p}"), true, e));
        }
    }
    out
}

/// The `config` of a unit merged from its sources, or every finding of its form and conflicts;
/// `None` when no source gives any.
pub fn resolve(binding: &Value, unit: &Unit) -> Result<Option<Value>, Vec<Finding>> {
    let mut out = Vec::new();
    let mut env: BTreeMap<String, (Value, String)> = BTreeMap::new();
    let mut args: Vec<Value> = Vec::new();
    let mut args_part: Option<String> = None;
    let mut files: Vec<Value> = Vec::new();
    for (at, is_part, entry) in sources(binding, unit) {
        if let Some(v) = entry.get(SECRET_FILES) {
            match file_form(&at, v) {
                Ok(list) => files.extend(list),
                Err(p) => out.extend(p.into_iter().map(invalid)),
            }
        }
        match entry.get("env") {
            None => {}
            Some(Value::Object(m)) => {
                for (k, v) in m {
                    if !is_env_name(k) {
                        out.push(invalid(format!(
                            "{at}: env: `{k}` is not an environment variable name ([A-Z_][A-Z0-9_]*)"
                        )));
                        continue;
                    }
                    let Some(v) = env_value(v) else {
                        out.push(invalid(format!(
                            "{at}: env: {k} is text, a number, a boolean or {{secret: <name>}}"
                        )));
                        continue;
                    };
                    match env.get(k) {
                        Some((first, by)) if *first != v => out.push(conflict(format!(
                            "unit {}: env {k} is given two different values, by {by} and by {at}",
                            unit.name
                        ))),
                        Some(_) => {}
                        None => {
                            env.insert(k.clone(), (v, at.clone()));
                        }
                    }
                }
            }
            Some(_) => out.push(invalid(format!(
                "{at}: env is a mapping from an environment variable name to its value"
            ))),
        }
        match entry.get("args") {
            None => {}
            Some(Value::Array(list)) if list.iter().all(Value::is_string) => {
                if list.is_empty() {
                    continue;
                }
                if is_part {
                    if let Some(first) = &args_part {
                        out.push(conflict(format!(
                            "unit {}: {first} and {at} both give args",
                            unit.name
                        )));
                        continue;
                    }
                    args_part = Some(at.clone());
                }
                args.extend(list.iter().cloned());
            }
            Some(_) => out.push(invalid(format!("{at}: args is a list of strings"))),
        }
    }
    if !out.is_empty() {
        return Err(out);
    }
    let mut config = Map::new();
    if !env.is_empty() {
        let m: Map<String, Value> = env.into_iter().map(|(k, (v, _))| (k, v)).collect();
        config.insert("env".into(), Value::Object(m));
    }
    if !args.is_empty() {
        config.insert("args".into(), Value::Array(args));
    }
    if !files.is_empty() {
        files.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));
        config.insert(SECRET_FILES.into(), Value::Array(files));
    }
    Ok((!config.is_empty()).then_some(Value::Object(config)))
}

/// Every finding of the configuration a binding gives the units of a packing: those of
/// [`resolve`] per unit, and a part in no unit that gives `env` or `args`.
pub fn findings(binding: &Value, packing: &Packing) -> Vec<Finding> {
    let mut out = Vec::new();
    for u in &packing.units {
        if let Err(f) = resolve(binding, u) {
            out.extend(f);
        }
    }
    for (p, entry) in binding["parts"].as_object().into_iter().flatten() {
        if packing.unit_of(p).is_some() {
            continue;
        }
        for k in KEYS.iter().filter(|k| entry.get(**k).is_some()) {
            out.push(invalid(format!(
                "parts: {p}: {k} is for a part of a unit, and {p} is in no unit"
            )));
        }
    }
    out
}

/// The form of a unit's `config` read back from a bundle: a mapping of `env` and `args`, not
/// empty; `env` a non-empty mapping from a variable name to text or `{secret: <name>}`; `args` a
/// non-empty list of strings. One line per problem.
fn form(config: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let Some(m) = config.as_object() else {
        return vec!["config is a mapping of `env`, `args` and `secret_files`".into()];
    };
    if m.is_empty() {
        out.push("config is empty, and an empty config is left out".into());
    }
    for (k, v) in m {
        match k.as_str() {
            "env" => match v.as_object() {
                Some(e) if !e.is_empty() => {
                    for (name, x) in e {
                        if !is_env_name(name) {
                            out.push(format!(
                                "config: env: `{name}` is not an environment variable name"
                            ));
                        }
                        let ok = x.is_string()
                            || x.as_object()
                                .is_some_and(|r| r.len() == 1 && r.contains_key(SECRET));
                        if !ok {
                            out.push(format!("config: env: {name} is text or {{secret: <name>}}"));
                        }
                    }
                }
                _ => out.push("config: env is a mapping that is not empty".into()),
            },
            "args" => {
                if !v
                    .as_array()
                    .is_some_and(|a| !a.is_empty() && a.iter().all(Value::is_string))
                {
                    out.push("config: args is a list of strings that is not empty".into());
                }
            }
            SECRET_FILES => match file_form("config", v) {
                Ok(list) if list.is_empty() => {
                    out.push("config: secret_files is a list that is not empty".into())
                }
                Ok(list) => {
                    for (i, f) in list.iter().enumerate() {
                        if !f["secret"].is_string() {
                            out.push(format!("config: secret_files[{i}]: secret is a name"));
                        }
                    }
                }
                Err(p) => out.extend(p),
            },
            other => out.push(format!("config: unknown key `{other}`")),
        }
    }
    out
}

/// The `unit_config_invalid` findings of the units of a written bundle: a `config` of the wrong
/// form, or one that is not the config the carried binding resolves for its unit (none for a unit
/// the packing does not have). A unit whose binding configuration is refused by [`findings`] is
/// left to that finding.
pub fn check_units(bundle: &Value, packing: &Packing) -> Vec<Finding> {
    let binding = &bundle["binding"]["document"];
    let mut out = Vec::new();
    for u in bundle["units"].as_array().into_iter().flatten() {
        let name = u["name"].as_str().unwrap_or("");
        let found = u.get("config");
        if let Some(c) = found {
            let problems = form(c);
            if !problems.is_empty() {
                for p in problems {
                    out.push(invalid(format!("unit {name}: {p}")));
                }
                continue;
            }
        }
        let expected = match packing.units.iter().find(|x| x.name == name) {
            Some(pu) => match resolve(binding, pu) {
                Ok(c) => c,
                Err(_) => continue,
            },
            None => None,
        };
        if found != expected.as_ref() {
            out.push(invalid(format!(
                "unit {name}: carries {}, and its binding resolves {}",
                if found.is_some() {
                    "a config"
                } else {
                    "no config"
                },
                match &expected {
                    Some(_) if found.is_some() => "another",
                    Some(_) => "one",
                    None => "none",
                }
            )));
        }
    }
    out
}
