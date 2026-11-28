//! A page's graph as an SVG, laid out deterministically.
//!
//! Nodes sit in rows by rank. Between two rows lies a label row, where every edge that crosses it
//! passes through an item of its own width: its label, a node drawn on the edge, or a narrow gap.
//! Items are ordered by barycentre sweeps and placed by isotonic regression, so no two items of a
//! row overlap and every edge label has the room its text needs.
//!
//! A large graph stays legible at its natural size: edges with one label into or out of one card
//! form a fan, with one label beside the card and one trunk through the rows between, which each
//! edge joins on a track of its own label row; a card that several cards of its row send to drops
//! to a row of its own; the passing items of a long line take one x where they fit, and a line
//! leaves a card shorter than its row straight, so no line crosses a card.
//!
//! A row of cards wider than [`page::WRAP_WIDTH`] wraps ([`layered`]): the layout is made again
//! with its cards in rows of their own, which every edge reaches as any other row. The cards a
//! frame holds wrap into lines the same way ([`place_lines`]), and an edge leaves a line through
//! the gap below it and the frame's corridor ([`Card::way_out`]).

use crate::fmt::{esc, mono_w, px, text_w};
use crate::page::{self, Direction, Mark, Page, Region};
use std::fmt::Write as _;

const MARGIN: f64 = 18.0;
const PAD: f64 = 12.0;
const ICON: f64 = 30.0;
const HEAD: f64 = 52.0;
const PROP: f64 = 17.0;
const CHIP: f64 = 25.0;
const MIN_W: f64 = 168.0;
const LINE_GAP: f64 = 12.0;
const ROW_GAP: f64 = 20.0;
const PILL_H: f64 = 20.0;
/// The gap before a chip on a label line.
const CHIP_GAP: f64 = 8.0;

/// The accessible title and description of the SVG.
#[derive(Clone, Default)]
pub(crate) struct Words {
    pub title: String,
    pub desc: String,
}

/// What a page of this crate draws beyond the render model: the rank and label of each group,
/// and per node, edge and mark what its card, line or chip shows besides. [`crate::page::render`]
/// draws with none of it.
#[derive(Clone, Default)]
pub(crate) struct Decor {
    pub words: Words,
    /// Group id, legend label, row. Without any, a node's row is the longest chain of edges that
    /// leads to it.
    pub groups: Vec<(String, String, u32)>,
    /// By node id.
    pub nodes: Vec<(String, NodeDecor)>,
    /// By edge index: a node the edge passes through. A node that is only ever passed through,
    /// between nodes of two rows, is drawn on the edge.
    pub vias: Vec<Option<String>>,
    /// By mark index: the chip drawn for the mark.
    pub chips: Vec<Option<Chip>>,
    /// The regions drawn open with a control that closes them.
    pub closable: Vec<String>,
}

#[derive(Clone, Default)]
pub(crate) struct NodeDecor {
    /// The line under the label; the group's label without it.
    pub subtitle: Option<String>,
    pub icon: Option<&'static str>,
    /// `key value` lines on the card.
    pub properties: Vec<(String, String)>,
    /// A dashed ring around the card and this text as its tag.
    pub outline: Option<String>,
    /// The accessible name of the card.
    pub description: Option<String>,
    /// The card stands for a collapsed region and opens it.
    pub opens: bool,
}

/// The colour slots of the stylesheet: a card of slot `s` has the class `f-s`.
const PALETTE: [&str; 10] = [
    "principal",
    "agent",
    "collab",
    "work",
    "intel",
    "capability",
    "place",
    "resource",
    "nested",
    "other",
];

/// The colour slot of each group name of a page.
pub(crate) struct Palette {
    extra: Vec<(String, &'static str)>,
}

impl Palette {
    /// A group named as a slot takes that slot; any other name takes the next slot no group of
    /// the page names, in order of first appearance; an empty name is `other`.
    pub fn of(page: &Page, declared: &[(String, String, u32)]) -> Self {
        let names: Vec<&str> = declared
            .iter()
            .map(|g| g.0.as_str())
            .chain(page.nodes.iter().map(|n| n.group.as_str()))
            .collect();
        let free: Vec<&'static str> = PALETTE[..8]
            .iter()
            .copied()
            .filter(|s| !names.contains(s))
            .collect();
        let free = if free.is_empty() {
            PALETTE[..8].to_vec()
        } else {
            free
        };
        let mut extra: Vec<(String, &'static str)> = Vec::new();
        for n in names {
            if n.is_empty() || PALETTE.contains(&n) || extra.iter().any(|(e, _)| e == n) {
                continue;
            }
            let slot = free[extra.len() % free.len()];
            extra.push((n.to_string(), slot));
        }
        Palette { extra }
    }

    pub fn slot(&self, group: &str) -> &'static str {
        if let Some(s) = PALETTE.iter().find(|s| **s == group) {
            return s;
        }
        self.extra
            .iter()
            .find(|(n, _)| n == group)
            .map(|(_, s)| *s)
            .unwrap_or("other")
    }
}

/// Every region of a page with the id of the region it lies in.
fn all_regions(page: &Page) -> Vec<(&Region, Option<String>)> {
    fn walk<'a>(
        r: &'a Region,
        parent: Option<String>,
        out: &mut Vec<(&'a Region, Option<String>)>,
    ) {
        out.push((r, parent));
        for inner in &r.regions {
            walk(inner, Some(r.id.clone()), out);
        }
    }
    let mut out = Vec::new();
    for r in &page.regions {
        walk(r, None, &mut out);
    }
    out
}

/// A 14 by 14 stroke icon by name.
fn icon(name: &str) -> &'static str {
    match name {
        "person" => {
            "M7 6.6a2.4 2.4 0 1 0 0-4.8 2.4 2.4 0 0 0 0 4.8zM2.4 12.6c0-2.6 2-4.2 4.6-4.2s4.6 1.6 4.6 4.2"
        }
        "bot" => {
            "M3.5 4.5h7a1.5 1.5 0 0 1 1.5 1.5v4.5a1.5 1.5 0 0 1-1.5 1.5h-7A1.5 1.5 0 0 1 2 10.5V6a1.5 1.5 0 0 1 1.5-1.5zM7 4.5V2M5.2 8.2h.01M8.8 8.2h.01"
        }
        "bubble" => "M2 2.5h10v7H6.5L3.5 12V9.5H2z",
        "gateway" => "M2 2.5v9M5 7h7M9 4l3 3-3 3",
        "chip" => {
            "M4 4h6v6H4zM5.5 1.5V4M8.5 1.5V4M5.5 10v2.5M8.5 10v2.5M1.5 5.5H4M1.5 8.5H4M10 5.5h2.5M10 8.5h2.5"
        }
        "wrench" => {
            "M9.6 1.8a3 3 0 0 0-3.4 4.1L2 10.1 3.9 12l4.2-4.2a3 3 0 0 0 4.1-3.4L10.4 6.2 8.6 5.4 7.8 3.6z"
        }
        "folder" => "M1.5 3.5h4l1.5 1.5h5.5v6.5h-11z",
        "shield" => "M7 1.5l5 2v3.6c0 2.9-2.1 4.7-5 5.4-2.9-.7-5-2.5-5-5.4V3.5z",
        "play" => "M4 2.5v9l7.5-4.5z",
        "stack" => "M2 2.5h10v2.5H2zM2 6.25h10v2.5H2zM2 10h10v2H2z",
        "checklist" => "M2 3.8l1.4 1.4L6 2.6M2 9l1.4 1.4L6 7.8M8 4.2h4.5M8 9.4h4.5",
        "lock" => "M3.5 6.5h7v5.5h-7zM5 6.5V4.8a2 2 0 0 1 4 0v1.7",
        "frame" => "M1.5 2.5h11v9h-11zM4 5h6v4H4zM1.5 7H4M10 7h2.5",
        _ => "M7 12A5 5 0 1 0 7 2a5 5 0 0 0 0 10z",
    }
}

/// What a chip on a card (or beside the graph) shows: a field, its value and the value it would
/// change to, a range in brackets or a plain detail.
#[derive(Clone, Default)]
pub(crate) struct Chip {
    pub field: String,
    pub value: Option<String>,
    pub best: Option<String>,
    pub range: String,
    pub detail: String,
    /// Written as `data-place`.
    pub id: Option<String>,
    pub refusal: bool,
}

impl Chip {
    /// The chip of a mark of the page.
    fn of(m: &Mark) -> Self {
        Chip {
            field: match &m.target.port {
                Some(p) => format!("{p} · {}", m.name),
                None => m.name.clone(),
            },
            detail: m.detail.clone().unwrap_or_default(),
            refusal: m.level == page::Level::Refusal,
            ..Chip::default()
        }
    }

    pub fn changed(&self) -> bool {
        self.best
            .as_ref()
            .is_some_and(|b| Some(b) != self.value.as_ref())
    }

    fn class(&self) -> &'static str {
        match (self.changed(), self.refusal) {
            (true, _) => "chip changed",
            (false, true) => "chip refusal",
            (false, false) => "chip",
        }
    }

    /// The attributes, each with a leading space.
    fn attrs(&self) -> String {
        let mut s = String::new();
        if let Some(id) = &self.id {
            let _ = write!(s, " data-place=\"{}\"", esc(id));
        }
        let value = self.value.as_deref().unwrap_or("");
        match &self.best {
            Some(b) => {
                let _ = write!(
                    s,
                    " data-baseline=\"{}\" data-best=\"{}\"",
                    esc(value),
                    esc(b)
                );
            }
            None if self.value.is_some() => {
                let _ = write!(s, " data-value=\"{}\"", esc(value));
            }
            None => {}
        }
        s
    }

    /// One line of text: `capacity 16, best 24, range 8…32`, or `name: detail`.
    pub fn describe(&self) -> String {
        let Some(v) = &self.value else {
            return if self.detail.is_empty() {
                self.field.clone()
            } else {
                format!("{}: {}", self.field, self.detail)
            };
        };
        let mut s = format!("{} {v}", self.field);
        if let Some(b) = &self.best {
            s.push_str(&format!(", best {b}"));
        }
        if !self.range.is_empty() {
            s.push_str(&format!(", range {}", self.range));
        }
        s
    }

    fn width(&self) -> f64 {
        let mut w = 9.0 + text_w(&self.field, 11.5, false);
        if let Some(v) = &self.value {
            w += 7.0;
            w += mono_w(v, 11.5);
        }
        if let Some(b) = &self.best {
            w += 6.0 + mono_w("→", 11.5) + 6.0 + mono_w(b, 11.5) + 4.0;
        }
        if !self.range.is_empty() {
            w += 7.0 + mono_w(&format!("[{}]", self.range), 11.0);
        }
        if !self.detail.is_empty() {
            w += 7.0 + mono_w(&self.detail, 11.0);
        }
        w + 9.0
    }

    /// The chip as SVG, its left edge at `x`, its top at `y`.
    fn svg(&self, out: &mut String, x: f64, y: f64) {
        let w = self.width();
        let _ = write!(
            out,
            "<g class=\"{}\"{}><title>{}</title><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{PILL_H}\" rx=\"10\"/>",
            self.class(),
            self.attrs(),
            esc(&self.describe()),
            px(x),
            px(y),
            px(w)
        );
        let base = y + 14.0;
        let mut cx = x + 9.0;
        let _ = write!(
            out,
            "<text class=\"chip-k\" x=\"{}\" y=\"{}\">{}</text>",
            px(cx),
            px(base),
            esc(&self.field)
        );
        cx += text_w(&self.field, 11.5, false);
        let changed = self.changed();
        if let Some(v) = &self.value {
            cx += 7.0;
            let was = if changed { "chip-v was" } else { "chip-v" };
            let _ = write!(
                out,
                "<text class=\"{was}\" x=\"{}\" y=\"{}\">{}</text>",
                px(cx),
                px(base),
                esc(v)
            );
            cx += mono_w(v, 11.5);
        }
        if let Some(b) = &self.best {
            cx += 6.0;
            let _ = write!(
                out,
                "<text class=\"chip-arrow\" x=\"{}\" y=\"{}\">→</text>",
                px(cx),
                px(base)
            );
            cx += mono_w("→", 11.5) + 6.0;
            let now = if changed { "chip-v now" } else { "chip-v" };
            let _ = write!(
                out,
                "<text class=\"{now}\" x=\"{}\" y=\"{}\">{}</text>",
                px(cx),
                px(base),
                esc(b)
            );
            cx += mono_w(b, 11.5) + 4.0;
        }
        if !self.range.is_empty() {
            cx += 7.0;
            let _ = write!(
                out,
                "<text class=\"chip-r\" x=\"{}\" y=\"{}\">[{}]</text>",
                px(cx),
                px(base),
                esc(&self.range)
            );
            cx += mono_w(&format!("[{}]", self.range), 11.0);
        }
        if !self.detail.is_empty() {
            cx += 7.0;
            let _ = write!(
                out,
                "<text class=\"chip-r\" x=\"{}\" y=\"{}\">{}</text>",
                px(cx),
                px(base),
                esc(&self.detail)
            );
        }
        out.push_str("</g>");
    }

    /// The chip as HTML, with or without its field.
    pub fn html(&self, with_field: bool) -> String {
        let changed = self.changed();
        let field = if with_field || self.value.is_none() {
            format!("<span class=\"chip-k\">{}</span> ", esc(&self.field))
        } else {
            String::new()
        };
        let mut s = format!("<span class=\"{}\"{}>{field}", self.class(), self.attrs(),);
        if let Some(v) = &self.value {
            let _ = write!(
                s,
                "<span class=\"chip-v{}\">{}</span>",
                if changed { " was" } else { "" },
                esc(v)
            );
        }
        if let Some(b) = &self.best {
            s.push_str(&format!(
                " <span class=\"chip-arrow\">→</span> <span class=\"chip-v{}\">{}</span>",
                if changed { " now" } else { "" },
                esc(b)
            ));
        }
        if !self.range.is_empty() {
            s.push_str(&format!(
                " <span class=\"chip-r\">[{}]</span>",
                esc(&self.range)
            ));
        }
        if !self.detail.is_empty() {
            s.push_str(&format!(
                " <span class=\"chip-r\">{}</span>",
                esc(&self.detail)
            ));
        }
        s.push_str("</span>");
        s
    }
}

/// A node as the layout reads it.
struct Part {
    id: String,
    label: String,
    slot: &'static str,
    icon: &'static str,
    /// The colour of its group, written as `--c` over the slot's.
    colour: Option<String>,
    /// A text drawn in the icon's tile instead of the icon.
    icon_text: Option<String>,
    subtitle: String,
    badges: Vec<String>,
    outline: Option<String>,
    refused: bool,
    /// A region that is no node: drawn only as the frame around what it holds.
    frame_only: bool,
    /// The card of a collapsed region, which opens it.
    opens: bool,
    props: Vec<(String, String)>,
    description: String,
    region: Option<String>,
    row: usize,
    ports: Vec<PortRow>,
    chips: Vec<Chip>,
}

/// A port as one row of its card.
struct PortRow {
    /// `node.port`.
    address: String,
    own: String,
    title: String,
    refused: bool,
    via: Option<String>,
    unresolved: bool,
    /// The names of the refusals that mark the port.
    refusals: Vec<String>,
}

impl PortRow {
    /// The text after `port`.
    fn text(&self) -> String {
        let mut s = self.own.clone();
        if let Some(v) = &self.via {
            s.push_str(&format!(" {v}"));
        }
        if self.unresolved && !self.refusals.is_empty() {
            s.push_str(&format!(" {}", self.refusals.join(", ")));
        }
        s
    }
}

/// An edge as the layout reads it.
struct Link {
    from: String,
    to: String,
    label: String,
    via: Option<String>,
    /// Whether the edge stands for more than one.
    multi: bool,
    /// The stroke: 1 for one edge, 2 for two or three, 3 for four or more.
    weight: u8,
}

/// The nodes and edges of a page, resolved for layout.
struct Scene {
    parts: Vec<Part>,
    links: Vec<Link>,
    /// Each region with the region it lies in, the heading of what it holds and whether it reads
    /// "per".
    regions: Vec<(String, Option<String>, String, bool)>,
    /// The part whose frame each part is drawn in.
    parent: Vec<Option<usize>>,
    /// The regions drawn open with a control that closes them.
    closable: Vec<String>,
}

impl Scene {
    fn part(&self, id: &str) -> Option<&Part> {
        self.parts.iter().find(|p| p.id == id)
    }

    /// The part an edge end names: a node, or the node of the port `<node>.<port>`.
    fn index(&self, name: &str) -> Option<usize> {
        self.parts.iter().position(|p| p.id == name).or_else(|| {
            let (head, _) = name.split_once('.')?;
            self.parts
                .iter()
                .position(|p| p.id == head && !p.ports.is_empty())
        })
    }

    fn read(page: &Page, decor: &Decor, frame: &Frame) -> Self {
        let palette = Palette::of(frame.palette.unwrap_or(page), &decor.groups);
        let group_label = |g: &str| -> String {
            decor
                .groups
                .iter()
                .find(|d| d.0 == g)
                .map(|d| d.1.clone())
                .unwrap_or_else(|| {
                    if g.is_empty() {
                        "other".to_string()
                    } else {
                        g.to_string()
                    }
                })
        };
        let regions: Vec<(String, Option<String>, String, bool)> = all_regions(page)
            .into_iter()
            .map(|(r, parent)| {
                let heading = match (&r.per, &r.r#in) {
                    (Some(p), _) => format!("per {p}"),
                    (None, Some(i)) => format!("in {i}"),
                    (None, None) => format!("in {}", r.label),
                };
                (r.id.clone(), parent, heading, r.per.is_some())
            })
            .collect();
        let rows = rows(page, &decor.groups);
        let none = NodeDecor::default();
        let mut parts: Vec<Part> = page
            .nodes
            .iter()
            .zip(&rows)
            .map(|(n, &row)| {
                let d = decor
                    .nodes
                    .iter()
                    .find(|(id, _)| *id == n.id)
                    .map(|(_, d)| d)
                    .unwrap_or(&none);
                let subtitle = d.subtitle.clone().unwrap_or_else(|| group_label(&n.group));
                let look = page.groups.iter().find(|g| g.name == n.group);
                let chosen = look.and_then(|g| g.icon.as_ref());
                let badges: Vec<String> = n
                    .badges
                    .iter()
                    .filter(|b| b.on == page::On::Card)
                    .map(|b| {
                        if b.name == "count" {
                            format!("×{}", b.value)
                        } else {
                            format!("{} {}", b.name, b.value)
                        }
                    })
                    .collect();
                let description = d.description.clone().unwrap_or_else(|| {
                    let mut s = format!("{}, {subtitle}", n.label);
                    for b in &badges {
                        s.push_str(&format!(", {b}"));
                    }
                    s
                });
                Part {
                    id: n.id.clone(),
                    label: n.label.clone(),
                    slot: palette.slot(&n.group),
                    icon: icon(chosen.and_then(page::Icon::name).or(d.icon).unwrap_or("")),
                    colour: look
                        .and_then(|g| g.colour.as_ref())
                        .map(|c| c.as_str().to_string()),
                    icon_text: chosen
                        .filter(|i| i.name().is_none())
                        .map(|i| i.as_str().to_string()),
                    description,
                    subtitle,
                    badges,
                    outline: d.outline.clone(),
                    refused: false,
                    frame_only: false,
                    opens: d.opens,
                    props: d.properties.clone(),
                    region: n.region.clone(),
                    row,
                    ports: Vec::new(),
                    chips: Vec::new(),
                }
            })
            .collect();
        // A region that is no node is a frame of its own, inside the region it lies in.
        for (r, parent) in all_regions(page) {
            if parts.iter().any(|p| p.id == r.id) {
                continue;
            }
            parts.push(Part {
                id: r.id.clone(),
                label: r.label.clone(),
                slot: "other",
                icon: icon("frame"),
                colour: None,
                icon_text: None,
                description: r.label.clone(),
                subtitle: String::new(),
                badges: Vec::new(),
                outline: None,
                refused: false,
                frame_only: true,
                opens: false,
                props: Vec::new(),
                region: parent,
                row: usize::MAX,
                ports: Vec::new(),
                chips: Vec::new(),
            });
        }
        for port in &page.ports {
            let Some(p) = parts.iter_mut().find(|p| p.id == port.node) else {
                continue;
            };
            let address = format!("{}.{}", port.node, port.name);
            let mut title = if port.kind.is_empty() {
                address.clone()
            } else {
                format!("{} {address}", port.kind)
            };
            match port.direction {
                Direction::In => title.push_str(", in"),
                Direction::Out => title.push_str(", out"),
                Direction::Both => {}
            }
            if let Some(c) = &port.contract {
                title.push_str(&format!(", contract {c}"));
            }
            if let Some(v) = &port.via {
                title.push_str(&format!(", via {v}"));
            }
            if port.unresolved {
                title.push_str(", unresolved");
            }
            p.ports.push(PortRow {
                address,
                own: port.name.clone(),
                title,
                refused: false,
                via: port.via.clone(),
                unresolved: port.unresolved,
                refusals: Vec::new(),
            });
        }
        for (i, m) in page.marks.iter().enumerate() {
            let Some(p) = parts.iter_mut().find(|p| p.id == m.target.node) else {
                continue;
            };
            if m.level == page::Level::Refusal {
                match &m.target.port {
                    Some(name) => {
                        for port in p.ports.iter_mut().filter(|x| x.own == *name) {
                            port.refused = true;
                            port.refusals.push(m.name.clone());
                        }
                    }
                    None => p.refused = true,
                }
            }
            let chip = decor
                .chips
                .get(i)
                .and_then(|c| c.clone())
                .unwrap_or_else(|| Chip::of(m));
            p.chips.push(chip);
        }
        // The frame each part is drawn in: the part its region names, never itself, and no
        // chain that comes back to where it started.
        let n = parts.len();
        let mut parent: Vec<Option<usize>> = (0..n)
            .map(|i| {
                let r = parts[i].region.as_deref()?;
                parts.iter().position(|p| p.id == r).filter(|&j| j != i)
            })
            .collect();
        for i in 0..n {
            let mut cur = i;
            for _ in 0..=n {
                match parent[cur] {
                    Some(j) if j == i => {
                        parent[i] = None;
                        break;
                    }
                    Some(j) => cur = j,
                    None => break,
                }
            }
        }
        // A frame of its own sits in the top row of what it holds.
        for _ in 0..n {
            for i in 0..n {
                if !parts[i].frame_only {
                    continue;
                }
                let r = (0..n)
                    .filter(|&c| parent[c] == Some(i))
                    .map(|c| parts[c].row)
                    .min()
                    .unwrap_or(usize::MAX);
                parts[i].row = r;
            }
        }
        for p in &mut parts {
            if p.row == usize::MAX {
                p.row = 0;
            }
        }
        let links = page
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| Link {
                from: e.from.clone(),
                to: e.to.clone(),
                label: match e.count {
                    Some(c) if e.label.is_empty() => format!("×{c}"),
                    Some(c) => format!("{} ×{c}", e.label),
                    None => e.label.clone(),
                },
                via: decor.vias.get(i).cloned().flatten(),
                multi: e.count.is_some_and(|c| c > 1),
                weight: weight_of(e.count),
            })
            .collect();
        Scene {
            parts,
            links,
            regions,
            parent,
            closable: decor.closable.clone(),
        }
    }

    /// The heading of what the frame of part `c` holds, and whether it reads "per".
    fn heading(&self, c: usize) -> (String, bool) {
        let p = &self.parts[c];
        match self.regions.iter().find(|r| r.0 == p.id) {
            Some((_, _, heading, per)) => (heading.clone(), *per),
            None => (format!("in {}", p.label), false),
        }
    }

    /// Whether part `c` is drawn as a frame: a region that is no node, or a node others are in.
    fn framed(&self, c: usize) -> bool {
        self.parts[c].frame_only || self.parent.contains(&Some(c))
    }

    /// The outermost part whose frame part `i` is drawn in.
    fn top(&self, i: usize) -> usize {
        let mut cur = i;
        while let Some(j) = self.parent[cur] {
            cur = j;
        }
        cur
    }
}

/// The row of each node: its group's rank; below every ranked row for a group without one; the
/// longest chain of edges that leads to it when no group has a rank.
fn rows(page: &Page, groups: &[(String, String, u32)]) -> Vec<usize> {
    if !groups.is_empty() {
        let below = groups.iter().map(|g| g.2 as usize).max().unwrap_or(0) + 1;
        return page
            .nodes
            .iter()
            .map(|n| {
                groups
                    .iter()
                    .find(|g| g.0 == n.group)
                    .map(|g| g.2 as usize)
                    .unwrap_or(below)
            })
            .collect();
    }
    let n = page.nodes.len();
    let index = |end: &str| {
        page.nodes.iter().position(|x| x.id == end).or_else(|| {
            let (head, _) = end.split_once('.')?;
            page.nodes.iter().position(|x| x.id == head)
        })
    };
    let edges: Vec<(usize, usize)> = page
        .edges
        .iter()
        .filter_map(|e| Some((index(&e.from)?, index(&e.to)?)))
        .filter(|(a, b)| a != b)
        .collect();
    // Edges that close a cycle, found by a depth-first walk in page order, are left out.
    let mut state = vec![0u8; n];
    let mut kept: Vec<(usize, usize)> = Vec::new();
    for start in 0..n {
        if state[start] != 0 {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = vec![(start, 0)];
        state[start] = 1;
        while let Some(top) = stack.last_mut() {
            let v = top.0;
            let out: Vec<usize> = edges.iter().filter(|e| e.0 == v).map(|e| e.1).collect();
            if top.1 < out.len() {
                let w = out[top.1];
                top.1 += 1;
                match state[w] {
                    0 => {
                        kept.push((v, w));
                        state[w] = 1;
                        stack.push((w, 0));
                    }
                    2 => kept.push((v, w)),
                    _ => {}
                }
            } else {
                state[v] = 2;
                stack.pop();
            }
        }
    }
    let mut row = vec![0usize; n];
    for _ in 0..n {
        let mut moved = false;
        for &(a, b) in &kept {
            if row[b] < row[a] + 1 {
                row[b] = row[a] + 1;
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }
    row
}

/// Padding of a frame, the height of its heading and the gap between what it holds.
const FPAD: f64 = 10.0;
const FHEAD: f64 = 24.0;
const FGAP: f64 = 16.0;

/// The size of the card of node `p`.
fn node_size(sc: &Scene, p: usize) -> (f64, f64) {
    let p = &sc.parts[p];
    let name_w = text_w(&p.label, 13.5, true);
    let badge: f64 = p.badges.iter().map(|b| badge_w(b) + 8.0).sum();
    let tag = p.outline.as_ref().map(|t| tag_w(t) + 8.0).unwrap_or(0.0);
    let head = ICON + 10.0 + (name_w + badge).max(text_w(&p.subtitle, 11.5, false) + tag);
    let mut inner = head;
    for (k, v) in &p.props {
        inner = inner.max(text_w(k, 11.5, false) + 6.0 + text_w(v, 11.5, false));
    }
    for port in &p.ports {
        inner = inner.max(text_w(&format!("port {}", port.text()), 11.5, false) + 8.0);
    }
    for c in &p.chips {
        inner = inner.max(c.width());
    }
    let w = (inner + 2.0 * PAD).max(MIN_W).ceil();
    let mut h = HEAD;
    h += p.props.len() as f64 * PROP;
    h += p.ports.len() as f64 * PROP;
    h += p.chips.len() as f64 * CHIP;
    let h = if h == HEAD { HEAD } else { h + 6.0 };
    (w, h)
}

/// One item of a row: a node's card, or the frame of a region with the cards and frames it
/// holds side by side.
struct Card {
    w: f64,
    h: f64,
    row: usize,
    /// Each node card: its part, its offset from the item's top left and its size.
    nodes: Vec<(usize, f64, f64, f64, f64)>,
    /// Each frame, outermost first: its part, offset and size.
    frames: Vec<(usize, f64, f64, f64, f64)>,
    /// Two node cards side by side in one frame: their parts and the gap's left and right edge
    /// and top, from the item's top left.
    gaps: Vec<(usize, usize, f64, f64, f64)>,
    /// Each frame whose cards wrap into lines.
    lines: Vec<Lines>,
}

/// A frame whose cards wrap into lines: its part, the top and bottom of each line and the left
/// edge of its corridor (the room beside the lines an edge runs along), from the item's top
/// left.
struct Lines {
    frame: usize,
    rows: Vec<(f64, f64)>,
    corridor: f64,
}

/// The gap between two lines of a frame whose cards wrap, and the room of its corridor. An edge
/// from a card of a line runs down into the gap below the line, along it to the corridor and
/// down the corridor out of the frame (up, to a card it enters from above).
const WRAP_GAP: f64 = 28.0;
const CORRIDOR: f64 = 22.0;

impl Card {
    fn new(sc: &Scene, part: usize) -> Self {
        let mut c = Card::empty(sc.parts[part].row);
        let (w, h) = place(sc, part, 0.0, 0.0, &mut c);
        c.w = w;
        c.h = h;
        c
    }

    fn empty(row: usize) -> Self {
        Card {
            w: 0.0,
            h: 0.0,
            row,
            nodes: Vec::new(),
            frames: Vec::new(),
            gaps: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// The way out of the frames that hold node card `p`, from the point `x`, `y` on its bottom
    /// (`up`: on its top): through each frame whose cards wrap, from a line that is not its last
    /// (`up`: first), into the gap next to the line, along it to the corridor and along the
    /// corridor out. The corners, from the item's top left; none when no wrapped frame is in the
    /// way, so the edge leaves the card straight. `k` spreads the edges of one gap and corridor.
    fn way_out(
        &self,
        sc: &Scene,
        p: usize,
        (x, y): (f64, f64),
        up: bool,
        k: usize,
    ) -> Vec<(f64, f64)> {
        let mut chain: Vec<usize> = Vec::new();
        if self.frames.iter().any(|f| f.0 == p) {
            chain.push(p);
        }
        let mut cur = sc.parent[p];
        while let Some(f) = cur {
            if !self.frames.iter().any(|x| x.0 == f) {
                break;
            }
            chain.push(f);
            cur = sc.parent[f];
        }
        let spread = 3.0 * (k % 5) as f64;
        let (mut x, mut y) = (x, y);
        let mut pts = Vec::new();
        for f in chain {
            let Some(&(_, _, fy, _, fh)) = self.frames.iter().find(|x| x.0 == f) else {
                continue;
            };
            if let Some(lines) = self.lines.iter().find(|l| l.frame == f) {
                let rows = &lines.rows;
                let line = if up {
                    rows.iter().position(|r| y >= r.0 - 0.5 && y < r.1)
                } else {
                    rows.iter().position(|r| y > r.0 && y <= r.1 + 0.5)
                };
                let next = match line {
                    Some(l) if up && l > 0 => Some(rows[l - 1].1),
                    Some(l) if !up && l + 1 < rows.len() => Some(rows[l].1),
                    _ => None,
                };
                if let Some(bottom) = next {
                    let gy = bottom + 8.0 + spread;
                    let cx = lines.corridor + 4.0 + spread;
                    pts.push((x, gy));
                    pts.push((cx, gy));
                    x = cx;
                }
            }
            y = if up { fy } else { fy + fh };
        }
        pts
    }

    /// The node card of part `p`: its offset and size.
    fn node(&self, p: usize) -> Option<(f64, f64, f64, f64)> {
        self.nodes
            .iter()
            .find(|n| n.0 == p)
            .map(|n| (n.1, n.2, n.3, n.4))
    }

    /// Whether the item is one node card alone.
    fn single(&self) -> bool {
        self.frames.is_empty()
    }

    /// Mirrors what the frames hold left to right.
    fn mirror(&mut self) {
        let w = self.w;
        for n in self.nodes.iter_mut().chain(self.frames.iter_mut()) {
            n.1 = w - n.1 - n.3;
        }
        for g in &mut self.gaps {
            *g = (g.1, g.0, w - g.3, w - g.2, g.4);
        }
        for l in &mut self.lines {
            l.corridor = w - l.corridor - CORRIDOR;
        }
    }
}

/// The text over a frame: the region's label when it is no node, then its heading.
fn frame_head(sc: &Scene, p: usize) -> (Option<String>, String, bool) {
    let (heading, per) = sc.heading(p);
    let label = sc.parts[p].frame_only.then(|| sc.parts[p].label.clone());
    (label, heading, per)
}

fn frame_head_w(sc: &Scene, p: usize) -> f64 {
    let (label, heading, _) = frame_head(sc, p);
    let close = if sc.closable.contains(&sc.parts[p].id) {
        close_w() + 8.0
    } else {
        0.0
    };
    label.map(|l| text_w(&l, 12.0, true) + 8.0).unwrap_or(0.0)
        + text_w(&heading, 11.0, false)
        + frame_chips_w(sc, p)
        + close
}

/// The chips of the marks on a region that is no node: drawn on its frame's label line, after
/// the heading. Those of a node others are in are drawn on its card.
fn frame_chips(sc: &Scene, p: usize) -> &[Chip] {
    if sc.parts[p].frame_only {
        &sc.parts[p].chips
    } else {
        &[]
    }
}

/// The width the chips of a label line take, the gap before each included.
fn chips_w(chips: &[Chip]) -> f64 {
    chips.iter().map(|c| CHIP_GAP + c.width()).sum()
}

fn frame_chips_w(sc: &Scene, p: usize) -> f64 {
    chips_w(frame_chips(sc, p))
}

/// Draws `chips` side by side on a label line, the first `CHIP_GAP` right of `x`, their top at
/// `y`. Returns where the last ends.
fn line_chips(svg: &mut String, chips: &[Chip], mut x: f64, y: f64) -> f64 {
    for c in chips {
        x += CHIP_GAP;
        c.svg(svg, x, y);
        x += c.width();
    }
    x
}

/// The ring around a refused frame or band, `off` outside its box.
fn frame_ring(svg: &mut String, x: f64, y: f64, w: f64, h: f64) {
    let off = 5.0;
    let _ = write!(
        svg,
        "<rect class=\"refusal-ring\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/>",
        px(x - off),
        px(y - off),
        px(w + 2.0 * off),
        px(h + 2.0 * off),
        px(12.0 + off)
    );
}

/// The width of the control that closes an open region.
fn close_w() -> f64 {
    (text_w("close", 11.0, false) + 22.0).ceil()
}

/// The control that closes the open region `id`, its right edge at `right` and its top at `y`.
fn close_control(svg: &mut String, id: &str, label: &str, right: f64, y: f64) {
    let w = close_w();
    let x = right - w;
    let _ = write!(
        svg,
        "<g class=\"region-close\" data-close=\"{}\"><rect class=\"close-bg\" tabindex=\"0\" role=\"button\" aria-expanded=\"true\" aria-label=\"Close region {}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"17\" rx=\"8.5\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\">close</text></g>",
        esc(id),
        esc(label),
        px(x),
        px(y),
        px(w),
        px(x + w / 2.0),
        px(y + 12.5)
    );
}

/// Lays out part `p` with its top left at `x`, `y`: its node card, or its frame holding its own
/// card first and then each part drawn in it, side by side. Returns the size.
fn place(sc: &Scene, p: usize, x: f64, y: f64, card: &mut Card) -> (f64, f64) {
    if !sc.framed(p) {
        let (w, h) = node_size(sc, p);
        card.nodes.push((p, x, y, w, h));
        return (w, h);
    }
    let fi = card.frames.len();
    card.frames.push((p, x, y, 0.0, 0.0));
    let mut members: Vec<usize> = Vec::new();
    if !sc.parts[p].frame_only {
        members.push(p);
    }
    members.extend((0..sc.parts.len()).filter(|&c| sc.parent[c] == Some(p)));
    let plain = |m: usize| m == p || !sc.framed(m);
    let gap = |a: usize, m: usize| -> f64 {
        let widest = if plain(a) && plain(m) {
            sc.links
                .iter()
                .filter(|l| {
                    let (f, t) = (sc.index(&l.from), sc.index(&l.to));
                    (f == Some(a) && t == Some(m)) || (f == Some(m) && t == Some(a))
                })
                .map(|l| text_w(&edge_label(sc, l, false), 11.5, false) + 18.0 + 24.0)
                .fold(0.0, f64::max)
        } else {
            0.0
        };
        FGAP.max(widest)
    };
    // The members in lines when side by side they would be wider than the wrap width.
    let widths: Vec<f64> = members
        .iter()
        .map(|&m| {
            if m == p {
                node_size(sc, p).0
            } else {
                place(sc, m, 0.0, 0.0, &mut Card::empty(0)).0
            }
        })
        .collect();
    let gaps: Vec<f64> = (0..members.len())
        .map(|k| {
            if k == 0 {
                0.0
            } else {
                gap(members[k - 1], members[k])
            }
        })
        .collect();
    let mut lines: Vec<Vec<usize>> = Vec::new();
    for (k, l) in wrap_lines(&widths, &gaps, page::WRAP_WIDTH)
        .into_iter()
        .enumerate()
    {
        if lines.len() <= l {
            lines.push(Vec::new());
        }
        lines[l].push(members[k]);
    }
    if lines.len() > 1 {
        return place_lines(sc, p, (x, y), &lines, &gap, card, fi);
    }
    let top = y + FHEAD;
    let mut cx = x + FPAD;
    let mut h: f64 = 0.0;
    let mut prev: Option<usize> = None;
    for &m in &members {
        let left = cx;
        if let Some(a) = prev {
            cx += gap(a, m);
            if plain(a) && plain(m) {
                card.gaps.push((a, m, left, cx, top));
            }
        }
        let (mw, mh) = if m == p {
            let (w, h) = node_size(sc, p);
            card.nodes.push((p, cx, top, w, h));
            (w, h)
        } else {
            place(sc, m, cx, top, card)
        };
        cx += mw;
        h = h.max(mh);
        prev = Some(m);
    }
    let w = (cx - x + FPAD).max(frame_head_w(sc, p) + 2.0 * FPAD).ceil();
    let h = FHEAD + h.max(20.0) + FPAD;
    card.frames[fi] = (p, x, y, w, h);
    (w, h)
}

/// The line of each of a row of cards of these widths, `gaps[k]` before card `k`: as few lines as
/// keep each within `target` (a card wider alone has a line of its own), in order, the width
/// shared out as evenly as the order allows. One line when the row fits.
fn wrap_lines(widths: &[f64], gaps: &[f64], target: f64) -> Vec<usize> {
    let total: f64 = widths.iter().sum::<f64>() + gaps.iter().skip(1).sum::<f64>();
    let mut n = ((total / target).ceil() as usize).max(1);
    loop {
        let per = total / n as f64;
        let mut lines = Vec::with_capacity(widths.len());
        let mut at = 0.0;
        for (k, &w) in widths.iter().enumerate() {
            let start = if k == 0 { 0.0 } else { at + gaps[k] };
            lines.push((((start + w / 2.0) / per) as usize).min(n - 1));
            at = start + w;
        }
        // Lines no card fell into are dropped.
        let mut seen = 0;
        let mut last = usize::MAX;
        for l in &mut lines {
            if *l != last {
                last = *l;
                seen += 1;
            }
            *l = seen - 1;
        }
        let mut fits = true;
        let mut used = 0.0;
        for k in 0..widths.len() {
            if k > 0 && lines[k] == lines[k - 1] {
                used += gaps[k] + widths[k];
            } else {
                used = widths[k];
            }
            let alone = (k == 0 || lines[k] != lines[k - 1])
                && (k + 1 == widths.len() || lines[k + 1] != lines[k]);
            if used > target && !alone {
                fits = false;
            }
        }
        if fits || n >= widths.len() {
            return lines;
        }
        n += 1;
    }
}

/// Each row of cards (indices into `widths`, in their order) that is wider than `target` with
/// [`CARD_GAP`] between its cards, cut into lines; none when every row fits.
fn cut(rows: &[Vec<usize>], widths: &[f64], target: f64) -> Option<Wrap> {
    let mut at: Vec<Option<(usize, usize)>> = vec![None; widths.len()];
    let mut wraps = false;
    for row in rows {
        let w: Vec<f64> = row.iter().map(|&c| widths[c]).collect();
        let lines = wrap_lines(&w, &vec![CARD_GAP; row.len()], target);
        if lines.last().is_none_or(|&l| l == 0) {
            continue;
        }
        wraps = true;
        for (k, (&c, line)) in row.iter().zip(lines).enumerate() {
            at[c] = Some((line, k));
        }
    }
    wraps.then_some(Wrap { at, target })
}

/// Lays out the frame of part `p` whose members wrap: each line left to right as [`place`] lays
/// out one, the lines top to bottom with [`WRAP_GAP`] between them, and the corridor right of the
/// widest line. Returns the size.
fn place_lines(
    sc: &Scene,
    p: usize,
    (x, y): (f64, f64),
    lines: &[Vec<usize>],
    gap: &dyn Fn(usize, usize) -> f64,
    card: &mut Card,
    fi: usize,
) -> (f64, f64) {
    let plain = |m: usize| m == p || !sc.framed(m);
    let mut top = y + FHEAD;
    let mut rows: Vec<(f64, f64)> = Vec::new();
    let mut widest: f64 = 0.0;
    for line in lines {
        let mut cx = x + FPAD;
        let mut h: f64 = 0.0;
        let mut prev: Option<usize> = None;
        for &m in line {
            let left = cx;
            if let Some(a) = prev {
                cx += gap(a, m);
                if plain(a) && plain(m) {
                    card.gaps.push((a, m, left, cx, top));
                }
            }
            let (mw, mh) = if m == p {
                let (w, h) = node_size(sc, p);
                card.nodes.push((p, cx, top, w, h));
                (w, h)
            } else {
                place(sc, m, cx, top, card)
            };
            cx += mw;
            h = h.max(mh);
            prev = Some(m);
        }
        rows.push((top, top + h));
        widest = widest.max(cx - x);
        top += h + WRAP_GAP;
    }
    let bottom = rows.last().map_or(top, |r| r.1);
    let w = (widest + CORRIDOR + FPAD)
        .max(frame_head_w(sc, p) + 2.0 * FPAD)
        .ceil();
    let h = bottom - y + FPAD;
    card.frames[fi] = (p, x, y, w, h);
    card.lines.push(Lines {
        frame: p,
        rows,
        corridor: x + widest,
    });
    (w, h)
}

fn badge_w(text: &str) -> f64 {
    mono_w(text, 11.0) + 12.0
}

fn tag_w(text: &str) -> f64 {
    text_w(text, 11.0, false) + 14.0
}

#[derive(Clone, Copy, PartialEq)]
enum ItemKind {
    Node(usize),
    Dummy(usize),
    Label(usize),
    Fan(usize),
    Trunk(usize),
}

struct Item {
    kind: ItemKind,
    layer: usize,
    w: f64,
    h: f64,
    x: f64,
    y: f64,
}

/// Edges between two cards on different rows, drawn together.
struct Bundle {
    upper: usize,
    lower: usize,
    links: Vec<usize>,
    channel: Option<usize>,
    label_layer: usize,
    items: Vec<usize>,
    /// For a channel bundle: which edges pass left of the card.
    left: Vec<usize>,
    right: Vec<usize>,
    /// The fan this single edge shares its label with.
    fan: Option<usize>,
}

/// Single edges with one label at one side of one card. They share one label beside the card
/// and one trunk from there through every row between: each edge joins the trunk on a track of
/// the label row next to its other card, so the edges draw as a comb, not as parallel lines.
struct Fan {
    card: usize,
    /// The node card of the item the edges meet at.
    member: Option<usize>,
    at_lower: bool,
    label: String,
    /// The label row next to the card, where the label sits.
    layer: usize,
    /// The shared item of each layer the trunk crosses: the label at `layer`, a trunk elsewhere.
    items: Vec<(usize, usize)>,
}

impl Fan {
    fn item(&self, layer: usize) -> Option<usize> {
        self.items
            .iter()
            .find(|(l, _)| *l == layer)
            .map(|(_, it)| *it)
    }
}

/// Track spacing of the combs in a label row.
const TRACK: f64 = 6.0;

/// Height of a lane below what an item holds.
const LANE: f64 = 26.0;

/// The stroke width of an edge that stands for two or three, and for four or more; one edge is
/// 1.4 wide (the stylesheet).
const WEIGHT_2: f64 = 2.4;
const WEIGHT_3: f64 = 3.6;

impl Bundle {
    fn label_size(&self, sc: &Scene) -> (f64, f64) {
        match self.channel {
            None => {
                let l = &sc.links[self.links[0]];
                (
                    text_w(&edge_label(sc, l, false), 11.5, false) + 18.0,
                    PILL_H,
                )
            }
            Some(c) => {
                let (wl, wr) = self.side_widths(sc);
                let (cw, ch) = channel_card_size(&sc.parts[c]);
                let nl = self.left.len() as f64;
                let nr = self.right.len() as f64;
                let w = wl + nl * LINE_GAP + 12.0 + cw + 12.0 + nr * LINE_GAP + wr;
                let rows = self.left.len().max(self.right.len()) as f64;
                (w, ch.max(rows * (PILL_H + 4.0)))
            }
        }
    }

    fn side_widths(&self, sc: &Scene) -> (f64, f64) {
        let side = |ls: &[usize]| {
            ls.iter()
                .map(|&l| text_w(&sc.links[l].label, 11.5, false) + 18.0 + 6.0)
                .fold(0.0, f64::max)
        };
        (side(&self.left), side(&self.right))
    }
}

fn channel_card_size(p: &Part) -> (f64, f64) {
    let w = 8.0 + 22.0 + 8.0 + text_w(&p.label, 13.0, true).max(text_w(&p.subtitle, 11.0, false));
    ((w + 12.0).ceil(), 44.0)
}

/// The port an edge end `node.port` addresses, by its own name.
fn port_of<'a>(sc: &Scene, end: &'a str) -> Option<&'a str> {
    let (head, own) = end.split_once('.')?;
    sc.part(head)?
        .ports
        .iter()
        .any(|p| p.address == end)
        .then_some(own)
}

fn edge_label(sc: &Scene, l: &Link, on_channel: bool) -> String {
    let base = match (&l.via, on_channel) {
        (Some(o), false) if sc.part(o).is_some() => format!("{} via {}", l.label, o),
        _ => l.label.clone(),
    };
    match port_of(sc, &l.to).or_else(|| port_of(sc, &l.from)) {
        Some(port) => format!("{base} · {port}"),
        None => base,
    }
}

/// Edges between two cards of one row.
struct Level {
    a: usize,
    b: usize,
    links: Vec<usize>,
}

pub(crate) struct Graph {
    pub svg: String,
    pub width: f64,
    pub height: f64,
}

fn pava(target: &[f64], weight: &[f64]) -> Vec<f64> {
    let mut blocks: Vec<(f64, f64, usize)> = Vec::new();
    for (i, &t) in target.iter().enumerate() {
        blocks.push((t * weight[i], weight[i], 1));
        while blocks.len() > 1 {
            let n = blocks.len();
            let (s1, w1, c1) = blocks[n - 2];
            let (s2, w2, c2) = blocks[n - 1];
            if s1 / w1 > s2 / w2 {
                blocks.truncate(n - 2);
                blocks.push((s1 + s2, w1 + w2, c1 + c2));
            } else {
                break;
            }
        }
    }
    let mut out = Vec::with_capacity(target.len());
    for (s, w, c) in blocks {
        out.extend(std::iter::repeat_n(s / w, c));
    }
    out
}

/// Where [`layered`] draws: the top left its layout starts from, and the page whose groups give
/// the colours (a band draws a part of a page in the colours of the whole).
#[derive(Default)]
struct Frame<'a> {
    origin: (f64, f64),
    palette: Option<&'a Page>,
}

/// A layout drawn without the `<svg>` around it.
struct Drawn {
    body: String,
    width: f64,
    height: f64,
    /// The highest y anything is drawn at.
    top: f64,
    /// Each node card, by part id.
    boxes: Vec<(String, [f64; 4])>,
    /// Each frame, by part id.
    frames: Vec<(String, [f64; 4])>,
}

/// The graph of a page, laid out as its `layout` says.
pub(crate) fn render(page: &Page, decor: &Decor) -> Graph {
    render_in(page, decor, page)
}

/// The graph of `page` in the colours of the groups of `whole`, the page it is a state of.
pub(crate) fn render_in(page: &Page, decor: &Decor, whole: &Page) -> Graph {
    match page.layout {
        page::Layout::Graph => {
            let d = layered(
                page,
                decor,
                &Frame {
                    origin: (0.0, 0.0),
                    palette: Some(whole),
                },
            );
            wrap(page, decor, &d.body, d.width, d.height)
        }
        page::Layout::Bands => {
            let (body, width, height) = bands(page, decor, whole);
            wrap(page, decor, &body, width, height)
        }
    }
}

/// The height of a band's head, the least gap between two bands, and the room of one line
/// beside the bands.
const BAND_HEAD: f64 = 26.0;
const BAND_GAP: f64 = 24.0;
const GUTTER: f64 = 14.0;

/// An edge between two bands: its edge, the bands it leaves and enters, the gap next to the band
/// it leaves, and, past a band between, the gap next to the band it enters and its line beside
/// the bands.
struct Crossing {
    link: usize,
    from: usize,
    to: usize,
    first: usize,
    second: Option<usize>,
    gutter: Option<usize>,
}

/// The least room between two runs that share a lane in a gap between bands.
const LANE_CLEAR: f64 = 16.0;

/// Which end the edges of a lane item share: none (one edge, or edges that run down the side of
/// the bands together), the card they leave or the card they enter.
#[derive(Clone, Copy, PartialEq)]
enum Shared {
    None,
    Start,
    Finish,
}

/// What takes one lane in a gap: one crossing, or a bundle of crossings with one label that
/// share an end (or run down the side of the bands together) and draw one label pill.
struct LaneItem {
    members: Vec<usize>,
    shared: Shared,
    /// The side of the bands the crossings of a bundle down the side leave and enter by.
    side: Option<bool>,
}

/// The crossings in lane items: crossings through one gap with one label out of one end, or
/// into one end, form a bundle, the largest first (out before into, then the earliest); the rest
/// are items of one. A crossing past a band between is always one. Items in the order of their
/// first crossing.
fn bundle(crossings: &[Crossing], page: &Page, labels: &[String]) -> Vec<LaneItem> {
    let n = crossings.len();
    let mut taken = vec![false; n];
    let mut items: Vec<LaneItem> = Vec::new();
    let end = |i: usize, shared: Shared| {
        let e = &page.edges[crossings[i].link];
        if shared == Shared::Start {
            e.from.as_str()
        } else {
            e.to.as_str()
        }
    };
    loop {
        let mut best: Option<(Shared, Vec<usize>)> = None;
        for k in 0..n {
            if taken[k] || crossings[k].gutter.is_some() {
                continue;
            }
            for shared in [Shared::Start, Shared::Finish] {
                let group: Vec<usize> = (k..n)
                    .filter(|&i| {
                        !taken[i]
                            && crossings[i].gutter.is_none()
                            && crossings[i].first == crossings[k].first
                            && end(i, shared) == end(k, shared)
                            && labels[i] == labels[k]
                    })
                    .collect();
                if group.len() >= 2 && best.as_ref().is_none_or(|b| group.len() > b.1.len()) {
                    best = Some((shared, group));
                }
            }
        }
        let Some((shared, members)) = best else {
            break;
        };
        for &m in &members {
            taken[m] = true;
        }
        items.push(LaneItem {
            members,
            shared,
            side: None,
        });
    }
    for (k, t) in taken.iter().enumerate() {
        if !t {
            items.push(LaneItem {
                members: vec![k],
                shared: Shared::None,
                side: None,
            });
        }
    }
    items.sort_by_key(|it| it.members[0]);
    items
}

/// Lanes for runs `(lo, hi)` in one gap: two runs share a lane when [`LANE_CLEAR`] lies between
/// them. Each run takes the first lane it fits in the order given; when sorting the runs by
/// their left end needs fewer lanes (it needs the fewest), they are placed in that order. Returns
/// the lane of each run and the number of lanes.
fn pack(runs: &[(f64, f64)]) -> (Vec<usize>, usize) {
    let fits = |lane: &[(f64, f64)], r: (f64, f64)| {
        lane.iter()
            .all(|o| o.1 + LANE_CLEAR <= r.0 || r.1 + LANE_CLEAR <= o.0)
    };
    let place = |order: &[usize]| {
        let mut lanes: Vec<Vec<(f64, f64)>> = Vec::new();
        let mut at = vec![0; runs.len()];
        for &i in order {
            match lanes.iter().position(|l| fits(l, runs[i])) {
                Some(l) => {
                    lanes[l].push(runs[i]);
                    at[i] = l;
                }
                None => {
                    at[i] = lanes.len();
                    lanes.push(vec![runs[i]]);
                }
            }
        }
        (at, lanes.len())
    };
    let given: Vec<usize> = (0..runs.len()).collect();
    let mut sorted = given.clone();
    sorted.sort_by(|&a, &b| runs[a].0.total_cmp(&runs[b].0).then(a.cmp(&b)));
    let first = place(&given);
    let least = place(&sorted);
    if least.1 < first.1 { least } else { first }
}

/// Where the cards and bands of a page in bands stand, for routing the crossings: each band's
/// box, and the boxes of its cards and frames by part id.
struct Placed {
    boxes: Vec<[f64; 4]>,
    cards: Vec<Vec<(String, [f64; 4])>>,
    frames: Vec<Vec<(String, [f64; 4])>>,
}

/// The way of one crossing through its bands, up to its gap and from there: its start on its
/// card, the points to its band's edge, the points from the other band's edge, its end.
struct Route {
    start: (f64, f64),
    out: Vec<(f64, f64)>,
    inn: Vec<(f64, f64)>,
    end: (f64, f64),
}

/// The way of each crossing through its bands where `p` places them. The ends at one side of
/// one card spread over its width in the order of where they go; the crossings of a bundle share
/// their shared end. An end with a card of its band in the way leaves by the side of the band
/// nearer to it (a crossing past a band between by the right, where the line beside the bands
/// runs), on a line of its own lane item.
fn routes(
    p: &Placed,
    items: &[LaneItem],
    crossings: &[Crossing],
    page: &Page,
    whole: &Scene,
    gutter_x: impl Fn(usize) -> f64,
) -> Vec<Route> {
    let centre = |b: [f64; 4]| b[0] + b[2] / 2.0;
    let mut item_of = vec![0; crossings.len()];
    for (i, it) in items.iter().enumerate() {
        for &k in &it.members {
            item_of[k] = i;
        }
    }
    // The crossing whose end stands for the end `start` of crossing `k`.
    let rep = |k: usize, start: bool| {
        let it = &items[item_of[k]];
        match (it.shared, start) {
            (Shared::Start, true) | (Shared::Finish, false) => it.members[0],
            _ => k,
        }
    };
    // The box an edge end lands on: its node's card, its region's frame, or its band.
    let end_box = |band: usize, end: &str| -> [f64; 4] {
        let id = whole.index(end).map(|q| whole.parts[q].id.as_str());
        id.and_then(|id| {
            p.cards[band]
                .iter()
                .chain(&p.frames[band])
                .find(|b| b.0 == id)
        })
        .map(|b| b.1)
        .unwrap_or(p.boxes[band])
    };
    // From a point on a card's side to the band's edge at `edge_y`: straight when no other card
    // of the band is in the way; else out to a row clear of cards, along it to the band's side
    // margin, where no card stands, and along the margin to the edge.
    let exit =
        |band: usize, own: [f64; 4], (x, y): (f64, f64), edge_y: f64, k: usize, right: bool| {
            let cards: Vec<[f64; 4]> = p.cards[band]
                .iter()
                .map(|b| b.1)
                .filter(|b| *b != own)
                .collect();
            let across = |b: &[f64; 4], x0: f64, x1: f64| x1 > b[0] && x0 < b[0] + b[2];
            let along = |b: &[f64; 4], y0: f64, y1: f64| y1 > b[1] && y0 < b[1] + b[3];
            let (lo, hi) = (y.min(edge_y), y.max(edge_y));
            if !cards.iter().any(|b| across(b, x, x) && along(b, lo, hi)) {
                return vec![(x, edge_y)];
            }
            let down = edge_y > y;
            let step = if down { 1.0 } else { -1.0 };
            let inset = 5.0 + 3.0 * (k % 4) as f64;
            let mx = if right {
                p.boxes[band][0] + p.boxes[band][2] - inset
            } else {
                p.boxes[band][0] + inset
            };
            let mut yd = y + step * (8.0 + 3.0 * (k % 3) as f64);
            for _ in 0..=cards.len() {
                let hit: Vec<&[f64; 4]> = cards
                    .iter()
                    .filter(|b| across(b, x.min(mx), x.max(mx)) && along(b, yd - 1.0, yd + 1.0))
                    .collect();
                if hit.is_empty() {
                    break;
                }
                yd = if down {
                    hit.iter().map(|b| b[1] + b[3]).fold(yd, f64::max) + 8.0
                } else {
                    hit.iter().map(|b| b[1]).fold(yd, f64::min) - 8.0
                };
            }
            if (down && yd >= edge_y) || (!down && yd <= edge_y) {
                return vec![(x, edge_y)];
            }
            vec![(x, yd), (mx, yd), (mx, edge_y)]
        };
    // crossing, start, box, bottom side, toward x
    let mut ends: Vec<(usize, bool, [f64; 4], bool, f64)> = Vec::new();
    for (k, c) in crossings.iter().enumerate() {
        let link = &page.edges[c.link];
        let (fb, tb) = (end_box(c.from, &link.from), end_box(c.to, &link.to));
        let down = c.from < c.to;
        let (tf, tt) = match c.gutter {
            Some(g) => (gutter_x(g), gutter_x(g)),
            None => (centre(tb), centre(fb)),
        };
        for (start, b, side, toward) in [(true, fb, down, tf), (false, tb, !down, tt)] {
            match ends
                .iter_mut()
                .find(|e| e.0 == rep(k, start) && e.1 == start)
            {
                // A shared end goes toward the middle of where its crossings go.
                Some(e) => e.4 += toward,
                None => ends.push((rep(k, start), start, b, side, toward)),
            }
        }
    }
    for e in &mut ends {
        let it = &items[item_of[e.0]];
        let shared = matches!(
            (it.shared, e.1),
            (Shared::Start, true) | (Shared::Finish, false)
        );
        if shared {
            e.4 /= it.members.len() as f64;
        }
    }
    let end_x = |k: usize, start: bool| -> f64 {
        let k = rep(k, start);
        let Some(&(_, _, b, side, _)) = ends.iter().find(|e| e.0 == k && e.1 == start) else {
            return 0.0;
        };
        let mut at: Vec<&(usize, bool, [f64; 4], bool, f64)> =
            ends.iter().filter(|e| e.2 == b && e.3 == side).collect();
        at.sort_by(|x, y| x.4.total_cmp(&y.4).then(x.0.cmp(&y.0)).then(y.1.cmp(&x.1)));
        let m = at.len() as f64;
        if m < 2.0 {
            return centre(b);
        }
        let j = at
            .iter()
            .position(|e| e.0 == k && e.1 == start)
            .unwrap_or(0) as f64;
        let span = (b[2] - 40.0).max(0.0).min(26.0 * (m - 1.0));
        centre(b) - span / 2.0 + span * j / (m - 1.0)
    };
    // Where each crossing starts and ends, and whether a card of its band is in the way of each.
    struct At {
        fb: [f64; 4],
        tb: [f64; 4],
        start: (f64, f64),
        end: (f64, f64),
        card_y: f64,
        down: bool,
        blocked: (bool, bool),
    }
    let blocked = |band: usize, own: [f64; 4], (x, y): (f64, f64), edge_y: f64| {
        let (lo, hi) = (y.min(edge_y), y.max(edge_y));
        p.cards[band]
            .iter()
            .any(|(_, b)| *b != own && x > b[0] && x < b[0] + b[2] && hi > b[1] && lo < b[1] + b[3])
    };
    let edge = |b: usize, bottom: bool| p.boxes[b][1] + if bottom { p.boxes[b][3] } else { 0.0 };
    let at: Vec<At> = crossings
        .iter()
        .enumerate()
        .map(|(k, c)| {
            let link = &page.edges[c.link];
            let (fb, tb) = (end_box(c.from, &link.from), end_box(c.to, &link.to));
            let down = c.from < c.to;
            let (sx, ex) = (end_x(k, true), end_x(k, false));
            let (sy, ey) = if down {
                (fb[1] + fb[3], tb[1] - 1.0)
            } else {
                (fb[1], tb[1] + tb[3] + 1.0)
            };
            let card_y = if down { tb[1] } else { tb[1] + tb[3] };
            At {
                fb,
                tb,
                start: (sx, sy),
                end: (ex, ey),
                card_y,
                down,
                blocked: (
                    blocked(c.from, fb, (sx, sy), edge(c.from, down)),
                    blocked(c.to, tb, (ex, card_y), edge(c.to, !down)),
                ),
            }
        })
        .collect();
    // The side an end in the way leaves by: the one nearer to where its crossing goes in the
    // other band; when the other end leaves by a side too, both take the side nearer to the
    // middle of the two, so the crossing runs straight down it. A shared end takes the side
    // nearer to the middle of where its crossings go. Past a band between, the right.
    let toward = |k: usize, start: bool| {
        let a = &at[k];
        let other = if start { a.blocked.1 } else { a.blocked.0 };
        match (other, start) {
            (true, _) => (a.start.0 + a.end.0) / 2.0,
            (false, true) => a.end.0,
            (false, false) => a.start.0,
        }
    };
    let right_of = |k: usize, start: bool, x: f64| {
        let c = &crossings[k];
        c.gutter.is_some() || x >= centre(p.boxes[if start { c.from } else { c.to }])
    };
    let shared_side = |k: usize, start: bool| -> Option<bool> {
        let it = &items[item_of[k]];
        let shared = matches!(
            (it.shared, start),
            (Shared::Start, true) | (Shared::Finish, false)
        );
        shared.then(|| {
            let mean =
                it.members.iter().map(|&m| toward(m, start)).sum::<f64>() / it.members.len() as f64;
            right_of(k, start, mean)
        })
    };
    crossings
        .iter()
        .enumerate()
        .map(|(k, c)| {
            let a = &at[k];
            let fixed = items[item_of[k]].side;
            let (own_out, own_in) = (
                fixed.or_else(|| shared_side(k, true)),
                fixed.or_else(|| shared_side(k, false)),
            );
            let both = a.blocked.0 && a.blocked.1;
            let side_out = own_out
                .or(if both { own_in } else { None })
                .unwrap_or_else(|| right_of(k, true, toward(k, true)));
            let side_in = own_in
                .or(if both { Some(side_out) } else { None })
                .unwrap_or_else(|| right_of(k, false, toward(k, false)));
            let o = item_of[k];
            let out = exit(c.from, a.fb, a.start, edge(c.from, a.down), o, side_out);
            let mut inn = exit(
                c.to,
                a.tb,
                (a.end.0, a.card_y),
                edge(c.to, !a.down),
                o,
                side_in,
            );
            inn.reverse();
            Route {
                start: a.start,
                out,
                inn,
                end: a.end,
            }
        })
        .collect()
}

/// The graph of a page in bands, one per top-level region in the order of the list and a last
/// one for the nodes in none: what a band holds is laid out by [`layered`] as a page of its own,
/// and an edge between two bands runs through the gaps. Returns the body, width and height.
fn bands(page: &Page, decor: &Decor, colours_of: &Page) -> (String, f64, f64) {
    let whole = Scene::read(
        page,
        decor,
        &Frame {
            origin: (0.0, 0.0),
            palette: Some(colours_of),
        },
    );
    let tops = &page.regions;
    let loose = tops.len();
    // The band of each region: the top-level region it lies in.
    fn walk<'a>(r: &'a Region, b: usize, out: &mut Vec<(&'a str, usize)>) {
        out.push((&r.id, b));
        for c in &r.regions {
            walk(c, b, out);
        }
    }
    let mut region_band: Vec<(&str, usize)> = Vec::new();
    for (b, r) in tops.iter().enumerate() {
        walk(r, b, &mut region_band);
    }
    let in_region = |r: &str| region_band.iter().find(|x| x.0 == r).map(|x| x.1);
    // A node whose id is a top-level region's is that band's own card.
    let node_band: Vec<usize> = page
        .nodes
        .iter()
        .map(|n| {
            tops.iter()
                .position(|r| r.id == n.id)
                .or_else(|| in_region(n.region.as_deref()?))
                .unwrap_or(loose)
        })
        .collect();
    let band_of = |id: &str| -> Option<usize> {
        match page.nodes.iter().position(|n| n.id == id) {
            Some(i) => Some(node_band[i]),
            None => in_region(id),
        }
    };
    let end_band = |end: &str| whole.index(end).and_then(|p| band_of(&whole.parts[p].id));
    let mut shown: Vec<usize> = (0..loose).collect();
    if node_band.contains(&loose) || shown.is_empty() {
        shown.push(loose);
    }

    // Edges within one band, and edges between two.
    let mut inner: Vec<Vec<usize>> = vec![Vec::new(); loose + 1];
    let mut crossings: Vec<Crossing> = Vec::new();
    let gaps_n = shown.len().saturating_sub(1);
    let mut gutters = 0;
    for (li, e) in page.edges.iter().enumerate() {
        let (Some(a), Some(b)) = (end_band(&e.from), end_band(&e.to)) else {
            continue;
        };
        if a == b {
            inner[a].push(li);
            continue;
        }
        let (g1, g2) = if a < b { (a, b - 1) } else { (a - 1, b) };
        let gutter = (g2 != g1).then(|| {
            gutters += 1;
            gutters - 1
        });
        crossings.push(Crossing {
            link: li,
            from: a,
            to: b,
            first: g1,
            second: gutter.map(|_| g2),
            gutter,
        });
    }
    let labels: Vec<String> = crossings
        .iter()
        .map(|c| edge_label(&whole, &whole.links[c.link], false))
        .collect();
    let mut items = bundle(&crossings, page, &labels);

    // Each band as a page of its own, laid out once to measure it.
    let pages: Vec<(Page, Decor)> = shown
        .iter()
        .map(|&b| {
            let top = tops.get(b);
            let mut sub = Page {
                format: page.format.clone(),
                title: page.title.clone(),
                regions: top.map(|r| r.regions.clone()).unwrap_or_default(),
                groups: page.groups.clone(),
                ..Page::default()
            };
            for (n, _) in page.nodes.iter().zip(&node_band).filter(|x| *x.1 == b) {
                let mut n = n.clone();
                if top.is_none_or(|r| r.id == n.id || n.region.as_deref() == Some(r.id.as_str())) {
                    n.region = None;
                }
                sub.nodes.push(n);
            }
            let ids: Vec<String> = sub.nodes.iter().map(|n| n.id.clone()).collect();
            let held = |id: &str| ids.iter().any(|n| n == id);
            sub.ports = page
                .ports
                .iter()
                .filter(|p| held(&p.node))
                .cloned()
                .collect();
            // A mark on a region inside the band is drawn on that region's frame in it.
            let inside = |id: &str| {
                top.is_some_and(|t| t.id != id)
                    && region_band.iter().any(|&(r, rb)| r == id && rb == b)
            };
            let marks: Vec<usize> = (0..page.marks.len())
                .filter(|&i| {
                    let t = &page.marks[i].target;
                    held(&t.node) || (t.port.is_none() && inside(&t.node))
                })
                .collect();
            let sd = Decor {
                words: Words::default(),
                groups: decor.groups.clone(),
                nodes: decor
                    .nodes
                    .iter()
                    .filter(|(id, _)| held(id))
                    .cloned()
                    .collect(),
                vias: inner[b]
                    .iter()
                    .map(|&i| decor.vias.get(i).cloned().flatten())
                    .collect(),
                chips: marks
                    .iter()
                    .map(|&i| decor.chips.get(i).cloned().flatten())
                    .collect(),
                closable: decor.closable.clone(),
            };
            sub.marks = marks.iter().map(|&i| page.marks[i].clone()).collect();
            sub.edges = inner[b].iter().map(|&i| page.edges[i].clone()).collect();
            (sub, sd)
        })
        .collect();
    let colours = |origin: (f64, f64)| Frame {
        origin,
        palette: Some(colours_of),
    };
    let measured: Vec<Drawn> = pages
        .iter()
        .map(|(sub, sd)| layered(sub, sd, &colours((0.0, 0.0))))
        .collect();
    let heads: Vec<Option<(String, String, bool)>> = shown
        .iter()
        .map(|&b| {
            tops.get(b).map(|r| {
                let heading = match (&r.per, &r.r#in) {
                    (Some(p), _) => format!("per {p}"),
                    (None, Some(i)) => format!("in {i}"),
                    (None, None) => format!("in {}", r.label),
                };
                (r.label.clone(), heading, r.per.is_some())
            })
        })
        .collect();
    // The marks on a band's region, when it is no node: chips on its label line, and whether one
    // is a refusal, which rings the band. Those on a region that is a node are on its card.
    let band_marks: Vec<(Vec<Chip>, bool)> = shown
        .iter()
        .map(|&b| {
            let Some(r) = tops
                .get(b)
                .filter(|r| !page.nodes.iter().any(|n| n.id == r.id))
            else {
                return (Vec::new(), false);
            };
            let on: Vec<usize> = (0..page.marks.len())
                .filter(|&i| {
                    page.marks[i].target.node == r.id && page.marks[i].target.port.is_none()
                })
                .collect();
            let chips = on
                .iter()
                .map(|&i| {
                    decor
                        .chips
                        .get(i)
                        .and_then(|c| c.clone())
                        .unwrap_or_else(|| Chip::of(&page.marks[i]))
                })
                .collect();
            let refused = on
                .iter()
                .any(|&i| page.marks[i].level == page::Level::Refusal);
            (chips, refused)
        })
        .collect();
    let band_w = measured
        .iter()
        .zip(&heads)
        .zip(&shown)
        .zip(&band_marks)
        .map(|(((d, h), &b), (chips, _))| {
            let close = tops
                .get(b)
                .filter(|r| decor.closable.contains(&r.id))
                .map_or(0.0, |_| close_w() + 8.0);
            let head = h.as_ref().map_or(0.0, |(l, s, _)| {
                text_w(l, 12.0, true)
                    + 8.0
                    + text_w(s, 11.0, false)
                    + chips_w(chips)
                    + 2.0 * FPAD
                    + close
            });
            d.width.max(head)
        })
        .fold(0.0, f64::max)
        .ceil();
    let width = MARGIN
        + band_w
        + if gutters > 0 {
            GUTTER * (gutters as f64 + 1.0)
        } else {
            0.0
        }
        + MARGIN;
    let gutter_x = |k: usize| MARGIN + band_w + GUTTER * (k as f64 + 1.0);
    let centre = |b: [f64; 4]| b[0] + b[2] / 2.0;
    // Each band's height and the offset of what it holds from the band's top left corner.
    let frames_of: Vec<(f64, f64, f64)> = measured
        .iter()
        .map(|d| {
            let lift = 8.0f64.max(BAND_HEAD - d.top).ceil();
            let ox = MARGIN + ((band_w - d.width) / 2.0).floor();
            (lift + d.height - 12.0, ox, lift)
        })
        .collect();

    // The crossings are routed first with every band at height 0: where a crossing meets a gap
    // depends on its two bands only, not on how far apart they stand, so the lanes of each gap,
    // and with them its height, are known before the bands are placed.
    let shift = |v: &[(String, [f64; 4])], o: (f64, f64)| -> Vec<(String, [f64; 4])> {
        v.iter()
            .map(|(id, b)| (id.clone(), [b[0] + o.0, b[1] + o.1, b[2], b[3]]))
            .collect()
    };
    let local = Placed {
        boxes: frames_of
            .iter()
            .map(|f| [MARGIN, 0.0, band_w, f.0])
            .collect(),
        cards: measured
            .iter()
            .zip(&frames_of)
            .map(|(d, f)| shift(&d.boxes, (f.1, f.2)))
            .collect(),
        frames: measured
            .iter()
            .zip(&frames_of)
            .map(|(d, f)| shift(&d.frames, (f.1, f.2)))
            .collect(),
    };
    let route =
        |p: &Placed, items: &[LaneItem]| routes(p, items, &crossings, page, &whole, gutter_x);
    let mut ways = route(&local, &items);
    // Crossings that run down the side of both bands of a gap go straight through it on the
    // side: those with one label on one side of one gap are one bundle with one label.
    let straight = |k: usize, w: &[Route]| {
        crossings[k].gutter.is_none()
            && w[k].out.len() > 1
            && w[k].inn.len() > 1
            && (w[k].out[w[k].out.len() - 1].0 - w[k].inn[0].0).abs() < 0.5
    };
    let mut buses: Vec<((usize, bool, &str), usize)> = Vec::new();
    for i in 0..items.len() {
        if !items[i].members.iter().all(|&k| straight(k, &ways)) {
            continue;
        }
        let k = items[i].members[0];
        let c = &crossings[k];
        let right = ways[k].inn[0].0 >= centre(local.boxes[c.to]);
        let key = (c.first, right, labels[k].as_str());
        match buses.iter().find(|b| b.0 == key) {
            Some(&(_, j)) => {
                let moved = std::mem::take(&mut items[i].members);
                items[j].members.extend(moved);
                items[j].members.sort_unstable();
                items[j].shared = Shared::None;
                items[j].side = Some(right);
            }
            None => buses.push((key, i)),
        }
    }
    if items.iter().any(|it| it.members.is_empty()) {
        items.retain(|it| !it.members.is_empty());
        items.sort_by_key(|it| it.members[0]);
        ways = route(&local, &items);
    }

    // The run of each item in its gap: from the leftmost to the rightmost point its crossings
    // reach there, with the centre of its label pill and the pill's half width.
    let run = |it: &LaneItem, w: &[Route]| -> (f64, f64, f64, f64) {
        let c = &crossings[it.members[0]];
        let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
        for &k in &it.members {
            let x1 = w[k].out[w[k].out.len() - 1].0;
            let turn = c.gutter.map_or(w[k].inn[0].0, gutter_x);
            lo = lo.min(x1.min(turn));
            hi = hi.max(x1.max(turn));
        }
        let half = (text_w(&labels[it.members[0]], 11.5, false) + 18.0) / 2.0;
        let mid = (lo + hi) / 2.0;
        let (a, b) = (half + 4.0, width - half - 4.0);
        let cx = if a <= b { mid.clamp(a, b) } else { mid };
        (lo, hi, cx, half)
    };
    // Lanes: the runs of the items in each gap, packed; past a band between, the run from the
    // line beside the bands to the card takes a lane of the gap next to the band it enters.
    let mut lane_of: Vec<(usize, usize)> = vec![(0, 0); items.len()];
    let mut tracks = vec![0usize; gaps_n];
    for (g, track) in tracks.iter_mut().enumerate() {
        let mut runs: Vec<(usize, bool, (f64, f64))> = Vec::new();
        for (i, it) in items.iter().enumerate() {
            let c = &crossings[it.members[0]];
            if c.first == g {
                let (lo, hi, cx, half) = run(it, &ways);
                runs.push((i, false, (lo.min(cx - half), hi.max(cx + half))));
            }
            if c.second == Some(g) {
                let k = it.members[0];
                let (x2, gx) = (ways[k].inn[0].0, c.gutter.map_or(0.0, gutter_x));
                runs.push((i, true, (x2.min(gx), x2.max(gx))));
            }
        }
        let spans: Vec<(f64, f64)> = runs.iter().map(|r| r.2).collect();
        let (at, n) = pack(&spans);
        for (r, l) in runs.iter().zip(at) {
            if r.1 {
                lane_of[r.0].1 = l;
            } else {
                lane_of[r.0].0 = l;
            }
        }
        *track = n;
    }
    let gap_h: Vec<f64> = tracks
        .iter()
        .map(|&n| BAND_GAP.max(n as f64 * LANE + 12.0))
        .collect();

    // Top to bottom: each band's box and the origin of what it holds.
    let mut boxes: Vec<[f64; 4]> = Vec::new();
    let mut origins: Vec<(f64, f64)> = Vec::new();
    let mut y = MARGIN;
    for (k, &(h, ox, lift)) in frames_of.iter().enumerate() {
        boxes.push([MARGIN, y, band_w, h]);
        origins.push((ox, y + lift));
        y += h;
        if let Some(g) = gap_h.get(k) {
            y += g;
        }
    }
    let height = (y + MARGIN).ceil();
    let drawn: Vec<Drawn> = pages
        .iter()
        .zip(&origins)
        .map(|((sub, sd), &o)| layered(sub, sd, &colours(o)))
        .collect();
    let placed = Placed {
        boxes: boxes.clone(),
        cards: drawn.iter().map(|d| d.boxes.clone()).collect(),
        frames: drawn.iter().map(|d| d.frames.clone()).collect(),
    };
    let ways = route(&placed, &items);
    let track_y = |g: usize, t: usize| {
        let b = boxes[g];
        b[1] + b[3] + 6.0 + LANE * (t as f64 + 0.5)
    };

    let mut svg = String::new();
    svg.push_str("<g class=\"bands\">");
    for (((&b, head), bx), (chips, refused)) in
        shown.iter().zip(&heads).zip(&boxes).zip(&band_marks)
    {
        let id = tops.get(b).map_or("", |r| r.id.as_str());
        let per = head.as_ref().is_some_and(|h| h.2);
        let _ = write!(
            svg,
            "<g class=\"region band{}{} f-other\" data-band=\"{}\">",
            if per { " per" } else { "" },
            if *refused { " refused" } else { "" },
            esc(id),
        );
        if *refused {
            frame_ring(&mut svg, bx[0], bx[1], bx[2], bx[3]);
        }
        let _ = write!(
            svg,
            "<rect class=\"region-bg\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"12\" data-band-box=\"{} {} {} {}\"/>",
            px(bx[0]),
            px(bx[1]),
            px(bx[2]),
            px(bx[3]),
            px(bx[0]),
            px(bx[1]),
            px(bx[2]),
            px(bx[3])
        );
        if let Some((label, heading, _)) = head {
            let tx = bx[0] + FPAD;
            let _ = write!(
                svg,
                "<text class=\"region-name\" x=\"{}\" y=\"{}\">{}</text><text class=\"region-sub\" x=\"{}\" y=\"{}\">{}</text>",
                px(tx),
                px(bx[1] + 17.0),
                esc(label),
                px(tx + text_w(label, 12.0, true) + 8.0),
                px(bx[1] + 17.0),
                esc(heading)
            );
            line_chips(
                &mut svg,
                chips,
                tx + text_w(label, 12.0, true) + 8.0 + text_w(heading, 11.0, false),
                bx[1] + 3.0,
            );
            if decor.closable.iter().any(|c| c == id) {
                close_control(&mut svg, id, label, bx[0] + bx[2] - FPAD, bx[1] + 5.0);
            }
        }
        svg.push_str("</g>");
    }
    svg.push_str("</g><g class=\"edges\">");
    let mut item_of = vec![0; crossings.len()];
    for (i, it) in items.iter().enumerate() {
        for &k in &it.members {
            item_of[k] = i;
        }
    }
    for (k, c) in crossings.iter().enumerate() {
        let link = &whole.links[c.link];
        let w = &ways[k];
        let lanes = lane_of[item_of[k]];
        let (x1, x2) = (w.out[w.out.len() - 1].0, w.inn[0].0);
        let y1 = track_y(c.first, lanes.0);
        let mut pts = vec![w.start];
        pts.extend(w.out.iter().copied());
        pts.push((x1, y1));
        match (c.second, c.gutter) {
            (Some(second), Some(g)) => {
                let (gx, y2) = (gutter_x(g), track_y(second, lanes.1));
                pts.extend([(gx, y1), (gx, y2), (x2, y2)]);
            }
            _ => pts.push((x2, y1)),
        }
        pts.extend(w.inn.iter().copied());
        pts.push(w.end);
        pts.dedup();
        // A corner that does not turn is no corner.
        let mut i = 1;
        while i + 1 < pts.len() {
            let (a, b, c) = (pts[i - 1], pts[i], pts[i + 1]);
            if (a.0 == b.0 && b.0 == c.0) || (a.1 == b.1 && b.1 == c.1) {
                pts.remove(i);
            } else {
                i += 1;
            }
        }
        let mut d = format!("M{} {}", px(pts[0].0), px(pts[0].1));
        for p in &pts[1..] {
            let _ = write!(d, "L{} {}", px(p.0), px(p.1));
        }
        edge_path(&mut svg, &d, link);
    }
    svg.push_str("</g><g class=\"labels\">");
    for (it, lanes) in items.iter().zip(&lane_of) {
        let (_, _, cx, _) = run(it, &ways);
        let y1 = track_y(crossings[it.members[0]].first, lanes.0);
        pill(&mut svg, cx, y1, &labels[it.members[0]], None);
    }
    svg.push_str("</g>");
    for d in &drawn {
        svg.push_str(&d.body);
    }
    (svg, width, height)
}

/// Where the cards of rows wider than [`page::WRAP_WIDTH`] go, by card: the line of its row and
/// its place in the order the row had in one line; and the width the lines were cut to.
struct Wrap {
    at: Vec<Option<(usize, usize)>>,
    target: f64,
}

/// A page laid out in rows. A row whose cards with their gaps would be wider than
/// [`page::WRAP_WIDTH`] wraps: the layout is made again with its cards in as many rows as keep
/// each within it, in the order the one row gave them, read row by row and kept there; edges to
/// and between the new rows are routed as between any two rows. When the edges that pass between
/// the new rows still make the layout wider, its rows are cut narrower and it is made again, at
/// most [`WRAP_PASSES`] times.
fn layered(page: &Page, decor: &Decor, frame: &Frame) -> Drawn {
    let mut wrap: Option<Wrap> = None;
    for pass in 0..WRAP_PASSES {
        match lay(page, decor, frame, wrap.as_ref(), pass + 1 == WRAP_PASSES) {
            Ok(d) => return d,
            Err(w) => wrap = Some(w),
        }
    }
    unreachable!("the last pass lays out as it is")
}

/// The least gap between two cards of a row, as [`page::WRAP_WIDTH`] counts it, and how often a
/// layout is made at most.
const CARD_GAP: f64 = 40.0;
const WRAP_PASSES: usize = 6;

fn lay(
    page: &Page,
    decor: &Decor,
    frame: &Frame,
    wrap: Option<&Wrap>,
    last: bool,
) -> Result<Drawn, Wrap> {
    let scene = Scene::read(page, decor, frame);
    let sc = &scene;
    let n = sc.parts.len();
    // An edge end `<node>.<port>` lands on the card of the node with that port.
    let index = |name: &str| sc.index(name);
    let endpoints: Vec<&str> = sc
        .links
        .iter()
        .flat_map(|l| [l.from.as_str(), l.to.as_str()])
        .collect();

    // A node in a region is drawn inside the frame of the outermost region it lies in; that frame
    // is one item of its row.
    let host: Vec<Option<usize>> = (0..n)
        .map(|i| Some(sc.top(i)).filter(|&t| t != i))
        .collect();
    let row_of = |i: usize| sc.parts[i].row;
    let resolve = |name: &str| index(name).map(|i| host[i].unwrap_or(i));

    // A node that is only ever passed through by edges between two rows is drawn on those edges.
    let mut on_edge = vec![false; n];
    for (i, p) in sc.parts.iter().enumerate() {
        if host[i].is_some()
            || p.region.is_some()
            || sc.framed(i)
            || endpoints.contains(&p.id.as_str())
        {
            continue;
        }
        on_edge[i] = sc.links.iter().any(|l| {
            l.via.as_deref() == Some(p.id.as_str())
                && match (resolve(&l.from), resolve(&l.to)) {
                    (Some(a), Some(b)) => row_of(a) != row_of(b),
                    _ => false,
                }
        });
    }

    // Cards in order of first appearance in the edges, then in the order of the page.
    let mut order: Vec<usize> = Vec::new();
    for name in &endpoints {
        if let Some(i) = resolve(name)
            && !order.contains(&i)
        {
            order.push(i);
        }
    }
    for i in 0..n {
        if host[i].is_none() && !on_edge[i] && !order.contains(&i) {
            order.push(i);
        }
    }
    let mut cards: Vec<Card> = order.iter().map(|&i| Card::new(sc, i)).collect();
    let card_of = |part: usize| order.iter().position(|&c| c == part);

    // An edge between two node cards of one item that is not drawn across a gap (they are not
    // side by side, or one of them is in a frame inside the item) is drawn below what the item
    // holds, on a lane of its own, the shortest lane on top. The item grows by its lanes.
    // card, link, lane centre from the item's top
    let mut lanes: Vec<(usize, usize, f64)> = Vec::new();
    for (ci, card) in cards.iter_mut().enumerate() {
        if card.single() {
            continue;
        }
        let mut own: Vec<(f64, usize)> = Vec::new();
        for (li, l) in sc.links.iter().enumerate() {
            let (Some(pa), Some(pb)) = (index(&l.from), index(&l.to)) else {
                continue;
            };
            if pa == pb
                || resolve(&l.from).and_then(card_of) != Some(ci)
                || resolve(&l.to).and_then(card_of) != Some(ci)
                || card
                    .gaps
                    .iter()
                    .any(|g| (g.0 == pa && g.1 == pb) || (g.0 == pb && g.1 == pa))
            {
                continue;
            }
            let (Some(a), Some(b)) = (card.node(pa), card.node(pb)) else {
                continue;
            };
            own.push((((a.0 + a.2 / 2.0) - (b.0 + b.2 / 2.0)).abs(), li));
        }
        if own.is_empty() {
            continue;
        }
        own.sort_by(|x, y| x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)));
        let widest = own
            .iter()
            .map(|&(_, li)| text_w(&edge_label(sc, &sc.links[li], false), 11.5, false) + 18.0)
            .fold(0.0, f64::max);
        let base = card.h - FPAD + 6.0;
        card.w = card.w.max((widest + 2.0 * FPAD).ceil());
        card.h += 6.0 + own.len() as f64 * LANE;
        if let Some(f) = card.frames.first_mut() {
            f.3 = card.w;
            f.4 = card.h;
        }
        for (k, &(_, li)) in own.iter().enumerate() {
            lanes.push((ci, li, base + LANE * (k as f64 + 0.5)));
        }
    }

    // A card that two or more cards of its own row send to moves to a row of its own just
    // below: its links then cross rows, where they share a label, instead of arching over the
    // cards between.
    let ends: Vec<(usize, usize)> = sc
        .links
        .iter()
        .filter_map(|l| Some((card_of(resolve(&l.from)?)?, card_of(resolve(&l.to)?)?)))
        .filter(|(a, b)| a != b && cards[*a].row == cards[*b].row)
        .collect();
    let sinks: Vec<bool> = (0..cards.len())
        .map(|c| {
            let mut from: Vec<usize> = ends.iter().filter(|e| e.1 == c).map(|e| e.0).collect();
            from.sort_unstable();
            from.dedup();
            from.len() >= 2
        })
        .collect();
    for (c, card) in cards.iter_mut().enumerate() {
        card.row = 2 * card.row + usize::from(sinks[c]);
    }
    // The rows of a row that wraps follow one another; each card keeps its place in the order.
    let base_row: Vec<usize> = cards.iter().map(|c| c.row).collect();
    let stride = cards.len() + 1;
    let mut pin: Vec<Option<usize>> = vec![None; cards.len()];
    if let Some(w) = wrap {
        for (c, card) in cards.iter_mut().enumerate() {
            let (line, place) = w.at.get(c).copied().flatten().unwrap_or((0, 0));
            card.row = card.row * stride + line;
            pin[c] = w.at.get(c).copied().flatten().map(|_| place);
        }
    }

    let mut rows: Vec<usize> = cards.iter().map(|c| c.row).collect();
    rows.sort_unstable();
    rows.dedup();
    let layer_of_card: Vec<usize> = cards
        .iter()
        .map(|c| 2 * rows.iter().position(|&r| r == c.row).unwrap_or(0))
        .collect();
    let layers_n = if rows.is_empty() {
        1
    } else {
        2 * rows.len() - 1
    };

    // Bundles across rows, levels within a row.
    let mut bundles: Vec<Bundle> = Vec::new();
    let mut levels: Vec<Level> = Vec::new();
    for (li, l) in sc.links.iter().enumerate() {
        let (Some(a), Some(b)) = (resolve(&l.from), resolve(&l.to)) else {
            continue;
        };
        let (Some(ca), Some(cb)) = (card_of(a), card_of(b)) else {
            continue;
        };
        if ca == cb {
            continue;
        }
        if layer_of_card[ca] == layer_of_card[cb] {
            let (x, y) = (ca.min(cb), ca.max(cb));
            match levels.iter_mut().find(|v| v.a == x && v.b == y) {
                Some(v) => v.links.push(li),
                None => levels.push(Level {
                    a: x,
                    b: y,
                    links: vec![li],
                }),
            }
            continue;
        }
        let (upper, lower) = if layer_of_card[ca] < layer_of_card[cb] {
            (ca, cb)
        } else {
            (cb, ca)
        };
        let channel = l.via.as_deref().and_then(index).filter(|&c| on_edge[c]);
        let existing = channel.and_then(|c| {
            bundles
                .iter()
                .position(|bd| bd.upper == upper && bd.lower == lower && bd.channel == Some(c))
        });
        match existing {
            Some(bi) => bundles[bi].links.push(li),
            None => {
                let (lu, ll) = (layer_of_card[upper], layer_of_card[lower]);
                let mid = (lu + ll) / 2;
                let label_layer = if mid % 2 == 1 { mid } else { mid + 1 };
                bundles.push(Bundle {
                    upper,
                    lower,
                    links: vec![li],
                    channel,
                    label_layer,
                    items: Vec::new(),
                    left: Vec::new(),
                    right: Vec::new(),
                    fan: None,
                });
            }
        }
    }

    // Fans: two or more single links with one label that end at, or start from, one side of one
    // card share one label beside that card, and their lines meet there.
    let mut fans: Vec<Fan> = Vec::new();
    for at_end in [true, false] {
        let mut keys: Vec<(usize, bool, String, usize, Option<usize>)> = Vec::new();
        for (bi, bd) in bundles.iter().enumerate() {
            if bd.channel.is_some() || bd.fan.is_some() {
                continue;
            }
            let l = &sc.links[bd.links[0]];
            let end = if at_end { &l.to } else { &l.from };
            let Some(c) = resolve(end).and_then(card_of) else {
                continue;
            };
            keys.push((c, c == bd.lower, edge_label(sc, l, false), bi, index(end)));
        }
        for &(c, at_lower, ref label, _, member) in &keys {
            if fans.iter().any(|f| {
                f.card == c && f.member == member && f.at_lower == at_lower && f.label == *label
            }) {
                continue;
            }
            let members: Vec<usize> = keys
                .iter()
                .filter(|k| k.0 == c && k.4 == member && k.1 == at_lower && k.2 == *label)
                .map(|k| k.3)
                .filter(|&bi| bundles[bi].fan.is_none())
                .collect();
            if members.len() < 2 {
                continue;
            }
            let layer = if at_lower {
                layer_of_card[c] - 1
            } else {
                layer_of_card[c] + 1
            };
            for &bi in &members {
                bundles[bi].fan = Some(fans.len());
            }
            fans.push(Fan {
                card: c,
                member,
                at_lower,
                label: label.clone(),
                layer,
                items: Vec::new(),
            });
        }
    }
    for bd in &mut bundles {
        if bd.channel.is_some() {
            for (k, &l) in bd.links.iter().enumerate() {
                if k % 2 == 0 {
                    bd.left.push(l);
                } else {
                    bd.right.push(l);
                }
            }
        }
    }

    // Items: cards, then the label or gap of each bundle in each layer it crosses.
    let mut items: Vec<Item> = Vec::new();
    let mut layers: Vec<Vec<usize>> = vec![Vec::new(); layers_n];
    for (ci, c) in cards.iter().enumerate() {
        layers[layer_of_card[ci]].push(items.len());
        items.push(Item {
            kind: ItemKind::Node(ci),
            layer: layer_of_card[ci],
            w: c.w,
            h: c.h,
            x: 0.0,
            y: 0.0,
        });
    }
    for (bi, bd) in bundles.iter_mut().enumerate() {
        let (lu, ll) = (layer_of_card[bd.upper], layer_of_card[bd.lower]);
        for (layer, members) in layers.iter_mut().enumerate().take(ll).skip(lu + 1) {
            if let Some(f) = bd.fan {
                let it = match fans[f].item(layer) {
                    Some(it) => it,
                    None => {
                        let it = items.len();
                        fans[f].items.push((layer, it));
                        members.push(it);
                        let (kind, w, h) = if layer == fans[f].layer {
                            let w = text_w(&fans[f].label, 11.5, false) + 18.0 + 8.0;
                            (ItemKind::Fan(f), w, PILL_H)
                        } else {
                            (ItemKind::Trunk(f), LINE_GAP + 4.0, 0.0)
                        };
                        items.push(Item {
                            kind,
                            layer,
                            w,
                            h,
                            x: 0.0,
                            y: 0.0,
                        });
                        it
                    }
                };
                bd.items.push(it);
                continue;
            }
            let (kind, w, h) = if layer == bd.label_layer && bd.fan.is_none() {
                let (w, h) = bd.label_size(sc);
                (ItemKind::Label(bi), w, h)
            } else {
                (
                    ItemKind::Dummy(bi),
                    bd.links.len() as f64 * LINE_GAP + 4.0,
                    0.0,
                )
            };
            bd.items.push(items.len());
            members.push(items.len());
            items.push(Item {
                kind,
                layer,
                w,
                h,
                x: 0.0,
                y: 0.0,
            });
        }
    }
    // Neighbours between consecutive layers.
    let mut up: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    let mut down: Vec<Vec<usize>> = vec![Vec::new(); items.len()];
    for bd in &bundles {
        let mut chain = vec![bd.upper];
        chain.extend(bd.items.iter().copied());
        chain.push(bd.lower);
        for w in chain.windows(2) {
            down[w[0]].push(w[1]);
            up[w[1]].push(w[0]);
        }
    }
    // Cards joined within a row move as one group.
    let mut group: Vec<usize> = (0..items.len()).collect();
    fn find(g: &mut [usize], i: usize) -> usize {
        let mut r = i;
        while g[r] != r {
            r = g[r];
        }
        g[i] = r;
        r
    }
    for v in &levels {
        let (ra, rb) = (find(&mut group, v.a), find(&mut group, v.b));
        if ra != rb {
            group[ra.max(rb)] = ra.min(rb);
        }
    }
    let groups: Vec<usize> = (0..items.len()).map(|i| find(&mut group, i)).collect();
    // The cards of a wrapped row keep their order: a sweep moves what lies between them only.
    let pinned = |layer: &mut Vec<usize>| {
        let at: Vec<usize> = (0..layer.len())
            .filter(|&k| pin.get(layer[k]).copied().flatten().is_some())
            .collect();
        let mut held: Vec<usize> = at.iter().map(|&k| layer[k]).collect();
        held.sort_by_key(|&it| pin[it]);
        for (&k, it) in at.iter().zip(held) {
            layer[k] = it;
        }
    };
    let has_pin = |layer: &[usize]| {
        layer
            .iter()
            .any(|&it| pin.get(it).copied().flatten().is_some())
    };
    for layer in &mut layers {
        pinned(layer);
    }

    let mut pos = vec![0.0; items.len()];
    let renumber = |layers: &Vec<Vec<usize>>, pos: &mut Vec<f64>| {
        for layer in layers {
            for (k, &it) in layer.iter().enumerate() {
                pos[it] = k as f64;
            }
        }
    };
    renumber(&layers, &mut pos);
    let crossings = |pos: &[f64]| -> usize {
        let mut segs: Vec<(f64, f64)> = Vec::new();
        let mut total = 0;
        for li in 0..layers_n.saturating_sub(1) {
            segs.clear();
            for (a, ds) in down.iter().enumerate() {
                if items[a].layer == li {
                    segs.extend(ds.iter().map(|&b| (pos[a], pos[b])));
                }
            }
            for i in 0..segs.len() {
                for j in i + 1..segs.len() {
                    let (s, t) = (segs[i], segs[j]);
                    if (s.0 - t.0) * (s.1 - t.1) < 0.0 {
                        total += 1;
                    }
                }
            }
        }
        total
    };
    let mut best_layers = layers.clone();
    let mut best_cross = crossings(&pos);
    for _ in 0..8 {
        for downward in [true, false] {
            let seq: Vec<usize> = if downward {
                (1..layers_n).collect()
            } else {
                (0..layers_n.saturating_sub(1)).rev().collect()
            };
            for li in seq {
                let (nb, other) = if downward { (&up, &down) } else { (&down, &up) };
                let key = |it: usize| -> Option<f64> {
                    let ns = if nb[it].is_empty() {
                        &other[it]
                    } else {
                        &nb[it]
                    };
                    (!ns.is_empty())
                        .then(|| ns.iter().map(|&x| pos[x]).sum::<f64>() / ns.len() as f64)
                };
                let layer = &layers[li];
                let mut gkeys: Vec<(usize, f64, f64)> = Vec::new();
                for &it in layer {
                    let g = groups[it];
                    if gkeys.iter().any(|(x, _, _)| *x == g) {
                        continue;
                    }
                    let members: Vec<usize> =
                        layer.iter().copied().filter(|&m| groups[m] == g).collect();
                    let ks: Vec<f64> = members.iter().filter_map(|&m| key(m)).collect();
                    let first = members
                        .iter()
                        .map(|&m| pos[m])
                        .fold(f64::INFINITY, f64::min);
                    let k = if ks.is_empty() {
                        members.iter().map(|&m| pos[m]).sum::<f64>() / members.len() as f64
                    } else {
                        ks.iter().sum::<f64>() / ks.len() as f64
                    };
                    gkeys.push((g, k, first));
                }
                gkeys.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.2.total_cmp(&b.2)));
                let mut next = Vec::with_capacity(layer.len());
                for (g, _, _) in &gkeys {
                    next.extend(layer.iter().copied().filter(|&m| groups[m] == *g));
                }
                pinned(&mut next);
                layers[li] = next;
                renumber(&layers, &mut pos);
            }
            let c = crossings(&pos);
            if c < best_cross {
                best_cross = c;
                best_layers = layers.clone();
            }
        }
    }
    layers = best_layers;
    // In a group of three or more cards joined within a row, the card with the most links in the
    // row goes to the middle, so a star of links draws between neighbours rather than as arcs.
    let degree = |c: usize| levels.iter().filter(|v| v.a == c || v.b == c).count();
    for layer in &mut layers {
        if has_pin(layer) {
            continue;
        }
        let mut done: Vec<usize> = Vec::new();
        let mut k = 0;
        while k < layer.len() {
            let g = groups[layer[k]];
            let len = layer[k..].iter().take_while(|&&m| groups[m] == g).count();
            if len >= 3 && !done.contains(&g) {
                let members = &mut layer[k..k + len];
                let hub = (0..len)
                    .max_by(|&a, &b| degree(members[a]).cmp(&degree(members[b])).then(b.cmp(&a)))
                    .unwrap_or(0);
                let card = members[hub];
                let mut rest: Vec<usize> = members.iter().copied().filter(|&m| m != card).collect();
                rest.insert(len / 2, card);
                members.copy_from_slice(&rest);
            }
            done.push(g);
            k += len.max(1);
        }
    }
    // A row whose cards with their gaps are wider than the wrap width: its cards in that order,
    // cut into rows that each stay within it, and the layout made again.
    let widths: Vec<f64> = cards.iter().map(|c| c.w).collect();
    if wrap.is_none() && !last {
        let rows: Vec<Vec<usize>> = layers
            .iter()
            .map(|layer| {
                layer
                    .iter()
                    .copied()
                    .filter(|&it| it < cards.len())
                    .collect()
            })
            .collect();
        if let Some(w) = cut(&rows, &widths, page::WRAP_WIDTH) {
            return Err(w);
        }
    }
    // A frame draws its own node card first, on the left. When it has more links within the row
    // to its right neighbour than to its left one, it is mirrored, so that card faces them.
    for layer in &layers {
        for (k, &it) in layer.iter().enumerate() {
            if it >= cards.len() || cards[it].single() {
                continue;
            }
            let count = |o: Option<&usize>| -> usize {
                o.map(|&o| {
                    levels
                        .iter()
                        .filter(|v| (v.a == it && v.b == o) || (v.a == o && v.b == it))
                        .map(|v| v.links.len())
                        .sum()
                })
                .unwrap_or(0)
            };
            let left = k.checked_sub(1).and_then(|j| layer.get(j));
            if count(layer.get(k + 1)) > count(left) {
                cards[it].mirror();
            }
        }
    }

    // Horizontal placement.
    let kinds: Vec<ItemKind> = items.iter().map(|it| it.kind).collect();
    let gap = |a: usize, b: usize| -> f64 {
        match (kinds[a], kinds[b]) {
            (ItemKind::Node(x), ItemKind::Node(y)) => {
                let (x, y) = (x.min(y), x.max(y));
                let widest = levels
                    .iter()
                    .filter(|v| v.a == x && v.b == y)
                    .flat_map(|v| v.links.iter())
                    .map(|&l| text_w(&edge_label(sc, &sc.links[l], false), 11.5, false) + 18.0)
                    .fold(0.0, f64::max);
                if widest > 0.0 {
                    (widest + 40.0).max(48.0)
                } else {
                    40.0
                }
            }
            (ItemKind::Node(_), _) | (_, ItemKind::Node(_)) => 22.0,
            _ => 16.0,
        }
    };
    for layer in &layers {
        let mut x = 0.0;
        for (k, &it) in layer.iter().enumerate() {
            if k > 0 {
                x += gap(layer[k - 1], it);
            }
            items[it].x = x + items[it].w / 2.0;
            x += items[it].w;
        }
    }
    // The cards of a wrapped row hold to where they stand packed from one left edge, so its rows
    // stand one under another; what passes between them moves around them.
    let mut fixed: Vec<Option<f64>> = vec![None; items.len()];
    for layer in &layers {
        let mut x = 0.0;
        for &it in layer
            .iter()
            .filter(|&&it| pin.get(it).copied().flatten().is_some())
        {
            fixed[it] = Some(x + items[it].w / 2.0);
            x += items[it].w + CARD_GAP;
        }
    }
    let place = |layer: &[usize], items: &mut Vec<Item>, desired: &[f64]| {
        let mut offs = vec![0.0; layer.len()];
        for k in 1..layer.len() {
            let (a, b) = (layer[k - 1], layer[k]);
            offs[k] = offs[k - 1] + (items[a].w + items[b].w) / 2.0 + gap(a, b);
        }
        let target: Vec<f64> = desired.iter().zip(&offs).map(|(d, o)| d - o).collect();
        let weight: Vec<f64> = layer
            .iter()
            .map(|&it| match items[it].kind {
                ItemKind::Node(_) if fixed[it].is_some() => 50.0,
                ItemKind::Node(_) => 1.0,
                ItemKind::Label(_) | ItemKind::Fan(_) => 1.5,
                ItemKind::Dummy(_) | ItemKind::Trunk(_) => 2.0,
            })
            .collect();
        let y = pava(&target, &weight);
        for (k, &it) in layer.iter().enumerate() {
            items[it].x = y[k] + offs[k];
        }
    };
    for _ in 0..40 {
        let seq: Vec<usize> = (0..layers_n).chain((0..layers_n).rev()).collect();
        for li in seq {
            let layer = layers[li].clone();
            let desired: Vec<f64> = layer
                .iter()
                .map(|&it| {
                    if let Some(x) = fixed[it] {
                        return x;
                    }
                    let ns: Vec<usize> = up[it].iter().chain(down[it].iter()).copied().collect();
                    if ns.is_empty() {
                        items[it].x
                    } else {
                        ns.iter().map(|&x| items[x].x).sum::<f64>() / ns.len() as f64
                    }
                })
                .collect();
            place(&layer, &mut items, &desired);
        }
    }
    // Straighten: the passing items of one bundle or trunk (a run of dummies) take one x where
    // every layer they cross has room for it, so a long line runs straight, not as a staircase.
    let slot_of: Vec<(usize, usize)> = {
        let mut s = vec![(0, 0); items.len()];
        for (li, layer) in layers.iter().enumerate() {
            for (k, &it) in layer.iter().enumerate() {
                s[it] = (li, k);
            }
        }
        s
    };
    let passing = |it: usize| matches!(kinds[it], ItemKind::Dummy(_) | ItemKind::Trunk(_));
    let mut runs: Vec<Vec<usize>> = Vec::new();
    let mut seen = vec![false; items.len()];
    for bd in &bundles {
        let mut run: Vec<usize> = Vec::new();
        for &it in bd.items.iter().chain(std::iter::once(&usize::MAX)) {
            if it != usize::MAX && passing(it) && !seen[it] {
                seen[it] = true;
                run.push(it);
            } else if !run.is_empty() {
                runs.push(std::mem::take(&mut run));
            }
        }
    }
    runs.sort_by_key(|r| std::cmp::Reverse(r.len()));
    for run in runs.iter().filter(|r| r.len() >= 2) {
        let fits = |x: f64, items: &Vec<Item>| {
            run.iter().all(|&it| {
                let (li, k) = slot_of[it];
                let layer = &layers[li];
                let w = items[it].w / 2.0;
                let left_ok = k == 0 || {
                    let a = layer[k - 1];
                    items[a].x + items[a].w / 2.0 + gap(a, it) <= x - w + 0.01
                };
                let right_ok = k + 1 == layer.len() || {
                    let b = layer[k + 1];
                    x + w + gap(it, b) <= items[b].x - items[b].w / 2.0 + 0.01
                };
                left_ok && right_ok
            })
        };
        let cost =
            |x: f64, items: &Vec<Item>| run.iter().map(|&it| (items[it].x - x).abs()).sum::<f64>();
        let mut best: Option<(f64, f64)> = None;
        for &c in run {
            let x = items[c].x;
            if fits(x, &items) {
                let cst = cost(x, &items);
                if best.is_none_or(|(_, b)| cst < b) {
                    best = Some((x, cst));
                }
            }
        }
        if let Some((x, _)) = best {
            for &it in run {
                items[it].x = x;
            }
        }
    }
    // In a layout whose rows wrapped, a row the placement moved out past the widest row (or the
    // wrap width, when wider) moves back in as a whole, so the rows stand one under another.
    if let Some(w) = wrap {
        let spans: Vec<(f64, f64)> = layers
            .iter()
            .map(|layer| {
                layer
                    .iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, r), &it| {
                        (
                            l.min(items[it].x - items[it].w / 2.0),
                            r.max(items[it].x + items[it].w / 2.0),
                        )
                    })
            })
            .collect();
        let widest = spans
            .iter()
            .filter(|s| s.0.is_finite())
            .max_by(|a, b| (a.1 - a.0).total_cmp(&(b.1 - b.0)));
        // Wider than the wrap width with the edges that pass between the rows: the rows that
        // wrapped are cut again, narrower by the excess, from the order they had in one row.
        if let Some(&(l, r)) = widest
            && r - l > page::WRAP_WIDTH + 0.5
            && !last
        {
            let mut rows: Vec<(usize, Vec<(usize, usize)>)> = Vec::new();
            for (c, a) in w.at.iter().enumerate() {
                let Some((_, k)) = *a else { continue };
                match rows.iter_mut().find(|x| x.0 == base_row[c]) {
                    Some(x) => x.1.push((k, c)),
                    None => rows.push((base_row[c], vec![(k, c)])),
                }
            }
            let rows: Vec<Vec<usize>> = rows
                .into_iter()
                .map(|(_, mut r)| {
                    r.sort_unstable();
                    r.into_iter().map(|x| x.1).collect()
                })
                .collect();
            // When that cuts the same lines, the widest line of them breaks.
            let line_w = |row: &Vec<usize>| -> f64 {
                let mut most: f64 = 0.0;
                let mut used = 0.0;
                for (k, &c) in row.iter().enumerate() {
                    let line = |c: usize| w.at[c].map(|a| a.0);
                    used = if k > 0 && line(row[k - 1]) == line(c) {
                        used + CARD_GAP + widths[c]
                    } else {
                        widths[c]
                    };
                    most = most.max(used);
                }
                most
            };
            let narrower = (w.target - (r - l - page::WRAP_WIDTH)).max(1.0);
            let next = match cut(&rows, &widths, narrower) {
                Some(n) if n.at != w.at => Some(n),
                _ => {
                    let widest_line = rows.iter().map(line_w).fold(0.0, f64::max);
                    cut(&rows, &widths, (widest_line - 1.0).min(narrower).max(1.0))
                }
            };
            if let Some(next) = next.filter(|n| n.at != w.at) {
                return Err(next);
            }
        }
        if let Some(&(l, r)) = widest {
            let room = (r - l).max(page::WRAP_WIDTH);
            let lo = l - (room - (r - l)) / 2.0;
            for (layer, &(l, r)) in layers.iter().zip(&spans) {
                let by = if !l.is_finite() {
                    0.0
                } else if l < lo {
                    lo - l
                } else if r > lo + room {
                    (lo + room - r).max(lo - l)
                } else {
                    0.0
                };
                for &it in layer {
                    items[it].x += by;
                }
            }
        }
    }
    let left = items
        .iter()
        .map(|it| it.x - it.w / 2.0)
        .fold(f64::INFINITY, f64::min);
    let shift = if left.is_finite() {
        MARGIN + frame.origin.0 - left
    } else {
        0.0
    };
    for it in &mut items {
        it.x = (it.x + shift).round();
    }
    let width = items
        .iter()
        .map(|it| it.x + it.w / 2.0)
        .fold(0.0, f64::max)
        .ceil()
        + MARGIN;

    // Vertical placement.
    let mut layer_top = vec![0.0; layers_n];
    let mut layer_h = vec![0.0; layers_n];
    // A label row holds, top to bottom: the tracks of the combs that join a trunk going down,
    // the labels, the tracks of the combs that join a trunk going up.
    let join_of = |bd: &Bundle| -> Option<usize> {
        let f = bd.fan?;
        if fans[f].at_lower {
            bd.items.first().copied()
        } else {
            bd.items.last().copied()
        }
    };
    let mut combs: Vec<(usize, bool, usize)> = Vec::new(); // layer, going up, fan
    for bd in &bundles {
        if let (Some(it), Some(f)) = (join_of(bd), bd.fan) {
            let key = (items[it].layer, !fans[f].at_lower, f);
            if !combs.contains(&key) {
                combs.push(key);
            }
        }
    }
    combs.sort_unstable();
    let band = |li: usize, up: bool| -> f64 {
        let n = combs.iter().filter(|c| c.0 == li && c.1 == up).count();
        if n == 0 { 0.0 } else { n as f64 * TRACK + 4.0 }
    };
    let mut mid_top = vec![0.0; layers_n];
    let mut mid_h = vec![0.0; layers_n];
    let mut y = MARGIN + 6.0 + frame.origin.1;
    for li in 0..layers_n {
        let h = layers[li].iter().map(|&it| items[it].h).fold(0.0, f64::max);
        let h = if li % 2 == 1 { h.max(16.0) } else { h };
        layer_top[li] = y;
        mid_top[li] = y + band(li, false);
        mid_h[li] = h;
        layer_h[li] = band(li, false) + h + band(li, true);
        y += layer_h[li] + ROW_GAP;
    }
    let height = (y - ROW_GAP + MARGIN + 6.0).ceil().max(80.0);
    for it in &mut items {
        it.y = mid_top[it.layer] + mid_h[it.layer] / 2.0;
    }
    let track_y = |li: usize, f: usize| -> f64 {
        let up = !fans[f].at_lower;
        let k = combs
            .iter()
            .filter(|c| c.0 == li && c.1 == up)
            .position(|c| c.2 == f)
            .unwrap_or(0) as f64;
        let start = if up {
            mid_top[li] + mid_h[li] + 2.0
        } else {
            layer_top[li] + 2.0
        };
        start + (k + 0.5) * TRACK
    };

    // Edge geometry: each link of a bundle as a list of points from the upper card down.
    struct Route {
        link: usize,
        bundle: usize,
        pts: Vec<(f64, f64)>,
    }
    let mut routes: Vec<Route> = Vec::new();
    for (bi, bd) in bundles.iter().enumerate() {
        let n_links = bd.links.len();
        for (k, &l) in bd.links.iter().enumerate() {
            let mut pts = Vec::new();
            let join = join_of(bd);
            for &it in &bd.items {
                let item = &items[it];
                if let ItemKind::Fan(f) | ItemKind::Trunk(f) = item.kind {
                    let (top, bottom) = match item.kind {
                        ItemKind::Fan(_) => (item.y, item.y),
                        _ => (
                            layer_top[item.layer],
                            layer_top[item.layer] + layer_h[item.layer],
                        ),
                    };
                    let (top, bottom) = match (join == Some(it), fans[f].at_lower) {
                        (true, true) => (track_y(item.layer, f), bottom),
                        (true, false) => (top, track_y(item.layer, f)),
                        _ => (top, bottom),
                    };
                    pts.push((item.x, top));
                    if bottom != top {
                        pts.push((item.x, bottom));
                    }
                    continue;
                }
                let x = match item.kind {
                    ItemKind::Label(_) if bd.channel.is_some() => {
                        let (wl, _) = bd.side_widths(sc);
                        let left_edge = item.x - item.w / 2.0;
                        let right_edge = item.x + item.w / 2.0;
                        let (_, wr) = bd.side_widths(sc);
                        if let Some(j) = bd.left.iter().position(|&x| x == l) {
                            left_edge + wl + (j as f64 + 0.5) * LINE_GAP
                        } else {
                            let j = bd.right.iter().position(|&x| x == l).unwrap_or(0);
                            right_edge - wr - (j as f64 + 0.5) * LINE_GAP
                        }
                    }
                    ItemKind::Label(_) => item.x,
                    _ => item.x + (k as f64 - (n_links as f64 - 1.0) / 2.0) * LINE_GAP,
                };
                let (top, bottom) = match item.kind {
                    ItemKind::Label(_) => (item.y - item.h / 2.0, item.y + item.h / 2.0),
                    _ => (
                        layer_top[item.layer],
                        layer_top[item.layer] + layer_h[item.layer],
                    ),
                };
                pts.push((x, top));
                pts.push((x, bottom));
            }
            routes.push(Route {
                link: l,
                bundle: bi,
                pts,
            });
        }
    }
    // The card of the node an edge end lands on inside an item: its centre x, top, bottom and
    // width.
    let node_box = |ci: usize, part: Option<usize>| -> (f64, f64, f64, f64) {
        let it = &items[ci];
        let c = &cards[ci];
        if c.single() {
            return (it.x, it.y - it.h / 2.0, it.y + it.h / 2.0, c.w);
        }
        let (x0, y0) = (it.x - c.w / 2.0, it.y - c.h / 2.0);
        match part.and_then(|p| c.node(p)) {
            Some((dx, dy, w, h)) => (x0 + dx + w / 2.0, y0 + dy, y0 + dy + h, w),
            None => (it.x, y0, y0 + c.h, c.w),
        }
    };
    // The node an end of a route lands on.
    let end_node = |ri: usize, upper_end: bool| -> Option<usize> {
        let bd = &bundles[routes[ri].bundle];
        let l = &sc.links[routes[ri].link];
        let from_upper = resolve(&l.from).and_then(card_of) == Some(bd.upper);
        index(if from_upper == upper_end {
            &l.from
        } else {
            &l.to
        })
    };
    // Ports: the ends of edges on one side of a card, spread in the order of where they go.
    // The links of a fan share one port at the fan's card: its key is past the routes'.
    let port_key = |ri: usize, upper_end: bool| -> usize {
        let bd = &bundles[routes[ri].bundle];
        let card = if upper_end { bd.upper } else { bd.lower };
        match bd.fan {
            Some(f) if fans[f].card == card => routes.len() + f,
            _ => ri,
        }
    };
    // card, bottom side, key, toward x, node
    let mut ports: Vec<(usize, bool, usize, f64, Option<usize>)> = Vec::new();
    for (ri, r) in routes.iter().enumerate() {
        let bd = &bundles[r.bundle];
        let first = r.pts.first().map(|p| p.0).unwrap_or(items[bd.lower].x);
        let last = r.pts.last().map(|p| p.0).unwrap_or(items[bd.upper].x);
        for (card, side, toward) in [(bd.upper, true, first), (bd.lower, false, last)] {
            let key = port_key(ri, side);
            if !ports.iter().any(|p| p.2 == key && p.1 == side) {
                ports.push((card, side, key, toward, end_node(ri, side)));
            }
        }
    }
    let mut port_x: Vec<(usize, bool, f64)> = Vec::new(); // port key, upper end, x
    for ci in 0..cards.len() {
        for side in [true, false] {
            let mut nodes: Vec<Option<usize>> = Vec::new();
            for p in ports.iter().filter(|p| p.0 == ci && p.1 == side) {
                if !nodes.contains(&p.4) {
                    nodes.push(p.4);
                }
            }
            for node in nodes {
                let (cx, _, _, w) = node_box(ci, node);
                let mut ends: Vec<&(usize, bool, usize, f64, Option<usize>)> = ports
                    .iter()
                    .filter(|p| p.0 == ci && p.1 == side && p.4 == node)
                    .collect();
                ends.sort_by(|a, b| a.3.total_cmp(&b.3).then(a.2.cmp(&b.2)));
                let m = ends.len();
                let span = if m > 1 {
                    (w - 40.0).min(26.0 * (m as f64 - 1.0))
                } else {
                    0.0
                };
                for (j, e) in ends.iter().enumerate() {
                    let x = if m > 1 {
                        cx - span / 2.0 + span * j as f64 / (m as f64 - 1.0)
                    } else {
                        cx
                    };
                    port_x.push((e.2, side, x));
                }
            }
        }
    }

    // The way out of the wrapped frames of item `ci` from a point on the bottom (`up`: top) of
    // node card `part`, in the page's coordinates ([`Card::way_out`]).
    let way_out = |ci: usize, part: Option<usize>, (x, y): (f64, f64), up: bool, k: usize| {
        let c = &cards[ci];
        let Some(p) = part.filter(|_| !c.lines.is_empty()) else {
            return Vec::new();
        };
        let (x0, y0) = (items[ci].x - c.w / 2.0, items[ci].y - c.h / 2.0);
        c.way_out(sc, p, (x - x0, y - y0), up, k)
            .into_iter()
            .map(|(a, b)| (a + x0, b + y0))
            .collect::<Vec<_>>()
    };

    let mut svg = String::new();
    // The highest y anything is drawn at: the top of the first row, or an arc over it.
    let mut reach = MARGIN + 6.0 + frame.origin.1;

    // Edges across rows.
    svg.push_str("<g class=\"edges\">");
    for (ri, r) in routes.iter().enumerate() {
        let bd = &bundles[r.bundle];
        let link = &sc.links[r.link];
        let ux = port_x
            .iter()
            .find(|p| p.0 == port_key(ri, true) && p.1)
            .map(|p| p.2)
            .unwrap_or(items[bd.upper].x);
        let lx = port_x
            .iter()
            .find(|p| p.0 == port_key(ri, false) && !p.1)
            .map(|p| p.2)
            .unwrap_or(items[bd.lower].x);
        // A card shorter than its row leaves it straight down (or up), so the line bends only
        // between rows, never beside a taller card of the same row.
        let (lu, ll) = (items[bd.upper].layer, items[bd.lower].layer);
        let row_bottom = layer_top[lu] + layer_h[lu];
        let upper_bottom = node_box(bd.upper, end_node(ri, true)).2;
        let mut pts = vec![(ux, upper_bottom)];
        // Out of a wrapped frame through the gap below its line and its corridor.
        let out = way_out(
            bd.upper,
            end_node(ri, true),
            (ux, upper_bottom),
            false,
            r.link,
        );
        let (ox, oy) = out.last().copied().unwrap_or((ux, upper_bottom));
        pts.extend(out);
        if row_bottom - oy > 0.5 {
            pts.push((ox, row_bottom));
        }
        // A link of a fan runs straight to its track, then along it to the trunk.
        let comb = bd.fan.map(|f| fans[f].at_lower);
        if comb == Some(true)
            && let Some(&(_, ty)) = r.pts.first()
        {
            pts.push((ox, ty));
        }
        pts.extend(r.pts.iter().copied());
        let lower_top = node_box(bd.lower, end_node(ri, false)).1 - 1.0;
        // Into a wrapped frame down its corridor and through the gap above the card's line.
        let mut into = way_out(
            bd.lower,
            end_node(ri, false),
            (lx, lower_top + 1.0),
            true,
            r.link,
        );
        into.reverse();
        let (ix, iy) = into.first().copied().unwrap_or((lx, lower_top));
        if comb == Some(false)
            && let Some(&(_, ty)) = r.pts.last()
        {
            pts.push((ix, ty));
        }
        if iy - layer_top[ll] > 0.5 {
            pts.push((ix, layer_top[ll]));
        }
        pts.extend(into);
        pts.push((lx, lower_top));
        let from_upper = resolve(&link.from).and_then(card_of) == Some(bd.upper);
        if !from_upper {
            pts[0].1 += 1.0;
            pts.reverse();
        }
        let mut d = format!("M{} {}", px(pts[0].0), px(pts[0].1));
        for w in pts.windows(2) {
            let ((x1, y1), (x2, y2)) = (w[0], w[1]);
            if (x1 - x2).abs() < 0.5 || (y1 - y2).abs() < 0.5 {
                let _ = write!(d, "L{} {}", px(x2), px(y2));
            } else {
                let ym = (y1 + y2) / 2.0;
                let _ = write!(
                    d,
                    "C{} {} {} {} {} {}",
                    px(x1),
                    px(ym),
                    px(x2),
                    px(ym),
                    px(x2),
                    px(y2)
                );
            }
        }
        edge_path(&mut svg, &d, link);
    }
    // Edges within a row.
    let mut row_labels: Vec<(f64, f64, String)> = Vec::new();
    let mut arcs = vec![0usize; layers_n];
    for v in &levels {
        let (a, b) = (v.a, v.b);
        let (l, r) = if items[a].x <= items[b].x {
            (a, b)
        } else {
            (b, a)
        };
        let layer = &layers[items[a].layer];
        let pa = layer.iter().position(|&x| x == a).unwrap_or(0);
        let pb = layer.iter().position(|&x| x == b).unwrap_or(0);
        let adjacent = pa.abs_diff(pb) == 1;
        let m = v.links.len() as f64;
        for (k, &li) in v.links.iter().enumerate() {
            let link = &sc.links[li];
            let from_left = resolve(&link.from).and_then(card_of) == Some(l);
            let label = edge_label(sc, link, false);
            // Two items side by side, one of them a frame: the edge runs straight when the node
            // card at each end faces the other item and the two cards share the height of the
            // lanes.
            let straight = (adjacent && !(cards[l].single() && cards[r].single()))
                .then(|| {
                    let (ln, rn) = if from_left {
                        (index(&link.from), index(&link.to))
                    } else {
                        (index(&link.to), index(&link.from))
                    };
                    let edge_x = |ci: usize, part: Option<usize>, right: bool| -> Option<f64> {
                        let it = &items[ci];
                        let c = &cards[ci];
                        if c.single() {
                            return Some(it.x + if right { it.w / 2.0 } else { -it.w / 2.0 });
                        }
                        let (dx, _, w, _) = c.node(part?)?;
                        let x0 = it.x - c.w / 2.0;
                        let faces = if right {
                            c.nodes.iter().all(|n| n.1 + n.3 <= dx + w + 0.5)
                        } else {
                            c.nodes.iter().all(|n| n.1 >= dx - 0.5)
                        };
                        faces.then_some(x0 + if right { dx + w } else { dx })
                    };
                    let x1 = edge_x(l, ln, true)?;
                    let x2 = edge_x(r, rn, false)?;
                    let (_, lt, lb, _) = node_box(l, ln);
                    let (_, rt, rb, _) = node_box(r, rn);
                    let (top, bottom) = (lt.max(rt), lb.min(rb));
                    let half = (m - 1.0) / 2.0 * 26.0;
                    (bottom - top >= 2.0 * half + 24.0).then(|| {
                        let y = (top + bottom) / 2.0 + (k as f64 - (m - 1.0) / 2.0) * 26.0;
                        (x1 + 1.0, x2 - 1.0, y)
                    })
                })
                .flatten();
            if let Some((x1, x2, y)) = straight {
                let (sx, ex) = if from_left { (x1, x2) } else { (x2, x1) };
                let d = format!("M{} {}L{} {}", px(sx), px(y), px(ex), px(y));
                edge_path(&mut svg, &d, link);
                row_labels.push(((x1 + x2) / 2.0, y, label));
            } else if adjacent && cards[l].single() && cards[r].single() {
                let y = items[l].y + (k as f64 - (m - 1.0) / 2.0) * 26.0;
                let x1 = items[l].x + items[l].w / 2.0 + 1.0;
                let x2 = items[r].x - items[r].w / 2.0 - 1.0;
                let (sx, ex) = if from_left { (x1, x2) } else { (x2, x1) };
                let d = format!("M{} {}L{} {}", px(sx), px(y), px(ex), px(y));
                edge_path(&mut svg, &d, link);
                row_labels.push(((x1 + x2) / 2.0, y, label));
            } else {
                // Over the taller item when a frame at either end wraps, which stands higher.
                let top = if cards[l].lines.is_empty() && cards[r].lines.is_empty() {
                    items[l].y - items[l].h / 2.0
                } else {
                    (items[l].y - items[l].h / 2.0).min(items[r].y - items[r].h / 2.0)
                };
                let j = arcs[items[a].layer];
                arcs[items[a].layer] += 1;
                let lift = 46.0 + 28.0 * j as f64;
                // An arc leaves the top of the node card it starts at and lands on the top of
                // the one it ends at, inside a frame or not.
                let (fc, tc) = if from_left { (l, r) } else { (r, l) };
                let (sx, sy, ex, ey) = if cards[l].single() && cards[r].single() {
                    let (sx, ex) = if from_left {
                        (items[l].x, items[r].x)
                    } else {
                        (items[r].x, items[l].x)
                    };
                    (sx, top, ex, top)
                } else {
                    let (sx, sy, _, _) = node_box(fc, index(&link.from));
                    let (ex, ey, _, _) = node_box(tc, index(&link.to));
                    (sx, sy, ex, ey)
                };
                // Out of a wrapped frame at either end, up its corridor, before the arc.
                let out = way_out(fc, index(&link.from), (sx, sy), true, li);
                let mut inn = way_out(tc, index(&link.to), (ex, ey), true, li);
                inn.reverse();
                let mut d = format!("M{} {}", px(sx), px(sy));
                for p in &out {
                    let _ = write!(d, "L{} {}", px(p.0), px(p.1));
                }
                let (a, ay) = out.last().copied().unwrap_or((sx, sy));
                let (b, by) = inn.first().copied().unwrap_or((ex, ey - 1.0));
                // Beside or out of a wrapped frame, which stands higher: straight up to the top of
                // the taller item at both ends, and the arc between, over everything.
                let level = !(cards[l].lines.is_empty() && cards[r].lines.is_empty());
                if level && top < ay - 0.5 {
                    let _ = write!(d, "L{} {}", px(a), px(top));
                }
                let _ = write!(
                    d,
                    "C{} {} {} {} {} {}",
                    px(a),
                    px(top - lift),
                    px(b),
                    px(top - lift),
                    px(b),
                    px(if level { top } else { by })
                );
                if level && by > top + 0.5 {
                    let _ = write!(d, "L{} {}", px(b), px(by));
                }
                if !inn.is_empty() {
                    for p in &inn[1..] {
                        let _ = write!(d, "L{} {}", px(p.0), px(p.1));
                    }
                    let _ = write!(d, "L{} {}", px(ex), px(ey - 1.0));
                }
                edge_path(&mut svg, &d, link);
                reach = reach.min(top - lift * 0.75 - PILL_H / 2.0);
                row_labels.push(((sx + ex) / 2.0, top - lift * 0.75, label));
            }
        }
    }
    // Edges between two node cards side by side in one frame run straight across the gap.
    let mut in_gap: Vec<usize> = Vec::new();
    for link in &sc.links {
        let (Some(ca), Some(cb)) = (
            resolve(&link.from).and_then(card_of),
            resolve(&link.to).and_then(card_of),
        ) else {
            continue;
        };
        let (Some(pa), Some(pb)) = (index(&link.from), index(&link.to)) else {
            continue;
        };
        if ca != cb || pa == pb {
            continue;
        }
        let Some(g) = cards[ca]
            .gaps
            .iter()
            .position(|g| (g.0 == pa && g.1 == pb) || (g.0 == pb && g.1 == pa))
        else {
            continue;
        };
        let (_, _, gl, gr, gt) = cards[ca].gaps[g];
        let k = in_gap.iter().filter(|&&x| x == g + 1000 * ca).count() as f64;
        in_gap.push(g + 1000 * ca);
        let x0 = items[ca].x - cards[ca].w / 2.0;
        let y = items[ca].y - cards[ca].h / 2.0 + gt + 26.0 + k * 26.0;
        let (x1, x2) = (x0 + gl + 1.0, x0 + gr - 1.0);
        let (sx, ex) = if cards[ca].gaps[g].0 == pa {
            (x1, x2)
        } else {
            (x2, x1)
        };
        let d = format!("M{} {}L{} {}", px(sx), px(y), px(ex), px(y));
        edge_path(&mut svg, &d, link);
        row_labels.push(((x1 + x2) / 2.0, y, edge_label(sc, link, false)));
    }
    // Edges on a lane: down from the bottom of the node card they leave, along the lane, up to
    // the bottom of the node card they reach. The ends at one card spread over its width in the
    // order of where they go.
    let lane_box = |ci: usize, p: usize| -> Option<(f64, f64, f64)> {
        let (dx, dy, w, h) = cards[ci].node(p)?;
        let x0 = items[ci].x - cards[ci].w / 2.0;
        let y0 = items[ci].y - cards[ci].h / 2.0;
        Some((x0 + dx + w / 2.0, y0 + dy + h, w))
    };
    // lane entry, from end, node, toward x
    let mut lane_ends: Vec<(usize, bool, usize, f64)> = Vec::new();
    for (k, &(ci, li, _)) in lanes.iter().enumerate() {
        let link = &sc.links[li];
        let (Some(pa), Some(pb)) = (index(&link.from), index(&link.to)) else {
            continue;
        };
        let (Some(a), Some(b)) = (lane_box(ci, pa), lane_box(ci, pb)) else {
            continue;
        };
        lane_ends.push((k, true, pa, b.0));
        lane_ends.push((k, false, pb, a.0));
    }
    let lane_x = |k: usize, from: bool| -> Option<f64> {
        let &(_, _, node, _) = lane_ends.iter().find(|e| e.0 == k && e.1 == from)?;
        let ci = lanes[k].0;
        let mut at: Vec<&(usize, bool, usize, f64)> = lane_ends
            .iter()
            .filter(|e| lanes[e.0].0 == ci && e.2 == node)
            .collect();
        at.sort_by(|a, b| a.3.total_cmp(&b.3).then(a.0.cmp(&b.0)).then(b.1.cmp(&a.1)));
        let (cx, _, w) = lane_box(ci, node)?;
        let m = at.len() as f64;
        let j = at.iter().position(|e| e.0 == k && e.1 == from)? as f64;
        let span = if m > 1.0 {
            (w - 40.0).min(26.0 * (m - 1.0))
        } else {
            0.0
        };
        Some(if m > 1.0 {
            cx - span / 2.0 + span * j / (m - 1.0)
        } else {
            cx
        })
    };
    for (k, &(ci, li, ly)) in lanes.iter().enumerate() {
        let link = &sc.links[li];
        let (Some(pa), Some(pb)) = (index(&link.from), index(&link.to)) else {
            continue;
        };
        let (Some((_, sb, _)), Some((_, eb, _)), Some(sx), Some(ex)) = (
            lane_box(ci, pa),
            lane_box(ci, pb),
            lane_x(k, true),
            lane_x(k, false),
        ) else {
            continue;
        };
        let y = items[ci].y - cards[ci].h / 2.0 + ly;
        // Out of wrapped frames at both ends first, through their gaps and corridors.
        let out = way_out(ci, Some(pa), (sx, sb), false, li);
        let mut inn = way_out(ci, Some(pb), (ex, eb), false, li);
        inn.reverse();
        let ax = out.last().map_or(sx, |p| p.0);
        let bx = inn.first().map_or(ex, |p| p.0);
        let mut pts = vec![(sx, sb)];
        pts.extend(out);
        pts.extend([(ax, y), (bx, y)]);
        pts.extend(inn);
        pts.push((ex, eb + 1.0));
        let mut d = format!("M{} {}", px(pts[0].0), px(pts[0].1));
        for p in &pts[1..] {
            let _ = write!(d, "L{} {}", px(p.0), px(p.1));
        }
        edge_path(&mut svg, &d, link);
        let label = edge_label(sc, link, false);
        let half = (text_w(&label, 11.5, false) + 18.0) / 2.0;
        let x0 = items[ci].x - cards[ci].w / 2.0;
        let (lo, hi) = (x0 + FPAD + half, x0 + cards[ci].w - FPAD - half);
        let mid = (ax + bx) / 2.0;
        row_labels.push((if lo <= hi { mid.clamp(lo, hi) } else { mid }, y, label));
    }
    svg.push_str("</g>");

    // Labels and channel cards.
    svg.push_str("<g class=\"labels\">");
    for (cx, cy, label) in &row_labels {
        pill(&mut svg, *cx, *cy, label, None);
    }
    for fan in &fans {
        if let Some(it) = fan.item(fan.layer) {
            pill(&mut svg, items[it].x, items[it].y, &fan.label, None);
        }
    }
    for bd in &bundles {
        let Some(&it) = bd
            .items
            .iter()
            .find(|&&it| matches!(items[it].kind, ItemKind::Label(_)))
        else {
            continue;
        };
        let item = &items[it];
        match bd.channel {
            None => {
                let link = &sc.links[bd.links[0]];
                pill(&mut svg, item.x, item.y, &edge_label(sc, link, false), None);
            }
            Some(c) => {
                let (wl, wr) = bd.side_widths(sc);
                let (cw, ch) = channel_card_size(&sc.parts[c]);
                let left_edge = item.x - item.w / 2.0;
                let right_edge = item.x + item.w / 2.0;
                let cl = left_edge + wl + bd.left.len() as f64 * LINE_GAP + 12.0;
                let ct = item.y - ch / 2.0;
                let line_l = left_edge + wl + (bd.left.len() as f64 - 0.5) * LINE_GAP;
                let line_r = right_edge - wr - (bd.right.len() as f64 - 0.5) * LINE_GAP;
                let _ = write!(
                    svg,
                    "<path class=\"over\" d=\"M{} {}H{}M{} {}H{}\"/>",
                    px(line_l),
                    px(item.y),
                    px(cl),
                    px(cl + cw),
                    px(item.y),
                    px(line_r)
                );
                for (side, ls) in [(true, &bd.left), (false, &bd.right)] {
                    let rows = ls.len() as f64;
                    for (j, &l) in ls.iter().enumerate() {
                        let text = &sc.links[l].label;
                        let w = text_w(text, 11.5, false) + 18.0;
                        let y = item.y + (j as f64 - (rows - 1.0) / 2.0) * (PILL_H + 4.0);
                        let x = if side {
                            left_edge + wl - 6.0 - w / 2.0
                        } else {
                            right_edge - wr + 6.0 + w / 2.0
                        };
                        pill(&mut svg, x, y, text, Some(w));
                    }
                }
                channel_card(&mut svg, sc, c, cl, ct, cw, ch);
            }
        }
    }
    svg.push_str("</g>");

    // Cards.
    svg.push_str("<g class=\"nodes\">");
    let mut boxes: Vec<(String, [f64; 4])> = Vec::new();
    let mut frames: Vec<(String, [f64; 4])> = Vec::new();
    for (ci, card) in cards.iter().enumerate() {
        let it = &items[ci];
        let (x, y) = (it.x - card.w / 2.0, it.y - card.h / 2.0);
        draw_card(&mut svg, sc, card, x, y);
        for &(p, dx, dy, w, h) in &card.nodes {
            boxes.push((sc.parts[p].id.clone(), [x + dx, y + dy, w, h]));
        }
        for &(p, dx, dy, w, h) in &card.frames {
            frames.push((sc.parts[p].id.clone(), [x + dx, y + dy, w, h]));
        }
    }
    svg.push_str("</g>");
    Ok(Drawn {
        body: svg,
        width,
        height,
        top: reach,
        boxes,
        frames,
    })
}

/// The SVG of a page's graph: its accessible title and description, the arrowheads, and what
/// `body` draws in a `width` × `height` box.
fn wrap(page: &Page, decor: &Decor, body: &str, width: f64, height: f64) -> Graph {
    let mut svg = String::with_capacity(body.len() + 2048);
    let _ = write!(
        svg,
        "<svg class=\"graph\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"group\" aria-labelledby=\"graph-title graph-desc\">",
        w = px(width),
        h = px(height)
    );
    let _ = write!(
        svg,
        "<title id=\"graph-title\">{}</title><desc id=\"graph-desc\">{}</desc>",
        esc(&decor.words.title),
        esc(&decor.words.desc)
    );
    svg.push_str(
        "<defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\"><path d=\"M0 0L10 5L0 10z\" class=\"arrowhead\"/></marker><marker id=\"arrow-hot\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\"><path d=\"M0 0L10 5L0 10z\" class=\"arrowhead hot\"/></marker>",
    );
    // A heavier edge's arrowhead is drawn smaller in its stroke's units, so it keeps one size.
    for (weight, w) in [(2, WEIGHT_2), (3, WEIGHT_3)] {
        if page.edges.iter().any(|e| weight_of(e.count) == weight) {
            let m = px(7.0 * 1.4 / w);
            let _ = write!(
                svg,
                "<marker id=\"arrow-{weight}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"{m}\" markerHeight=\"{m}\" orient=\"auto-start-reverse\"><path d=\"M0 0L10 5L0 10z\" class=\"arrowhead\"/></marker>"
            );
        }
    }
    svg.push_str("</defs>");
    svg.push_str(body);
    svg.push_str("</svg>");
    Graph { svg, width, height }
}

/// The stroke of an edge that stands for `count`: 1 for one edge, 2 for two or three, 3 for four
/// or more.
fn weight_of(count: Option<u64>) -> u8 {
    match count.unwrap_or(1) {
        0..=1 => 1,
        2..=3 => 2,
        _ => 3,
    }
}

fn edge_path(svg: &mut String, d: &str, link: &Link) {
    let over = link
        .via
        .as_ref()
        .map(|o| format!(" over {o}"))
        .unwrap_or_default();
    // The stroke of a heavier edge, and an arrowhead that keeps the size of a single edge's.
    let weight = match link.weight {
        2 => format!(" style=\"stroke-width:{WEIGHT_2};marker-end:url(#arrow-2)\""),
        3 => format!(" style=\"stroke-width:{WEIGHT_3};marker-end:url(#arrow-3)\""),
        _ => String::new(),
    };
    let _ = write!(
        svg,
        "<path class=\"edge{}\" d=\"{d}\" data-from=\"{}\" data-to=\"{}\"{}{weight}><title>{} → {}: {}{}</title></path>",
        if link.multi { " multi" } else { "" },
        esc(&link.from),
        esc(&link.to),
        link.via
            .as_ref()
            .map(|o| format!(" data-over=\"{}\"", esc(o)))
            .unwrap_or_default(),
        esc(&link.from),
        esc(&link.to),
        esc(&link.label),
        esc(&over)
    );
}

fn pill(svg: &mut String, cx: f64, cy: f64, text: &str, w: Option<f64>) {
    let w = w.unwrap_or_else(|| text_w(text, 11.5, false) + 18.0);
    let (x, y) = (cx - w / 2.0, cy - PILL_H / 2.0);
    let _ = write!(
        svg,
        "<g class=\"pill\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{PILL_H}\" rx=\"10\" data-box=\"{} {} {} {PILL_H}\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
        px(x),
        px(y),
        px(w),
        px(x),
        px(y),
        px(w),
        px(cx),
        px(cy + 4.0),
        esc(text)
    );
}

/// The style attribute of a part whose group names a colour, with a leading space. The colour is
/// a [`page::Colour`], so it holds nothing but a colour.
fn look(p: &Part) -> String {
    p.colour
        .as_ref()
        .map(|c| format!(" style=\"--c:{}\"", esc(c)))
        .unwrap_or_default()
}

/// The icon tile of a part: its stroke icon, or the text its group names.
fn icon_tile(svg: &mut String, p: &Part, x: f64, y: f64, size: f64) {
    let scale = size / 30.0;
    if let Some(t) = &p.icon_text {
        let _ = write!(
            svg,
            "<rect class=\"icon-bg\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/><text class=\"icon-text\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" style=\"fill:var(--c);font-size:{}px;font-weight:600\">{}</text>",
            px(x),
            px(y),
            px(size),
            px(size),
            px(7.0 * scale),
            px(x + size / 2.0),
            px(y + size / 2.0 + 4.5 * scale),
            px(13.0 * scale),
            esc(t)
        );
        return;
    }
    let icon = p.icon;
    let _ = write!(
        svg,
        "<rect class=\"icon-bg\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/><path class=\"icon\" transform=\"translate({} {}) scale({})\" d=\"{}\"/>",
        px(x),
        px(y),
        px(size),
        px(size),
        px(7.0 * scale),
        px(x + 8.0 * scale),
        px(y + 8.0 * scale),
        (scale * 100.0).round() / 100.0,
        icon
    );
}

fn channel_card(svg: &mut String, sc: &Scene, c: usize, x: f64, y: f64, w: f64, h: f64) {
    let p = &sc.parts[c];
    let _ = write!(
        svg,
        "<g class=\"node channel f-{}\"{} data-part=\"{}\"><rect class=\"card-bg\" tabindex=\"0\" role=\"button\" aria-pressed=\"false\" aria-label=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"10\" data-box=\"{} {} {} {}\"/>",
        p.slot,
        look(p),
        esc(&p.id),
        esc(&p.description),
        px(x),
        px(y),
        px(w),
        px(h),
        px(x),
        px(y),
        px(w),
        px(h)
    );
    icon_tile(svg, p, x + 8.0, y + 11.0, 22.0);
    let _ = write!(
        svg,
        "<text class=\"name sm\" x=\"{}\" y=\"{}\">{}</text><text class=\"sub\" x=\"{}\" y=\"{}\">{}</text></g>",
        px(x + 38.0),
        px(y + 19.0),
        esc(&p.label),
        px(x + 38.0),
        px(y + 34.0),
        esc(&p.subtitle)
    );
}

fn draw_card(svg: &mut String, sc: &Scene, card: &Card, x: f64, y: f64) {
    // Frames first, outermost first, so the cards they hold sit on them.
    for &(p, dx, dy, w, h) in &card.frames {
        let (label, heading, per) = frame_head(sc, p);
        let part = &sc.parts[p];
        let (fx, fy) = (x + dx, y + dy);
        let refused = part.frame_only && part.refused;
        let _ = write!(
            svg,
            "<g class=\"region{}{} f-{}\"{} data-region=\"{}\">",
            if per { " per" } else { "" },
            if refused { " refused" } else { "" },
            part.slot,
            look(part),
            esc(&part.id),
        );
        if refused {
            frame_ring(svg, fx, fy, w, h);
        }
        let _ = write!(
            svg,
            "<rect class=\"region-bg\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"12\"/>",
            px(fx),
            px(fy),
            px(w),
            px(h)
        );
        let mut tx = fx + FPAD;
        if let Some(l) = label {
            let _ = write!(
                svg,
                "<text class=\"region-name\" x=\"{}\" y=\"{}\">{}</text>",
                px(tx),
                px(fy + 16.0),
                esc(&l)
            );
            tx += text_w(&l, 12.0, true) + 8.0;
        }
        let _ = write!(
            svg,
            "<text class=\"region-sub\" x=\"{}\" y=\"{}\">{}</text>",
            px(tx),
            px(fy + 16.0),
            esc(&heading)
        );
        line_chips(
            svg,
            frame_chips(sc, p),
            tx + text_w(&heading, 11.0, false),
            fy + 2.0,
        );
        if sc.closable.contains(&part.id) {
            close_control(svg, &part.id, &part.label, fx + w - FPAD, fy + 4.0);
        }
        svg.push_str("</g>");
    }
    for &(p, dx, dy, w, h) in &card.nodes {
        draw_node(svg, sc, p, x + dx, y + dy, w, h);
    }
}

fn draw_node(svg: &mut String, sc: &Scene, part: usize, x: f64, y: f64, w: f64, h: f64) {
    let p = &sc.parts[part];
    let border = p.outline.is_some();
    let _ = write!(
        svg,
        "<g class=\"node f-{}{}{}{}\"{} data-part=\"{}\"{}>",
        p.slot,
        if border { " on-border" } else { "" },
        if p.refused { " refused" } else { "" },
        if p.opens { " collapsed" } else { "" },
        look(p),
        esc(&p.id),
        if p.opens {
            format!(" data-open=\"{}\"", esc(&p.id))
        } else {
            String::new()
        },
    );
    if border {
        let _ = write!(
            svg,
            "<rect class=\"border-mark\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"14\"/>",
            px(x - 5.0),
            px(y - 5.0),
            px(w + 10.0),
            px(h + 10.0)
        );
    }
    if p.refused {
        let off = if border { 9.0 } else { 5.0 };
        let _ = write!(
            svg,
            "<rect class=\"refusal-ring\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\"/>",
            px(x - off),
            px(y - off),
            px(w + 2.0 * off),
            px(h + 2.0 * off),
            px(10.0 + off)
        );
    }
    let _ = write!(
        svg,
        "<rect class=\"card-bg\" tabindex=\"0\" role=\"button\" {} aria-label=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"10\" data-box=\"{} {} {} {}\"/>",
        // The card of a collapsed region opens it; any other card is selected.
        if p.opens {
            "aria-expanded=\"false\""
        } else {
            "aria-pressed=\"false\""
        },
        esc(&p.description),
        px(x),
        px(y),
        px(w),
        px(h),
        px(x),
        px(y),
        px(w),
        px(h)
    );
    icon_tile(svg, p, x + PAD, y + PAD, ICON);
    let tx = x + PAD + ICON + 10.0;
    let _ = write!(
        svg,
        "<text class=\"name\" x=\"{}\" y=\"{}\">{}</text><text class=\"sub\" x=\"{}\" y=\"{}\">{}</text>",
        px(tx),
        px(y + PAD + 12.0),
        esc(&p.label),
        px(tx),
        px(y + PAD + 27.0),
        esc(&p.subtitle)
    );
    // Badges sit at the top right, the last one rightmost.
    let mut right = x + w - PAD;
    for t in p.badges.iter().rev() {
        let bw = badge_w(t);
        let bx = right - bw;
        let _ = write!(
            svg,
            "<g class=\"badge\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"18\" rx=\"9\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
            px(bx),
            px(y + PAD - 1.0),
            px(bw),
            px(bx + bw / 2.0),
            px(y + PAD + 12.0),
            esc(t)
        );
        right = bx - 8.0;
    }
    if let Some(tag) = &p.outline {
        let tw = tag_w(tag);
        let bx = x + w - PAD - tw;
        let _ = write!(
            svg,
            "<g class=\"tag\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"16\" rx=\"8\"/><text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text></g>",
            px(bx),
            px(y + PAD + 16.0),
            px(tw),
            px(bx + tw / 2.0),
            px(y + PAD + 27.5),
            esc(tag)
        );
    }
    let mut cy = y + HEAD;
    for (k, v) in &p.props {
        let _ = write!(
            svg,
            "<text class=\"prop\" x=\"{}\" y=\"{}\"><tspan class=\"prop-k\">{}</tspan> {}</text>",
            px(x + PAD),
            px(cy + 12.0),
            esc(k),
            esc(v)
        );
        cy += PROP;
    }
    // The ports of a node sit on the left edge of its card, one row each. An unresolved port
    // leaves its row as a dashed stub that reaches nothing, named by the refusals on it.
    for port in &p.ports {
        let mut class = String::from("port");
        if port.refused {
            class.push_str(" refused");
        }
        if port.unresolved {
            class.push_str(" unresolved");
        }
        let _ = write!(
            svg,
            "<g class=\"{class}\" data-port=\"{}\"><title>{}</title>",
            esc(&port.address),
            esc(&port.title),
        );
        if port.unresolved {
            let _ = write!(
                svg,
                "<path class=\"unresolved-end\" d=\"M{} {}H{}\"/>",
                px(x - 5.0),
                px(cy + 8.5),
                px(x - 21.0)
            );
        }
        let _ = write!(
            svg,
            "<rect class=\"port-mark\" x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" rx=\"2\"/><text class=\"prop\" x=\"{}\" y=\"{}\"><tspan class=\"prop-k\">port</tspan> {}",
            px(x - 5.0),
            px(cy + 3.5),
            px(x + PAD),
            px(cy + 12.0),
            esc(&port.own)
        );
        if let Some(v) = &port.via {
            let _ = write!(svg, " <tspan class=\"via\">{}</tspan>", esc(v));
        }
        if port.unresolved && !port.refusals.is_empty() {
            let _ = write!(
                svg,
                " <tspan class=\"rule\">{}</tspan>",
                esc(&port.refusals.join(", "))
            );
        }
        svg.push_str("</text></g>");
        cy += PROP;
    }
    for chip in &p.chips {
        chip.svg(svg, x + PAD, cy + 2.0);
        cy += CHIP;
    }
    svg.push_str("</g>");
}
