//! The settings a part may carry, per block, each with its type: one table,
//! `blocks/settings.yaml` (`agenticspec-settings/1`), that the check, the contract and the docs
//! read. Where a setting takes the values of an enum a block declares, the table names the enum
//! and its variants are read from the block files (`unavailable::SOURCES`), in snake case.

use crate::doc::{num, parse_duration};
use serde_json::Value;
use std::fmt::Write as _;
use std::sync::OnceLock;

pub const FORMAT: &str = "agenticspec-settings/1";

/// The table as committed.
pub const SOURCE: &str = include_str!("../../../blocks/settings.yaml");

/// The keys of a part that are not settings: its block, and the resolved document of a nested
/// architecture (`nest::INNER`).
pub const NOT_SETTINGS: &[&str] = &["block", crate::nest::INNER];

/// The setting every part may carry whose keys and values are free: what a document records about
/// a part beyond its typed settings. The check takes it as written (a map) and the simulator and
/// the search never read it.
pub const NOTES: &str = "notes";

/// What values a setting takes.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Number,
    Integer,
    Boolean,
    /// One of these words; `from` is the block enum they come from, if any.
    Enum {
        values: Vec<String>,
        from: Option<String>,
    },
    Duration,
    /// A text, or with `list` a non-empty list of non-empty texts.
    String {
        list: bool,
    },
    /// The name of another part, or a list of names.
    Part {
        list: bool,
    },
    /// A map from the name of a part to a number of at least 0.
    Shares,
    /// A map with keys and values of any kind, never typed further and never read.
    Notes,
}

/// The type of a setting.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: Kind,
    pub unit: Option<String>,
    /// For a number or integer, the bound; for a duration, the bound in seconds.
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// The ports of the contract a setting narrows.
#[derive(Debug, Clone, PartialEq)]
pub enum Narrows {
    None,
    /// Every port the part uses.
    All,
    Ports(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Setting {
    pub name: String,
    pub ty: Type,
    pub default: Option<Value>,
    pub narrows: Narrows,
    pub summary: String,
}

/// The settings of the parts of one block.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The block as a part writes it (`pool`).
    pub kind: String,
    /// The block's domain (`agentic.resource`).
    pub domain: String,
    pub settings: Vec<Setting>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// The settings every part may carry, whatever its block.
    pub every_part: Vec<Setting>,
    pub blocks: Vec<Block>,
}

/// A variant of an enum as a document writes it: `AgentMessage` is `agent_message`.
pub fn snake(variant: &str) -> String {
    let mut out = String::new();
    for (i, c) in variant.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// The variants of enum `name` of the blocks, in snake case, or `None` when no block declares it.
///
/// Only a block file whose text declares the enum (`name: <enum>`) is parsed: parsing every block
/// file costs about 0.8 s in a test build, and the settings table is read once in each test
/// process.
pub fn enum_variants(name: &str) -> Option<Vec<String>> {
    let declared = format!("name: {name}");
    for (_, text) in crate::unavailable::SOURCES {
        if !text.contains(&declared) {
            continue;
        }
        let Ok(doc) = serde_yaml_ng::from_str::<Value>(text) else {
            continue;
        };
        let found = doc["types"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|t| t["name"] == name && t["kind"] == "enum");
        if let Some(t) = found {
            return Some(
                t["variants"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|v| v.as_str().or_else(|| v["name"].as_str()))
                    .map(snake)
                    .collect(),
            );
        }
    }
    None
}

/// The ports of the contract a setting may narrow: those of `blocks/ports.json`, built in.
fn ports() -> &'static [String] {
    static PORTS: OnceLock<Vec<String>> = OnceLock::new();
    PORTS.get_or_init(|| {
        let doc: Value =
            serde_json::from_str(include_str!("../../../blocks/ports.json")).unwrap_or_default();
        doc["ports"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|p| p["name"].as_str().map(str::to_string))
            .collect()
    })
}

/// The non-empty list of distinct words under `key`, or why it is not one.
fn words(at: &str, key: &str, v: &Value) -> Result<Vec<String>, String> {
    let list = v
        .as_array()
        .filter(|l| !l.is_empty())
        .ok_or_else(|| format!("{at}: `{key}` is {v}, expected a non-empty list of words"))?;
    let mut out: Vec<String> = Vec::new();
    for x in list {
        let w = x
            .as_str()
            .filter(|w| !w.is_empty())
            .ok_or_else(|| format!("{at}: `{key}` holds {x}, which is no word"))?;
        if out.iter().any(|o| o == w) {
            return Err(format!("{at}: `{key}` holds `{w}` twice"));
        }
        out.push(w.to_string());
    }
    Ok(out)
}

/// The keys only some types take, and the types that take each.
const TYPED_KEYS: &[(&str, &[&str])] = &[
    ("min", &["number", "integer", "duration"]),
    ("max", &["number", "integer", "duration"]),
    ("unit", &["number", "integer"]),
    ("list", &["string", "part"]),
    ("enum", &["enum"]),
    ("values", &["enum"]),
];

fn setting(at: &str, v: &Value) -> Result<Setting, String> {
    let name = v["name"]
        .as_str()
        .ok_or_else(|| format!("{at}: a setting without `name`"))?
        .to_string();
    let at = format!("{at}.{name}");
    const KEYS: &[&str] = &[
        "name", "type", "unit", "min", "max", "default", "enum", "values", "list", "narrows",
        "summary",
    ];
    if let Some(k) = v
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, _)| k)
        .find(|k| !KEYS.contains(&k.as_str()))
    {
        return Err(format!("{at}: unknown key `{k}`"));
    }
    let bound = |key: &str| -> Result<Option<f64>, String> {
        match &v[key] {
            Value::Null => Ok(None),
            b => num(b)
                .map(Some)
                .ok_or_else(|| format!("{at}: `{key}` is not a number or duration")),
        }
    };
    let ty = v["type"].as_str().unwrap_or("");
    for (key, types) in TYPED_KEYS {
        if !v[*key].is_null() && !types.contains(&ty) {
            return Err(format!(
                "{at}: `{key}` belongs to a setting of type {}, not {ty}",
                types.join(" or ")
            ));
        }
    }
    if !v["list"].is_null() && !v["list"].is_boolean() {
        return Err(format!(
            "{at}: `list` is {}, expected true or false",
            v["list"]
        ));
    }
    if !v["unit"].is_null() && !v["unit"].is_string() {
        return Err(format!("{at}: `unit` is {}, expected a text", v["unit"]));
    }
    let kind = match v["type"].as_str() {
        Some("number") => Kind::Number,
        Some("integer") => Kind::Integer,
        Some("boolean") => Kind::Boolean,
        Some("duration") => Kind::Duration,
        Some("string") => Kind::String {
            list: v["list"] == true,
        },
        Some("shares") => Kind::Shares,
        Some("notes") => Kind::Notes,
        Some("part") => Kind::Part {
            list: v["list"] == true,
        },
        Some("enum") => match (v["enum"].as_str(), v["values"].is_null()) {
            (Some(e), true) => {
                let values = enum_variants(e)
                    .ok_or_else(|| format!("{at}: no block declares the enum `{e}`"))?;
                if values.is_empty() {
                    return Err(format!("{at}: the enum `{e}` has no values"));
                }
                Kind::Enum {
                    values,
                    from: Some(e.to_string()),
                }
            }
            (None, false) if v["enum"].is_null() => Kind::Enum {
                values: words(&at, "values", &v["values"])?,
                from: None,
            },
            _ => {
                return Err(format!(
                    "{at}: an enum names `enum` or `values`, one of them"
                ));
            }
        },
        other => return Err(format!("{at}: unknown type {other:?}")),
    };
    let narrows = match &v["narrows"] {
        Value::Null => Narrows::None,
        Value::String(s) if s == "all" => Narrows::All,
        Value::Array(_) => {
            let named = words(&at, "narrows", &v["narrows"])?;
            if let Some(p) = named.iter().find(|p| !ports().contains(p)) {
                return Err(format!(
                    "{at}: narrows `{p}`, which is no port of the contract ({})",
                    ports().join(", ")
                ));
            }
            Narrows::Ports(named)
        }
        other => {
            return Err(format!(
                "{at}: narrows {other}, expected `all` or a list of ports"
            ));
        }
    };
    let s = Setting {
        name,
        ty: Type {
            kind,
            unit: v["unit"].as_str().map(str::to_string),
            min: bound("min")?,
            max: bound("max")?,
        },
        default: (!v["default"].is_null()).then(|| v["default"].clone()),
        narrows,
        summary: v["summary"].as_str().unwrap_or("").to_string(),
    };
    if let (Some(a), Some(b)) = (s.ty.min, s.ty.max)
        && a > b
    {
        return Err(format!(
            "{at}: min {} is above max {}: no value is in range",
            plain(a),
            plain(b)
        ));
    }
    if let Some(d) = &s.default {
        s.ty.accepts(d)
            .map_err(|e| format!("{at}: default {d}: {e}"))?;
    }
    Ok(s)
}

fn settings(at: &str, v: &Value) -> Result<Vec<Setting>, String> {
    let list = v
        .as_array()
        .ok_or_else(|| format!("{at}: expected a list of settings"))?;
    let out: Vec<Setting> = list
        .iter()
        .map(|s| setting(at, s))
        .collect::<Result<_, _>>()?;
    for (i, s) in out.iter().enumerate() {
        if out[..i].iter().any(|o| o.name == s.name) {
            return Err(format!("{at}: setting `{}` is declared twice", s.name));
        }
    }
    Ok(out)
}

/// Reads a table in the `agenticspec-settings/1` format.
pub fn parse(text: &str) -> Result<Table, String> {
    let doc: Value = serde_yaml_ng::from_str(text).map_err(|e| e.to_string())?;
    if doc["format"] != FORMAT {
        return Err(format!("format {}, expected {FORMAT}", doc["format"]));
    }
    let every_part = settings("every_part", &doc["every_part"])?;
    let mut blocks = Vec::new();
    for b in doc["blocks"].as_array().into_iter().flatten() {
        let kind = b["kind"]
            .as_str()
            .ok_or("blocks: an entry without `kind`")?
            .to_string();
        let domain = b["domain"]
            .as_str()
            .ok_or_else(|| format!("blocks.{kind}: no `domain`"))?
            .to_string();
        let own = settings(&format!("blocks.{kind}"), &b["settings"])?;
        if let Some(s) = own
            .iter()
            .find(|s| every_part.iter().any(|e| e.name == s.name))
        {
            return Err(format!(
                "blocks.{kind}: `{}` is a setting of every part already",
                s.name
            ));
        }
        if blocks.iter().any(|x: &Block| x.kind == kind) {
            return Err(format!("blocks: `{kind}` is listed twice"));
        }
        blocks.push(Block {
            kind,
            domain,
            settings: own,
        });
    }
    Ok(Table { every_part, blocks })
}

/// The committed table, read once.
pub fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| parse(SOURCE).unwrap_or_else(|e| panic!("blocks/settings.yaml: {e}")))
}

fn bounds(t: &Type, show: impl Fn(f64) -> String) -> String {
    match (t.min, t.max) {
        (Some(a), Some(b)) => format!(" from {} to {}", show(a), show(b)),
        (Some(a), None) => format!(" of at least {}", show(a)),
        (None, Some(b)) => format!(" of at most {}", show(b)),
        (None, None) => String::new(),
    }
}

fn plain(x: f64) -> String {
    crate::doc::number(x).to_string()
}

impl Type {
    /// The bounds in words, without the unit: ` from 0 to 1`, ` of at least 0`, or nothing.
    pub fn range(&self) -> String {
        match &self.kind {
            Kind::Number | Kind::Integer => bounds(self, plain),
            Kind::Duration => bounds(self, crate::doc::format_duration),
            _ => String::new(),
        }
    }

    /// The type in words: `an integer of at least 0 (slots)`.
    pub fn describe(&self) -> String {
        let mut s = match &self.kind {
            Kind::Number => format!("a number{}", self.range()),
            Kind::Integer => format!("an integer{}", self.range()),
            Kind::Boolean => "true or false".to_string(),
            Kind::Enum { values, .. } => format!("one of {}", values.join(", ")),
            Kind::Duration => format!("a duration{}", self.range()),
            Kind::String { list: false } => "a text".to_string(),
            Kind::String { list: true } => "a non-empty list of texts".to_string(),
            Kind::Part { list: false } => "the name of a part".to_string(),
            Kind::Part { list: true } => "a list of names of parts".to_string(),
            Kind::Shares => "a map from model parts to shares of at least 0".to_string(),
            Kind::Notes => "a map of free keys and values, never read".to_string(),
        };
        if let Some(u) = &self.unit {
            let _ = write!(s, " ({u})");
        }
        s
    }

    /// The type as a table cell names it: `integer`, `enum`, `part list`.
    pub fn name(&self) -> &'static str {
        match &self.kind {
            Kind::Number => "number",
            Kind::Integer => "integer",
            Kind::Boolean => "boolean",
            Kind::Enum { .. } => "enum",
            Kind::Duration => "duration",
            Kind::String { list: false } => "string",
            Kind::String { list: true } => "string list",
            Kind::Part { list: false } => "part",
            Kind::Part { list: true } => "part list",
            Kind::Shares => "shares",
            Kind::Notes => "notes",
        }
    }

    fn within(&self, x: f64) -> bool {
        self.min.is_none_or(|m| x >= m) && self.max.is_none_or(|m| x <= m)
    }

    /// Whether `v` is a value of the type; the error says what was expected.
    pub fn accepts(&self, v: &Value) -> Result<(), String> {
        let ok = match &self.kind {
            Kind::Number => v.as_f64().is_some_and(|x| self.within(x)),
            Kind::Integer => v
                .as_f64()
                .is_some_and(|x| x.fract() == 0.0 && self.within(x)),
            Kind::Boolean => v.is_boolean(),
            // A word in snake case, or the variant as the block writes it (`Execute`).
            Kind::Enum { values, from } => v.as_str().is_some_and(|w| {
                values.iter().any(|x| x == w)
                    || (from.is_some() && is_variant(w) && values.contains(&snake(w)))
            }),
            Kind::Duration => match v {
                Value::Number(n) => n.as_f64().is_some_and(|x| self.within(x)),
                Value::String(s) => parse_duration(s).is_some_and(|x| self.within(x)),
                _ => false,
            },
            Kind::String { list: false } => v.is_string(),
            Kind::String { list: true } => v.as_array().is_some_and(|xs| {
                !xs.is_empty() && xs.iter().all(|x| x.as_str().is_some_and(|s| !s.is_empty()))
            }),
            Kind::Part { list } => match v {
                Value::String(s) => !s.is_empty(),
                Value::Array(xs) => {
                    *list && xs.iter().all(|x| x.as_str().is_some_and(|s| !s.is_empty()))
                }
                _ => false,
            },
            Kind::Shares => v.as_object().is_some_and(|o| {
                o.values()
                    .all(|x| num(crate::check::value(x)).is_some_and(|x| x >= 0.0))
            }),
            Kind::Notes => v.is_object(),
        };
        if ok { Ok(()) } else { Err(self.describe()) }
    }
}

/// Whether `w` is written as an enum variant is: `Execute`, `AgentMessage`.
fn is_variant(w: &str) -> bool {
    w.starts_with(|c: char| c.is_ascii_uppercase()) && w.chars().all(|c| c.is_ascii_alphanumeric())
}

/// The word of a block's enum that `v` writes as the block writes the variant, if it does.
fn word_of(values: &[String], v: &Value) -> Option<String> {
    let w = v.as_str().filter(|w| is_variant(w))?;
    let word = snake(w);
    values.contains(&word).then_some(word)
}

/// `doc` with every setting of a block's enum in its one spelling, the word: a variant as the
/// block writes it (`SharedReadOnly`, `Execute`) becomes `shared_read_only`, `execute`, as the
/// value of a part, of a marked place and among its candidates, in a resolved nested document
/// too. The check takes both spellings; every reader after it reads this one. A word of
/// `values`, a value outside the type and a part of a block the table does not list stay as
/// written.
pub fn canonical(doc: &Value) -> Value {
    let mut out = doc.clone();
    canonicalize(&mut out);
    out
}

fn canonicalize(doc: &mut Value) {
    let table = table();
    let Some(parts) = doc.get_mut("parts").and_then(Value::as_object_mut) else {
        return;
    };
    for p in parts.values_mut() {
        let Some(part) = p.as_object_mut() else {
            continue;
        };
        if let Some(inner) = part.get_mut(crate::nest::INNER) {
            canonicalize(inner);
        }
        let Some(block) = part
            .get("block")
            .and_then(Value::as_str)
            .map(str::to_string)
        else {
            continue;
        };
        for (key, v) in part.iter_mut() {
            let Some(Kind::Enum {
                values,
                from: Some(_),
            }) = table.setting(&block, key).map(|s| &s.ty.kind)
            else {
                continue;
            };
            let fix = |x: &mut Value| {
                if let Some(w) = word_of(values, x) {
                    *x = Value::String(w);
                }
            };
            if v.get("vary").is_some() {
                if let Some(x) = v.get_mut("value") {
                    fix(x);
                }
                if let Some(Value::Array(xs)) = v.get_mut("vary") {
                    xs.iter_mut().for_each(fix);
                }
            } else {
                fix(v);
            }
        }
    }
}

impl Table {
    /// The settings of the parts of `kind`, when the table lists the block.
    pub fn block(&self, kind: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.kind == kind)
    }

    /// The blocks whose parts belong to `domain`, in the table's order (`agentic.trigger`:
    /// trigger, schedule).
    pub fn of_domain<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = &'a Block> + 'a {
        self.blocks.iter().filter(move |b| b.domain == domain)
    }

    /// Setting `name` of a part of `kind`: its block's own, else one of every part. A block the
    /// table does not list (a block of a pack) takes only the settings of every part.
    pub fn setting(&self, kind: &str, name: &str) -> Option<&Setting> {
        self.block(kind)
            .and_then(|b| b.settings.iter().find(|s| s.name == name))
            .or_else(|| self.every_part.iter().find(|s| s.name == name))
    }

    /// The names a part of `kind` may carry: its block's own, then those of every part.
    pub fn names(&self, kind: &str) -> Vec<&str> {
        self.block(kind)
            .into_iter()
            .flat_map(|b| b.settings.iter())
            .chain(self.every_part.iter())
            .map(|s| s.name.as_str())
            .collect()
    }

    /// The settings of a part of `kind` the contract copies into `port`, in the table's order.
    pub fn narrowing(&self, kind: &str, port: &str) -> Vec<&str> {
        self.block(kind)
            .into_iter()
            .flat_map(|b| b.settings.iter())
            .filter(|s| match &s.narrows {
                Narrows::None => false,
                Narrows::All => true,
                Narrows::Ports(ps) => ps.iter().any(|p| p == port),
            })
            .map(|s| s.name.as_str())
            .collect()
    }

    /// One line per setting, with its type resolved: what a result's digest reads.
    pub fn canonical(&self) -> String {
        let mut out = String::new();
        let mut line = |kind: &str, s: &Setting| {
            let _ = writeln!(out, "{kind}.{}: {}", s.name, s.ty.describe());
        };
        for s in &self.every_part {
            line("*", s);
        }
        for b in &self.blocks {
            for s in &b.settings {
                line(&b.kind, s);
            }
        }
        out
    }
}
