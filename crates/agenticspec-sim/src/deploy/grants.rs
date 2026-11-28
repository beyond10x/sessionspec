//! What a unit is granted: secrets and egress, each by name, declared per part in the binding
//! (`parts.<part>.secrets`, `parts.<part>.egress`). Nothing undeclared is granted: a link that
//! leaves its unit needs its destination declared by the part it starts at, and a secret an
//! egress entry or a unit names needs a part of that unit that declares it. A secret is only
//! ever a name; anything that looks like its value is refused (`secret_value`). A unit's
//! configuration ([`super::config`]) refers to a secret as `{secret: <name>}`, under the same rules.
//!
//! The rule of a name ([`is_name`]): `^[a-z][a-z0-9-]*$`, at most 63 characters, no word between
//! hyphens longer than 20 characters (a key or a token is one long run; a name is words). A key
//! of [`VALUE_KEYS`] where the binding or the bundle holds fields is refused as a value. A
//! finding names where it is, never the text it found.

use super::admit::Finding;
use super::config;
use super::packing::Packing;
use crate::contract::derive::link_name;
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// The keys that hold a value rather than a name, refused where the binding or the bundle holds
/// fields: the top level, `target`, a part's entry, an egress entry, a runtime, a claim, a unit.
pub const VALUE_KEYS: &[&str] = &[
    "value",
    "values",
    "data",
    "string_data",
    "literal",
    "password",
    "token",
];

/// The longest name.
pub const NAME_MAX: usize = 63;

/// The longest word of a name between hyphens.
pub const WORD_MAX: usize = 20;

/// Whether `s` is a name: `^[a-z][a-z0-9-]*$`, at most [`NAME_MAX`] characters, no word
/// between hyphens longer than [`WORD_MAX`].
pub fn is_name(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && s.len() <= NAME_MAX
        && s.split('-').all(|w| w.len() <= WORD_MAX)
}

fn finding(refusal: &'static str, detail: impl Into<String>) -> Finding {
    Finding {
        refusal,
        detail: detail.into(),
    }
}

fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

/// One declared egress entry of a part: a destination, and the secret its link authenticates
/// with when it names one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Egress {
    pub to: String,
    pub secret: Option<String>,
    /// Where the target finds a destination outside the document, when the entry names it: a
    /// CIDR block or `<host>:<port>`, or `{namespace, labels}`.
    pub address: Option<Value>,
}

/// The secret names part `part` declares, those that are names.
pub fn declared_secrets(binding: &Value, part: &str) -> Vec<String> {
    binding["parts"][part]["secrets"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|s| is_name(s))
        .map(str::to_owned)
        .collect()
}

/// The egress entries part `part` declares: a destination by name, or `{to, secret, address}`.
pub fn declared_egress(binding: &Value, part: &str) -> Vec<Egress> {
    let mut out = Vec::new();
    for e in binding["parts"][part]["egress"]
        .as_array()
        .into_iter()
        .flatten()
    {
        match e {
            Value::String(to) => out.push(Egress {
                to: to.clone(),
                secret: None,
                address: None,
            }),
            Value::Object(m) => {
                if let Some(to) = m.get("to").and_then(Value::as_str) {
                    out.push(Egress {
                        to: to.to_owned(),
                        secret: m.get("secret").and_then(Value::as_str).map(str::to_owned),
                        address: m.get("address").cloned(),
                    });
                }
            }
            _ => {}
        }
    }
    out
}

/// The keys of [`VALUE_KEYS`] a mapping holds, as findings at `at`.
fn value_keys(m: &Map<String, Value>, at: &str, out: &mut Vec<Finding>) {
    for k in m.keys().filter(|k| VALUE_KEYS.contains(&k.as_str())) {
        out.push(finding(
            "secret_value",
            format!(
                "{at} has `{k}:`, a key that holds a value where only a secret's name may stand"
            ),
        ));
    }
}

/// A secret name at `at`, or the finding that it is none.
fn secret_name(v: &Value, at: &str, out: &mut Vec<Finding>) {
    if !v.as_str().is_some_and(is_name) {
        out.push(finding(
            "secret_value",
            format!(
                "{at} is not a secret name (^[a-z][a-z0-9-]*$, at most {NAME_MAX} characters, words of at most {WORD_MAX})"
            ),
        ));
    }
}

/// The `secret_value` findings of a binding: a secret that is no name, a `secrets:` that is no
/// list, a key of [`VALUE_KEYS`] where the binding holds fields, an `env:` value `{secret: <s>}` of
/// a part or a listed unit whose `<s>` is no name.
pub fn values(binding: &Value) -> Vec<Finding> {
    let mut out = Vec::new();
    let Some(top) = binding.as_object() else {
        return out;
    };
    value_keys(top, "the binding", &mut out);
    if let Some(t) = binding["target"].as_object() {
        value_keys(t, "target", &mut out);
    }
    for (section, at) in [("runtimes", "runtimes"), ("claims", "claims")] {
        for (name, v) in binding[section].as_object().into_iter().flatten() {
            if let Some(m) = v.as_object() {
                value_keys(m, &format!("{at}: {name}"), &mut out);
            }
        }
    }
    for (part, entry) in binding["parts"].as_object().into_iter().flatten() {
        let Some(m) = entry.as_object() else { continue };
        value_keys(m, &format!("parts: {part}"), &mut out);
        if let Some(im) = m.get("image").and_then(Value::as_object) {
            value_keys(im, &format!("parts: {part}: image"), &mut out);
        }
        match m.get("secrets") {
            None => {}
            Some(Value::Array(list)) => {
                for (i, s) in list.iter().enumerate() {
                    secret_name(s, &format!("parts: {part}: secrets[{i}]"), &mut out);
                }
            }
            Some(_) => out.push(finding(
                "secret_value",
                format!("parts: {part}: secrets is not a list of secret names"),
            )),
        }
        for (i, e) in m
            .get("egress")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            if let Some(em) = e.as_object() {
                let at = format!("parts: {part}: egress[{i}]");
                value_keys(em, &at, &mut out);
                if let Some(s) = em.get("secret") {
                    secret_name(s, &format!("{at}: secret"), &mut out);
                }
            }
        }
        if let Some(env) = m.get("env") {
            env_secrets(env, &format!("parts: {part}"), &mut out);
        }
        if let Some(files) = m.get(config::SECRET_FILES) {
            file_secrets(files, &format!("parts: {part}"), &mut out);
        }
    }
    for u in binding["packing"]["units"].as_array().into_iter().flatten() {
        let at = format!("packing: unit {}", text(&u["name"]));
        env_secrets(&u["env"], &at, &mut out);
        file_secrets(&u[config::SECRET_FILES], &at, &mut out);
    }
    out
}

/// The `secret_value` findings of a `secret_files:` at `at` (a part's or a listed unit's, R120):
/// an entry's `secret` that is no secret name, a key of [`VALUE_KEYS`] in an entry.
fn file_secrets(files: &Value, at: &str, out: &mut Vec<Finding>) {
    for (i, e) in files.as_array().into_iter().flatten().enumerate() {
        let Some(m) = e.as_object() else { continue };
        let at = format!("{at}: secret_files[{i}]");
        value_keys(m, &at, out);
        if let Some(s) = m.get(config::SECRET) {
            secret_name(s, &format!("{at}: secret"), out);
        }
    }
}

/// The secret names the `secret_files:` of `entry` refers to, those that are names, by index.
fn file_refs(entry: &Value) -> Vec<(usize, &str)> {
    entry[config::SECRET_FILES]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, e)| {
            e[config::SECRET]
                .as_str()
                .filter(|s| is_name(s))
                .map(|s| (i, s))
        })
        .collect()
}

/// The `secret_value` findings of an `env:` at `at` (a part's or a listed unit's): a value
/// `{secret: <s>}` whose `<s>` is no secret name, a key of [`VALUE_KEYS`] in a value that is a
/// mapping.
fn env_secrets(env: &Value, at: &str, out: &mut Vec<Finding>) {
    for (k, v) in env.as_object().into_iter().flatten() {
        let Some(m) = v.as_object() else { continue };
        let at = format!("{at}: env: {k}");
        value_keys(m, &at, out);
        if let Some(s) = m.get(config::SECRET) {
            secret_name(s, &format!("{at}: secret"), out);
        }
    }
}

/// The secret names the `env:` of `entry` refers to (`{secret: <s>}`), those that are names, by
/// variable.
fn env_refs(entry: &Value) -> Vec<(&str, &str)> {
    entry["env"]
        .as_object()
        .into_iter()
        .flatten()
        .filter_map(|(k, v)| {
            config::secret_ref(v)
                .and_then(Value::as_str)
                .filter(|s| is_name(s))
                .map(|s| (k.as_str(), s))
        })
        .collect()
}

/// A copy of a binding without what [`values`] refuses, for the check of its form: the keys of
/// [`VALUE_KEYS`] removed, a `secrets:` that is no list removed, a secret that is no name
/// dropped, an egress entry's `secret` that is no name removed.
pub fn without_values(binding: &Value) -> Value {
    fn strip(m: &mut Map<String, Value>) {
        m.retain(|k, _| !VALUE_KEYS.contains(&k.as_str()));
    }
    let mut b = binding.clone();
    let Some(top) = b.as_object_mut() else {
        return b;
    };
    strip(top);
    if let Some(Value::Object(t)) = top.get_mut("target") {
        strip(t);
    }
    for section in ["runtimes", "claims"] {
        if let Some(Value::Object(s)) = top.get_mut(section) {
            for v in s.values_mut() {
                if let Value::Object(m) = v {
                    strip(m);
                }
            }
        }
    }
    if let Some(Value::Object(parts)) = top.get_mut("parts") {
        for entry in parts.values_mut() {
            let Value::Object(m) = entry else { continue };
            strip(m);
            if let Some(Value::Object(im)) = m.get_mut("image") {
                strip(im);
            }
            match m.get_mut("secrets") {
                Some(Value::Array(list)) => list.retain(|s| s.as_str().is_some_and(is_name)),
                Some(_) => {
                    m.remove("secrets");
                }
                None => {}
            }
            if let Some(Value::Array(list)) = m.get_mut("egress") {
                for e in list.iter_mut() {
                    if let Value::Object(em) = e {
                        strip(em);
                        if !em
                            .get("secret")
                            .is_none_or(|s| s.as_str().is_some_and(is_name))
                        {
                            em.remove("secret");
                        }
                    }
                }
            }
        }
    }
    b
}

/// Every finding of secrets and egress for an architecture packed under a binding:
/// `secret_value` ([`values`]); `secret_undeclared`, an egress entry's secret that no part of
/// its unit declares, or that an `env:` value or a `secret_files:` entry of the unit refers to;
/// `egress_undeclared`, a link
/// from a part of a unit to an external part or a part of another unit whose destination that
/// part does not declare.
pub fn check(arch: &Value, binding: &Value, packing: &Packing) -> Vec<Finding> {
    let mut out = values(binding);
    for u in &packing.units {
        let held: BTreeSet<String> = u
            .parts
            .iter()
            .flat_map(|p| declared_secrets(binding, p))
            .collect();
        for p in &u.parts {
            for e in declared_egress(binding, p) {
                if let Some(s) = e.secret.as_deref().filter(|s| is_name(s))
                    && !held.contains(s)
                {
                    out.push(finding(
                        "secret_undeclared",
                        format!(
                            "part {p}: its egress to {} uses secret {s}, and no part of unit {} declares it",
                            e.to, u.name
                        ),
                    ));
                }
            }
        }
        for (at, _, entry) in config::sources(binding, u) {
            for (k, s) in env_refs(entry) {
                if !held.contains(s) {
                    out.push(finding(
                        "secret_undeclared",
                        format!(
                            "{at}: env {k} refers to secret {s}, and no part of unit {} declares it",
                            u.name
                        ),
                    ));
                }
            }
            for (i, s) in file_refs(entry) {
                if !held.contains(s) {
                    out.push(finding(
                        "secret_undeclared",
                        format!(
                            "{at}: secret_files[{i}] refers to secret {s}, and no part of unit {} declares it",
                            u.name
                        ),
                    ));
                }
            }
        }
    }
    for l in arch["links"].as_array().into_iter().flatten() {
        let (from, to) = (text(&l["from"]), text(&l["to"]));
        let Some(a) = packing.unit_of(from) else {
            continue;
        };
        let where_to = if packing.external.iter().any(|e| e == to) {
            "external".to_owned()
        } else {
            match packing.unit_of(to) {
                Some(b) if b.name != a.name => format!("in unit {}", b.name),
                _ => continue,
            }
        };
        if !declared_egress(binding, from).iter().any(|e| e.to == to) {
            out.push(finding(
                "egress_undeclared",
                format!(
                    "link `{}` leaves unit {} for part {to} ({where_to}), and part {from} declares no egress to {to}",
                    link_name(l),
                    a.name
                ),
            ));
        }
    }
    out
}

/// The findings of secrets and egress in the units of a written bundle, against the binding it
/// carries: a key of [`VALUE_KEYS`] in a unit or an egress entry, a unit's secret or an egress
/// entry's secret that is no name (`secret_value`) or that no part of the unit declares
/// (`secret_undeclared`), the same of a secret its `config.env` or `config.secret_files` refers
/// to, an egress entry from a part outside the unit or to a destination its part does not declare
/// (`egress_undeclared`).
pub fn check_units(bundle: &Value) -> Vec<Finding> {
    let binding = &bundle["binding"]["document"];
    let mut out = Vec::new();
    for u in bundle["units"].as_array().into_iter().flatten() {
        let name = text(&u["name"]);
        let parts: Vec<&str> = u["parts"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let held: BTreeSet<String> = parts
            .iter()
            .flat_map(|p| declared_secrets(binding, p))
            .collect();
        if let Some(m) = u.as_object() {
            value_keys(m, &format!("unit {name}"), &mut out);
        }
        let secret = |v: &Value, at: String, out: &mut Vec<Finding>| {
            secret_name(v, &at, out);
            if let Some(s) = v.as_str().filter(|s| is_name(s))
                && !held.contains(s)
            {
                out.push(finding(
                    "secret_undeclared",
                    format!("{at} is secret {s}, and no part of unit {name} declares it"),
                ));
            }
        };
        match &u["secrets"] {
            Value::Array(list) => {
                for (i, s) in list.iter().enumerate() {
                    secret(s, format!("unit {name}: secrets[{i}]"), &mut out);
                }
            }
            Value::Null => {}
            _ => out.push(finding(
                "secret_value",
                format!("unit {name}: secrets is not a list of secret names"),
            )),
        }
        for (k, v) in u["config"]["env"].as_object().into_iter().flatten() {
            if let Some(m) = v.as_object() {
                value_keys(m, &format!("unit {name}: config: env: {k}"), &mut out);
            }
            if let Some(s) = config::secret_ref(v) {
                secret(
                    s,
                    format!("unit {name}: config: env: {k}: secret"),
                    &mut out,
                );
            }
        }
        for (i, f) in u["config"][config::SECRET_FILES]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
        {
            let at = format!("unit {name}: config: secret_files[{i}]");
            if let Some(m) = f.as_object() {
                value_keys(m, &at, &mut out);
            }
            if !f[config::SECRET].is_null() {
                secret(&f[config::SECRET], format!("{at}: secret"), &mut out);
            }
        }
        for (i, e) in u["egress"].as_array().into_iter().flatten().enumerate() {
            let at = format!("unit {name}: egress[{i}]");
            if let Some(m) = e.as_object() {
                value_keys(m, &at, &mut out);
            }
            if !e["secret"].is_null() {
                secret(&e["secret"], format!("{at}: secret"), &mut out);
            }
            let (from, to) = (text(&e["from"]), text(&e["to"]));
            if !parts.contains(&from) {
                out.push(finding(
                    "egress_undeclared",
                    format!("{at} starts at `{from}`, which is no part of unit {name}"),
                ));
            } else if !declared_egress(binding, from).iter().any(|d| d.to == to) {
                out.push(finding(
                    "egress_undeclared",
                    format!("{at} goes to {to}, and part {from} declares no egress to {to}"),
                ));
            }
        }
    }
    out
}
