//! The deployment bundle (`agenticspec-deployment/1`): one YAML file with every member inline,
//! the input a translator turns into an architecture of its target. [`build`] admits the
//! architecture, its contract, the binding and the packing, and only then builds the bundle;
//! the same inputs give the same bytes. A UI the binding asks for is a unit of its own whose
//! archive the bundle carries in `files` ([`super::ui`]). The bundle names no secret value, host,
//! provider or target.

use super::admit::{Accepts, Finding, Plan, PlannedUnit, admit, sort};
use super::binding::{self as binding_form, Binding, UnitImage};
use super::config;
use super::grants;
use super::packing::{self, Packing, Unit, clock_link, schedule_of, schedule_part};
use super::scratch;
use super::ui;
use super::{digest, sha256};
use crate::check::value;
use crate::contract::derive::{Contract, link_name};
use crate::contract::ingress::Kpis;
use crate::doc::parse_yaml;
use crate::nest;
use crate::requirements as req;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub const FORMAT: &str = "agenticspec-deployment/1";

/// The members of a bundle, in the order they are written.
pub const MEMBERS: &[&str] = &[
    "format",
    "generator",
    "architecture",
    "nested",
    "contract",
    "binding",
    "packing",
    "units",
    "ingress",
    "requirements",
    "agents",
    "runtimes",
    "digest",
];

/// The order members are written in: [`MEMBERS`] with `files`, which a bundle has only when it
/// carries a file (a UI's archive, [`super::ui`]), before `digest`.
const WRITTEN: &[&str] = &[
    "format",
    "generator",
    "architecture",
    "nested",
    "contract",
    "binding",
    "packing",
    "units",
    "ingress",
    "requirements",
    "agents",
    "runtimes",
    "files",
    "digest",
];

/// The claims of a unit the binding names none for.
pub const DEFAULT_CPU_MILLIS: u64 = 500;
pub const DEFAULT_MEMORY_MIB: u64 = 512;

/// What a bundle is built from.
pub struct Inputs<'a> {
    /// The architecture's file, which a UI's build reads; its bytes and the document they parse
    /// to, as written.
    pub architecture: &'a Path,
    pub arch_bytes: &'a [u8],
    pub written: &'a Value,
    /// The document flattened, its nested architectures as parts of its own (`nest::flatten`):
    /// what the packing, the units and admission read. A document that nests none is `written`.
    pub arch: &'a Value,
    /// Every document the architecture references, by canonical digest ([`nested_documents`]).
    pub nested: &'a BTreeMap<String, Carried>,
    /// The KPI catalogue the requirements are checked against.
    pub kpis: &'a Kpis,
    pub contract: &'a Contract,
    pub binding: &'a Binding,
    pub packing: &'a Packing,
    pub accepts: &'a Accepts,
}

/// One nested document a bundle carries: the parts that refer to it (dotted from the outermost
/// document, `console`, `console.vault`), the file it was read from, its exact bytes and the
/// document they parse to.
#[derive(Debug, Clone, PartialEq)]
pub struct Carried {
    pub parts: Vec<String>,
    pub file: PathBuf,
    pub bytes: Vec<u8>,
    pub document: Value,
}

/// Every document the architecture `doc`, read from `path`, references, and the documents those
/// reference, keyed by the canonical digest a referring part pins (`nest::digest`). A document
/// referred to from several parts is one entry. Read after `nest::load` resolved the chain, so a
/// reference is readable and leads to no cycle; a read that fails is an error all the same.
pub fn nested_documents(path: &Path, doc: &Value) -> Result<BTreeMap<String, Carried>, String> {
    fn walk(
        dir: &Path,
        doc: &Value,
        prefix: &str,
        depth: usize,
        out: &mut BTreeMap<String, Carried>,
    ) -> Result<(), String> {
        if depth > 64 {
            return Err(format!("{prefix}: nested deeper than 64 documents"));
        }
        for (name, p) in doc["parts"].as_object().into_iter().flatten() {
            if p["block"] != nest::BLOCK {
                continue;
            }
            let reference = p["ref"].as_str().ok_or(format!("part {name}: no `ref`"))?;
            let file = dir.join(reference);
            let bytes = std::fs::read(&file).map_err(|e| format!("{}: {e}", file.display()))?;
            let document = std::str::from_utf8(&bytes)
                .map_err(|e| e.to_string())
                .and_then(parse_yaml)
                .map_err(|e| format!("{}: {e}", file.display()))?;
            let part = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{prefix}.{name}")
            };
            let inner_dir = file.parent().unwrap_or(Path::new(".")).to_path_buf();
            let entry = out.entry(nest::digest(&document)).or_insert(Carried {
                parts: Vec::new(),
                file,
                bytes,
                document: document.clone(),
            });
            entry.parts.push(part.clone());
            walk(&inner_dir, &document, &part, depth + 1, out)?;
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    walk(
        path.parent().unwrap_or(Path::new(".")),
        doc,
        "",
        0,
        &mut out,
    )?;
    Ok(out)
}

/// The `nested` member of a bundle: per canonical digest the nested document's name, the parts
/// that refer to it, the SHA-256 of its bytes, the parsed `document` and `content_base64`, the
/// file's exact bytes.
fn nested_member(nested: &BTreeMap<String, Carried>) -> Value {
    let mut out = Map::new();
    for (d, c) in nested {
        out.insert(
            d.clone(),
            json!({
                "architecture": text(&c.document["architecture"]),
                "parts": c.parts,
                "sha256": sha256(&c.bytes),
                "document": c.document,
                "content_base64": ui::base64(&c.bytes),
            }),
        );
    }
    Value::Object(out)
}

/// The requirements of the architecture as the bundle carries them: each as written with its
/// stage, its state and the value it was decided on (`crate::requirements::at_check`); or a
/// finding per refusal of its `requirements:` (`requirement_kpi_unknown`, `requirement_invalid`).
fn requirements(arch: &Value, kpis: &Kpis, findings: &mut Vec<Finding>) -> Vec<Value> {
    let refusals = req::refusals(arch, kpis);
    if refusals.is_empty() {
        return req::at_check(arch, kpis);
    }
    findings.extend(refusals.into_iter().map(|r| Finding {
        refusal: r.rule,
        detail: format!("{}: {}", r.subject, r.detail),
    }));
    Vec::new()
}

/// A built bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct Bundle {
    pub value: Value,
    pub text: String,
    pub digest: String,
    pub units: usize,
}

fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

/// The revision digest of an architecture: SHA-256 of the canonical JSON of its parts, links
/// and border (`agentic.architecture.Digest`).
pub fn revision_digest(arch: &Value) -> String {
    let or = |v: &Value, d: Value| if v.is_null() { d } else { v.clone() };
    digest(&json!({
        "parts": or(&arch["parts"], json!({})),
        "links": or(&arch["links"], json!([])),
        "border": or(&arch["border"], json!([])),
    }))
}

/// An agent document the binding names, read and digested.
struct AgentDoc {
    digest: String,
    document: Value,
    bytes: Vec<u8>,
}

/// Read every agent document the binding names. A unit that stays an agent (`agent`, `job`,
/// `scheduled`) needs one for each agent part it holds; a named document must be readable and,
/// when the binding pins a digest, match it.
fn agent_documents(
    inputs: &Inputs,
    packing: &Packing,
    findings: &mut Vec<Finding>,
) -> BTreeMap<String, AgentDoc> {
    let mut docs = BTreeMap::new();
    for (part, file) in &inputs.binding.agent_files {
        let bytes = match std::fs::read(file) {
            Ok(b) => b,
            Err(e) => {
                findings.push(Finding {
                    refusal: "agent_document_missing",
                    detail: format!("part {part}: agent document {}: {e}", file.display()),
                });
                continue;
            }
        };
        let d = sha256(&bytes);
        if let Some(pin) = inputs.binding.doc["parts"][part]["digest"].as_str()
            && pin != d
        {
            findings.push(Finding {
                refusal: "agent_document_digest",
                detail: format!("part {part}: the agent document is {d}, the binding pins {pin}"),
            });
            continue;
        }
        let document = match std::str::from_utf8(&bytes)
            .map_err(|e| e.to_string())
            .and_then(parse_yaml)
        {
            Ok(v) => v,
            Err(e) => {
                findings.push(Finding {
                    refusal: "agent_document_missing",
                    detail: format!("part {part}: agent document {}: {e}", file.display()),
                });
                continue;
            }
        };
        docs.insert(
            part.clone(),
            AgentDoc {
                digest: d,
                document,
                bytes,
            },
        );
    }
    for u in &packing.units {
        if !matches!(u.kind.as_str(), "agent" | "job" | "scheduled") {
            continue;
        }
        for p in &u.parts {
            if inputs.arch["parts"][p]["block"] == "agent"
                && !inputs.binding.agent_files.contains_key(p)
            {
                findings.push(Finding {
                    refusal: "agent_document_missing",
                    detail: format!(
                        "unit {} stays an agent, and the binding names no agent document for its part {p}",
                        u.name
                    ),
                });
            }
        }
    }
    docs
}

/// Build the bundle of an admitted architecture, or every finding of admission.
pub fn build(inputs: &Inputs) -> Result<Bundle, Vec<Finding>> {
    let mut findings = Vec::new();
    let plan = match admit(
        inputs.arch,
        &inputs.binding.doc,
        inputs.packing,
        inputs.accepts,
    ) {
        Ok(plan) => Some(plan),
        Err(f) => {
            findings.extend(f);
            None
        }
    };
    let docs = agent_documents(inputs, inputs.packing, &mut findings);
    findings.extend(served(inputs.arch, &inputs.binding.doc, inputs.packing));
    findings.extend(config::findings(&inputs.binding.doc, inputs.packing));
    findings.extend(scratch::findings(&inputs.binding.doc, inputs.packing));
    findings.extend(grants::check(
        inputs.arch,
        &inputs.binding.doc,
        inputs.packing,
    ));
    findings.extend(addresses(inputs.arch));
    let requirements = requirements(inputs.arch, inputs.kpis, &mut findings);
    let contract = inputs.contract.to_value();
    let ingress = ingress(&contract["ingress"], &inputs.binding.doc, inputs.packing);
    findings.extend(ui::findings(
        inputs.binding,
        &ingress,
        inputs.packing,
        inputs.accepts,
    ));
    let Some(plan) = plan.filter(|_| findings.is_empty()) else {
        sort(&mut findings);
        return Err(findings);
    };
    let (ui_units, files) = ui::units(inputs.architecture, &contract, inputs.binding, &ingress)?;

    let binding = &inputs.binding.doc;
    let mut units = Vec::new();
    for pu in &plan.units {
        units.push(unit(inputs, &plan, pu, &docs));
    }
    units.extend(ui_units);
    let mut agents = Map::new();
    let mut by_digest: BTreeMap<&str, (Vec<&str>, &AgentDoc)> = BTreeMap::new();
    for (part, d) in &docs {
        let e = by_digest
            .entry(d.digest.as_str())
            .or_insert((Vec::new(), d));
        e.0.push(part);
    }
    for (d, (parts, doc)) in by_digest {
        agents.insert(
            d.to_owned(),
            json!({
                "parts": parts,
                "document": doc.document,
                "content_base64": ui::base64(&doc.bytes),
            }),
        );
    }
    let mut runtimes = Map::new();
    let local = binding_form::images(binding) == "local";
    // Every runtime the binding pins, by its name, then the image of each core or service unit
    // that is not one of them, by the unit's name; an image already carried keeps its entry.
    let pinned = binding["runtimes"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(name, r)| (name.as_str(), r, true));
    let own =
        plan.units
            .iter()
            .filter_map(|pu| match binding_form::unit_image(binding, &pu.unit) {
                UnitImage::Unit(r) | UnitImage::Parts(r) => Some((pu.unit.name.as_str(), r, false)),
                _ => None,
            });
    for (name, r, is_pinned) in pinned.chain(own) {
        let key = binding_form::image_key(binding, r);
        if !is_pinned && runtimes.contains_key(&key) {
            continue;
        }
        let entry = if local {
            // What the binding gave: the image, and its tag, its digest (config id) or both;
            // keyed by the digest, else by `<image>:<tag>`.
            let mut entry = json!({ "name": name, "image": text(&r["image"]) });
            for k in ["tag", "digest"] {
                if let Some(x) = r[k].as_str() {
                    entry[k] = json!(x);
                }
            }
            entry
        } else {
            let image = binding_form::pinned_reference(binding, r).unwrap_or_default();
            json!({ "name": name, "image": image })
        };
        runtimes.insert(key, entry);
    }
    let packing = inputs.packing.to_value(inputs.arch);
    let mut bundle = json!({
        "format": FORMAT,
        "generator": format!("agenticspec {}", env!("CARGO_PKG_VERSION")),
        "architecture": {
            "name": text(&inputs.written["architecture"]),
            "revision": revision_digest(inputs.written),
            "sha256": sha256(inputs.arch_bytes),
            "document": inputs.written,
        },
        "nested": nested_member(inputs.nested),
        "contract": { "digest": digest(&contract), "document": contract },
        "binding": {
            "name": text(&binding["binding"]),
            "digest": digest(binding),
            "document": binding,
        },
        "packing": {
            "digest": digest(&packing),
            "strategy": inputs.packing.strategy,
            "document": packing,
        },
        "units": units,
        "ingress": ingress,
        "requirements": requirements,
        "agents": agents,
        "runtimes": runtimes,
    });
    // Two images under one key of `runtimes` (one digest, two names) leave a unit's image
    // uncarried: refused as `deploy check` would refuse the bundle.
    let mut uncarried = unit_runtimes(&bundle, inputs.arch, binding, inputs.packing);
    if !uncarried.is_empty() {
        sort(&mut uncarried);
        return Err(uncarried);
    }
    if !files.is_empty() {
        bundle["files"] = Value::Object(files);
    }
    let d = digest(&bundle);
    bundle["digest"] = json!(d);
    Ok(Bundle {
        text: write(&bundle),
        digest: d,
        units: plan.units.len(),
        value: bundle,
    })
}

/// The bundle as YAML, members in the order of [`MEMBERS`] with `files` before `digest`, keys
/// sorted inside each.
pub fn write(bundle: &Value) -> String {
    let mut s = String::new();
    for m in WRITTEN {
        if let Some(v) = bundle.get(*m) {
            let mut one = Map::new();
            one.insert((*m).to_owned(), v.clone());
            s.push_str(&serde_yaml_ng::to_string(&Value::Object(one)).unwrap_or_default());
        }
    }
    s
}

/// Recompute the digest of a bundle read back: the canonical JSON of every member but `digest`.
pub fn recompute(bundle: &Value) -> String {
    let mut v = bundle.clone();
    if let Value::Object(m) = &mut v {
        m.remove("digest");
    }
    digest(&v)
}

/// The part that serves an export the binding names no part for: the control plane, whose
/// adapter in the core unit serves every ingress. It is the reserved path no part of a document
/// may take, under which the implied audiences `control.operator` and `control.monitor` sit.
pub const CONTROL: &str = "control";

/// The ingress of a bundle: one entry per export of each ingress of the contract
/// (`contract_ingress`, the contract's `ingress` as data), in the contract's order: the
/// ingress, its protocol, audience and whether callers authenticate; the unit and part that
/// serve it (the binding's `part`, else [`CONTROL`] in the core unit); its `bind` (the
/// binding's, else [`binding_form::DEFAULT_BIND`]); its `port` and `address` when the binding
/// gives them.
pub fn ingress(contract_ingress: &Value, binding: &Value, packing: &Packing) -> Vec<Value> {
    let core = packing
        .units
        .iter()
        .find(|u| u.kind == "core")
        .map_or("core", |u| u.name.as_str());
    let mut out = Vec::new();
    for i in contract_ingress.as_array().into_iter().flatten() {
        let name = text(&i["name"]);
        for protocol in i["exports"].as_array().into_iter().flatten() {
            let protocol = text(protocol);
            let e = &binding["ingress"][name]["exports"][protocol];
            let (unit, part) = match e["part"].as_str() {
                Some(p) => (packing.unit_of(p).map_or(core, |u| u.name.as_str()), p),
                None => (core, CONTROL),
            };
            let mut entry = json!({
                "ingress": name,
                "protocol": protocol,
                "audience": i["audience"],
                "authenticated": i["authenticated"],
                "unit": unit,
                "part": part,
                "bind": e["bind"].as_str().unwrap_or(binding_form::DEFAULT_BIND),
            });
            if let Some(p) = e["port"].as_u64() {
                entry["port"] = json!(p);
            }
            if let Some(a) = e["address"].as_str() {
                entry["address"] = json!(a);
            }
            out.push(entry);
        }
    }
    ui::serve(&mut out, binding);
    out
}

/// The mount the binding gives a volume of a unit: `parts.<p>.volumes[].mount` of the first part
/// `p` of the unit, in name order, that names the volume.
fn mount_of<'a>(binding: &'a Value, parts: &[String], volume: &str) -> Option<&'a Value> {
    parts.iter().find_map(|p| {
        binding["parts"][p]["volumes"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|v| v["name"] == volume)
            .map(|v| &v["mount"])
    })
}

/// An absolute path: it starts with `/`.
fn absolute(v: &Value) -> bool {
    v.as_str().is_some_and(|m| m.starts_with('/'))
}

/// The findings of what the target is asked to serve, from the architecture, the binding and
/// the packing: `network_isolation_unmet`, a part marked `network: isolated` on a target that
/// enforces no network policy; `image_unpinned`, a runtime, part or unit image from the target's
/// local store with neither tag nor digest; `unit_image_conflict`, a core or service unit whose
/// parts name different images and which names none itself; `unit_image_missing`, a core or
/// service unit that holds a runtime part and has no image ([`binding_form::unit_image`]);
/// `durable_state_unplaceable`, a runtime part with durable state in a unit of
/// [`STATELESS_KINDS`], or whose state volume's name is taken by a part;
/// `volume_mount_missing`, a volume of a unit (a workspace's, or a runtime part's durable state)
/// no part of it mounts at an absolute path. A finding names the unit, part, runtime or volume,
/// never a value.
pub fn served(arch: &Value, binding: &Value, packing: &Packing) -> Vec<Finding> {
    let mut out = Vec::new();
    if binding["target"]["network_policy"] == "not_enforced" {
        for (p, entry) in binding["parts"].as_object().into_iter().flatten() {
            if entry["network"] == "isolated" && packing.unit_of(p).is_some() {
                out.push(Finding {
                    refusal: "network_isolation_unmet",
                    detail: format!(
                        "part {p} is marked `network: isolated`, and the target does not enforce a network policy"
                    ),
                });
            }
        }
    }
    if binding_form::images(binding) == "local" {
        let runtimes = binding["runtimes"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(name, r)| (format!("runtimes: {name}"), r));
        let parts = binding["parts"]
            .as_object()
            .into_iter()
            .flatten()
            .filter_map(|(p, e)| e.get("image").map(|r| (format!("parts: {p}: image"), r)));
        let units = packing.units.iter().filter_map(|u| {
            binding_form::listed_image(binding, &u.name)
                .map(|r| (format!("packing: unit {}: image", u.name), r))
        });
        for (at, r) in runtimes.chain(parts).chain(units) {
            if r["tag"].is_null() && r["digest"].is_null() {
                out.push(Finding {
                    refusal: "image_unpinned",
                    detail: format!(
                        "{at}: an image of the target's local store is pinned by its tag, its digest or both, and names neither"
                    ),
                });
            }
        }
    }
    for u in &packing.units {
        match binding_form::unit_image(binding, u) {
            UnitImage::Conflict(named) => out.push(Finding {
                refusal: "unit_image_conflict",
                detail: format!(
                    "unit {}: its parts {} name different images under `parts.<p>.image`, and the unit names none under `packing.units`",
                    u.name,
                    named.join(", ")
                ),
            }),
            UnitImage::None if binding_form::IMAGE_KINDS.contains(&u.kind.as_str()) => {
                let runtimes: Vec<&str> = u
                    .parts
                    .iter()
                    .filter(|p| arch["parts"][*p]["block"] == binding_form::RUNTIME_BLOCK)
                    .map(String::as_str)
                    .collect();
                if runtimes.is_empty() {
                    continue;
                }
                let from = if u.kind == "core" {
                    "`packing.units[].image`, `parts.<p>.image` or `runtimes.core`"
                } else {
                    "`packing.units[].image` or `parts.<p>.image` (`runtimes.core` is the core unit's only)"
                };
                out.push(Finding {
                    refusal: "unit_image_missing",
                    detail: format!(
                        "{} unit {} holds the runtime part {} and has no image: name one in {from}",
                        u.kind,
                        u.name,
                        runtimes.join(", ")
                    ),
                });
            }
            _ => {}
        }
    }
    for u in &packing.units {
        for v in u
            .parts
            .iter()
            .filter(|p| arch["parts"][*p]["block"] == "workspace")
        {
            let detail = match mount_of(binding, &u.parts, v) {
                Some(m) if absolute(m) => continue,
                Some(_) => format!(
                    "unit {}: volume {v}: its mount is not an absolute path",
                    u.name
                ),
                None => format!(
                    "unit {}: volume {v} has no mount (a part of the unit names it under `volumes:` with `mount:`, an absolute path)",
                    u.name
                ),
            };
            out.push(Finding {
                refusal: "volume_mount_missing",
                detail,
            });
        }
        for p in u
            .parts
            .iter()
            .filter(|p| binding_form::durable_state(&arch["parts"][*p]))
        {
            let v = binding_form::state_volume(p);
            if STATELESS_KINDS.contains(&u.kind.as_str()) {
                out.push(Finding {
                    refusal: "durable_state_unplaceable",
                    detail: format!(
                        "runtime part {p} keeps durable state, and its unit {} is a {} unit, which holds no volume for it (place {p} in a core, service, agent or store unit)",
                        u.name, u.kind
                    ),
                });
                continue;
            }
            if !arch["parts"][&v].is_null() {
                out.push(Finding {
                    refusal: "durable_state_unplaceable",
                    detail: format!(
                        "runtime part {p} keeps durable state in the volume {v}, and a part of the document has that name"
                    ),
                });
                continue;
            }
            let detail = match mount_of(binding, &u.parts, &v) {
                Some(m) if absolute(m) => continue,
                Some(_) => format!(
                    "unit {}: volume {v} (durable state of {p}): its mount is not an absolute path",
                    u.name
                ),
                None => format!(
                    "unit {}: volume {v} (durable state of {p}) has no mount (a part of the unit names it under `volumes:` with `mount:`, an absolute path)",
                    u.name
                ),
            };
            out.push(Finding {
                refusal: "volume_mount_missing",
                detail,
            });
        }
    }
    out
}

/// The kinds of unit that hold no volume for a runtime part's durable state: they run until
/// done (`job`, `scheduled`) or serve a model the target runs (`model`)
/// (`durable_state_unplaceable`).
pub const STATELESS_KINDS: &[&str] = &["job", "scheduled", "model"];

/// The refusals only `deploy check` gives, beside those of [`super::ui::CHECK_REFUSALS`].
pub const CHECK_REFUSALS: &[&str] = &[
    "agent_document_mismatch",
    "nested_document_mismatch",
    "nested_document_missing",
];

/// The `nested_document_mismatch` findings of the `nested` member of a written bundle: an entry
/// whose `content_base64` is missing or not standard base64, whose bytes are not its `sha256`,
/// do not parse to its `document`, or whose document's canonical digest (`nest::digest`) is not
/// the digest it is keyed by.
fn nested_contents(nested: &Map<String, Value>) -> Vec<Finding> {
    let mut out = Vec::new();
    for (d, n) in nested {
        let detail = match n["content_base64"].as_str().and_then(ui::unbase64) {
            None => Some(format!(
                "nested: {d}: carries no `content_base64` in standard base64"
            )),
            Some(bytes) => {
                let parsed = std::str::from_utf8(&bytes)
                    .map_err(|e| e.to_string())
                    .and_then(parse_yaml)
                    .ok();
                let canonical = parsed.as_ref().map(nest::digest);
                if n["sha256"].as_str() != Some(sha256(&bytes).as_str()) {
                    Some(format!(
                        "nested: {d}: its content_base64 is {}, and `sha256` says {}",
                        sha256(&bytes),
                        n["sha256"].as_str().unwrap_or("nothing")
                    ))
                } else if canonical.as_deref() != Some(d.as_str()) {
                    Some(format!(
                        "nested: {d}: its content_base64 holds a document of digest {}",
                        canonical.as_deref().unwrap_or("none (it does not parse)")
                    ))
                } else if parsed.as_ref() != Some(&n["document"]) {
                    Some(format!(
                        "nested: {d}: `document` is not the document its content_base64 holds"
                    ))
                } else {
                    None
                }
            }
        };
        if let Some(detail) = detail {
            out.push(Finding {
                refusal: "nested_document_mismatch",
                detail,
            });
        }
    }
    out
}

/// `doc` with the carried document of each of its nested architectures under the key `inner` of
/// its part, recursively, as `nest::load` resolves them from files: each found by the digest
/// its part pins, its marked places at their values. A reference whose digest the bundle carries
/// no document of is a `nested_document_missing` finding.
fn resolve_carried(
    doc: &Value,
    nested: &Map<String, Value>,
    prefix: &str,
    depth: usize,
) -> Result<Value, Vec<Finding>> {
    let mut out = doc.clone();
    let mut missing = Vec::new();
    let names: Vec<String> = doc["parts"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, p)| p["block"] == nest::BLOCK)
        .map(|(n, _)| n.clone())
        .collect();
    for name in names {
        let part = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}.{name}")
        };
        let p = &doc["parts"][&name];
        let pin = text(&p["digest"]);
        let Some(entry) = nested.get(pin).filter(|_| depth < 64) else {
            missing.push(Finding {
                refusal: "nested_document_missing",
                detail: format!(
                    "part {part}: references `{}` pinned {pin}, and the bundle carries no nested document of that digest",
                    text(&p["ref"])
                ),
            });
            continue;
        };
        let inner = match crate::marked::resolve(&entry["document"], &Map::new()) {
            Ok(v) => v,
            Err(e) => {
                missing.push(Finding {
                    refusal: "nested_document_missing",
                    detail: format!(
                        "part {part}: the carried document {pin} does not resolve: {e}"
                    ),
                });
                continue;
            }
        };
        match resolve_carried(&inner, nested, &part, depth + 1) {
            Ok(inner) => out["parts"][&name][nest::INNER] = inner,
            Err(f) => missing.extend(f),
        }
    }
    if missing.is_empty() {
        Ok(out)
    } else {
        Err(missing)
    }
}

/// The `agent_document_mismatch` findings of the `agents` of a written bundle: an entry whose
/// `content_base64` is missing or not standard base64, whose bytes are not the digest it is keyed
/// by, or whose `document` is not what those bytes parse to.
fn agent_contents(agents: &Map<String, Value>) -> Vec<Finding> {
    let mut out = Vec::new();
    for (d, a) in agents {
        let detail = match a["content_base64"].as_str().and_then(ui::unbase64) {
            None => Some(format!(
                "agents: {d}: carries no `content_base64` in standard base64"
            )),
            Some(bytes) => {
                let got = sha256(&bytes);
                if got != *d {
                    Some(format!("agents: {d}: its content_base64 is {got}"))
                } else if std::str::from_utf8(&bytes)
                    .map_err(|e| e.to_string())
                    .and_then(parse_yaml)
                    .ok()
                    .as_ref()
                    != Some(&a["document"])
                {
                    Some(format!(
                        "agents: {d}: `document` is not the document its content_base64 holds"
                    ))
                } else {
                    None
                }
            }
        };
        if let Some(detail) = detail {
            out.push(Finding {
                refusal: "agent_document_mismatch",
                detail,
            });
        }
    }
    out
}

/// The `schedule_not_cron` findings of the scheduled units of a written bundle whose `schedule`
/// is not the cron rule of [`schedule_of`]: the binding's, else the one its clock's `every`
/// states. An `every` no cron rule states is admission's finding already.
fn unit_schedules(bundle: &Value, arch: &Value, packing: &Packing) -> Vec<Finding> {
    let mut out = Vec::new();
    for u in packing.units.iter().filter(|u| u.kind == "scheduled") {
        let Ok(want) = schedule_of(arch, u) else {
            continue;
        };
        let Some(written) = bundle["units"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|x| x["name"] == u.name.as_str())
        else {
            continue;
        };
        let got = written["schedule"].as_str();
        if got != want.as_deref() {
            out.push(Finding {
                refusal: "schedule_not_cron",
                detail: format!(
                    "unit {}: carries schedule {}, and its binding and clock give {}",
                    u.name,
                    got.unwrap_or("(none)"),
                    want.as_deref().unwrap_or("none")
                ),
            });
        }
    }
    out
}

/// The `volume_mount_missing` findings of the units of a written bundle: a volume claim without
/// a `mount`, or one that is not an absolute path.
fn unit_volumes(bundle: &Value) -> Vec<Finding> {
    let mut out = Vec::new();
    for u in bundle["units"].as_array().into_iter().flatten() {
        for v in u["claims"]["volumes"].as_array().into_iter().flatten() {
            if !absolute(&v["mount"]) {
                out.push(Finding {
                    refusal: "volume_mount_missing",
                    detail: format!(
                        "unit {}: volume {} carries no absolute `mount`",
                        text(&u["name"]),
                        text(&v["name"])
                    ),
                });
            }
        }
    }
    out
}

/// The reference a target runs an image the bundle's `runtimes` carry by: from a registry the
/// entry's `image`, already `<image>@<digest>`; from the target's local store `<image>:<tag>`,
/// else its digest ([`binding_form::pinned_reference`]).
fn carried_reference(binding: &Value, entry: &Value) -> Option<String> {
    if binding_form::images(binding) == "local" {
        binding_form::pinned_reference(binding, entry)
    } else {
        entry["image"].as_str().map(str::to_owned)
    }
}

/// The `unit_runtime_mismatch` findings of a bundle: a core or service unit whose
/// `code.runtime` is not the image the binding resolves for it ([`binding_form::unit_image`],
/// none when it resolves none), or names an image the bundle's `runtimes` do not carry. A unit
/// the binding gives no image because of a refusal of its own (`unit_image_conflict`,
/// `unit_image_missing`, `image_unpinned`) is left to that refusal. The finding names the unit,
/// the image found and the image expected.
pub fn unit_runtimes(
    bundle: &Value,
    arch: &Value,
    binding: &Value,
    packing: &Packing,
) -> Vec<Finding> {
    let carried: BTreeSet<String> = bundle["runtimes"]
        .as_object()
        .into_iter()
        .flatten()
        .filter_map(|(_, e)| carried_reference(binding, e))
        .collect();
    let mut out = Vec::new();
    for u in packing
        .units
        .iter()
        .filter(|u| binding_form::IMAGE_KINDS.contains(&u.kind.as_str()))
    {
        let resolved = binding_form::unit_image(binding, u);
        let expected = match &resolved {
            UnitImage::Conflict(_) => continue,
            UnitImage::None
                if u.parts
                    .iter()
                    .any(|p| arch["parts"][p]["block"] == binding_form::RUNTIME_BLOCK) =>
            {
                continue;
            }
            UnitImage::None => None,
            r => match r
                .image()
                .and_then(|i| binding_form::pinned_reference(binding, i))
            {
                Some(x) => Some(x),
                None => continue,
            },
        };
        let Some(written) = bundle["units"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|x| x["name"] == u.name.as_str())
        else {
            continue;
        };
        let found = written["code"]["runtime"].as_str();
        let detail = if found != expected.as_deref() {
            format!(
                "unit {}: its code.runtime is {}, and the binding resolves {}",
                u.name,
                found.unwrap_or("none"),
                expected.as_deref().unwrap_or("none")
            )
        } else if let Some(f) = found.filter(|f| !carried.contains(*f)) {
            format!(
                "unit {}: its code.runtime {f} is no image the bundle's runtimes carry, and the binding resolves {f}",
                u.name
            )
        } else {
            continue;
        };
        out.push(Finding {
            refusal: "unit_runtime_mismatch",
            detail,
        });
    }
    out
}

/// The keys that hold where something is reached. An address is the binding's (per export, and
/// the named destinations of a part's `egress`), never the architecture's, which the bundle
/// copies as written.
pub const ADDRESS_KEYS: &[&str] = &["address", "host", "hostname", "url"];

/// One `address_in_architecture` finding per key of [`ADDRESS_KEYS`] anywhere in the
/// architecture document, by its path; never the address itself.
pub fn addresses(arch: &Value) -> Vec<Finding> {
    fn walk(v: &Value, at: &str, out: &mut Vec<Finding>) {
        match v {
            Value::Object(m) => {
                for (k, x) in m {
                    let path = if at.is_empty() {
                        k.clone()
                    } else {
                        format!("{at}.{k}")
                    };
                    if ADDRESS_KEYS.contains(&k.as_str()) {
                        out.push(Finding {
                            refusal: "address_in_architecture",
                            detail: format!(
                                "the architecture has `{path}`, and an address is the binding's, never the architecture's"
                            ),
                        });
                    } else {
                        walk(x, &path, out);
                    }
                }
            }
            Value::Array(a) => {
                for (i, x) in a.iter().enumerate() {
                    walk(x, &format!("{at}[{i}]"), out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(arch, "", &mut out);
    out
}

/// Why a written bundle is refused when it is read back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// Its form: the format, a missing member, a digest that does not hold, the binding's form or
    /// the packing's; one line each.
    Invalid(Vec<String>),
    /// Every finding of admission, sorted.
    NotAdmitted(Vec<Finding>),
}

/// Admit a written bundle again from what it carries, for a reader that did not write it: its
/// form and digests; its architecture resolved from the nested documents it carries, each found
/// by the digest its part pins and its bytes recomputed, then flattened; admission of that
/// architecture under its binding and packing ([`admit`]), its agent documents against the parts that need them and the digests the
/// binding pins, the bytes each agent document carries ([`CHECK_REFUSALS`]), the cron rule of
/// each scheduled unit ([`packing::schedule_of`]), the runtime image of each core and service
/// unit ([`unit_runtimes`]), the configuration of each unit ([`config::findings`],
/// [`config::check_units`]), the scratch directories of each unit ([`scratch::findings`],
/// [`scratch::check_units`]), and the secrets and
/// egress of the binding and of each unit
/// ([`grants::check`], [`grants::check_units`]), and its UI units ([`ui::check`]).
pub fn check(bundle: &Value, accepts: &Accepts) -> Result<(), Refused> {
    let mut problems = Vec::new();
    if bundle["format"] != FORMAT {
        problems.push(format!("not {FORMAT}"));
        return Err(Refused::Invalid(problems));
    }
    for m in MEMBERS {
        if bundle.get(*m).is_none() {
            problems.push(format!("no member `{m}`"));
        }
    }
    if !problems.is_empty() {
        return Err(Refused::Invalid(problems));
    }
    if bundle["digest"].as_str() != Some(recompute(bundle).as_str()) {
        problems.push("`digest` is not the digest of the other members".into());
    }
    let written = &bundle["architecture"]["document"];
    if bundle["architecture"]["revision"].as_str() != Some(revision_digest(written).as_str()) {
        problems.push("architecture: `revision` is not the digest of its document".into());
    }
    let nested = bundle["nested"].as_object().cloned().unwrap_or_default();
    let mut nested_findings = nested_contents(&nested);
    let flat = match resolve_carried(written, &nested, "", 0) {
        Ok(doc) => nest::flatten(&doc),
        Err(missing) => {
            if !problems.is_empty() {
                return Err(Refused::Invalid(problems));
            }
            nested_findings.extend(missing);
            sort(&mut nested_findings);
            nested_findings.dedup();
            return Err(Refused::NotAdmitted(nested_findings));
        }
    };
    let arch = &flat;
    for m in ["contract", "binding", "packing"] {
        if bundle[m]["digest"].as_str() != Some(digest(&bundle[m]["document"]).as_str()) {
            problems.push(format!("{m}: `digest` is not the digest of its document"));
        }
    }
    let binding = &bundle["binding"]["document"];
    problems.extend(
        binding_form::problems(&grants::without_values(binding), arch)
            .into_iter()
            .map(|p| format!("binding: {p}")),
    );
    let packing = match packing::resolve(arch, &bundle["packing"]["document"]) {
        Ok(p) => Some(p),
        Err(e) => {
            problems.extend(e.into_iter().map(|p| format!("packing: {p}")));
            None
        }
    };
    if let Some(p) = &packing {
        let contract_ingress = &bundle["contract"]["document"]["ingress"];
        problems.extend(
            binding_form::served_problems(binding, contract_ingress, p)
                .into_iter()
                .map(|x| format!("binding: {x}")),
        );
    }
    let Some(packing) = packing.filter(|_| problems.is_empty()) else {
        return Err(Refused::Invalid(problems));
    };

    let mut findings = admit(arch, binding, &packing, accepts)
        .err()
        .unwrap_or_default();
    let agents = bundle["agents"].as_object().cloned().unwrap_or_default();
    let agent_of = |part: &str| {
        agents.iter().find_map(|(d, a)| {
            a["parts"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|p| p == part)
                .then_some(d.as_str())
        })
    };
    for u in &packing.units {
        if !matches!(u.kind.as_str(), "agent" | "job" | "scheduled") {
            continue;
        }
        for p in u
            .parts
            .iter()
            .filter(|p| arch["parts"][*p]["block"] == "agent")
        {
            if agent_of(p).is_none() {
                findings.push(Finding {
                    refusal: "agent_document_missing",
                    detail: format!(
                        "unit {} stays an agent, and the bundle carries no agent document for its part {p}",
                        u.name
                    ),
                });
            }
        }
    }
    for (part, entry) in binding["parts"].as_object().into_iter().flatten() {
        if let (Some(pin), Some(d)) = (entry["digest"].as_str(), agent_of(part))
            && pin != d
        {
            findings.push(Finding {
                refusal: "agent_document_digest",
                detail: format!("part {part}: the agent document is {d}, the binding pins {pin}"),
            });
        }
    }
    findings.extend(agent_contents(&agents));
    findings.extend(nested_findings);
    findings.extend(unit_schedules(bundle, arch, &packing));
    findings.extend(served(arch, binding, &packing));
    findings.extend(unit_runtimes(bundle, arch, binding, &packing));
    findings.extend(unit_volumes(bundle));
    findings.extend(config::findings(binding, &packing));
    findings.extend(config::check_units(bundle, &packing));
    findings.extend(scratch::findings(binding, &packing));
    findings.extend(scratch::check_units(bundle, &packing));
    findings.extend(grants::check(arch, binding, &packing));
    findings.extend(grants::check_units(bundle));
    findings.extend(addresses(arch));
    findings.extend(ui::check(bundle, accepts));
    if findings.is_empty() {
        return Ok(());
    }
    sort(&mut findings);
    findings.dedup();
    Err(Refused::NotAdmitted(findings))
}

/// Whether part `p` names the volume `name` under `parts.<p>.volumes[]` of the binding.
fn names_volume(binding: &Value, p: &str, name: &str) -> bool {
    binding["parts"][p]["volumes"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|v| v["name"] == name)
}

/// The access of unit `u`'s claim on the volume `name`, from who mounts it: `read_only_many`
/// for a shared read-only workspace; `read_write_many` when a part of another unit names the
/// volume under `parts.<p>.volumes[]`, or when the claim is not one per copy (`per_instance`)
/// and the unit does not run exactly one copy (more replicas, or a `job` or `scheduled` unit,
/// whose runs start on demand and may overlap); `read_write_once` otherwise, whatever the
/// sharing among the parts of the unit.
fn access(
    plan: &Plan,
    binding: &Value,
    u: &Unit,
    name: &str,
    read_only: bool,
    per_instance: bool,
) -> &'static str {
    if read_only {
        return "read_only_many";
    }
    let other_unit = plan.units.iter().any(|o| {
        o.unit.name != u.name && o.unit.parts.iter().any(|p| names_volume(binding, p, name))
    });
    if !other_unit && (per_instance || u.replicas == Some(1)) {
        "read_write_once"
    } else {
        "read_write_many"
    }
}

/// The volumes a unit claims: one per workspace part and one per runtime part with durable
/// state, `<part>-state`, which names the programs outside the runtime that also write it
/// (`writers`, from `state_writers`) when the part names any. Per copy of its part when it is
/// `per` one; its access from who mounts it ([`access`]); its size is `parts.<p>.size` of the
/// binding, its mount `parts.<p>.volumes[].mount` of a part of the unit that names it.
fn volumes(inputs: &Inputs, plan: &Plan, u: &Unit) -> Vec<Value> {
    let parts = &u.parts;
    let binding = &inputs.binding.doc;
    let mut out = Vec::new();
    for p in parts {
        let part = &inputs.arch["parts"][p];
        let per_instance = !part["per"].is_null();
        if binding_form::durable_state(part) {
            let name = binding_form::state_volume(p);
            let mut v = json!({
                "name": name,
                "access": access(plan, binding, u, &name, false, per_instance),
                "per_instance": per_instance,
            });
            if let Some(w) = binding_form::state_writers(part) {
                v["writers"] = w.clone();
            }
            if let Some(size) = inputs.binding.doc["parts"][p]["size"].as_str() {
                v["size"] = json!(size);
            }
            if let Some(m) = mount_of(&inputs.binding.doc, parts, &name).and_then(Value::as_str) {
                v["mount"] = json!(m);
            }
            out.push(v);
            continue;
        }
        if part["block"] != "workspace" {
            continue;
        }
        let read_only = value(&part["sharing"]).as_str() == Some("shared_read_only");
        let mut v = json!({
            "name": p,
            "access": access(plan, binding, u, p, read_only, per_instance),
            "per_instance": per_instance,
        });
        if let Some(size) = inputs.binding.doc["parts"][p]["size"].as_str() {
            v["size"] = json!(size);
        }
        if let Some(m) = mount_of(&inputs.binding.doc, parts, p).and_then(Value::as_str) {
            v["mount"] = json!(m);
        }
        out.push(v);
    }
    out
}

fn unit(
    inputs: &Inputs,
    plan: &Plan,
    pu: &PlannedUnit,
    docs: &BTreeMap<String, AgentDoc>,
) -> Value {
    let u = &pu.unit;
    let arch = inputs.arch;
    let binding = &inputs.binding.doc;
    let parts = &arch["parts"];
    let holds = |p: &str| u.parts.iter().any(|x| x == p);
    let agentic = matches!(u.kind.as_str(), "agent" | "job" | "scheduled");

    let code = if agentic {
        let agents: Vec<Value> = u
            .parts
            .iter()
            .filter_map(|p| docs.get(p).map(|d| json!({ "part": p, "agent": d.digest })))
            .collect();
        let mut c = json!({ "role": "agent-runtime", "agents": agents });
        if let Some(r) = binding_form::image_reference(binding, binding_form::AGENT_RUNTIME) {
            c["runtime"] = json!(r);
        }
        c
    } else {
        match u.kind.as_str() {
            "core" | "service" => match binding_form::unit_image(binding, u)
                .image()
                .and_then(|r| binding_form::pinned_reference(binding, r))
            {
                Some(r) => json!({ "runtime": r }),
                None => json!({ "role": "core-runtime" }),
            },
            "model" => json!({ "role": "model-server" }),
            _ => json!({ "role": "store" }),
        }
    };

    let c = &binding["claims"][&u.name];
    let accelerators = c["accelerators"]
        .as_u64()
        .unwrap_or(u64::from(u.kind == "model"));
    let claims = json!({
        "cpu_millis": c["cpu_millis"].as_u64().unwrap_or(DEFAULT_CPU_MILLIS),
        "memory_mib": c["memory_mib"].as_u64().unwrap_or(DEFAULT_MEMORY_MIB),
        "accelerators": accelerators,
        "volumes": volumes(inputs, plan, u),
    });

    let isolation = u
        .parts
        .iter()
        .filter(|p| parts[*p]["block"] == "sandbox")
        .filter_map(|p| value(&parts[p]["isolation"]).as_str())
        .max_by_key(|l| super::binding::isolation_rank(l))
        .unwrap_or("none");

    let mut checks = match u.kind.as_str() {
        "agent" | "job" | "scheduled" => json!({
            "startup": { "kind": "role", "role": "agent-runtime", "step": "check" },
        }),
        "core" | "service" => json!({
            "ready": { "kind": "http", "path": "/ready" },
            "alive": { "kind": "http", "path": "/alive" },
        }),
        "model" => json!({ "ready": { "kind": "role", "role": "model-server", "step": "ready" } }),
        _ => json!({}),
    };
    for (k, c) in binding["checks"][&u.name].as_object().into_iter().flatten() {
        if let (Some(port), Some(check)) = (c["port"].as_u64(), checks.get_mut(k)) {
            check["port"] = json!(port);
        }
    }

    let secrets: BTreeSet<String> = u
        .parts
        .iter()
        .flat_map(|p| grants::declared_secrets(binding, p))
        .collect();

    // Every link that leaves the unit, then every named destination its parts declare; each
    // with the secret its declaration names and `name`, what the target resolves: an external
    // part's reach (its name, or its address), the unit a crossing goes to, a named
    // destination's own name.
    let mut egress = Vec::new();
    let secret_of = |from: &str, to: &str| {
        grants::declared_egress(binding, from)
            .into_iter()
            .find(|d| d.to == to)
            .and_then(|d| d.secret)
    };
    for l in arch["links"].as_array().into_iter().flatten() {
        let (from, to) = (text(&l["from"]), text(&l["to"]));
        if !holds(from) || holds(to) {
            continue;
        }
        let name = link_name(l);
        let mut e = json!({ "link": name, "from": from, "to": to, "carries": text(&l["carries"]), "name": to });
        if plan.external.iter().any(|x| x == to) {
            e["external"] = json!(true);
            let reach = inputs.packing.reach_of(to);
            if let Some(a) = reach["address"].as_str() {
                e["name"] = json!(a);
                e["address"] = json!(a);
            } else if let Some(n) = reach["name"].as_str() {
                e["name"] = json!(n);
            }
        } else if let Some(c) = plan.crossings.iter().find(|c| c.link == name) {
            e["unit"] = json!(c.to_unit);
            e["transport"] = json!(c.transport);
            e["name"] = json!(c.to_unit);
        }
        if let Some(s) = secret_of(from, to) {
            e["secret"] = json!(s);
        }
        egress.push(e);
    }
    for p in &u.parts {
        for d in grants::declared_egress(binding, p) {
            if parts.get(&d.to).is_some() {
                continue;
            }
            let mut e = json!({ "from": p, "to": d.to, "named": true, "name": d.to });
            if let Some(s) = d.secret {
                e["secret"] = json!(s);
            }
            match d.address {
                Some(a) => e["address"] = a,
                None => e["unresolved"] = json!(true),
            }
            egress.push(e);
        }
    }
    egress.sort_by(|a, b| {
        (text(&a["link"]), text(&a["from"]), text(&a["to"])).cmp(&(
            text(&b["link"]),
            text(&b["from"]),
            text(&b["to"]),
        ))
    });

    let ports: Vec<Value> = inputs
        .contract
        .ports
        .iter()
        .filter(|p| holds(&p.part))
        .map(|p| {
            let settings: Map<String, Value> = p.settings.clone().into_iter().collect();
            json!({
                "port": p.port,
                "part": p.part,
                "instances": p.instances,
                "settings": settings,
            })
        })
        .collect();

    let (kind, shape) = agent_shape(&u.kind);
    let mut v = json!({
        "name": u.name,
        "kind": kind,
        "parts": u.parts,
        "code": code,
        "claims": claims,
        "isolation": isolation,
        "checks": checks,
        "secrets": secrets.into_iter().collect::<Vec<_>>(),
        "egress": egress,
        "roles": pu.roles,
        "ports": ports,
        "source": { "unit": u.name, "parts": u.parts },
    });
    if let Some(r) = u.replicas {
        v["replicas"] = json!(r);
    }
    // Admission refused a configuration of the wrong form or in conflict, so `Err` does not
    // reach here; none is left out, and a bundle without one keeps its bytes.
    if let Ok(Some(c)) = config::resolve(binding, u) {
        v["config"] = c;
    }
    // The same for its scratch directories (R118): a unit without any carries no `scratch`.
    if let Ok(Some(s)) = scratch::resolve(binding, u) {
        v["scratch"] = s;
    }
    if let Some(s) = shape {
        v["shape"] = json!(s);
    }
    if u.kind == "scheduled" {
        if let Some(c) = clock(arch, &u.parts) {
            v["clock"] = c;
        }
        // Admission refused an `every` no cron rule states, so `Err` does not reach here.
        if let Ok(Some(cron)) = schedule_of(arch, u) {
            v["schedule"] = json!(cron);
        }
    }
    v
}

/// The kind a unit of a packing kind is written with in a bundle, and its shape: an `agent`,
/// `job` or `scheduled` unit is an `agent` unit of shape `service`, `job` or `scheduled`; any
/// other kind is written as it is, without a shape.
pub fn agent_shape(kind: &str) -> (&str, Option<&'static str>) {
    match kind {
        "agent" => ("agent", Some("service")),
        "job" => ("agent", Some("job")),
        "scheduled" => ("agent", Some("scheduled")),
        other => (other, None),
    }
}

/// The clock of a scheduled unit: the trigger or schedule that wakes its part, the schedule's
/// interval and first firing, and the trigger's `policy` and `max_parallel` as the overlap rule.
fn clock(arch: &Value, parts: &[String]) -> Option<Value> {
    let l = clock_link(arch, parts)?;
    let from = text(&l["from"]);
    let p = &arch["parts"][from];
    let sched = schedule_part(arch, from);
    let trigger = (p["block"] == "trigger").then_some(from);
    let s = &arch["parts"][sched];
    let mut out = json!({ "schedule": sched });
    if let Some(t) = trigger {
        out["trigger"] = json!(t);
        for k in ["policy", "max_parallel"] {
            let v = value(&p[k]);
            if !v.is_null() {
                out[k] = v.clone();
            }
        }
    }
    for k in ["every", "next_fire_at"] {
        let v = value(&s[k]);
        if !v.is_null() {
            out[k] = v.clone();
        }
    }
    Some(out)
}
