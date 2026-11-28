//! Nested architectures. A part of block `architecture` is another architecture document:
//! `{ block: architecture, ref: <path relative to this document>, digest: sha256:<hex> }`, the
//! digest being [`digest`] of the referenced document: the SHA-256 of the canonical JSON of its
//! parts, links and border, so a reformatted file keeps its pin. The nested document's `border` is its
//! interface: a name `<part>.<border part>` addresses one of its border parts from outside, and no
//! other inner part is addressable.
//!
//! [`load`] reads a document and resolves its references recursively: the result is the document
//! as written with the nested document (resolved the same way, its markers at their values) under
//! the key `inner` of its part. [`flatten`] turns that into one document the check and the
//! simulator read as before: the nested parts become parts named `<part>.<inner part>`, the
//! nested links join the outer ones under those names, and a link that addresses a border part
//! already names its flattened part.

use crate::check::{self, value};
use crate::doc::parse_yaml;
use crate::marked;
use serde_json::{Map, Value, json};
use sha2::{Digest as _, Sha256};
use std::fmt;
use std::path::{Path, PathBuf};

/// The block of a part that is another architecture document.
pub const BLOCK: &str = "architecture";

/// The key under which [`load`] puts the resolved nested document beside its part.
pub const INNER: &str = "inner";

/// The keys of a document its digest covers (`agentic.architecture.Digest`: the canonical form
/// of an architecture's parts, links and border).
pub const DIGESTED: &[&str] = &["border", "links", "parts"];

/// `v` as canonical JSON: object keys sorted at every level, no whitespace, strings and numbers as
/// JSON writes them.
pub fn canonical(v: &Value) -> String {
    fn write(v: &Value, out: &mut String) {
        match v {
            Value::Object(o) => {
                let mut keys: Vec<&String> = o.keys().collect();
                keys.sort();
                out.push('{');
                for (i, k) in keys.into_iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&Value::from(k.as_str()).to_string());
                    out.push(':');
                    write(&o[k], out);
                }
                out.push('}');
            }
            Value::Array(xs) => {
                out.push('[');
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write(x, out);
                }
                out.push(']');
            }
            other => out.push_str(&other.to_string()),
        }
    }
    let mut out = String::new();
    write(v, &mut out);
    out
}

/// `sha256:<hex>` of the canonical form of `doc`'s parts, links and border, the pin a referring
/// document writes as `digest`. Reformatting the file, reordering keys, comments, the summary and
/// the policies leave it unchanged.
pub fn digest(doc: &Value) -> String {
    let covered: Map<String, Value> = DIGESTED
        .iter()
        .filter_map(|k| doc.get(*k).map(|v| ((*k).to_string(), v.clone())))
        .collect();
    let hex: String = Sha256::digest(canonical(&Value::Object(covered)).as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!("sha256:{hex}")
}

/// The digest of the architecture document in `text`.
pub fn digest_text(text: &str) -> Result<String, String> {
    let doc = parse_yaml(text)?;
    if !doc.is_object() {
        return Err("no architecture document".into());
    }
    Ok(digest(&doc))
}

fn well_formed(pin: &str) -> bool {
    pin.strip_prefix("sha256:")
        .is_some_and(|h| h.len() == 64 && h.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')))
}

/// One refusal of a reference. `part` is the referring part, its name dotted from the outermost
/// document (`console`, `console.vault`), or for [`Refusal::NotInBorder`] the address as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// No `ref` or `digest`, a digest that is not `sha256:<64 hex>`, a `count` other than 1, or a
    /// file that is no architecture document.
    ReferenceInvalid { part: String, detail: String },
    /// The referenced file cannot be read.
    Missing {
        part: String,
        reference: String,
        detail: String,
    },
    /// The referenced document does not have the pinned digest.
    DigestMismatch {
        part: String,
        reference: String,
        expected: String,
        actual: String,
    },
    /// The reference leads back to a document that is being resolved.
    Cycle { part: String, chain: Vec<String> },
    /// `<part>.<name>` where `name` is not in the border of the nested document.
    NotInBorder {
        part: String,
        nested: String,
        name: String,
    },
    /// The nested document's own check refuses it.
    InnerRefused { part: String, detail: String },
}

impl Refusal {
    pub fn rule(&self) -> &'static str {
        match self {
            Refusal::ReferenceInvalid { .. } => "nested_reference_invalid",
            Refusal::Missing { .. } => "nested_missing",
            Refusal::DigestMismatch { .. } => "nested_digest_mismatch",
            Refusal::Cycle { .. } => "nested_cycle",
            Refusal::NotInBorder { .. } => "nested_not_in_border",
            Refusal::InnerRefused { .. } => "nested_refused",
        }
    }

    pub fn part(&self) -> &str {
        match self {
            Refusal::ReferenceInvalid { part, .. }
            | Refusal::Missing { part, .. }
            | Refusal::DigestMismatch { part, .. }
            | Refusal::Cycle { part, .. }
            | Refusal::NotInBorder { part, .. }
            | Refusal::InnerRefused { part, .. } => part,
        }
    }

    /// The same refusal seen from the document that refers to `outer`.
    fn within(self, outer: &str) -> Self {
        let up = |p: String| format!("{outer}.{p}");
        match self {
            Refusal::ReferenceInvalid { part, detail } => Refusal::ReferenceInvalid {
                part: up(part),
                detail,
            },
            Refusal::Missing {
                part,
                reference,
                detail,
            } => Refusal::Missing {
                part: up(part),
                reference,
                detail,
            },
            Refusal::DigestMismatch {
                part,
                reference,
                expected,
                actual,
            } => Refusal::DigestMismatch {
                part: up(part),
                reference,
                expected,
                actual,
            },
            Refusal::Cycle { part, chain } => Refusal::Cycle {
                part: up(part),
                chain,
            },
            Refusal::NotInBorder { part, nested, name } => Refusal::NotInBorder {
                part: up(part),
                nested: up(nested),
                name,
            },
            Refusal::InnerRefused { part, detail } => Refusal::InnerRefused {
                part: up(part),
                detail,
            },
        }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let detail = match self {
            Refusal::ReferenceInvalid { detail, .. } => detail.clone(),
            Refusal::Missing {
                reference, detail, ..
            } => format!("ref `{reference}` cannot be read: {detail}"),
            Refusal::DigestMismatch {
                reference,
                expected,
                actual,
                ..
            } => format!(
                "ref `{reference}`: digest mismatch: expected {expected}, actual {actual}; pin the file with `agenticspec sim digest`"
            ),
            Refusal::Cycle { chain, .. } => {
                format!("a cycle of references: {}", chain.join(" -> "))
            }
            Refusal::NotInBorder { nested, name, .. } => format!(
                "`{name}` is not in the border of `{nested}`; only its border parts are addressable"
            ),
            Refusal::InnerRefused { detail, .. } => {
                format!("the nested architecture is refused: {detail}")
            }
        };
        write!(f, "{}: part `{}`: {detail}", self.rule(), self.part())
    }
}

/// Why [`load`] returns no document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The document itself cannot be read or parsed.
    Unreadable(String),
    /// Its references are refused.
    Refused(Vec<Refusal>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unreadable(e) => f.write_str(e),
            Error::Refused(list) => {
                let names: Vec<String> = list.iter().map(ToString::to_string).collect();
                f.write_str(&names.join("; "))
            }
        }
    }
}

/// The parts of `doc` of block `architecture`, by name.
fn nested_parts(doc: &Value) -> Vec<String> {
    doc["parts"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(_, p)| p["block"] == BLOCK)
        .map(|(n, _)| n.clone())
        .collect()
}

/// Reads the document at `path` and resolves its nested architectures; each setting of a block's
/// enum comes back as its word (`settings::canonical`).
pub fn load(path: &Path) -> Result<Value, Error> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| Error::Unreadable(format!("{}: {e}", path.display())))?;
    let doc =
        parse_yaml(&text).map_err(|e| Error::Unreadable(format!("{}: {e}", path.display())))?;
    let shown = path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    );
    let canon = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut stack = vec![(canon, shown)];
    let mut out = Vec::new();
    let dir = path.parent().unwrap_or(Path::new("."));
    let doc = annotate(doc, dir, &mut stack, &mut out);
    if out.is_empty() {
        // Each nested document was held to its pin as written; what a reader reads is the
        // document with the one spelling of each enum setting.
        Ok(crate::settings::canonical(&doc))
    } else {
        Err(Error::Refused(out))
    }
}

/// Whether `doc` has a part of block `architecture` that [`load`] did not resolve.
pub fn unresolved(doc: &Value) -> Option<String> {
    nested_parts(doc)
        .into_iter()
        .find(|n| !doc["parts"][n][INNER].is_object())
}

/// `doc` with the nested document of each part of block `architecture` under the key `inner`.
fn annotate(
    mut doc: Value,
    dir: &Path,
    stack: &mut Vec<(PathBuf, String)>,
    out: &mut Vec<Refusal>,
) -> Value {
    for name in nested_parts(&doc) {
        if let Some(inner) = resolve_part(&name, &doc["parts"][&name], dir, stack, out) {
            doc["parts"][&name][INNER] = inner;
        }
    }
    out.extend(addresses(&doc));
    doc
}

/// The resolved nested document of part `name`, or `None` with its refusals pushed to `out`.
fn resolve_part(
    name: &str,
    p: &Value,
    dir: &Path,
    stack: &mut Vec<(PathBuf, String)>,
    out: &mut Vec<Refusal>,
) -> Option<Value> {
    let invalid = |detail: String| Refusal::ReferenceInvalid {
        part: name.to_string(),
        detail,
    };
    let Some(reference) = p["ref"].as_str().filter(|r| !r.is_empty()) else {
        out.push(invalid(
            "a nested architecture names its file in `ref`".into(),
        ));
        return None;
    };
    let Some(pin) = p["digest"].as_str() else {
        out.push(invalid(format!(
            "a nested architecture pins `{reference}` with `digest: sha256:<hex>`; `agenticspec sim digest {reference}` prints it"
        )));
        return None;
    };
    if !well_formed(pin) {
        out.push(invalid(format!(
            "digest `{pin}` is not `sha256:` and 64 lowercase hex digits"
        )));
        return None;
    }
    match value(&p["count"]) {
        Value::Null => {}
        c if c.as_f64() == Some(1.0) => {}
        c => {
            out.push(invalid(format!(
                "a nested architecture is one copy; count {c} is not 1"
            )));
            return None;
        }
    }
    let file = dir.join(reference);
    let missing = |detail: String| Refusal::Missing {
        part: name.to_string(),
        reference: reference.to_string(),
        detail,
    };
    let canon = match std::fs::canonicalize(&file) {
        Ok(c) => c,
        Err(e) => {
            out.push(missing(e.to_string()));
            return None;
        }
    };
    if stack.iter().any(|(c, _)| *c == canon) {
        let mut chain: Vec<String> = stack.iter().map(|(_, s)| s.clone()).collect();
        chain.push(reference.to_string());
        out.push(Refusal::Cycle {
            part: name.to_string(),
            chain,
        });
        return None;
    }
    let text = match std::fs::read_to_string(&file) {
        Ok(t) => t,
        Err(e) => {
            out.push(missing(e.to_string()));
            return None;
        }
    };
    let inner = match parse_yaml(&text) {
        Ok(v) if v.is_object() => v,
        Ok(_) => {
            out.push(invalid(format!(
                "`{reference}` is no architecture document"
            )));
            return None;
        }
        Err(e) => {
            out.push(invalid(format!("`{reference}`: {e}")));
            return None;
        }
    };
    let actual = digest(&inner);
    if actual != pin {
        out.push(Refusal::DigestMismatch {
            part: name.to_string(),
            reference: reference.to_string(),
            expected: pin.to_string(),
            actual,
        });
        return None;
    }
    if let Some(f) = inner["format"].as_str()
        && f != "agenticspec-architecture/1"
    {
        out.push(invalid(format!(
            "`{reference}` has format `{f}`, expected agenticspec-architecture/1"
        )));
        return None;
    }
    // The nested document is searched by its own experiment, not by the outer one.
    let inner = match marked::resolve(&inner, &Map::new()) {
        Ok(v) => v,
        Err(e) => {
            out.push(invalid(format!("`{reference}`: {e}")));
            return None;
        }
    };
    stack.push((canon, reference.to_string()));
    let mut below = Vec::new();
    let inner_dir = file.parent().unwrap_or(Path::new(".")).to_path_buf();
    let inner = annotate(inner, &inner_dir, stack, &mut below);
    stack.pop();
    if !below.is_empty() {
        out.extend(below.into_iter().map(|r| r.within(name)));
        return None;
    }
    let refused = check::check(&flatten(&inner));
    if !refused.is_empty() {
        out.extend(refused.iter().map(|r| Refusal::InnerRefused {
            part: name.to_string(),
            detail: r.to_string(),
        }));
        return None;
    }
    Some(inner)
}

fn strings(v: &Value) -> Vec<&str> {
    match v {
        Value::String(s) => vec![s.as_str()],
        Value::Array(xs) => xs.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    }
}

/// The keys of a trigger part whose value names a part.
const TRIGGER_KEYS: [&str; 3] = ["schedule", "channel", "source"];

/// Every name of a part that `doc` writes, as written: links, `per`, `acts_for`, `egress`, the
/// model parts of a `model_share`, a trigger's `schedule`, `channel` and `source`, and the border.
fn names_written(doc: &Value) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for l in doc["links"].as_array().into_iter().flatten() {
        for key in ["from", "to", "over"] {
            out.extend(l[key].as_str().map(str::to_string));
        }
    }
    for (_, p) in doc["parts"].as_object().into_iter().flatten() {
        for key in ["per", "acts_for", "egress"] {
            out.extend(strings(&p[key]).into_iter().map(str::to_string));
        }
        if let Some(shares) = value(&p["model_share"]).as_object() {
            out.extend(shares.keys().cloned());
        }
        if p["block"] == "trigger" {
            for key in TRIGGER_KEYS {
                out.extend(value(&p[key]).as_str().map(str::to_string));
            }
        }
    }
    out.extend(strings(&doc["border"]).into_iter().map(str::to_string));
    out
}

/// The addresses `<part>.<name>` of `doc` whose `name` is not in the border of the resolved
/// nested document of `part`.
fn addresses(doc: &Value) -> Vec<Refusal> {
    let mut out: Vec<Refusal> = Vec::new();
    for n in names_written(doc) {
        let Some((head, rest)) = n.split_once('.') else {
            continue;
        };
        let p = &doc["parts"][head];
        if p["block"] != BLOCK || !p[INNER].is_object() {
            continue;
        }
        if !strings(&p[INNER]["border"]).contains(&rest) {
            let r = Refusal::NotInBorder {
                part: n.clone(),
                nested: head.to_string(),
                name: rest.to_string(),
            };
            if !out.contains(&r) {
                out.push(r);
            }
        }
    }
    out
}

fn prefixed(prefix: &str, name: &str) -> String {
    format!("{prefix}.{name}")
}

/// `v` with every name of a part in it prefixed: a string or a list of strings.
fn prefix_names(prefix: &str, v: &Value) -> Value {
    match v {
        Value::String(s) => Value::from(prefixed(prefix, s)),
        Value::Array(xs) => Value::Array(xs.iter().map(|x| prefix_names(prefix, x)).collect()),
        other => other.clone(),
    }
}

fn prefix_part(prefix: &str, p: &Value) -> Value {
    let mut p = p.clone();
    let trigger = p["block"] == "trigger";
    if let Some(o) = p.as_object_mut() {
        for key in ["per", "acts_for", "egress"] {
            if let Some(v) = o.get(key) {
                let v = prefix_names(prefix, v);
                o.insert(key.into(), v);
            }
        }
        if let Some(Value::Object(shares)) = o.get("model_share") {
            let shares: Map<String, Value> = shares
                .iter()
                .map(|(k, v)| (prefixed(prefix, k), v.clone()))
                .collect();
            o.insert("model_share".into(), Value::Object(shares));
        }
        if trigger {
            for key in TRIGGER_KEYS {
                if let Some(v) = o.get(key) {
                    let v = prefix_names(prefix, v);
                    o.insert(key.into(), v);
                }
            }
        }
    }
    p
}

fn prefix_link(prefix: &str, l: &Value) -> Value {
    let mut l = l.clone();
    if let Some(o) = l.as_object_mut() {
        for key in ["from", "to", "over"] {
            if let Some(v) = o.get(key) {
                let v = prefix_names(prefix, v);
                o.insert(key.into(), v);
            }
        }
    }
    l
}

/// One document from a document [`load`] resolved: each nested architecture's parts as
/// `<part>.<inner part>`, its links among them, and under `nested` one entry per nested
/// architecture (its part, name, ref, digest, counts of its own parts and links, and its border
/// parts by their flattened names). The outer `policies` hold; the nested ones do not. A
/// document without nested architectures comes back with its parts unchanged. Every setting of
/// a block's enum is in its one spelling, the word (`settings::canonical`): this is the document
/// as the check's rules, the simulator, the contract and the bundle read it.
pub fn flatten(doc: &Value) -> Value {
    crate::settings::canonical(&merge(doc))
}

fn merge(doc: &Value) -> Value {
    if nested_parts(doc).is_empty() {
        return doc.clone();
    }
    let mut out = doc.clone();
    let mut parts = Map::new();
    let mut links: Vec<Value> = doc["links"].as_array().cloned().unwrap_or_default();
    let mut nested: Vec<Value> = doc["nested"].as_array().cloned().unwrap_or_default();
    for (name, p) in doc["parts"].as_object().into_iter().flatten() {
        if p["block"] != BLOCK || !p[INNER].is_object() {
            parts.insert(name.clone(), p.clone());
            continue;
        }
        let raw = &p[INNER];
        let inner = merge(raw);
        for (n, ip) in inner["parts"].as_object().into_iter().flatten() {
            parts.insert(prefixed(name, n), prefix_part(name, ip));
        }
        for l in inner["links"].as_array().into_iter().flatten() {
            links.push(prefix_link(name, l));
        }
        nested.push(json!({
            "part": name,
            "architecture": raw["architecture"].as_str().unwrap_or(name),
            "reference": p["ref"],
            "digest": p["digest"],
            "parts": raw["parts"].as_object().map_or(0, |o| o.len()),
            "links": raw["links"].as_array().map_or(0, |l| l.len()),
            "border": prefix_names(name, &Value::from(strings(&raw["border"]))),
        }));
        for e in inner["nested"].as_array().into_iter().flatten() {
            let mut e = e.clone();
            e["part"] = prefix_names(name, &e["part"]);
            e["border"] = prefix_names(name, &e["border"]);
            nested.push(e);
        }
    }
    out["parts"] = Value::Object(parts);
    out["links"] = Value::Array(links);
    out["nested"] = Value::Array(nested);
    out
}

/// The nested architectures of a document [`load`] resolved, outermost first: part, name,
/// reference, digest and the counts of its own parts and links.
pub fn listed(doc: &Value) -> Vec<Value> {
    flatten(doc)["nested"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| {
            let mut e = e.clone();
            if let Some(o) = e.as_object_mut() {
                o.remove("border");
            }
            e
        })
        .collect()
}

/// `doc` without the resolved nested documents: the document as its author writes it.
pub fn strip(doc: &Value) -> Value {
    let mut out = doc.clone();
    for name in nested_parts(doc) {
        if let Some(o) = out["parts"][&name].as_object_mut() {
            o.remove(INNER);
        }
    }
    out
}

/// `doc` with the resolved nested documents of `from` beside the parts of the same name and
/// reference.
pub fn graft(doc: &Value, from: &Value) -> Value {
    let mut out = doc.clone();
    for name in nested_parts(doc) {
        let src = &from["parts"][&name];
        if src[INNER].is_object()
            && src["ref"] == doc["parts"][&name]["ref"]
            && src["digest"] == doc["parts"][&name]["digest"]
        {
            out["parts"][&name][INNER] = src[INNER].clone();
        }
    }
    out
}
