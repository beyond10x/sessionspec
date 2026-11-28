//! Regions drawn collapsed.
//!
//! A region with `collapsed: true` is drawn as one card: its label and the count of the nodes
//! inside it, nested regions included. Its nodes, their ports and marks are left out, and the
//! edges that reach them are drawn to the card instead: one edge per pair of visible ends, labelled
//! with the count of edges it stands for when that is more than one. A refusal inside rings the
//! card.
//!
//! The page holds each state it can show, laid out in full by [`graph::render_in`]: the page as
//! served, with every collapsible region collapsed, and per collapsible region the state in which
//! it is open, together with the collapsible regions around it. A region inside an open one keeps
//! its own `collapsed`. So `k` collapsible regions give `k + 1` layouts, not `2^k`, and one region
//! (with those around it) is open at a time. The script of the page swaps the states; without it
//! the page shows the state as served.

use crate::graph::{self, Decor, Graph, NodeDecor};
use crate::page::{Edge, Level, Mark, Node, Page, Region, Target, plural};

/// One region of a page with the region it lies in.
pub(crate) struct Info {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
    pub collapsed: bool,
}

/// Every region of a page, outermost first.
pub(crate) struct Tree {
    pub regions: Vec<Info>,
}

impl Tree {
    pub fn of(page: &Page) -> Self {
        fn walk(r: &Region, parent: Option<&str>, out: &mut Vec<Info>) {
            out.push(Info {
                id: r.id.clone(),
                label: r.label.clone(),
                parent: parent.map(str::to_string),
                collapsed: r.collapsed,
            });
            for inner in &r.regions {
                walk(inner, Some(&r.id), out);
            }
        }
        let mut regions = Vec::new();
        for r in &page.regions {
            walk(r, None, &mut regions);
        }
        Tree { regions }
    }

    fn info(&self, id: &str) -> Option<&Info> {
        self.regions.iter().find(|r| r.id == id)
    }

    /// `start` and the regions around it, innermost first. A chain that comes back to a region
    /// it passed ends there.
    fn chain<'a>(&'a self, start: Option<&'a str>) -> Vec<&'a str> {
        let mut out: Vec<&str> = Vec::new();
        let mut cur = start;
        while let Some(id) = cur {
            if out.contains(&id) {
                break;
            }
            out.push(id);
            cur = self.info(id).and_then(|r| r.parent.as_deref());
        }
        out
    }

    /// The regions node `n` lies in, innermost first: its region and those around it, and for the
    /// node that is a region's own card, the regions around that region.
    fn around<'a>(&'a self, n: &'a Node) -> Vec<&'a str> {
        let mut out = self.chain(n.region.as_deref());
        if let Some(r) = self.info(&n.id) {
            for id in self.chain(r.parent.as_deref()) {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
        out.retain(|id| *id != n.id);
        out
    }

    /// How many nodes of `page` lie inside region `id`, its own node left out.
    pub fn inside(&self, page: &Page, id: &str) -> usize {
        page.nodes
            .iter()
            .filter(|n| n.id != id && self.around(n).contains(&id))
            .count()
    }
}

/// One state of the page besides the one it is served in: the region open in it, the state its
/// control returns to (empty for the page as served) and its graph.
pub(crate) struct State {
    pub id: String,
    pub up: String,
    pub graph: Graph,
}

/// The graph of the page as served, and the state of each collapsible region opened. A page with
/// no collapsible region has the one graph of [`graph::render`].
pub(crate) fn render(page: &Page, decor: &Decor) -> (Graph, Vec<State>) {
    let tree = Tree::of(page);
    if !tree.regions.iter().any(|r| r.collapsed) {
        return (graph::render(page, decor), Vec::new());
    }
    let draw = |open: &[&str]| {
        let (p, d) = fold(page, decor, &tree, open);
        graph::render_in(&p, &d, page)
    };
    let served = draw(&[]);
    let states = tree
        .regions
        .iter()
        .filter(|r| r.collapsed)
        .map(|r| {
            let open: Vec<&str> = tree
                .chain(Some(&r.id))
                .into_iter()
                .filter(|id| tree.info(id).is_some_and(|i| i.collapsed))
                .collect();
            State {
                id: r.id.clone(),
                up: open.get(1).map(|s| s.to_string()).unwrap_or_default(),
                graph: draw(&open),
            }
        })
        .collect();
    (served, states)
}

/// The page and decor of one state: every collapsible region not in `open` collapsed.
fn fold(page: &Page, decor: &Decor, tree: &Tree, open: &[&str]) -> (Page, Decor) {
    let closed = |id: &str| tree.info(id).is_some_and(|r| r.collapsed) && !open.contains(&id);
    // The outermost collapsed region a node lies in.
    let hider = |n: &Node| -> Option<String> {
        tree.around(n)
            .into_iter()
            .rfind(|id| closed(id))
            .map(str::to_string)
    };
    // The collapsed regions drawn as a card: those in no collapsed region.
    let cards: Vec<&Info> = tree
        .regions
        .iter()
        .filter(|r| r.collapsed && closed(&r.id))
        .filter(|r| !tree.chain(r.parent.as_deref()).iter().any(|id| closed(id)))
        .collect();
    let is_card = |id: &str| cards.iter().any(|c| c.id == id);
    let hidden: Vec<Option<String>> = page.nodes.iter().map(hider).collect();
    let shows = |id: &str| {
        page.nodes
            .iter()
            .zip(&hidden)
            .any(|(n, h)| n.id == id && h.is_none())
    };

    let card_node = |c: &Info| Node {
        id: c.id.clone(),
        label: c.label.clone(),
        group: String::new(),
        badges: Vec::new(),
        region: c.parent.clone(),
    };
    let mut nodes: Vec<Node> = Vec::new();
    let mut placed: Vec<String> = Vec::new();
    for (n, h) in page.nodes.iter().zip(&hidden) {
        match h {
            Some(r) => {
                if !placed.contains(r) && !shows(r) {
                    if let Some(c) = cards.iter().find(|c| c.id == *r) {
                        nodes.push(card_node(c));
                    }
                    placed.push(r.clone());
                }
            }
            None => nodes.push(n.clone()),
        }
    }
    for c in &cards {
        if !nodes.iter().any(|n| n.id == c.id) {
            nodes.push(card_node(c));
        }
    }

    fn prune(r: &Region, closed: &dyn Fn(&str) -> bool) -> Region {
        let mut c = r.clone();
        c.regions = if closed(&r.id) {
            Vec::new()
        } else {
            r.regions.iter().map(|x| prune(x, closed)).collect()
        };
        c
    }
    let regions = page.regions.iter().map(|r| prune(r, &closed)).collect();

    let visible = |id: &str| nodes.iter().any(|n| n.id == id);
    let ports = page
        .ports
        .iter()
        .filter(|p| visible(&p.node))
        .cloned()
        .collect();

    // The card a mark's target is hidden in: for a node, the outermost collapsed region it lies
    // in; for a region that is no node, the outermost collapsed region around it, itself left
    // out when it is a card (a mark on a card is drawn on it).
    let hidden_in = |m: &Mark| -> Option<String> {
        let id = m.target.node.as_str();
        match page.nodes.iter().position(|n| n.id == id) {
            Some(i) => hidden[i].clone(),
            None if is_card(id) => None,
            None => tree
                .chain(Some(id))
                .into_iter()
                .rfind(|r| closed(r))
                .map(str::to_string),
        }
    };
    // Marks on what is drawn keep their chip, a mark on a region drawn as a card included; a
    // refusal inside a card rings the card.
    let mut marks: Vec<Mark> = Vec::new();
    let mut chips = Vec::new();
    for (i, m) in page.marks.iter().enumerate() {
        let region = !page.nodes.iter().any(|n| n.id == m.target.node)
            && tree.info(&m.target.node).is_some();
        if shows(&m.target.node) || (region && hidden_in(m).is_none()) {
            marks.push(m.clone());
            chips.push(decor.chips.get(i).cloned().flatten());
        }
    }
    for c in &cards {
        let refusals = page
            .marks
            .iter()
            .filter(|m| m.level == Level::Refusal)
            .filter(|m| hidden_in(m).as_deref() == Some(c.id.as_str()))
            .count();
        if refusals > 0 {
            marks.push(Mark {
                target: Target {
                    node: c.id.clone(),
                    port: None,
                },
                name: "refused inside".to_string(),
                detail: Some(plural(refusals, "refusal")),
                level: Level::Refusal,
            });
            chips.push(None);
        }
    }

    // An edge end that lies inside a card is drawn to the card. Edges between ends that do not
    // reach a card are kept as they are; the others are one per pair of visible ends.
    let node_of = |end: &str| -> Option<usize> {
        page.nodes.iter().position(|n| n.id == end).or_else(|| {
            let p = page
                .ports
                .iter()
                .find(|p| end.strip_prefix(p.node.as_str()) == Some(&format!(".{}", p.name)))?;
            page.nodes.iter().position(|n| n.id == p.node)
        })
    };
    let end = |e: &str| -> (String, bool) {
        match node_of(e).and_then(|i| hidden[i].clone()) {
            Some(r) => (r, true),
            None => (e.to_string(), false),
        }
    };
    let via_shown = |v: Option<String>| v.filter(|id| shows(id));
    // Edge, via, how many edges it stands for, whether it is one per pair of visible ends.
    let mut kept: Vec<(Edge, Option<String>, Vec<u64>, bool)> = Vec::new();
    for (i, e) in page.edges.iter().enumerate() {
        let via = decor.vias.get(i).cloned().flatten();
        let ((from, fm), (to, tm)) = (end(&e.from), end(&e.to));
        let merged = fm || tm || is_card(&from) || is_card(&to);
        if !merged {
            kept.push((e.clone(), via_shown(via), vec![1], false));
            continue;
        }
        if from == to {
            continue;
        }
        let n = e.count.unwrap_or(1);
        match kept
            .iter_mut()
            .find(|k| k.3 && k.0.from == from && k.0.to == to)
        {
            Some(k) => k.2.push(n),
            None => kept.push((
                Edge {
                    from,
                    to,
                    label: e.label.clone(),
                    count: e.count,
                },
                via_shown(via),
                vec![n],
                true,
            )),
        }
    }
    let mut edges = Vec::new();
    let mut vias = Vec::new();
    for (mut e, via, members, _) in kept {
        if members.len() > 1 {
            e.label = String::new();
            e.count = Some(members.iter().sum());
            vias.push(None);
        } else {
            vias.push(via);
        }
        edges.push(e);
    }

    let mut node_decor: Vec<(String, NodeDecor)> = decor
        .nodes
        .iter()
        .filter(|(id, _)| visible(id) && !is_card(id))
        .cloned()
        .collect();
    for c in &cards {
        let own = decor.nodes.iter().find(|(id, _)| *id == c.id).map(|x| &x.1);
        let count = plural(tree.inside(page, &c.id), "node");
        node_decor.push((
            c.id.clone(),
            NodeDecor {
                description: Some(format!("Open region {}, {count}", c.label)),
                subtitle: Some(count),
                icon: own
                    .and_then(|d| d.icon)
                    .or_else(|| (!shows(&c.id)).then_some("frame")),
                properties: own.map(|d| d.properties.clone()).unwrap_or_default(),
                outline: own.and_then(|d| d.outline.clone()),
                opens: true,
            },
        ));
    }
    let mut words = decor.words.clone();
    if !cards.is_empty() {
        let labels: Vec<&str> = cards.iter().map(|c| c.label.as_str()).collect();
        words.desc.push_str(&format!(
            " Collapsed to one card each: {}.",
            labels.join(", ")
        ));
    }
    let folded = Page {
        format: page.format.clone(),
        title: page.title.clone(),
        summary: page.summary.clone(),
        regions,
        nodes,
        ports,
        edges,
        marks,
        panels: Vec::new(),
        groups: page.groups.clone(),
        layout: page.layout,
    };
    let d = Decor {
        words,
        groups: decor.groups.clone(),
        nodes: node_decor,
        vias,
        chips,
        closable: open.iter().map(|s| s.to_string()).collect(),
    };
    (folded, d)
}
