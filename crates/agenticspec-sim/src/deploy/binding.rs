//! The binding (`agenticspec-binding/1`): for one architecture and one target, which concrete
//! thing plays each part, the transports of links that cross units, the runtimes pinned by
//! digest (or, from the target's local image store, by tag), the claims and check ports of each
//! unit, where each export of an ingress listens (`ingress:`), what the target gives a unit
//! (`target:` isolation, images, network policy), the UIs it asks for (`ui:`, [`super::ui`]),
//! and the packing (`packing:`). A binding may `refines:` a base binding; [`load`] resolves the
//! chain into one document, the child's keys over the base's.

use crate::doc::load_yaml;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const FORMAT: &str = "agenticspec-binding/1";

/// The keys a binding may hold.
const KEYS: &[&str] = &[
    "format",
    "binding",
    "summary",
    "architecture",
    "refines",
    "target",
    "parts",
    "transports",
    "runtimes",
    "claims",
    "checks",
    "ingress",
    "ui",
    "packing",
];

/// The keys of one part's entry under `parts:`. `env`, `args` and `secret_files`, the
/// configuration of its unit, are checked by admission ([`super::config`], `unit_config_invalid`),
/// and so is `scratch`, the scratch directories of its unit ([`super::scratch`],
/// `unit_scratch_invalid`).
const PART_KEYS: &[&str] = &[
    "agent",
    "digest",
    "state",
    "inside",
    "secrets",
    "egress",
    "size",
    "volumes",
    "network",
    "image",
    "env",
    "args",
    "scratch",
    "secret_files",
];

/// The keys of a runtime pinned under `runtimes:`, and of an image a part or a listed unit
/// names (`parts.<p>.image`, `packing.units[].image`).
const RUNTIME_KEYS: &[&str] = &["image", "tag", "digest"];

/// The runtime of the core and service units, under `runtimes:`.
pub const CORE_RUNTIME: &str = "core";

/// The runtime of the agent units, under `runtimes:`: the name of the role it plays.
pub const AGENT_RUNTIME: &str = "agent-runtime";

/// The isolation levels of agenticspec, lowest first (`agentic.shared.IsolationLevel`).
pub const ISOLATION: &[&str] = &["none", "process", "container", "user_kernel", "micro_vm"];

/// The isolation a target gives one unit when the binding names none under `target:`.
pub const DEFAULT_ISOLATION: &str = "container";

/// A binding with its `refines:` chain resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    /// The resolved document, without `refines`.
    pub doc: Value,
    /// Per part that names an agent document, the file it names, read from the directory of the
    /// binding file that named it.
    pub agent_files: BTreeMap<String, PathBuf>,
    /// Per UI of the consumer's own (`ui.<name>.source`), the file its source names, read from
    /// the directory of the binding file that named it.
    pub ui_files: BTreeMap<String, PathBuf>,
}

/// `base` with `over` laid over it: objects merge key by key, any other value replaces.
fn merge(base: &mut Value, over: &Value) {
    match (base, over) {
        (Value::Object(b), Value::Object(o)) => {
            for (k, v) in o {
                merge(b.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (b, o) => *b = o.clone(),
    }
}

/// Read a binding file and every base it refines, nearest last, and resolve them into one.
pub fn load(path: &Path) -> Result<Binding, String> {
    let mut chain: Vec<(Value, PathBuf)> = Vec::new();
    let mut seen: Vec<PathBuf> = Vec::new();
    let mut at = path.to_path_buf();
    loop {
        let key = std::fs::canonicalize(&at).unwrap_or_else(|_| at.clone());
        if seen.contains(&key) {
            return Err(format!(
                "{}: `refines:` comes back to a binding of its own chain",
                at.display()
            ));
        }
        seen.push(key);
        let doc = load_yaml(&at)?;
        if doc["format"] != FORMAT {
            return Err(format!("{}: not {FORMAT}", at.display()));
        }
        let dir = at.parent().unwrap_or(Path::new("")).to_path_buf();
        let next = match &doc["refines"] {
            Value::Null => None,
            Value::String(base) => Some(dir.join(base)),
            _ => {
                return Err(format!(
                    "{}: `refines:` names one binding file",
                    at.display()
                ));
            }
        };
        chain.push((doc, dir));
        match next {
            Some(n) => at = n,
            None => break,
        }
    }
    let mut doc = Value::Object(Map::new());
    let mut agent_files = BTreeMap::new();
    let mut ui_files = BTreeMap::new();
    for (layer, dir) in chain.iter().rev() {
        merge(&mut doc, layer);
        if let Some(parts) = layer["parts"].as_object() {
            for (name, entry) in parts {
                if let Some(file) = entry["agent"].as_str() {
                    agent_files.insert(name.clone(), dir.join(file));
                }
            }
        }
        for (name, entry) in layer["ui"].as_object().into_iter().flatten() {
            let source = &entry["source"];
            if let Some(file) = source["archive"].as_str().or(source["ess_ui"].as_str()) {
                ui_files.insert(name.clone(), dir.join(file));
            }
        }
    }
    if let Value::Object(m) = &mut doc {
        m.remove("refines");
    }
    Ok(Binding {
        doc,
        agent_files,
        ui_files,
    })
}

/// The problems of a resolved binding's form for one architecture document, one line each:
/// an unknown key, a part that is not in the document, a value of the wrong kind. Empty when
/// the form is right. The packing section is checked by [`crate::deploy::packing::resolve`].
pub fn problems(binding: &Value, arch: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let Some(top) = binding.as_object() else {
        return vec!["a binding is a mapping".into()];
    };
    for k in top.keys() {
        if !KEYS.contains(&k.as_str()) {
            out.push(format!("unknown key `{k}`"));
        }
    }
    if !binding["binding"].is_string() {
        out.push("`binding:` names the binding".into());
    }
    if let Some(name) = binding["architecture"].as_str()
        && Some(name) != arch["architecture"].as_str()
    {
        out.push(format!(
            "`architecture: {name}` is not the document's `{}`",
            arch["architecture"].as_str().unwrap_or("")
        ));
    }
    match &binding["target"] {
        Value::Null => {}
        Value::Object(t) => {
            for (k, v) in t {
                match k.as_str() {
                    "isolation" if v.as_str().is_some_and(|l| ISOLATION.contains(&l)) => {}
                    "isolation" => out.push(format!(
                        "target: isolation is one of {}",
                        ISOLATION.join(", ")
                    )),
                    "images" if v.as_str().is_some_and(|m| IMAGES.contains(&m)) => {}
                    "images" => out.push(format!("target: images is one of {}", IMAGES.join(", "))),
                    "network_policy" if v.as_str().is_some_and(|m| NETWORK_POLICY.contains(&m)) => {
                    }
                    "network_policy" => out.push(format!(
                        "target: network_policy is one of {}",
                        NETWORK_POLICY.join(", ")
                    )),
                    other => out.push(format!("target: unknown key `{other}`")),
                }
            }
        }
        _ => out.push("`target:` is a mapping".into()),
    }
    let parts = &arch["parts"];
    let local = images(binding) == "local";
    if let Some(entries) = binding["parts"].as_object() {
        for (name, entry) in entries {
            if parts.get(name).is_none() {
                out.push(format!("parts: `{name}` is not a part of the document"));
            }
            let Some(entry) = entry.as_object() else {
                out.push(format!("parts: `{name}` is a mapping"));
                continue;
            };
            for (k, v) in entry {
                if !PART_KEYS.contains(&k.as_str()) {
                    out.push(format!("parts: {name}: unknown key `{k}`"));
                    continue;
                }
                if k == "egress" {
                    egress_problems(name, v, parts, &mut out);
                    continue;
                }
                if k == "volumes" {
                    volume_problems(name, v, parts, &mut out);
                    continue;
                }
                if k == "image" {
                    image_problems(&format!("parts: {name}: image"), v, local, &mut out);
                    continue;
                }
                if super::config::KEYS.contains(&k.as_str()) || k == super::scratch::KEY {
                    continue;
                }
                let ok = match k.as_str() {
                    "state" => matches!(v.as_str(), Some("memory" | "service")),
                    "secrets" => v.as_array().is_some_and(|a| a.iter().all(Value::is_string)),
                    "network" => v == "isolated",
                    _ => v.is_string(),
                };
                if !ok {
                    out.push(match k.as_str() {
                        "state" => format!("parts: {name}: state is memory or service"),
                        "secrets" => format!("parts: {name}: secrets is a list of names"),
                        "network" => format!("parts: {name}: network is `isolated`"),
                        _ => format!("parts: {name}: `{k}` is text"),
                    });
                }
            }
        }
    } else if !binding["parts"].is_null() {
        out.push("`parts:` is a mapping".into());
    }
    if let Some(t) = binding["transports"].as_object() {
        for (carries, v) in t {
            if !v.is_string() {
                out.push(format!("transports: `{carries}` names one transport"));
            }
        }
    } else if !binding["transports"].is_null() {
        out.push("`transports:` is a mapping from a carried value to a transport".into());
    }
    if let Some(r) = binding["runtimes"].as_object() {
        for (name, v) in r {
            if name == "agent" {
                out.push(format!(
                    "runtimes: `agent` is named `{AGENT_RUNTIME}`, the role the agent units' runtime plays"
                ));
            }
            if !v.is_object() {
                out.push(format!("runtimes: `{name}` is a mapping"));
                continue;
            }
            image_problems(&format!("runtimes: {name}"), v, local, &mut out);
        }
    } else if !binding["runtimes"].is_null() {
        out.push("`runtimes:` is a mapping".into());
    }
    for u in binding["packing"]["units"].as_array().into_iter().flatten() {
        if let Some(v) = u.get("image") {
            let at = format!("packing: unit {}: image", u["name"].as_str().unwrap_or(""));
            image_problems(&at, v, local, &mut out);
        }
    }
    if let Some(c) = binding["claims"].as_object() {
        for (unit, v) in c {
            let Some(m) = v.as_object() else {
                out.push(format!("claims: `{unit}` is a mapping"));
                continue;
            };
            for (k, x) in m {
                let positive = x.as_u64().is_some_and(|n| n > 0);
                match k.as_str() {
                    "cpu_millis" | "memory_mib" if positive => {}
                    "accelerators" if x.as_u64().is_some() => {}
                    "cpu_millis" | "memory_mib" | "accelerators" => {
                        out.push(format!("claims: {unit}: `{k}` is a whole number above 0"))
                    }
                    other => out.push(format!("claims: {unit}: unknown key `{other}`")),
                }
            }
        }
    } else if !binding["claims"].is_null() {
        out.push("`claims:` is a mapping from a unit name to its claims".into());
    }
    if let Some(c) = binding["checks"].as_object() {
        for (unit, v) in c {
            let Some(m) = v.as_object() else {
                out.push(format!("checks: `{unit}` is a mapping"));
                continue;
            };
            for (k, x) in m {
                if !CHECKS.contains(&k.as_str()) {
                    out.push(format!(
                        "checks: {unit}: `{k}` is not one of {}",
                        CHECKS.join(", ")
                    ));
                    continue;
                }
                let Some(cm) = x.as_object() else {
                    out.push(format!("checks: {unit}: {k} is a mapping"));
                    continue;
                };
                for (ck, cv) in cm {
                    match ck.as_str() {
                        "port" if is_port(cv) => {}
                        "port" => out.push(format!(
                            "checks: {unit}: {k}: port is a whole number from 1 to 65535"
                        )),
                        other => out.push(format!("checks: {unit}: {k}: unknown key `{other}`")),
                    }
                }
            }
        }
    } else if !binding["checks"].is_null() {
        out.push("`checks:` is a mapping from a unit name to its checks".into());
    }
    if let Some(i) = binding["ingress"].as_object() {
        for (name, v) in i {
            ingress_problems(name, v, parts, &mut out);
        }
    } else if !binding["ingress"].is_null() {
        out.push("`ingress:` is a mapping from an ingress name to its exports".into());
    }
    super::ui::problems(binding, &mut out);
    out
}

/// The problems of one image the binding pins, `{image, tag?, digest}`, at `at` (`runtimes:
/// core`, `parts: <p>: image`, `packing: unit <u>: image`). From a registry it is pinned by its
/// repository digest and takes no tag; from the target's local store by its tag, its digest (the
/// image's config id) or both, and one with neither is refused by admission (`image_unpinned`).
fn image_problems(at: &str, v: &Value, local: bool, out: &mut Vec<String>) {
    let Some(m) = v.as_object() else {
        out.push(format!("{at} is a mapping: {{image, tag, digest}}"));
        return;
    };
    for k in m.keys() {
        if !RUNTIME_KEYS.contains(&k.as_str()) {
            out.push(format!("{at}: unknown key `{k}`"));
        }
    }
    let digest = m.get("digest");
    if local {
        if digest.is_some_and(|d| !d.as_str().is_some_and(is_digest)) {
            out.push(format!(
                "{at}: `digest:` is sha256:<64 hex digits>, the image's config id"
            ));
        }
        if m.get("tag")
            .is_some_and(|t| !t.as_str().is_some_and(is_tag))
        {
            out.push(format!(
                "{at}: `tag:` is an image tag ([A-Za-z0-9_][A-Za-z0-9_.-]*, at most 128)"
            ));
        }
    } else {
        if !digest.and_then(Value::as_str).is_some_and(is_digest) {
            out.push(format!(
                "{at}: `digest:` is sha256:<64 hex digits>; a runtime is pinned by its digest"
            ));
        }
        if m.contains_key("tag") {
            out.push(format!(
                "{at}: `tag:` goes with `target: {{images: local}}`; a registry image is pinned by its digest"
            ));
        }
    }
    if !m.get("image").and_then(Value::as_str).is_some_and(is_image) {
        out.push(format!(
            "{at}: `image:` names the image without tag or digest (`registry/name`); the digest goes in `digest:`"
        ));
    }
}

/// The checks of a unit a binding may give a port.
pub const CHECKS: &[&str] = &["startup", "ready", "alive"];

/// How an export listens: on the loopback interface only, or on every interface.
pub const BIND: &[&str] = &["loopback", "all"];

/// The bind of an export the binding names none for.
pub const DEFAULT_BIND: &str = "loopback";

/// Where a target takes runtime images from: a registry (pinned by repository digest) or the
/// target's local image store (pinned by tag or config id). The first is the default.
pub const IMAGES: &[&str] = &["registry", "local"];

/// Whether the target enforces a network policy.
pub const NETWORK_POLICY: &[&str] = &["enforced", "not_enforced"];

/// A port: a whole number from 1 to 65535.
pub fn is_port(v: &Value) -> bool {
    v.as_u64().is_some_and(|p| (1..=65535).contains(&p))
}

/// The keys of one export under `ingress.<name>.exports.<protocol>`.
const EXPORT_KEYS: &[&str] = &["port", "bind", "address", "part"];

/// The problems of one ingress's entry: `{exports: {<protocol>: {port, bind, address, part}}}`.
/// Whether the ingress and its protocols are the contract's is checked by [`served_problems`].
fn ingress_problems(name: &str, v: &Value, parts: &Value, out: &mut Vec<String>) {
    let Some(m) = v.as_object() else {
        out.push(format!("ingress: `{name}` is a mapping"));
        return;
    };
    for k in m.keys().filter(|k| *k != "exports") {
        out.push(format!("ingress: {name}: unknown key `{k}`"));
    }
    let Some(exports) = m.get("exports").and_then(Value::as_object) else {
        out.push(format!(
            "ingress: {name}: `exports:` is a mapping from a protocol to its port, bind and address"
        ));
        return;
    };
    for (protocol, e) in exports {
        let at = format!("ingress: {name}: exports: {protocol}");
        let Some(em) = e.as_object() else {
            out.push(format!("{at} is a mapping"));
            continue;
        };
        for (k, x) in em {
            if !EXPORT_KEYS.contains(&k.as_str()) {
                out.push(format!("{at}: unknown key `{k}`"));
                continue;
            }
            let ok = match k.as_str() {
                "port" => is_port(x),
                "bind" => x.as_str().is_some_and(|b| BIND.contains(&b)),
                "address" => x.as_str().is_some_and(is_text),
                _ => x.as_str().is_some_and(|p| parts.get(p).is_some()),
            };
            if !ok {
                out.push(match k.as_str() {
                    "port" => format!("{at}: port is a whole number from 1 to 65535"),
                    "bind" => format!("{at}: bind is one of {}", BIND.join(", ")),
                    "address" => format!("{at}: address is text without spaces"),
                    _ => format!("{at}: part names a part of the document"),
                });
            }
        }
    }
}

/// The problems of a binding that need the contract and the packing: a claim or a check of a
/// unit the packing does not have (nor the `service` unit of a UI the binding asks for, named by
/// its role or its key), a check its unit's kind has not, an ingress or a protocol
/// the contract does not export, an export served by a part no unit holds. `contract_ingress`
/// is the contract's `ingress` as data.
pub fn served_problems(
    binding: &Value,
    contract_ingress: &Value,
    packing: &super::packing::Packing,
) -> Vec<String> {
    let mut out = Vec::new();
    // The kind of a unit by name: a unit of the packing, or the `service` unit of a UI the
    // binding asks for.
    let ui_units: Vec<String> = super::ui::asked(binding)
        .into_iter()
        .map(|u| u.name)
        .collect();
    let kind = |name: &str| {
        packing
            .units
            .iter()
            .find(|u| u.name == name)
            .map(|u| u.kind.as_str())
            .or_else(|| ui_units.iter().any(|u| u == name).then_some("service"))
    };
    for name in binding["claims"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, _)| k)
    {
        if kind(name).is_none() {
            out.push(format!("claims: `{name}` is not a unit of the packing"));
        }
    }
    for (name, v) in binding["checks"].as_object().into_iter().flatten() {
        let Some(kind) = kind(name) else {
            out.push(format!("checks: `{name}` is not a unit of the packing"));
            continue;
        };
        let has = unit_checks(kind);
        for k in v.as_object().into_iter().flatten().map(|(k, _)| k) {
            if CHECKS.contains(&k.as_str()) && !has.contains(&k.as_str()) {
                out.push(format!("checks: {name}: a {kind} unit has no {k} check"));
            }
        }
    }
    let ingresses: Vec<&Value> = contract_ingress.as_array().into_iter().flatten().collect();
    for (name, v) in binding["ingress"].as_object().into_iter().flatten() {
        let Some(i) = ingresses.iter().find(|i| i["name"] == name.as_str()) else {
            out.push(format!(
                "ingress: `{name}` is not an ingress of the contract"
            ));
            continue;
        };
        for (protocol, e) in v["exports"].as_object().into_iter().flatten() {
            if !i["exports"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|p| p == protocol.as_str())
            {
                out.push(format!(
                    "ingress: {name}: exports: `{protocol}` is not an export of the ingress"
                ));
            }
            if let Some(p) = e["part"].as_str()
                && packing.unit_of(p).is_none()
            {
                out.push(format!(
                    "ingress: {name}: exports: {protocol}: part {p} is in no unit of the packing"
                ));
            }
        }
    }
    for (p, entry) in binding["parts"].as_object().into_iter().flatten() {
        if entry.get("image").is_none() {
            continue;
        }
        match packing.unit_of(p) {
            Some(u) if IMAGE_KINDS.contains(&u.kind.as_str()) => {}
            Some(u) => out.push(format!(
                "parts: {p}: image is for a part of a core or service unit, and {p} is in the {} unit {}",
                u.kind, u.name
            )),
            None => out.push(format!(
                "parts: {p}: image is for a part of a core or service unit, and {p} is in no unit"
            )),
        }
    }
    for u in &packing.units {
        if listed_image(binding, &u.name).is_some() && !IMAGE_KINDS.contains(&u.kind.as_str()) {
            out.push(format!(
                "packing: unit {}: image is for a core or service unit, and it is a {} unit",
                u.name, u.kind
            ));
        }
    }
    out.extend(super::ui::served_problems(binding, packing));
    out
}

/// The checks a unit of a kind has (`startup`, `ready`, `alive`).
pub fn unit_checks(kind: &str) -> &'static [&'static str] {
    match kind {
        "agent" | "job" | "scheduled" => &["startup"],
        "core" | "service" => &["ready", "alive"],
        "model" => &["ready"],
        _ => &[],
    }
}

/// The problems of one part's `volumes:`: a list of `{name, mount}`, each naming a workspace
/// part of the document or the durable state of a runtime part, `<part>-state`
/// ([`state_owner`]). Whether a mount is an absolute path is admission's
/// (`volume_mount_missing`).
fn volume_problems(part: &str, v: &Value, parts: &Value, out: &mut Vec<String>) {
    let Some(list) = v.as_array() else {
        out.push(format!(
            "parts: {part}: volumes is a list of {{name, mount}}"
        ));
        return;
    };
    for (i, e) in list.iter().enumerate() {
        let at = format!("parts: {part}: volumes[{i}]");
        let Some(m) = e.as_object() else {
            out.push(format!("{at} is {{name, mount}}"));
            continue;
        };
        for (k, x) in m {
            match k.as_str() {
                "name" => {
                    if !x.as_str().is_some_and(|n| {
                        parts[n]["block"] == "workspace" || state_owner(parts, n).is_some()
                    }) {
                        out.push(format!(
                            "{at}: name names a workspace part of the document, or `<part>-state` of a runtime part with `durable_state: true`"
                        ));
                    }
                }
                "mount" if x.is_string() => {}
                "mount" => out.push(format!("{at}: mount is a path")),
                other => out.push(format!("{at}: unknown key `{other}`")),
            }
        }
        if !m.contains_key("name") {
            out.push(format!(
                "{at} needs a `name:`, the workspace or durable state it mounts"
            ));
        }
    }
}

/// Text without spaces, not empty.
pub fn is_text(s: &str) -> bool {
    !s.is_empty() && !s.chars().any(char::is_whitespace)
}

/// A network address an egress entry may name: a CIDR block (`10.0.0.0/8`, `fd00::/8`) or
/// `<host>:<port>` (a host name, an IPv4 address or a bracketed IPv6 address).
pub fn is_address(s: &str) -> bool {
    use std::net::IpAddr;
    if let Some((ip, bits)) = s.split_once('/') {
        let Ok(bits) = bits.parse::<u8>() else {
            return false;
        };
        return match ip.parse::<IpAddr>() {
            Ok(IpAddr::V4(_)) => bits <= 32,
            Ok(IpAddr::V6(_)) => bits <= 128,
            Err(_) => false,
        };
    }
    let Some((host, port)) = s.rsplit_once(':') else {
        return false;
    };
    if !port.parse::<u16>().is_ok_and(|p| p > 0) {
        return false;
    }
    if let Some(v6) = host.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
        return v6.parse::<std::net::Ipv6Addr>().is_ok();
    }
    !host.is_empty()
        && !host.starts_with(['-', '.'])
        && host
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.'))
}

/// The problems of an egress entry's `address`: a string ([`is_address`]) or
/// `{namespace, labels}` (at least one; `labels` a mapping from text to text).
fn address_problems(at: &str, v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) if is_address(s) => {}
        Value::String(_) => out.push(format!(
            "{at}: address is a CIDR block or <host>:<port>, or {{namespace, labels}}"
        )),
        Value::Object(m) => {
            if m.is_empty() {
                out.push(format!("{at}: address names a namespace, labels or both"));
            }
            for (k, x) in m {
                let ok = match k.as_str() {
                    "namespace" => x.as_str().is_some_and(is_text),
                    "labels" => x.as_object().is_some_and(|l| {
                        l.iter()
                            .all(|(lk, lv)| is_text(lk) && lv.as_str().is_some_and(is_text))
                    }),
                    _ => {
                        out.push(format!("{at}: address: unknown key `{k}`"));
                        continue;
                    }
                };
                if !ok {
                    out.push(format!(
                        "{at}: address: {k} is {}",
                        if k == "labels" {
                            "a mapping from text to text"
                        } else {
                            "text"
                        }
                    ));
                }
            }
        }
        _ => out.push(format!(
            "{at}: address is a CIDR block or <host>:<port>, or {{namespace, labels}}"
        )),
    }
}

/// The problems of one part's `egress:`: a list of destinations, each a part of the document or
/// a name the target resolves ([`super::grants::is_name`]), written as the name or as
/// `{to: <name>, secret: <secret name>, address: <address>}`, `address` only for a name outside
/// the document ([`is_address`], or `{namespace, labels}`). Whether a secret is a name is checked
/// by [`super::grants::values`].
fn egress_problems(part: &str, v: &Value, parts: &Value, out: &mut Vec<String>) {
    let Some(list) = v.as_array() else {
        out.push(format!("parts: {part}: egress is a list of destinations"));
        return;
    };
    let destination = |to: &str| parts.get(to).is_some() || super::grants::is_name(to);
    for (i, e) in list.iter().enumerate() {
        let to = match e {
            Value::String(to) => Some(to.as_str()),
            Value::Object(m) => {
                for k in m.keys() {
                    if !["to", "secret", "address"].contains(&k.as_str()) {
                        out.push(format!("parts: {part}: egress[{i}]: unknown key `{k}`"));
                    }
                }
                let to = m.get("to").and_then(Value::as_str);
                if let Some(a) = m.get("address") {
                    let at = format!("parts: {part}: egress[{i}]");
                    if to.is_some_and(|t| parts.get(t).is_some()) {
                        out.push(format!(
                            "{at}: address is for a destination outside the document; an external part's address is its `reach`"
                        ));
                    } else {
                        address_problems(&at, a, out);
                    }
                }
                to
            }
            _ => None,
        };
        match to {
            Some(to) if destination(to) => {}
            Some(_) => out.push(format!(
                "parts: {part}: egress[{i}] names neither a part of the document nor a destination name (^[a-z][a-z0-9-]*$)"
            )),
            None => out.push(format!(
                "parts: {part}: egress[{i}] is a destination name or {{to: <name>, secret: <name>}}"
            )),
        }
    }
}

/// An image name without tag or digest: not empty, no space, no `@`, and no `:` after its last
/// `/` (a `:` before it is a registry's port).
pub fn is_image(s: &str) -> bool {
    let last = s.rsplit('/').next().unwrap_or(s);
    !s.is_empty() && !last.is_empty() && !s.contains(['@', ' ']) && !last.contains(':')
}

/// Where the target takes runtime images from (`target.images`): `registry` unless the binding
/// names `local`.
pub fn images(binding: &Value) -> &str {
    match binding["target"]["images"].as_str() {
        Some("local") => "local",
        _ => "registry",
    }
}

/// The image reference of a runtime the binding pins. From a registry `<image>@<digest>`; from
/// the target's local store `<image>:<tag>` when it names a tag, else its digest (the image's
/// config id, which a local engine runs by). `None` when it is not pinned.
pub fn image_reference(binding: &Value, runtime: &str) -> Option<String> {
    pinned_reference(binding, &binding["runtimes"][runtime])
}

/// The image reference of one image the binding pins, `{image, tag?, digest}`, as
/// [`image_reference`] gives it.
pub fn pinned_reference(binding: &Value, r: &Value) -> Option<String> {
    let image = r["image"].as_str()?;
    if images(binding) == "local" {
        return match (r["tag"].as_str(), r["digest"].as_str()) {
            (Some(tag), _) => Some(format!("{image}:{tag}")),
            (None, Some(d)) => Some(d.to_owned()),
            (None, None) => None,
        };
    }
    Some(format!("{image}@{}", r["digest"].as_str()?))
}

/// The key an image is carried under in a bundle's `runtimes`: from a registry its digest; from
/// the target's local store its digest, else `<image>:<tag>`.
pub fn image_key(binding: &Value, r: &Value) -> String {
    let image = r["image"].as_str().unwrap_or("");
    match (images(binding), r["digest"].as_str()) {
        (_, Some(d)) => d.to_owned(),
        ("local", None) => format!("{image}:{}", r["tag"].as_str().unwrap_or("")),
        _ => String::new(),
    }
}

/// The kinds of unit that take a runtime image of their own (`parts.<p>.image`,
/// `packing.units[].image`).
pub const IMAGE_KINDS: &[&str] = &["core", "service"];

/// The block of a part that, held by a core or service unit, needs an image
/// (`unit_image_missing`): a runtime part.
pub const RUNTIME_BLOCK: &str = "runtime";

/// Whether a part is a runtime part that keeps durable state of its own (`durable_state: true`,
/// a marked place judged by its value).
pub fn durable_state(part: &Value) -> bool {
    part["block"] == RUNTIME_BLOCK
        && *crate::check::value(&part["durable_state"]) == Value::Bool(true)
}

/// The setting of a runtime part that names the programs outside the runtime that also write its
/// durable state (R109); only beside `durable_state: true`.
pub const STATE_WRITERS: &str = "state_writers";

/// The outside writers of a runtime part's durable state (`state_writers`, a marked place judged
/// by its value), when it keeps durable state and names any.
pub fn state_writers(part: &Value) -> Option<&Value> {
    let w = crate::check::value(&part[STATE_WRITERS]);
    (durable_state(part) && w.as_array().is_some_and(|xs| !xs.is_empty())).then_some(w)
}

/// The name of the volume that holds the durable state of runtime part `part`: `<part>-state`.
pub fn state_volume(part: &str) -> String {
    format!("{part}-state")
}

/// The runtime part with durable state whose volume `name` is (`<part>-state`), if any.
pub fn state_owner<'a>(parts: &Value, name: &'a str) -> Option<&'a str> {
    name.strip_suffix("-state")
        .filter(|p| durable_state(&parts[*p]))
}

/// Where the image of a core or service unit comes from.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitImage<'a> {
    /// `image:` of the unit as `packing.units` lists it; it wins over its parts' images.
    Unit(&'a Value),
    /// `parts.<p>.image` of the unit's parts that name one, all the same image.
    Parts(&'a Value),
    /// `runtimes.core`, for the core unit only.
    Core(&'a Value),
    /// The unit's parts that name an image, by name, when they do not all name the same one
    /// (`unit_image_conflict`).
    Conflict(Vec<String>),
    /// None of them.
    None,
}

impl<'a> UnitImage<'a> {
    /// The image, when there is one.
    pub fn image(&self) -> Option<&'a Value> {
        match self {
            UnitImage::Unit(v) | UnitImage::Parts(v) | UnitImage::Core(v) => Some(v),
            UnitImage::Conflict(_) | UnitImage::None => None,
        }
    }
}

/// `image:` of a unit as the binding's `packing.units` lists it, by its name.
pub fn listed_image<'a>(binding: &'a Value, unit: &str) -> Option<&'a Value> {
    binding["packing"]["units"]
        .as_array()?
        .iter()
        .find(|u| u["name"] == unit)
        .and_then(|u| u.get("image"))
}

/// The image of a core or service unit: its own (`packing.units[].image`), else the one its
/// parts name (`parts.<p>.image`, all alike), else for the core unit `runtimes.core`. `runtimes.core`
/// is never the image of a service unit. Any other kind of unit has none.
pub fn unit_image<'a>(binding: &'a Value, unit: &super::packing::Unit) -> UnitImage<'a> {
    if !IMAGE_KINDS.contains(&unit.kind.as_str()) {
        return UnitImage::None;
    }
    if let Some(v) = listed_image(binding, &unit.name) {
        return UnitImage::Unit(v);
    }
    let named: Vec<(&String, &Value)> = unit
        .parts
        .iter()
        .filter_map(|p| binding["parts"][p].get("image").map(|v| (p, v)))
        .collect();
    if let Some((_, first)) = named.first() {
        if named.iter().all(|(_, v)| v == first) {
            return UnitImage::Parts(first);
        }
        return UnitImage::Conflict(named.into_iter().map(|(p, _)| p.clone()).collect());
    }
    match &binding["runtimes"][CORE_RUNTIME] {
        v @ Value::Object(_) if unit.kind == "core" => UnitImage::Core(v),
        _ => UnitImage::None,
    }
}

/// An image tag: `[A-Za-z0-9_][A-Za-z0-9_.-]*`, at most 128 characters.
pub fn is_tag(s: &str) -> bool {
    s.len() <= 128
        && s.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

/// `sha256:` and 64 lower-case hex digits.
pub fn is_digest(s: &str) -> bool {
    s.strip_prefix("sha256:").is_some_and(|h| {
        h.len() == 64
            && h.chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
    })
}

/// The isolation level the target gives one unit.
pub fn target_isolation(binding: &Value) -> &str {
    binding["target"]["isolation"]
        .as_str()
        .unwrap_or(DEFAULT_ISOLATION)
}

/// The rank of an isolation level, lowest 0; an unknown level ranks lowest.
pub fn isolation_rank(level: &str) -> usize {
    ISOLATION.iter().position(|l| *l == level).unwrap_or(0)
}
