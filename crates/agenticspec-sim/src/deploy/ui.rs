//! The UIs of a bundle: one unit per UI the binding asks for under `ui:`, a `service` unit that
//! serves static files (`code: {kind: static, archive, digest}`) and reads the adapter of an
//! upstream unit (`upstream: {unit, port, path, secret?}`, `path` from `ui.<ui>.upstream.path`,
//! `/api/` when absent; `secret` from `ui.<ui>.upstream.secret`, the name of the credential the UI
//! presents to that unit, which a part of that unit declares). The archive is carried inline in
//! the bundle's `files` member, base64, by its file name. The UI unit serves its export of an
//! ingress; the other exports stay where they were.
//!
//! A UI is one row ([`Ui`]): its key under `ui:`, its unit's name and role, the ingress and export
//! it serves, and the source of its files ([`Source`]). Two sources are built in, keyed
//! `operator` and `control-plane` ([`BUILT_IN`]): the operator UI and the control-plane UI (the
//! operator UI with the pages across parts, `ui build --control-plane`), built in-process with the
//! library `agenticspec ui build` uses, the same bytes as `ui build --archive` writes, both serving
//! the operator ingress's `ui` export. Any other key is a UI of the consumer's own,
//! `ui.<name>: {source, export, upstream?, port?, bind?}`: its source is a static archive it built
//! (`{archive: <file>, digest}`), or an `ess-ui/1` document (`{ess_ui: <file>}`), which is
//! refused because its pages need node and vite to build. The first UI the binding asks for that
//! serves an export serves it, the built-in ones first and then the others by key; another one
//! beside it listens on the port `ui.<ui>.port` gives it. A binding without `ui:` gives the bundle
//! it gave before.

use super::admit::{Accepts, Finding};
use super::binding::{BIND, Binding, is_digest, is_port, is_text};
use super::bundle::CONTROL;
use super::grants;
use super::packing::Packing;
use super::sha256;
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};

/// The files of a UI from the architecture's file and its contract: by name, sorted.
pub type Build = fn(&Path, &Value, &str) -> Result<Vec<(String, Vec<u8>)>, String>;

/// A UI the binary builds itself.
pub struct BuiltIn {
    /// Its key under the binding's `ui:`.
    pub key: &'static str,
    /// The role of its unit, which is also the unit's name.
    pub role: &'static str,
    pub build: Build,
}

/// The UIs the binary builds itself. Both serve the operator ingress's `ui` export.
pub const BUILT_IN: &[BuiltIn] = &[
    BuiltIn {
        key: "operator",
        role: "operator-ui",
        build: operator_files,
    },
    BuiltIn {
        key: "control-plane",
        role: "control-plane-ui",
        build: control_plane_files,
    },
];

/// The ingress and export a built-in UI serves.
const BUILT_IN_EXPORT: (&str, &str) = ("operator", "ui");

/// The role of the unit of a UI of the consumer's own.
pub const OWN_ROLE: &str = "ui";

/// The roles of UI units.
pub const ROLES: &[&str] = &["operator-ui", "control-plane-ui", OWN_ROLE];

/// The export of a UI's ingress whose unit is its default upstream.
const API: &str = "http";

/// Where the files of a UI come from.
#[derive(Clone)]
pub enum Source {
    /// Built by the binary from the architecture and its contract.
    Built(Build),
    /// An `ess-ui/1` document, as the binding names it.
    EssUi(String),
    /// A static archive the consumer built, as the binding names it, and its digest.
    Archive { file: String, digest: String },
}

/// One UI the binding asks for.
#[derive(Clone)]
pub struct Ui {
    /// Its key under the binding's `ui:`.
    pub key: String,
    /// The name of its unit: the role of a built-in UI, else the key.
    pub name: String,
    /// The role of its unit.
    pub role: &'static str,
    /// The ingress whose export it serves.
    pub ingress: String,
    /// The export of that ingress it serves.
    pub export: String,
    pub source: Source,
}

/// The base path of the runtime's adapter a UI reads when its binding names none
/// (`ui.<ui>.upstream.path`), written into its `config.json` and its unit's `upstream.path`.
pub const API_BASE: &str = agenticspec_ui::operator::API_BASE;

/// Whether `path` may be the base path a UI reads its upstream's adapter at: it starts with `/`.
fn is_upstream_path(path: &Value) -> bool {
    path.as_str().is_some_and(|p| p.starts_with('/'))
}

/// The keys of a built-in UI's entry under `ui:`.
const KEYS: &[&str] = &["port", "bind", "upstream"];

/// The keys of the entry of a UI of the consumer's own.
const OWN_KEYS: &[&str] = &["source", "export", "upstream", "port", "bind"];

/// The keys of its `upstream:`.
const UPSTREAM_KEYS: &[&str] = &["unit", "port", "path", "secret"];

/// The refusals of a written bundle that a build cannot reach: the archive a UI unit names is not
/// the archive the bundle carries.
pub const CHECK_REFUSALS: &[&str] = &["ui_archive_mismatch"];

fn operator_files(
    architecture: &Path,
    contract: &Value,
    api_base: &str,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    ui_files(architecture, contract, api_base, None)
}

fn control_plane_files(
    architecture: &Path,
    contract: &Value,
    api_base: &str,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    let control = agenticspec_ui::derive::ControlInputs::default();
    ui_files(architecture, contract, api_base, Some(&control))
}

/// The files of `agenticspec ui build` (with `--control-plane` when `control` is given).
fn ui_files(
    architecture: &Path,
    contract: &Value,
    api_base: &str,
    control: Option<&agenticspec_ui::derive::ControlInputs<'_>>,
) -> Result<Vec<(String, Vec<u8>)>, String> {
    use agenticspec_ui::derive::Failure;
    let root = architecture.parent().unwrap_or(Path::new("."));
    agenticspec_ui::operator::build_ui(root, architecture, contract, "contract", api_base, control)
        .map_err(|f| match f {
            Failure::Unreadable(e) => e,
            Failure::Refused(rs) => rs
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        })
}

/// Whether `name` may name the unit of a UI of the consumer's own: lower-case letters, digits and
/// `-`, starting with a letter, at most 63 bytes.
fn is_unit_name(name: &str) -> bool {
    name.len() < 64
        && name.starts_with(|c: char| c.is_ascii_lowercase())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The ingress and protocol of `<ingress>.<protocol>`.
fn split_export(v: &Value) -> Option<(&str, &str)> {
    v.as_str()?
        .split_once('.')
        .filter(|(i, p)| is_text(i) && is_text(p) && !p.contains('.'))
}

/// The source of a UI of the consumer's own, when it is of the right form.
fn own_source(v: &Value) -> Option<Source> {
    let m = v.as_object()?;
    match (&m.get("ess_ui"), &m.get("archive"), &m.get("digest")) {
        (Some(Value::String(f)), None, None) if is_text(f) => Some(Source::EssUi(f.clone())),
        (None, Some(Value::String(f)), Some(Value::String(d))) if is_text(f) && is_digest(d) => {
            Some(Source::Archive {
                file: f.clone(),
                digest: d.clone(),
            })
        }
        _ => None,
    }
}

/// Whether `u` serves its export: the first of `all` (the UIs the binding asks for, in their
/// order) that serves that export of that ingress does; another one beside it listens on a port
/// of its own.
pub fn serves_export(u: &Ui, all: &[Ui]) -> bool {
    all.iter()
        .find(|x| x.ingress == u.ingress && x.export == u.export)
        .is_some_and(|x| x.key == u.key)
}

/// The file name of a UI's archive among the bundle's `files`: `<unit>.tar`.
pub fn archive_name(u: &Ui) -> String {
    format!("{}.tar", u.name)
}

/// The UIs the binding asks for: the built-in ones in the order of [`BUILT_IN`], then the
/// consumer's own by key. An entry of the consumer's own of the wrong form is left out
/// ([`problems`] names it).
pub fn asked(binding: &Value) -> Vec<Ui> {
    let Some(ui) = binding["ui"].as_object() else {
        return Vec::new();
    };
    let mut out: Vec<Ui> = BUILT_IN
        .iter()
        .filter(|b| ui.contains_key(b.key))
        .map(|b| Ui {
            key: b.key.to_owned(),
            name: b.role.to_owned(),
            role: b.role,
            ingress: BUILT_IN_EXPORT.0.to_owned(),
            export: BUILT_IN_EXPORT.1.to_owned(),
            source: Source::Built(b.build),
        })
        .collect();
    for (key, v) in ui {
        if BUILT_IN.iter().any(|b| b.key == key) || !is_unit_name(key) {
            continue;
        }
        let (Some((ingress, export)), Some(source)) =
            (split_export(&v["export"]), own_source(&v["source"]))
        else {
            continue;
        };
        out.push(Ui {
            key: key.clone(),
            name: key.clone(),
            role: OWN_ROLE,
            ingress: ingress.to_owned(),
            export: export.to_owned(),
            source,
        });
    }
    out
}

/// The problems of the form of the binding's `ui:`, one line each: `{<ui>: {port?, bind?,
/// upstream?: {unit?, port?, path?}}}` for a built-in UI, and with `source` and `export` for a UI
/// of the consumer's own. A `path` that is text but does not start with `/` is a finding of
/// admission ([`findings`], `ui_upstream_path_invalid`).
pub fn problems(binding: &Value, out: &mut Vec<String>) {
    let ui = match &binding["ui"] {
        Value::Null => return,
        Value::Object(m) => m,
        _ => {
            out.push("`ui:` is a mapping from a UI to where it listens and what it reads".into());
            return;
        }
    };
    for (key, v) in ui {
        let built_in = BUILT_IN.iter().any(|b| b.key == key);
        if !built_in && !is_unit_name(key) {
            out.push(format!(
                "ui: `{key}` is neither a built-in UI ({}) nor a unit name (lower-case letters, digits and `-`, starting with a letter)",
                BUILT_IN.iter().map(|b| b.key).collect::<Vec<_>>().join(", ")
            ));
            continue;
        }
        let entry = match v {
            Value::Null if built_in => continue,
            Value::Object(m) => m,
            _ if built_in => {
                out.push(format!("ui: `{key}` is a mapping"));
                continue;
            }
            _ => {
                out.push(format!(
                    "ui: `{key}` is a mapping with `source` and `export`"
                ));
                continue;
            }
        };
        let keys = if built_in { KEYS } else { OWN_KEYS };
        for (k, x) in entry {
            match k.as_str() {
                "port" if is_port(x) => {}
                "port" => out.push(format!("ui: {key}: port is a whole number from 1 to 65535")),
                "bind" if x.as_str().is_some_and(|b| BIND.contains(&b)) => {}
                "bind" => out.push(format!("ui: {key}: bind is one of {}", BIND.join(", "))),
                "upstream" => upstream_problems(key, x, out),
                "source" if !built_in && own_source(x).is_some() => {}
                "source" if !built_in => out.push(format!(
                    "ui: {key}: source is {{ess_ui: <file>}} or {{archive: <file>, digest: sha256:<hex>}}"
                )),
                "export" if !built_in && split_export(x).is_some() => {}
                "export" if !built_in => out.push(format!(
                    "ui: {key}: export is `<ingress>.<protocol>`, the export the UI serves"
                )),
                other => out.push(format!(
                    "ui: {key}: unknown key `{other}` (one of {})",
                    keys.join(", ")
                )),
            }
        }
        if !built_in {
            for needed in ["source", "export"] {
                if !entry.contains_key(needed) {
                    out.push(format!("ui: {key}: needs `{needed}`"));
                }
            }
        }
    }
}

fn upstream_problems(key: &str, v: &Value, out: &mut Vec<String>) {
    let Some(m) = v.as_object() else {
        out.push(format!(
            "ui: {key}: upstream is {{unit, port, path, secret}}"
        ));
        return;
    };
    for (k, x) in m {
        match k.as_str() {
            "unit" if x.as_str().is_some_and(is_text) => {}
            "unit" => out.push(format!("ui: {key}: upstream: unit names a unit")),
            "port" if is_port(x) => {}
            "port" => out.push(format!(
                "ui: {key}: upstream: port is a whole number from 1 to 65535"
            )),
            "path" if x.is_string() => {}
            "path" => out.push(format!(
                "ui: {key}: upstream: path is text, the base path of the adapter, starting with `/`"
            )),
            "secret" if x.is_string() => {}
            "secret" => out.push(format!(
                "ui: {key}: upstream: secret is the name of a secret, never its value"
            )),
            other => out.push(format!(
                "ui: {key}: upstream: unknown key `{other}` (one of {})",
                UPSTREAM_KEYS.join(", ")
            )),
        }
    }
}

/// The problems of the binding's `ui:` that need the packing: a unit of the packing or of another
/// UI that takes the UI unit's name, a `part` for the export the UI unit serves, and a port or
/// bind given twice, differently, under `ui:` and under the export. An export the architecture
/// does not have is a finding of admission ([`findings`], `ui_export_unknown`).
pub fn served_problems(binding: &Value, packing: &Packing) -> Vec<String> {
    let mut out = Vec::new();
    let all = asked(binding);
    for (i, u) in all.iter().enumerate() {
        if packing.units.iter().any(|x| x.name == u.name) {
            out.push(format!(
                "ui: {}: the packing has a unit {}, the name of the UI's unit",
                u.key, u.name
            ));
        }
        if let Some(other) = all[..i].iter().find(|x| x.name == u.name) {
            out.push(format!(
                "ui: {}: {} is the name of the unit of the UI {}",
                u.key, u.name, other.key
            ));
        }
        if !serves_export(u, &all) {
            continue;
        }
        let e = &binding["ingress"][&u.ingress]["exports"][&u.export];
        if !e["part"].is_null() {
            out.push(format!(
                "ingress: {}: exports: {}: part: the {} UI's unit serves this export",
                u.ingress, u.export, u.key
            ));
        }
        let ui = &binding["ui"][&u.key];
        for field in ["port", "bind"] {
            if !ui[field].is_null() && !e[field].is_null() && ui[field] != e[field] {
                out.push(format!(
                    "ui: {}: {field} {} is not the {field} of ingress {}'s {} export, {}; give it in one place",
                    u.key, ui[field], u.ingress, u.export, e[field]
                ));
            }
        }
    }
    out
}

/// Point each export a UI serves at the UI's unit and the reserved part [`CONTROL`], with the
/// port and bind of `ui.<ui>` where the binding's export gives none.
pub fn serve(entries: &mut [Value], binding: &Value) {
    let all = asked(binding);
    for u in all.iter().filter(|u| serves_export(u, &all)) {
        let ui = &binding["ui"][&u.key];
        let e = &binding["ingress"][&u.ingress]["exports"][&u.export];
        for entry in entries
            .iter_mut()
            .filter(|x| x["ingress"] == u.ingress.as_str() && x["protocol"] == u.export.as_str())
        {
            entry["unit"] = json!(u.name);
            entry["part"] = json!(CONTROL);
            if e["port"].is_null()
                && let Some(p) = ui["port"].as_u64()
            {
                entry["port"] = json!(p);
            }
            if e["bind"].is_null()
                && let Some(b) = ui["bind"].as_str()
            {
                entry["bind"] = json!(b);
            }
        }
    }
}

/// The upstream of a UI: the unit the binding names, else the unit that serves the UI's
/// ingress's API export (`ingress` the bundle's entries); its port, the binding's, else that
/// export's when it is served by the same unit; and the base path written into `config.json`, the
/// binding's ([`path`]); the name of the secret the UI presents to it, the binding's
/// `ui.<ui>.upstream.secret` when it is text.
pub fn upstream(u: &Ui, binding: &Value, ingress: &[Value]) -> Value {
    let up = &binding["ui"][&u.key]["upstream"];
    let api = ingress
        .iter()
        .find(|e| e["ingress"] == u.ingress.as_str() && e["protocol"] == API);
    let unit = up["unit"]
        .as_str()
        .or_else(|| api.and_then(|e| e["unit"].as_str()))
        .unwrap_or("core");
    let mut out = json!({ "unit": unit, "path": path(u, binding) });
    let port = up["port"].as_u64().or_else(|| {
        api.filter(|e| e["unit"] == unit)
            .and_then(|e| e["port"].as_u64())
    });
    if let Some(p) = port {
        out["port"] = json!(p);
    }
    if let Some(s) = up["secret"].as_str() {
        out["secret"] = json!(s);
    }
    out
}

/// The base path a UI reads its upstream's adapter at: `ui.<ui>.upstream.path`, else [`API_BASE`].
pub fn path<'a>(u: &Ui, binding: &'a Value) -> &'a str {
    binding["ui"][&u.key]["upstream"]["path"]
        .as_str()
        .unwrap_or(API_BASE)
}

/// The file a UI of the consumer's own names, read from the directory of the binding file that
/// named it.
fn source_path(u: &Ui, binding: &Binding, file: &str) -> PathBuf {
    binding
        .ui_files
        .get(&u.key)
        .cloned()
        .unwrap_or_else(|| PathBuf::from(file))
}

/// The archive of a UI whose files the binding gives (`None` for a built-in UI): the bytes of
/// `{archive}` when they are its `digest` (else `ui_archive_mismatch`, or `ui_build_refused` for
/// a file that cannot be read); `{ess_ui}` is refused (`ui_build_needs_toolchain`).
fn given_archive(u: &Ui, binding: &Binding) -> Option<Result<Vec<u8>, Finding>> {
    match &u.source {
        Source::Built(_) => None,
        Source::EssUi(file) => Some(Err(Finding {
            refusal: "ui_build_needs_toolchain",
            detail: format!(
                "ui: {}: source ess_ui {file}: the pages of an ess-ui/1 document are Vue components (agenticspec ui generate) that need node and vite to build, which this binary does not run. Build them, pack the output as a tar and name it as source: {{archive: <file>, digest: sha256:<hex>}}",
                u.key
            ),
        })),
        Source::Archive { file, digest } => {
            let path = source_path(u, binding, file);
            Some(match std::fs::read(&path) {
                Err(e) => Err(Finding {
                    refusal: "ui_build_refused",
                    detail: format!("ui: {}: source archive {file}: {e}", u.key),
                }),
                Ok(bytes) if sha256(&bytes) != *digest => Err(Finding {
                    refusal: "ui_archive_mismatch",
                    detail: format!(
                        "ui: {}: source archive {file} is {}, and the binding names {digest}",
                        u.key,
                        sha256(&bytes)
                    ),
                }),
                Ok(bytes) => Ok(bytes),
            })
        }
    }
}

/// The findings of the UIs the binding asks for that do not need a build: an export the
/// architecture's ingresses do not have (`ui_export_unknown`), an upstream that names no unit of
/// the packing (`ui_upstream_unknown`), an upstream path that does not start with `/`
/// (`ui_upstream_path_invalid`), an archive that is not the digest the binding names
/// (`ui_archive_mismatch`), an `ess-ui/1` source (`ui_build_needs_toolchain`), a UI role or the
/// `service` kind the accepted list does not hold (`unit_not_hosted`).
pub fn findings(
    binding: &Binding,
    ingress: &[Value],
    packing: &Packing,
    accepts: &Accepts,
) -> Vec<Finding> {
    let mut out = Vec::new();
    for u in asked(&binding.doc) {
        if !ingress
            .iter()
            .any(|e| e["ingress"] == u.ingress.as_str() && e["protocol"] == u.export.as_str())
        {
            out.push(Finding {
                refusal: "ui_export_unknown",
                detail: format!(
                    "ui: {}: export {}.{} is no export of an ingress of the architecture",
                    u.key, u.ingress, u.export
                ),
            });
        }
        let up = upstream(&u, &binding.doc, ingress);
        let unit = up["unit"].as_str().unwrap_or("");
        if !packing.units.iter().any(|x| x.name == unit) {
            out.push(Finding {
                refusal: "ui_upstream_unknown",
                detail: format!(
                    "ui: {}: upstream unit {unit} is no unit of the bundle",
                    u.key
                ),
            });
        }
        if !is_upstream_path(&up["path"]) {
            out.push(path_invalid(&format!("ui: {}", u.key), &up["path"]));
        }
        if let Some(Err(f)) = given_archive(&u, binding) {
            out.push(f);
        }
        if let Some(s) = up["secret"].as_str().filter(|s| grants::is_name(s))
            && let Some(upstream_unit) = packing.units.iter().find(|x| x.name == unit)
            && !upstream_unit.parts.iter().any(|p| {
                grants::declared_secrets(&binding.doc, p)
                    .iter()
                    .any(|d| d == s)
            })
        {
            out.push(Finding {
                refusal: "secret_undeclared",
                detail: format!(
                    "ui: {}: upstream secret {s} is not declared by any part of unit {unit}, the UI's upstream",
                    u.key
                ),
            });
        }
        out.extend(hosted(&u.name, u.role, accepts));
    }
    out
}

/// `ui_upstream_path_invalid` of the upstream path `path` of `at` (`ui: <ui>` or `unit <unit>`).
fn path_invalid(at: &str, path: &Value) -> Finding {
    Finding {
        refusal: "ui_upstream_path_invalid",
        detail: format!(
            "{at}: upstream path {path} does not start with `/`, as the path the adapter answers at on its unit does (such as {API_BASE})"
        ),
    }
}

fn hosted(name: &str, role: &str, accepts: &Accepts) -> Vec<Finding> {
    let mut out = Vec::new();
    if !accepts.unit_kinds.contains("service") {
        out.push(Finding {
            refusal: "unit_not_hosted",
            detail: format!("unit {name}: kind service is not in the accepted list"),
        });
    }
    if !accepts.roles.contains(role) {
        out.push(Finding {
            refusal: "unit_not_hosted",
            detail: format!("unit {name}: role {role} is not in the accepted list"),
        });
    }
    out
}

/// The UI units of a bundle and its `files` member.
pub type Built = (Vec<Value>, Map<String, Value>);

/// The units of the UIs the binding asks for and the files they carry (the `files` member: per
/// archive name its `digest`, its size and its bytes in base64), or the finding of a UI whose
/// files cannot be had (`ui_build_refused`, and those of [`findings`] about the source).
pub fn units(
    architecture: &Path,
    contract: &Value,
    binding: &Binding,
    ingress: &[Value],
) -> Result<Built, Vec<Finding>> {
    let mut units = Vec::new();
    let mut files = Map::new();
    let mut findings = Vec::new();
    let doc = &binding.doc;
    let all = asked(doc);
    for u in &all {
        let tar = match (&u.source, given_archive(u, binding)) {
            (_, Some(given)) => given,
            (Source::Built(build), None) => build(architecture, contract, path(u, doc))
                .and_then(|f| agenticspec_ui::operator::archive(&f))
                .map_err(|e| Finding {
                    refusal: "ui_build_refused",
                    detail: format!("ui: {}: {e}", u.key),
                }),
            (_, None) => continue,
        };
        let tar = match tar {
            Ok(t) => t,
            Err(f) => {
                findings.push(f);
                continue;
            }
        };
        let digest = sha256(&tar);
        let archive = archive_name(u);
        let name = u.name.as_str();
        // The export's port when the unit serves the export, else the port `ui.<ui>` gives it.
        let port = if serves_export(u, &all) {
            ingress
                .iter()
                .find(|e| e["ingress"] == u.ingress.as_str() && e["protocol"] == u.export.as_str())
                .and_then(|e| e["port"].as_u64())
        } else {
            doc["ui"][&u.key]["port"].as_u64()
        };
        // Each check on the export's port, unless `checks.<unit>.<check>.port` gives its own.
        let given = &doc["checks"][name];
        let check = |c: &str| {
            let mut v = json!({ "kind": "http", "path": "/" });
            if let Some(p) = given[c]["port"].as_u64().or(port) {
                v["port"] = json!(p);
            }
            v
        };
        // The claims `claims.<unit>` gives, as for any other unit.
        let c = &doc["claims"][name];
        let up = upstream(u, doc, ingress);
        let secrets: Vec<&str> = up["secret"].as_str().into_iter().collect();
        units.push(json!({
            "name": name,
            "kind": "service",
            "parts": [],
            "replicas": 1,
            "code": { "kind": "static", "archive": archive, "digest": digest },
            "upstream": up,
            "claims": {
                "cpu_millis": c["cpu_millis"].as_u64().unwrap_or(super::bundle::DEFAULT_CPU_MILLIS),
                "memory_mib": c["memory_mib"].as_u64().unwrap_or(super::bundle::DEFAULT_MEMORY_MIB),
                "accelerators": c["accelerators"].as_u64().unwrap_or(0),
                "volumes": [],
            },
            "isolation": "none",
            "checks": { "ready": check("ready"), "alive": check("alive") },
            "secrets": secrets,
            "egress": [],
            "roles": [u.role],
            "ports": [],
            "source": { "unit": name, "parts": [] },
        }));
        files.insert(
            archive,
            json!({ "digest": digest, "size": tar.len(), "base64": base64(&tar) }),
        );
    }
    if findings.is_empty() {
        Ok((units, files))
    } else {
        Err(findings)
    }
}

/// The findings of the UI units of a written bundle: an archive the bundle does not carry, or
/// whose bytes are not the digest the unit names (`ui_archive_mismatch`); an upstream that names
/// no other unit of the bundle (`ui_upstream_unknown`); an upstream path that does not start with
/// `/` (`ui_upstream_path_invalid`); a UI role the accepted list does not hold
/// (`unit_not_hosted`).
pub fn check(bundle: &Value, accepts: &Accepts) -> Vec<Finding> {
    let mut out = Vec::new();
    let units: Vec<&Value> = bundle["units"].as_array().into_iter().flatten().collect();
    for u in &units {
        if u["code"]["kind"] != "static" {
            continue;
        }
        let name = u["name"].as_str().unwrap_or("");
        let archive = u["code"]["archive"].as_str().unwrap_or("");
        let named = u["code"]["digest"].as_str().unwrap_or("");
        let file = &bundle["files"][archive];
        let mismatch = match file["base64"].as_str().and_then(unbase64) {
            None => Some(format!(
                "unit {name}: the bundle carries no archive `{archive}` in `files`"
            )),
            Some(bytes) => {
                let d = sha256(&bytes);
                if d != named {
                    Some(format!(
                        "unit {name}: archive {archive} is {d}, and the unit names {named}"
                    ))
                } else if file["digest"] != named {
                    Some(format!(
                        "unit {name}: files: {archive}: digest is not the digest of its bytes"
                    ))
                } else {
                    None
                }
            }
        };
        if let Some(detail) = mismatch {
            out.push(Finding {
                refusal: "ui_archive_mismatch",
                detail,
            });
        }
        let up = u["upstream"]["unit"].as_str().unwrap_or("");
        if up == name || !units.iter().any(|x| x["name"] == up) {
            out.push(Finding {
                refusal: "ui_upstream_unknown",
                detail: format!("unit {name}: upstream unit {up} is no other unit of the bundle"),
            });
        }
        let path = &u["upstream"]["path"];
        if !path.is_null() && !is_upstream_path(path) {
            out.push(path_invalid(&format!("unit {name}"), path));
        }
        for r in u["roles"].as_array().into_iter().flatten() {
            if let Some(role) = ROLES.iter().find(|x| *r == **x) {
                out.extend(hosted(name, role, accepts));
            }
        }
    }
    out
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 with padding.
pub fn base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// The bytes of standard base64 with padding; `None` for anything else.
pub fn unbase64(text: &str) -> Option<Vec<u8>> {
    let s = text.as_bytes();
    if !s.len().is_multiple_of(4) {
        return None;
    }
    let value = |c: u8| ALPHABET.iter().position(|a| *a == c).map(|p| p as u32);
    let mut out = Vec::with_capacity(s.len() / 4 * 3);
    for (i, chunk) in s.chunks(4).enumerate() {
        let last = i + 1 == s.len() / 4;
        let pad = chunk.iter().rev().take_while(|c| **c == b'=').count();
        if pad > 2 || (pad > 0 && !last) {
            return None;
        }
        let mut n = 0u32;
        for c in &chunk[..4 - pad] {
            n = (n << 6) | value(*c)?;
        }
        n <<= 6 * pad as u32;
        let bytes = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        out.extend_from_slice(&bytes[..3 - pad]);
    }
    Some(out)
}
