//! The render model `agenticspec-render-model/1` and its one entry point, [`render`].
//!
//! A [`Page`] is a graph in neutral terms: nodes in groups, ports on nodes, edges between them,
//! regions that hold nodes, marks (a refusal or a piece of information) on a node or a port, and
//! panels of plain tables. [`render`] draws it as one complete HTML page in this crate's visual
//! language: the graph whole (fitted to its view when larger, else at its natural size) with pan,
//! zoom and fit, its legend, and the panels beside it. The page holds its CSS, its script and its
//! SVG inline, embeds the page document as JSON, and loads nothing from the network. The output is a function of the page only.
//!
//! The JSON Schema of the format is `schema/render-model.schema.json` in this crate. Unknown
//! fields are refused, by the schema and by the types.
//!
//! How it draws:
//!
//! - A node is a card coloured by its `group`. Rows run top to bottom by the longest chain of
//!   edges that leads to a node; edges between two rows carry their label in the row between.
//! - A group listed in `groups` takes its `colour` (a [`Colour`]: a hex colour, `rgb()`, `hsl()`
//!   or one of [`NAMED_COLOURS`]) and its `icon` (one of [`ICONS`], or a text of one or two
//!   characters); a group not listed keeps the palette and the plain icon.
//! - A port is one row of its node's card; an edge end `node.port` lands on that card and its
//!   label names the port.
//! - A node with a `region` is drawn as a small card inside the card of the outermost region it
//!   lies in: the node whose id is the region's id, or a card of the region itself. Its edges land
//!   on that card. An edge between two cards inside one region is drawn too: straight across
//!   when they stand side by side, else on a lane of its own below what the region holds.
//! - A row is at most [`WRAP_WIDTH`] wide: the cards of a row, and the cards a region's card
//!   holds, that side by side would be wider go into as many rows as keep each within it, shared
//!   out evenly, in the order the one row had them, read row by row. Edges to and between those
//!   rows run like edges between any two rows; inside a region's card, an edge from a card of a
//!   row runs into the gap below that row and leaves along the side of the card. A row that fits
//!   is drawn in one.
//! - An edge with a `count` is drawn heavier: 2 or 3 wider than 1, 4 or more wider again.
//! - A badge is a pill on its card, or with `on: panel` a row of the table "Details" under the
//!   graph.
//! - A mark is a chip on its card. A refusal also rings the card, or marks its port.
//! - A panel is a table in a card beside the graph, or with `place: below` across the page under
//!   it. A cell value longer than 32 characters wraps its long words instead of being cut, and
//!   has its full value as its title.
//!
//! A region with `"collapsed": true` is drawn as one card until it is opened:
//!
//! - The card shows the region's label and the count of the nodes inside it, nested regions
//!   included; it is a button named `Open region <label>, <n> nodes`. A refusal inside rings it.
//! - An edge between two collapsed regions, or between one and a node outside it, is drawn once
//!   per pair of visible ends. It reads `×<n>` when it stands for more than one edge; one edge
//!   keeps its own label. An edge inside a collapsed region is not drawn.
//! - Selecting the card (click, Enter or Space) opens the region in place: its nodes and inner
//!   regions, each edge on its own. A `close` control on its frame (or band) closes it again.
//! - A region inside an open one keeps its own `collapsed`. One collapsible region is open at a
//!   time, with the collapsible regions around it; opening another closes it.
//! - The state lives in the page only. The page as served shows every collapsible region
//!   collapsed, and holds each opened state laid out in full in a `<template>` beside the graph,
//!   which the script swaps in.
//!
//! An edge end names a node, or a port the page declares as `node.port`. A page read from JSON
//! with an end that names neither is refused as `edge_end_unknown` ([`Page::check`]). So is every
//! other reference that names nothing: a port's `node` (`port_node_unknown`), a mark's target
//! (`mark_target_unknown`) and a node's `region` (`node_region_unknown`). A
//! `collapsed` that is no boolean is refused as `region_collapsed_invalid`, and a collapsed region
//! that holds no node besides its own as `region_collapsed_empty`.
//!
//! With `"layout": "bands"` the graph is drawn in bands instead (`"graph"`, the default, draws it
//! as above; any other value is refused):
//!
//! - Each top-level region is one horizontal band, labelled like a region's frame, the bands top
//!   to bottom in the order `regions` lists them and each as wide as the page.
//! - Inside a band, its nodes and nested regions are laid out as above, as if they were a page of
//!   their own; the region's own node, if any, is a card of the band.
//! - An edge between two bands leaves its card toward the other band, runs along the gap next to
//!   its band, where its label sits, and enters the other band from the gap next to that one. Past
//!   a band between, it runs down (or up) beside the bands, never through one.
//! - Nodes in no top-level region fill a last band without a label.
//!
//! One page:
//!
//! ```json
//! {
//!   "format": "agenticspec-render-model/1",
//!   "title": "assistant",
//!   "summary": "One session per user; the session calls a model.",
//!   "regions": [{ "id": "session", "label": "session", "per": "session" }],
//!   "nodes": [
//!     { "id": "user", "label": "user", "group": "principal" },
//!     { "id": "session", "label": "session", "group": "agent", "badges": [{ "name": "count", "value": "8" }] },
//!     { "id": "scratch", "label": "scratch", "group": "place", "region": "session" },
//!     { "id": "model", "label": "model", "group": "intel", "badges": [{ "name": "region", "value": "eu", "on": "panel" }] }
//!   ],
//!   "ports": [{ "node": "model", "name": "complete", "kind": "http", "direction": "in", "contract": "complete/1" }],
//!   "edges": [
//!     { "from": "user", "to": "session", "label": "prompt" },
//!     { "from": "session", "to": "model.complete", "label": "call", "count": 2 }
//!   ],
//!   "marks": [
//!     { "target": { "node": "model", "port": "complete" }, "name": "rate_limit", "detail": "over 60 a minute" },
//!     { "target": { "node": "session" }, "name": "budget", "detail": "40 % used", "level": "info" }
//!   ],
//!   "panels": [{ "title": "Owners", "columns": ["node", "owner"], "rows": [["session", "team a"], ["model", "team b"]] }],
//!   "groups": [{ "name": "agent", "colour": "#2f7d5b", "icon": "bot" }, { "name": "intel", "icon": "AI" }]
//! }
//! ```

use crate::fmt::{self, esc};
use crate::graph;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

const CSS: &str = include_str!("../assets/page.css");
const JS: &str = include_str!("../assets/page.js");

/// The format name, the value of `format` of every [`Page`].
pub const FORMAT: &str = "agenticspec-render-model/1";

/// The widest a row of cards is drawn, in px: twice a reference width of the graph's view
/// (1200 px). The cards of one row of a page or a band, and the cards a region's frame holds,
/// that with their gaps would be wider go into as many rows as keep each within it.
pub const WRAP_WIDTH: f64 = 2400.0;

/// One page: a titled graph and the panels beside it.
///
/// Read from JSON, a page is checked as [`Page::check`] says, and refused when it fails.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, try_from = "Unchecked")]
pub struct Page {
    /// Always [`FORMAT`].
    pub format: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Containment, as a tree. A node is drawn inside the outermost region its `region` lies in.
    #[serde(default)]
    pub regions: Vec<Region>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub ports: Vec<Port>,
    #[serde(default)]
    pub edges: Vec<Edge>,
    #[serde(default)]
    pub marks: Vec<Mark>,
    #[serde(default)]
    pub panels: Vec<Panel>,
    /// The colour and icon of a group, by its name. A group not listed keeps the palette.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
    /// How the graph is laid out: in rows (the default), or in one band per top-level region.
    #[serde(default, skip_serializing_if = "Layout::is_graph")]
    pub layout: Layout,
}

/// How the graph of a [`Page`] is laid out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    /// Rows by the longest chain of edges that leads to a node; a top-level region is a card of
    /// its row.
    #[default]
    Graph,
    /// One horizontal band per top-level region, top to bottom in the order of `regions`, each
    /// as wide as the page. What a band holds is laid out inside it as [`Layout::Graph`] lays out
    /// a page; an edge between two bands runs through the gaps between the bands. Nodes in no
    /// top-level region fill a last band without a label.
    Bands,
}

impl Layout {
    fn is_graph(&self) -> bool {
        *self == Layout::Graph
    }
}

/// A [`Page`] as read, before [`Page::check`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Unchecked {
    format: String,
    title: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    regions: Vec<Region>,
    #[serde(default)]
    nodes: Vec<Node>,
    #[serde(default)]
    ports: Vec<Port>,
    #[serde(default)]
    edges: Vec<Edge>,
    #[serde(default)]
    marks: Vec<Mark>,
    #[serde(default)]
    panels: Vec<Panel>,
    #[serde(default)]
    groups: Vec<Group>,
    #[serde(default)]
    layout: Layout,
}

impl TryFrom<Unchecked> for Page {
    type Error = String;
    fn try_from(u: Unchecked) -> Result<Self, String> {
        let page = Page {
            format: u.format,
            title: u.title,
            summary: u.summary,
            regions: u.regions,
            nodes: u.nodes,
            ports: u.ports,
            edges: u.edges,
            marks: u.marks,
            panels: u.panels,
            groups: u.groups,
            layout: u.layout,
        };
        page.check()?;
        Ok(page)
    }
}

/// The look of the nodes of one group: a colour for card, frame and legend swatch, an icon for
/// the card. Either may be left out; the palette colour and the plain icon stand in.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    /// The `group` of the nodes it styles.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<Colour>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

/// The named colours a [`Colour`] may be, besides a hex colour, `rgb()` and `hsl()`.
pub const NAMED_COLOURS: [&str; 40] = [
    "black",
    "white",
    "gray",
    "grey",
    "silver",
    "red",
    "maroon",
    "crimson",
    "tomato",
    "coral",
    "salmon",
    "orange",
    "darkorange",
    "gold",
    "yellow",
    "khaki",
    "olive",
    "lime",
    "green",
    "darkgreen",
    "forestgreen",
    "seagreen",
    "teal",
    "turquoise",
    "cyan",
    "aqua",
    "skyblue",
    "steelblue",
    "blue",
    "darkblue",
    "navy",
    "slateblue",
    "indigo",
    "purple",
    "violet",
    "orchid",
    "plum",
    "magenta",
    "fuchsia",
    "pink",
];

/// A CSS colour that can be written into a style attribute and nothing else:
///
/// - a hex colour, `#` and 3, 4, 6 or 8 hex digits: `#3a7`, `#33aa77cc`;
/// - `rgb(…)`, `rgba(…)`, `hsl(…)` or `hsla(…)` holding 3 or 4 numbers, each optionally followed
///   by `%` or `deg`, apart by spaces, commas or one `/`: `rgb(51 170 119)`,
///   `hsl(150deg, 50%, 43%)`, `rgb(51 170 119 / 50%)`;
/// - one of [`NAMED_COLOURS`], in lower case.
///
/// Anything else is refused, by [`Colour::new`], by deserialization and by the schema.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Colour(String);

impl Colour {
    pub fn new(text: impl Into<String>) -> Result<Self, String> {
        let text = text.into();
        if colour_ok(&text) {
            Ok(Colour(text))
        } else {
            Err(format!(
                "colour {text:?} is not a hex colour, rgb(), hsl() or a named colour of the list"
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Colour {
    type Error = String;
    fn try_from(s: String) -> Result<Self, String> {
        Colour::new(s)
    }
}

impl From<Colour> for String {
    fn from(c: Colour) -> String {
        c.0
    }
}

fn colour_ok(s: &str) -> bool {
    if let Some(hex) = s.strip_prefix('#') {
        return matches!(hex.len(), 3 | 4 | 6 | 8) && hex.bytes().all(|b| b.is_ascii_hexdigit());
    }
    if NAMED_COLOURS.contains(&s) {
        return true;
    }
    let Some(args) = ["rgba(", "rgb(", "hsla(", "hsl("]
        .iter()
        .find_map(|f| s.strip_prefix(f))
        .and_then(|r| r.strip_suffix(')'))
    else {
        return false;
    };
    // Numbers apart by at least one space, comma or slash: `-?([0-9]+|[0-9]*\.[0-9]+)(%|deg)?`.
    let b = args.as_bytes();
    let mut i = 0;
    let mut n = 0;
    let skip_ws = |i: &mut usize| {
        while *i < b.len() && b[*i] == b' ' {
            *i += 1;
        }
    };
    skip_ws(&mut i);
    loop {
        if i < b.len() && b[i] == b'-' {
            i += 1;
        }
        let int = b[i..].iter().take_while(|c| c.is_ascii_digit()).count();
        i += int;
        if i < b.len() && b[i] == b'.' {
            i += 1;
            let frac = b[i..].iter().take_while(|c| c.is_ascii_digit()).count();
            if frac == 0 {
                return false;
            }
            i += frac;
        } else if int == 0 {
            return false;
        }
        if b[i..].starts_with(b"%") {
            i += 1;
        } else if b[i..].starts_with(b"deg") {
            i += 3;
        }
        n += 1;
        let start = i;
        skip_ws(&mut i);
        let sep = i < b.len() && matches!(b[i], b',' | b'/');
        if sep {
            i += 1;
            skip_ws(&mut i);
        }
        if i == b.len() && !sep {
            break;
        }
        if i == start || i == b.len() {
            return false;
        }
    }
    matches!(n, 3 | 4)
}

/// The icon names an [`Icon`] may be, each a stroke icon this crate draws.
pub const ICONS: [&str; 14] = [
    "person",
    "bot",
    "bubble",
    "gateway",
    "chip",
    "wrench",
    "folder",
    "shield",
    "play",
    "stack",
    "checklist",
    "lock",
    "frame",
    "circle",
];

/// The icon of a card: one of [`ICONS`], drawn as that stroke icon, or a text of one or two
/// characters (no control character, not only spaces), drawn in the icon's tile. Anything else is
/// refused, by [`Icon::new`], by deserialization and by the schema.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Icon(String);

impl Icon {
    pub fn new(text: impl Into<String>) -> Result<Self, String> {
        let text = text.into();
        let chars = text.chars().count();
        let short = (1..=2).contains(&chars)
            && !text.chars().any(char::is_control)
            && !text.trim().is_empty();
        if ICONS.contains(&text.as_str()) || short {
            Ok(Icon(text))
        } else {
            Err(format!(
                "icon {text:?} is not an icon name of the list or a text of 1 or 2 characters"
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The name of the stroke icon, or `None` for a text.
    pub fn name(&self) -> Option<&'static str> {
        ICONS.iter().copied().find(|n| *n == self.0)
    }
}

impl TryFrom<String> for Icon {
    type Error = String;
    fn try_from(s: String) -> Result<Self, String> {
        Icon::new(s)
    }
}

impl From<Icon> for String {
    fn from(i: Icon) -> String {
        i.0
    }
}

impl Page {
    /// An empty page of format [`FORMAT`] with this title.
    pub fn new(title: impl Into<String>) -> Self {
        Page {
            format: FORMAT.to_string(),
            title: title.into(),
            ..Page::default()
        }
    }

    /// Whether every edge end resolves: to a node's id, or as `node.port` to a port the page
    /// declares on that node. The first end that does not is refused as `edge_end_unknown`,
    /// naming the edge and the end. Every other reference resolves too, or the first that does
    /// not is refused: a port whose `node` is no node as `port_node_unknown`, naming the port; a
    /// mark whose target is no node, or names a port the page does not declare on that node, as
    /// `mark_target_unknown`, naming the mark and its target; a node whose `region` is no region
    /// and no node as `node_region_unknown`, naming the node and the region. A region with
    /// `collapsed: true` that holds no node besides
    /// its own (in it or in a region inside it) would be a card that opens onto nothing; it is
    /// refused as `region_collapsed_empty`, naming the region. Reading a page from JSON runs this
    /// check.
    pub fn check(&self) -> Result<(), String> {
        for e in &self.edges {
            for end in [&e.from, &e.to] {
                if !self.resolves(end) {
                    return Err(format!(
                        "edge_end_unknown: the edge from {:?} to {:?}: {end:?} is no node and no port of a node",
                        e.from, e.to
                    ));
                }
            }
        }
        for p in &self.ports {
            if !self.has_node(&p.node) {
                return Err(format!(
                    "port_node_unknown: the port {:?}: {:?} is no node",
                    format!("{}.{}", p.node, p.name),
                    p.node
                ));
            }
        }
        for m in &self.marks {
            let t = &m.target;
            let on = match &t.port {
                None => t.node.clone(),
                Some(port) => format!("{}.{port}", t.node),
            };
            let resolves = self.has_node(&t.node)
                && t.port.as_ref().is_none_or(|port| {
                    self.ports
                        .iter()
                        .any(|p| p.node == t.node && p.name == *port)
                });
            if !resolves {
                return Err(format!(
                    "mark_target_unknown: the mark {:?} on {on:?}: {on:?} is no node and no port of a node",
                    m.name
                ));
            }
        }
        let tree = crate::collapse::Tree::of(self);
        for n in &self.nodes {
            if let Some(region) = &n.region
                && !tree.regions.iter().any(|r| r.id == *region)
                && !self.has_node(region)
            {
                return Err(format!(
                    "node_region_unknown: the node {:?}: its region {region:?} is no region and no node",
                    n.id
                ));
            }
        }
        for r in tree.regions.iter().filter(|r| r.collapsed) {
            if tree.inside(self, &r.id) == 0 {
                return Err(format!(
                    "region_collapsed_empty: the region {:?} is collapsed but holds no node",
                    r.id
                ));
            }
        }
        Ok(())
    }

    fn has_node(&self, id: &str) -> bool {
        self.nodes.iter().any(|n| n.id == id)
    }

    fn resolves(&self, end: &str) -> bool {
        self.has_node(end)
            || self.ports.iter().any(|p| {
                end.strip_prefix(p.node.as_str()) == Some(&format!(".{}", p.name))
                    && self.has_node(&p.node)
            })
    }
}

/// A region: a box that holds nodes. A region whose `id` is a node's id is that node's card; any
/// other outermost region is drawn as a card of its own. The nodes inside are headed `per <per>`
/// or `in <in>` (`in <label>` when neither is given).
///
/// Read from JSON, a `collapsed` that is not `true` or `false` is refused as
/// `region_collapsed_invalid`, naming the region.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, try_from = "RawRegion")]
pub struct Region {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
    /// Drawn as one card (its label and the count of the nodes inside it, nested regions
    /// included) until it is opened on the page; edges into it are merged per pair of visible
    /// ends. A collapsed region holds at least one node besides its own.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub collapsed: bool,
    /// The regions inside this one.
    #[serde(default)]
    pub regions: Vec<Region>,
}

/// A [`Region`] as read: `collapsed` as any value, so that one that is no boolean is refused
/// naming its region.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegion {
    id: String,
    label: String,
    #[serde(default)]
    r#in: Option<String>,
    #[serde(default)]
    per: Option<String>,
    #[serde(default, deserialize_with = "present")]
    collapsed: Option<serde_json::Value>,
    #[serde(default)]
    regions: Vec<Region>,
}

/// A field that is present, `null` included.
fn present<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<serde_json::Value>, D::Error> {
    serde_json::Value::deserialize(d).map(Some)
}

impl TryFrom<RawRegion> for Region {
    type Error = String;
    fn try_from(r: RawRegion) -> Result<Self, String> {
        let collapsed = match r.collapsed {
            None => false,
            Some(serde_json::Value::Bool(b)) => b,
            Some(other) => {
                return Err(format!(
                    "region_collapsed_invalid: the region {:?}: collapsed is {other}, not true or false",
                    r.id
                ));
            }
        };
        Ok(Region {
            id: r.id,
            label: r.label,
            r#in: r.r#in,
            per: r.per,
            collapsed,
            regions: r.regions,
        })
    }
}

/// A node: one card of the graph.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    pub label: String,
    /// Colour and legend entry. One of the palette names of the stylesheet (`principal`,
    /// `agent`, `collab`, `work`, `intel`, `capability`, `place`, `resource`, `nested`, `other`)
    /// takes that colour; any other name takes the next one no group of the page names. An entry
    /// of the page's `groups` with this name sets its colour and icon.
    pub group: String,
    /// Pills at the card's top right: `count` reads `×<value>`, any other `<name> <value>`. A
    /// badge `on: panel` is a row of the table "Details" under the graph instead.
    #[serde(default)]
    pub badges: Vec<Badge>,
    /// The id of the region the node is drawn in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Badge {
    pub name: String,
    pub value: String,
    /// Where the badge is shown: a pill on the card (the default), or a row of the table
    /// "Details" under the graph.
    #[serde(default, skip_serializing_if = "On::is_card")]
    pub on: On,
}

/// Where a [`Badge`] is shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum On {
    #[default]
    Card,
    Panel,
}

impl On {
    fn is_card(&self) -> bool {
        *self == On::Card
    }
}

/// A named port on a node, addressed by an edge end `node.port`. A port is one row of its
/// node's card.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Port {
    pub node: String,
    pub name: String,
    pub kind: String,
    pub direction: Direction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Text on the port row after its name, such as the protocols it runs over.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
    /// The port names a target that does not resolve: a dashed stub leaves its row and reaches
    /// nothing; a refusal mark on the port names why.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unresolved: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    Out,
    #[default]
    Both,
}

/// An edge from one node (or `node.port`) to another, drawn as an arrow with its label.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: String,
    /// How many edges this one stands for, shown as `×n` after the label. The stroke has three
    /// widths: up to 1, 2 or 3, and 4 or more.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

/// What a mark is shown on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub node: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

/// A mark on a node or a port, drawn as a chip on the card. A refusal also rings its card (or
/// its port); a piece of information does not.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mark {
    pub target: Target,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub level: Level,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Info,
    #[default]
    Refusal,
}

/// A plain table beside the graph.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Panel {
    pub title: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    /// Where the panel is drawn: in the column beside the graph (the default), or across the
    /// page under it.
    #[serde(default, skip_serializing_if = "Place::is_side")]
    pub place: Place,
}

/// Where a [`Panel`] is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Place {
    #[default]
    Side,
    Below,
}

impl Place {
    fn is_side(&self) -> bool {
        *self == Place::Side
    }
}

/// The page of `page`: one complete HTML document.
pub fn render(page: &Page) -> String {
    render_with(page, &Shell::of(page))
}

/// The header of a page: an eyebrow line, the heading, a summary and a list of facts.
pub(crate) struct Header {
    pub eyebrow: Option<String>,
    pub heading: String,
    pub summary: Option<String>,
    pub facts: Vec<String>,
}

/// Everything around the graph of a page. [`render`] builds it from the page alone; a host page
/// of this crate writes its own.
pub(crate) struct Shell {
    /// The document title.
    pub title: String,
    pub header: Header,
    /// The accessible name of the section that holds the graph.
    pub section: String,
    /// The heading of the graph's card and its hint.
    pub graph_heading: String,
    pub graph_hint: String,
    /// The legend, HTML.
    pub legend: String,
    /// The cards beside the graph, HTML.
    pub side: String,
    /// The sections after the graph's, HTML.
    pub after: String,
    /// The footer, HTML.
    pub footer: String,
    /// The JSON embedded in the page.
    pub data: serde_json::Value,
    /// What the graph draws beyond the page.
    pub decor: graph::Decor,
}

impl Shell {
    fn of(page: &Page) -> Self {
        let refusals = page
            .marks
            .iter()
            .filter(|m| m.level == Level::Refusal)
            .count();
        let mut facts = vec![
            plural(page.nodes.len(), "node"),
            plural(page.edges.len(), "edge"),
        ];
        if refusals > 0 {
            facts.push(plural(refusals, "refusal"));
        }
        let mut side = String::new();
        let mut below = String::new();
        for p in &page.panels {
            match p.place {
                Place::Side => panel(&mut side, p),
                Place::Below => {
                    let _ = write!(
                        below,
                        "<section class=\"section-gap\" aria-label=\"{}\">",
                        esc(&p.title)
                    );
                    panel(&mut below, p);
                    below.push_str("</section>\n");
                }
            }
        }
        let format = if page.format.is_empty() {
            FORMAT
        } else {
            page.format.as_str()
        };
        Shell {
            title: page.title.clone(),
            header: Header {
                eyebrow: None,
                heading: page.title.clone(),
                summary: page.summary.clone().filter(|s| !s.is_empty()),
                facts,
            },
            section: "Graph".to_string(),
            graph_heading: "Graph".to_string(),
            graph_hint: if crate::collapse::Tree::of(page)
                .regions
                .iter()
                .any(|r| r.collapsed)
            {
                "Select a node to follow its edges, or the card of a region to open it".to_string()
            } else {
                "Select a node to follow its edges".to_string()
            },
            legend: legend(page),
            side,
            after: below + &details(page),
            footer: format!(
                "{} · rendered by agenticspec, self-contained, no network",
                esc(format)
            ),
            data: serde_json::to_value(page).unwrap_or_default(),
            decor: graph::Decor {
                words: graph::Words {
                    title: format!("Graph {}", page.title),
                    desc: format!(
                        "{} {}, {} as arrows with their labels.",
                        plural(page.nodes.len(), "node"),
                        match page.layout {
                            Layout::Graph => "in rows",
                            Layout::Bands => "in bands, one per region",
                        },
                        plural(page.edges.len(), "edge")
                    ),
                },
                ..graph::Decor::default()
            },
        }
    }
}

pub(crate) fn plural(n: usize, word: &str) -> String {
    if n == 1 {
        format!("1 {word}")
    } else {
        format!("{n} {word}s")
    }
}

/// The page around its graph.
pub(crate) fn render_with(page: &Page, shell: &Shell) -> String {
    let (g, states) = crate::collapse::render(page, &shell.decor);
    let mut h = String::with_capacity(64 * 1024);
    let _ = write!(
        h,
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"color-scheme\" content=\"light dark\">\n<title>{}</title>\n<style>\n{CSS}</style>\n</head>\n<body>\n",
        esc(&shell.title)
    );
    header(&mut h, &shell.header);
    h.push_str("<main>\n");
    let _ = write!(
        h,
        "<section class=\"top-grid\" aria-label=\"{}\">\n<div class=\"card graph-card\"><div class=\"card-head\"><h2>{}</h2><span class=\"hint\">{}</span></div><div class=\"card-body\">",
        esc(&shell.section),
        esc(&shell.graph_heading),
        esc(&shell.graph_hint)
    );
    h.push_str(&shell.legend);
    // The graph is drawn at its natural size. `data-initial="fit"` tells the script to open it at
    // Fit (whole in its view) when it is wider than its card or taller than the view's highest;
    // the script then shows the zoom tools, and the card pans by drag, wheel and arrow keys
    // whenever the graph overflows it.
    h.push_str(
        "<div class=\"graph-tools\" hidden role=\"toolbar\" aria-label=\"Zoom\">\
<button type=\"button\" data-zoom=\"out\" aria-label=\"Zoom out\">−</button>\
<output class=\"zoom-level\" aria-live=\"polite\">100 %</output>\
<button type=\"button\" data-zoom=\"in\" aria-label=\"Zoom in\">+</button>\
<button type=\"button\" data-zoom=\"fit\" aria-label=\"Fit the whole graph\">Fit</button>\
<button type=\"button\" data-zoom=\"actual\" aria-label=\"Actual size\">Actual size</button>\
<span class=\"hint\">drag, scroll or use the arrow keys to move around</span></div>",
    );
    let _ = write!(
        h,
        "<div class=\"graph-view\" data-initial=\"fit\" style=\"--graph-w:{}px\">{}",
        fmt::px(g.width),
        g.svg
    );
    // Each state of the graph with one collapsed region open, laid out in full: the script swaps
    // it in when the card of that region is selected, and back when its control closes it.
    for s in &states {
        let _ = write!(
            h,
            "<template data-state=\"{}\" data-up=\"{}\">{}</template>",
            esc(&s.id),
            esc(&s.up),
            s.graph.svg
        );
    }
    h.push_str("</div>");
    h.push_str("</div></div>\n");
    if !shell.side.is_empty() {
        let _ = writeln!(h, "<aside class=\"side\">{}</aside>", shell.side);
    }
    h.push_str("</section>\n");
    h.push_str(&shell.after);
    h.push_str("</main>\n");
    let _ = writeln!(h, "<footer>{}</footer>", shell.footer);
    let json = serde_json::to_string(&shell.data)
        .unwrap_or_else(|_| "{}".to_string())
        .replace('<', "\\u003c");
    let _ = write!(
        h,
        "<script id=\"agenticspec-data\" type=\"application/json\">{json}</script>\n<script>\n{JS}</script>\n</body>\n</html>\n"
    );
    h
}

fn header(h: &mut String, hd: &Header) {
    h.push_str("<header class=\"top\"><div class=\"title\">");
    if let Some(e) = &hd.eyebrow {
        let _ = write!(h, "<div class=\"eyebrow\">{}</div>", esc(e));
    }
    let _ = write!(h, "<h1>{}</h1>", esc(&hd.heading));
    if let Some(s) = &hd.summary {
        let _ = write!(h, "<p class=\"summary\">{}</p>", esc(s));
    }
    h.push_str("<ul class=\"facts\">");
    for f in &hd.facts {
        let _ = write!(h, "<li>{}</li>", esc(f));
    }
    h.push_str("</ul></div>");
    h.push_str("<div class=\"segmented\" role=\"group\" aria-label=\"Theme\"><button type=\"button\" data-theme-choice=\"system\" aria-pressed=\"true\">System</button><button type=\"button\" data-theme-choice=\"light\" aria-pressed=\"false\">Light</button><button type=\"button\" data-theme-choice=\"dark\" aria-pressed=\"false\">Dark</button></div></header>\n");
}

/// The legend of a page: each group its nodes use, then regions, refusals and other marks.
fn legend(page: &Page) -> String {
    let mut h = String::from("<ul class=\"legend\" aria-label=\"Legend\">");
    let palette = graph::Palette::of(page, &[]);
    let mut groups: Vec<&str> = Vec::new();
    for n in &page.nodes {
        if !groups.contains(&n.group.as_str()) {
            groups.push(&n.group);
        }
    }
    for g in groups {
        let style = page
            .groups
            .iter()
            .find(|x| x.name == g)
            .and_then(|x| x.colour.as_ref())
            .map(|c| format!(" style=\"--c:{}\"", esc(c.as_str())))
            .unwrap_or_default();
        let _ = write!(
            h,
            "<li><span class=\"swatch f-{}\"{style}></span>{}</li>",
            palette.slot(g),
            esc(if g.is_empty() { "other" } else { g })
        );
    }
    if page.nodes.iter().any(|n| n.region.is_some()) {
        h.push_str(
            "<li><span class=\"swatch per-sample\"></span>a region, framed around its nodes</li>",
        );
    }
    if crate::collapse::Tree::of(page)
        .regions
        .iter()
        .any(|r| r.collapsed)
    {
        h.push_str(
            "<li><span class=\"swatch collapsed-sample\"></span>a collapsed region: its card opens it</li>",
        );
    }
    if page.marks.iter().any(|m| m.level == Level::Refusal) {
        h.push_str("<li><span class=\"swatch refusal-sample\"></span>refused</li>");
    }
    if page.marks.iter().any(|m| m.level == Level::Info) {
        h.push_str("<li><span class=\"swatch chip-sample\"></span>information</li>");
    }
    h.push_str("</ul>");
    h
}

/// The badges shown on a panel: one table "Details" under the graph, a row per badge, the rows
/// of one node together in the order of the nodes. Empty when no badge is shown on a panel.
fn details(page: &Page) -> String {
    let mut rows: Vec<Vec<String>> = Vec::new();
    for n in &page.nodes {
        for b in n.badges.iter().filter(|b| b.on == On::Panel) {
            rows.push(vec![n.label.clone(), b.name.clone(), b.value.clone()]);
        }
    }
    if rows.is_empty() {
        return String::new();
    }
    let mut h = String::from("<section class=\"section-gap\" aria-label=\"Details\">");
    panel(
        &mut h,
        &Panel {
            title: "Details".to_string(),
            columns: vec!["node".to_string(), "name".to_string(), "value".to_string()],
            rows,
            place: Place::Side,
        },
    );
    h.push_str("</section>\n");
    h
}

/// A cell value longer than this many characters may not fit its column.
const LONG: usize = 32;

/// The attributes of a cell that may not fit its column, each with a leading space: its full
/// value as a title, and its long words wrapped instead of cut. None for a short value.
fn long(cell: &str) -> String {
    if cell.chars().count() <= LONG {
        return String::new();
    }
    format!(
        " title=\"{}\" style=\"white-space:normal;overflow-wrap:anywhere;min-width:12ch\"",
        esc(cell)
    )
}

/// A panel as a card with one table; the first cell of a row heads it.
fn panel(h: &mut String, p: &Panel) {
    let _ = write!(
        h,
        "<div class=\"card\"><div class=\"card-head\"><h2>{}</h2><span class=\"hint\">{}</span></div><div class=\"table-wrap\"><table class=\"data mini\"><thead><tr>",
        esc(&p.title),
        plural(p.rows.len(), "row")
    );
    for c in &p.columns {
        let _ = write!(h, "<th scope=\"col\">{}</th>", esc(c));
    }
    h.push_str("</tr></thead><tbody>");
    for row in &p.rows {
        h.push_str("<tr>");
        for (i, cell) in row.iter().enumerate() {
            let (tag, scope) = if i == 0 {
                ("th", " scope=\"row\"")
            } else {
                ("td", "")
            };
            let _ = write!(h, "<{tag}{scope}{}>{}</{tag}>", long(cell), esc(cell));
        }
        h.push_str("</tr>");
    }
    h.push_str("</tbody></table></div></div>");
}
