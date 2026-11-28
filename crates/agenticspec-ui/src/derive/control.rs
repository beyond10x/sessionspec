//! The control-plane mode of `agenticspec ui derive` (`--control-plane`): the pages across parts
//! of the default control plane, beside the overviews and part pages of the operator UI.
//!
//! Every page is selected by port class and by the split of entry commands into configuration
//! and administration, read from the contract and the port catalogue, never by a view name:
//!
//! - Inbox: the views over the entity each principal port of a principal part opens, scoped to
//!   that part, with the port's answers (`principal: all`) as row actions;
//! - Activity: the views over the entities the other ports the parts use open, across parts,
//!   newest first, one collapsed frame per domain;
//! - Operations: per part, the views over the entities a configuration command of its domains
//!   creates or an administration command moves, with the administration commands of the
//!   operator ingress as row actions on the entity each moves; then where each entity is placed;
//! - Metrics: the KPIs of the `metrics` ingress, each with the simulator's prediction of the same
//!   revision when a result is given;
//! - Architecture: the views of the architecture block, read-only;
//! - Events: the channel of the top level as a live log;
//! - tiles on the overview: the open rows of each Inbox view, the rows per state of each
//!   Operations view, and each KPI.
//!
//! Never exposed, whatever the contract's ingress lists ([`Hidden`]): a configuration command; a
//! command that only actors holding the report of a port that is no principal port may issue;
//! machine traffic, the commands of a port a link carrying a value of [`MACHINE_TRAFFIC`] stands
//! for. Like the rest of the derivation, this module names no block other than the architecture
//! block, no view, command or KPI: only that block's placement entity, the ingresses of the
//! default control plane, the enum of the principal kinds and the values of machine traffic.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::model::{Contract, Model, Panels, Port, Tree};
use super::{Frame, LevelPlan, Section, capital, section_of, snake};
use crate::Refusal;

/// The enum of the compiled blocks whose values are the principal kinds, below the system's name
/// (`<system>.shared.PrincipalKind`).
pub const PRINCIPAL_KIND: &str = "shared.PrincipalKind";

/// The principal kinds as an architecture writes them (`human`, `organization`, ...): the variants
/// of [`PRINCIPAL_KIND`] in the compiled blocks `ir` the derivation reads, a root's or the copy
/// built into the binary. A part of one of them is a principal part.
pub fn principal_kinds(ir: &Value) -> BTreeSet<String> {
    let model = Model { ir };
    model
        .variants(&format!("{}.{PRINCIPAL_KIND}", model.system()))
        .iter()
        .map(|v| snake(v))
        .collect()
}

/// The values of a link's `carries` that are machine traffic, never an operation of an interface.
pub const MACHINE_TRAFFIC: [&str; 3] = ["model_call", "tool_call", "claim"];
/// The block of the architecture, whose views the Architecture page shows.
const ARCHITECTURE: &str = "architecture";
/// The entity of the architecture block that says where each entity is placed.
const PLACEMENT: &str = "Placement";
/// The ingress of the default control plane whose operations the pages may use.
pub const OPERATOR: &str = "operator";
/// The ingress whose metrics the Metrics page shows.
pub const METRICS: &str = "metrics";
/// The format of a simulator result.
pub const RESULT_FORMAT: &str = "agenticspec-sim-result/1";

/// The role of an entry command in the port catalogue that creates an entity.
const CONFIGURATION: &str = "configuration";
/// The role of an entry command that moves an existing entity.
const ADMINISTRATION: &str = "administration";
/// The port kind of a principal port.
const PRINCIPAL_PORT: &str = "Principal";

/// The pages, in the order of the navigation: name, label, kind.
pub const PAGES: [(&str, &str, &str); 6] = [
    ("control.inbox", "Inbox", "inbox"),
    ("control.activity", "Activity", "activity"),
    ("control.operations", "Operations", "operations"),
    ("control.metrics", "Metrics", "metrics"),
    ("control.architecture", "Architecture", "architecture"),
    ("control.events", "Events", "events"),
];

/// Why a command is never exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hidden {
    /// It creates an entity: the architecture document, through a revision, is its only source.
    Configuration,
    /// Every actor that may issue it holds the report of a port that is no principal port: it is
    /// the implementation's to issue.
    ReportActor,
    /// It is a command of a port that machine traffic stands for.
    MachineTraffic,
}

/// One page across parts.
#[derive(Clone, Debug)]
pub struct ControlPage {
    pub page: String,
    pub label: String,
    pub kind: &'static str,
    pub frames: Vec<Frame>,
}

/// A KPI of the Metrics page and the simulator's prediction of it.
#[derive(Clone, Debug)]
pub struct Metric {
    pub kpi: String,
    pub prediction: Option<f64>,
}

/// What a tile counts: the rows of a view, or its rows per value of a field.
#[derive(Clone, Debug)]
pub enum Count {
    Open,
    States(Option<String>),
}

/// A tile of the overview over a section of a page.
#[derive(Clone, Debug)]
pub struct Tile {
    pub label: String,
    pub page: String,
    pub count: Count,
    pub section: Section,
}

/// The pages across parts and the overview's tiles.
#[derive(Clone, Debug)]
pub struct Control {
    pub pages: Vec<ControlPage>,
    pub metrics: Vec<Metric>,
    pub tiles: Vec<Tile>,
    /// The channel of the top level, which the Events page shows.
    pub channel: String,
    /// The events of the panels of the pages across parts.
    pub events: BTreeSet<String>,
}

/// The rules every command of a page passes: the roles of the port catalogue, the actors that
/// hold a port's report, and the commands of machine traffic.
pub struct Rules {
    roles: BTreeMap<String, String>,
    actors: Vec<(String, Vec<String>)>,
    reports: BTreeSet<String>,
    machine: BTreeSet<String>,
}

impl Rules {
    /// The rules of the port catalogue `ports` for the contract `contract` (its links name the
    /// ports machine traffic stands for).
    pub fn read(ports: &Value, contract: &Contract, model: &Model<'_>) -> Rules {
        let mut roles = BTreeMap::new();
        for (_, d) in ports["domains"].as_object().into_iter().flatten() {
            for (c, r) in d["commands"].as_object().into_iter().flatten() {
                roles.insert(c.clone(), r.as_str().unwrap_or("").to_string());
            }
        }
        let actors = ports["actors"]
            .as_object()
            .into_iter()
            .flatten()
            .map(|(a, may)| {
                let list = may
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect();
                (a.clone(), list)
            })
            .collect();
        let mut reports = BTreeSet::new();
        let mut machine = BTreeSet::new();
        let machine_ports: BTreeSet<&str> = contract
            .links
            .iter()
            .filter(|(link, _)| carries(link).is_some_and(|c| MACHINE_TRAFFIC.contains(&c)))
            .map(|(_, port)| port.as_str())
            .collect();
        for p in ports["ports"].as_array().into_iter().flatten() {
            let name = p["name"].as_str().unwrap_or("");
            let block = p["block"]
                .as_str()
                .unwrap_or_else(|| name.split('.').next().unwrap_or(""));
            let domain = model.domain_of(block).unwrap_or_default();
            let qualify = |c: &str| {
                if c.contains('.') {
                    c.to_string()
                } else {
                    format!("{domain}.{c}")
                }
            };
            let listed: Vec<String> = p["reports"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(qualify)
                .collect();
            if p["kind"].as_str() != Some(PRINCIPAL_PORT) {
                reports.extend(listed.iter().cloned());
            }
            if machine_ports.contains(name) {
                machine.extend(listed);
                let words = p["protocol"].as_str().unwrap_or("");
                machine.extend(
                    words
                        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                        .filter(|w| w.starts_with(|c: char| c.is_ascii_uppercase()))
                        .map(qualify),
                );
                for c in p["intent_command"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .chain(std::iter::once(&p["intent_command"]))
                    .filter_map(Value::as_str)
                {
                    machine.insert(qualify(c));
                }
            }
        }
        Rules {
            roles,
            actors,
            reports,
            machine,
        }
    }

    /// The role of `command` in the port catalogue.
    pub fn role(&self, command: &str) -> Option<&str> {
        self.roles.get(command).map(String::as_str)
    }

    /// Why `command` is never exposed, or `None` when a page may offer it.
    pub fn hidden(&self, command: &str) -> Option<Hidden> {
        if self.role(command) == Some(CONFIGURATION) {
            return Some(Hidden::Configuration);
        }
        if self.machine.contains(command) {
            return Some(Hidden::MachineTraffic);
        }
        let mayers: Vec<&Vec<String>> = self
            .actors
            .iter()
            .filter(|(_, may)| may.iter().any(|c| c == command))
            .map(|(_, may)| may)
            .collect();
        let implementation = |may: &Vec<String>| may.iter().any(|c| self.reports.contains(c));
        if !mayers.is_empty() && mayers.iter().all(|m| implementation(m)) {
            return Some(Hidden::ReportActor);
        }
        None
    }
}

/// The value a link carries, from the link as the contract writes it.
fn carries(link: &str) -> Option<&str> {
    let mut words = link.split_whitespace();
    words.find(|w| *w == "carries")?;
    words.next()
}

/// The operations of the contract's ingress `name` of `kind`, in the contract's order; `None`
/// when the contract declares no such ingress.
pub fn operations(contract: &Value, name: &str, kind: &str) -> Option<Vec<String>> {
    let ingress = contract["ingress"]
        .as_array()?
        .iter()
        .find(|i| i["name"].as_str() == Some(name))?;
    Some(
        ingress["operations"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|o| o["kind"].as_str() == Some(kind))
            .filter_map(|o| o["name"].as_str())
            .map(str::to_string)
            .collect(),
    )
}

/// The entity a command creates or moves: the entity of its domain whose key is its first input.
fn entity_of_command(command: &str, model: &Model<'_>, panels: &Panels) -> Option<String> {
    let domain = command.rsplit_once('.')?.0;
    let first = model.command_inputs(command).into_iter().next()?;
    panels
        .of_domain(domain)
        .find(|p| p.key == first)
        .and_then(|p| model.view_source(&p.widget))
        .map(str::to_string)
}

/// The first column of a panel drawn as a time: the order of Activity, newest first.
fn time_column(columns: &[Value]) -> Option<String> {
    columns
        .iter()
        .find(|c| c["as"].as_str() == Some("timestamp"))
        .and_then(|c| c["field"].as_str())
        .map(str::to_string)
}

/// The field a tile of an Operations view counts rows by: `state` when the view has it, else its
/// first badge column.
fn state_column(columns: &[Value]) -> Option<String> {
    let field = |c: &Value| {
        c.as_str()
            .or_else(|| c["field"].as_str())
            .map(str::to_string)
    };
    columns
        .iter()
        .filter_map(field)
        .find(|f| f == "state")
        .or_else(|| {
            columns
                .iter()
                .find(|c| c["as"].as_str() == Some("badge"))
                .and_then(field)
        })
}

/// The predictions of a simulator result for the document `doc`: the KPIs of its baseline, the
/// document as written. A result of another revision is refused.
fn predictions(result: &Value, doc: &Value) -> Result<BTreeMap<String, f64>, Refusal> {
    let refused = |m: String| Refusal {
        path: "prediction".into(),
        message: m,
    };
    if result["format"].as_str() != Some(RESULT_FORMAT) {
        return Err(refused(format!("not an {RESULT_FORMAT} document")));
    }
    if result["architecture"] != *doc {
        return Err(refused(format!(
            "the result is of another revision of {}: run the simulator on this document again",
            doc["architecture"].as_str().unwrap_or("the architecture")
        )));
    }
    Ok(result["baseline"]["kpis"]
        .as_object()
        .into_iter()
        .flatten()
        .filter_map(|(k, v)| v["value"].as_f64().map(|x| (k.clone(), x)))
        .collect())
}

/// The domains a part sets up: its own block's and that of each setting named like a block (an
/// agent's `harness`). The domains of the ports it only uses are set up by the parts that are them.
fn own_domains(part: &super::model::Part, model: &Model<'_>) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = model.domain_of(&part.block).into_iter().collect();
    for (k, v) in &part.settings {
        if let Some(d) = model.domain_of(k).filter(|_| !v.is_null()) {
            out.insert(d);
        }
    }
    out
}

/// What the pages across parts read.
pub struct Inputs<'a> {
    pub tree: &'a Tree,
    pub model: &'a Model<'a>,
    pub panels: &'a Panels,
    pub levels: &'a [LevelPlan],
    pub ports_doc: &'a Value,
    pub ports: &'a BTreeMap<String, Port>,
    pub contract_doc: &'a Value,
    pub contract: &'a Contract,
    /// A simulator result of the architecture, whose baseline predicts each KPI.
    pub result: Option<&'a Value>,
}

/// The pages across parts of the architecture, or why they cannot be derived.
///
/// # Errors
///
/// The contract declares no operator ingress, or the result is of another revision.
pub fn derive(i: &Inputs<'_>) -> Result<Control, Vec<Refusal>> {
    let Some(views) = operations(i.contract_doc, OPERATOR, "view") else {
        return Err(vec![Refusal {
            path: "contract/ingress".into(),
            message: format!(
                "the contract has no ingress {OPERATOR}: the control plane is the default operator ingress, and the architecture opts out of it"
            ),
        }]);
    };
    let views: BTreeSet<String> = views.into_iter().collect();
    let commands = operations(i.contract_doc, OPERATOR, "command").unwrap_or_default();
    let rules = Rules::read(i.ports_doc, i.contract, i.model);
    let offered: BTreeSet<&str> = commands
        .iter()
        .filter(|c| rules.hidden(c).is_none())
        .map(String::as_str)
        .collect();
    let predicted = match i.result {
        Some(r) => predictions(r, &i.tree.levels[0].doc).map_err(|e| vec![e])?,
        None => BTreeMap::new(),
    };
    let top = &i.tree.levels[0];
    let channel = super::channel_name(top);
    let in_views = |w: &str| views.contains(w);
    let mut events = BTreeSet::new();
    let mut section = |p: &super::model::Panel, part: Option<&str>, prefix: &str| -> Section {
        events.extend(p.events.iter().cloned());
        let mut s = section_of(p, top, part.unwrap_or(""), &channel, prefix);
        if part.is_none() {
            s.args.retain(|(k, _)| k != "part");
        }
        s
    };

    // Parts in the order of their levels and parts tables, nested levels after their parent.
    let mut parts: Vec<&super::model::Part> = Vec::new();
    for lp in i.levels {
        let level = &i.tree.levels[lp.level];
        for name in &lp.order {
            if let Some(p) = level.part(name).filter(|p| p.nested.is_none()) {
                parts.push(p);
            }
        }
    }

    // Inbox: per principal part, the entities its principal ports open.
    let mut inbox = Vec::new();
    let mut inbox_entities = BTreeSet::new();
    let principal_kinds = principal_kinds(i.model.ir);
    for part in parts
        .iter()
        .filter(|p| p.kind.as_ref().is_some_and(|k| principal_kinds.contains(k)))
    {
        let mut sections = Vec::new();
        for port in i
            .contract
            .ports_of(&part.path)
            .filter_map(|n| i.ports.get(n))
            .filter(|p| p.kind == PRINCIPAL_PORT)
        {
            let Some(entity) = port.opens.as_deref() else {
                continue;
            };
            inbox_entities.insert(entity.to_string());
            let widgets: Vec<String> = i
                .panels
                .list
                .iter()
                .filter(|p| i.model.view_source(&p.widget) == Some(entity) && in_views(&p.widget))
                .map(|p| p.widget.clone())
                .collect();
            let answers: Vec<String> = port
                .intent
                .iter()
                .filter(|c| port.explicit && offered.contains(c.as_str()))
                .cloned()
                .collect();
            let actions = super::row_actions(i.model, i.panels, &answers, &widgets);
            for w in &widgets {
                let Some(p) = i.panels.get(w) else { continue };
                let mut s = section(
                    p,
                    Some(&part.path),
                    &format!("inbox_{}_", snake(&part.path)),
                );
                s.title = format!("{} for {}", p.title, part.path);
                s.row_actions = actions.get(w).cloned().unwrap_or_default();
                sections.push(s);
            }
        }
        if !sections.is_empty() {
            inbox.push(Frame {
                area: format!("area_{}", snake(&part.path)),
                title: Some(part.path.clone()),
                collapsed: false,
                sections,
            });
        }
    }

    // Activity: the entities the other ports the parts use open, across parts.
    let mut opened: BTreeSet<String> = BTreeSet::new();
    for part in &parts {
        for port in i
            .contract
            .ports_of(&part.path)
            .filter_map(|n| i.ports.get(n))
            .filter(|p| p.kind != PRINCIPAL_PORT)
        {
            if let Some(e) = &port.opens
                && !inbox_entities.contains(e)
            {
                opened.insert(e.clone());
            }
        }
    }
    let mut activity = Vec::new();
    for d in i.panels.domains() {
        let sections: Vec<Section> = i
            .panels
            .of_domain(&d)
            .filter(|p| {
                i.model
                    .view_source(&p.widget)
                    .is_some_and(|e| opened.contains(e))
                    && in_views(&p.widget)
            })
            .map(|p| {
                let mut s = section(p, None, "activity_");
                s.order = time_column(&p.columns);
                s
            })
            .collect();
        if !sections.is_empty() {
            activity.push(Frame {
                area: format!("area_{}", snake(d.rsplit('.').next().unwrap_or(&d))),
                title: Some(i.model.domain_label(&d)),
                collapsed: true,
                sections,
            });
        }
    }

    // Operations: per part, what configuration creates and administration moves.
    let mut moves: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in &commands {
        if rules.role(c) == Some(ADMINISTRATION)
            && offered.contains(c.as_str())
            && let Some(e) = entity_of_command(c, i.model, i.panels)
        {
            moves.entry(e).or_default().push(c.clone());
        }
    }
    // An entity a port opens is activity, even when a configuration command creates it.
    let port_opened: BTreeSet<&str> = i
        .ports
        .values()
        .filter_map(|p| p.opens.as_deref())
        .collect();
    let mut operations_frames = Vec::new();
    let mut tiles = Vec::new();
    for part in &parts {
        let domains = own_domains(part, i.model);
        let mut entities: BTreeSet<String> = BTreeSet::new();
        for (c, role) in &rules.roles {
            let domain = c.rsplit_once('.').map_or("", |x| x.0);
            if role == CONFIGURATION
                && domains.contains(domain)
                && let Some(e) = entity_of_command(c, i.model, i.panels)
                && !port_opened.contains(e.as_str())
            {
                entities.insert(e);
            }
        }
        entities.extend(
            moves
                .keys()
                .filter(|e| domains.contains(e.rsplit_once('.').map_or("", |x| x.0)))
                .cloned(),
        );
        let mut sections = Vec::new();
        for d in i.panels.domains().iter().filter(|d| domains.contains(*d)) {
            for p in i.panels.of_domain(d) {
                let Some(e) = i.model.view_source(&p.widget) else {
                    continue;
                };
                if !entities.contains(e) || !in_views(&p.widget) {
                    continue;
                }
                let mut s = section(p, Some(&part.path), &format!("ops_{}_", snake(&part.path)));
                s.title = format!("{} of {}", p.title, part.path);
                let acts = moves.get(e).cloned().unwrap_or_default();
                s.row_actions =
                    super::row_actions(i.model, i.panels, &acts, std::slice::from_ref(&p.widget))
                        .remove(&p.widget)
                        .unwrap_or_default();
                sections.push(s);
            }
        }
        if !sections.is_empty() {
            let acts = sections.iter().any(|s| !s.row_actions.is_empty());
            operations_frames.push(Frame {
                area: format!("area_{}", snake(&part.path)),
                title: Some(part.path.clone()),
                collapsed: !acts,
                sections,
            });
        }
    }
    let architecture = i.model.domain_of(ARCHITECTURE).unwrap_or_default();
    let placement = format!("{architecture}.{PLACEMENT}");
    let placed: Vec<Section> = i
        .panels
        .of_domain(&architecture)
        .filter(|p| i.model.view_source(&p.widget) == Some(placement.as_str()))
        .map(|p| section(p, None, "ops_"))
        .collect();
    let placed_widgets: BTreeSet<String> = placed.iter().map(|s| s.widget.clone()).collect();
    if !placed.is_empty() {
        operations_frames.push(Frame {
            area: "area_placements".into(),
            title: Some("Placements".into()),
            collapsed: true,
            sections: placed,
        });
    }

    // Architecture: the views of the architecture block, read-only.
    let arch_sections: Vec<Section> = i
        .panels
        .of_domain(&architecture)
        .filter(|p| !placed_widgets.contains(&p.widget))
        .map(|p| section(p, None, "architecture_"))
        .collect();
    let architecture_frames = if arch_sections.is_empty() {
        Vec::new()
    } else {
        vec![Frame {
            area: "area_architecture".into(),
            title: None,
            collapsed: false,
            sections: arch_sections,
        }]
    };

    let metrics: Vec<Metric> = operations(i.contract_doc, METRICS, "metric")
        .unwrap_or_default()
        .into_iter()
        .map(|kpi| Metric {
            prediction: predicted.get(&kpi).copied(),
            kpi,
        })
        .collect();

    // Tiles: each Inbox view, each Operations view of a part.
    for f in &inbox {
        for s in &f.sections {
            tiles.push(Tile {
                label: format!("Open: {}", s.title),
                page: PAGES[0].0.to_string(),
                count: Count::Open,
                section: s.clone(),
            });
        }
    }
    for f in operations_frames
        .iter()
        .filter(|f| f.area != "area_placements")
    {
        for s in &f.sections {
            let columns = i.panels.get(&s.widget).map(|p| p.columns.clone());
            tiles.push(Tile {
                label: s.title.clone(),
                page: PAGES[2].0.to_string(),
                count: Count::States(columns.as_deref().and_then(state_column)),
                section: s.clone(),
            });
        }
    }

    let frames_of = |kind: &str| -> Vec<Frame> {
        match kind {
            "inbox" => inbox.clone(),
            "activity" => activity.clone(),
            "operations" => operations_frames.clone(),
            "architecture" => architecture_frames.clone(),
            _ => Vec::new(),
        }
    };
    let pages = PAGES
        .iter()
        .map(|(page, label, kind)| ControlPage {
            page: page.to_string(),
            label: label.to_string(),
            kind,
            frames: frames_of(kind),
        })
        .collect();
    Ok(Control {
        pages,
        metrics,
        tiles,
        channel,
        events,
    })
}

/// A KPI's label: `cost_usd` is `Cost usd`.
pub fn kpi_label(kpi: &str) -> String {
    capital(kpi)
}
