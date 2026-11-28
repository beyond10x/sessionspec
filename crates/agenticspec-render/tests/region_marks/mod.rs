//! A mark whose target is a region, such as a check result about a scope: drawn as a chip on the
//! region's frame label, or on its card while the region is collapsed, and in the label of its
//! band with `layout: bands`. A refusal on a region rings its frame, card or band; a refusal on a
//! region hidden inside a collapsed one counts as "refused inside" on that card.

use agenticspec_render::page::{Layout, Level, Mark, Node, Page, Region, Target, render};

fn node(id: &str, region: Option<&str>) -> Node {
    Node {
        id: id.to_string(),
        label: format!("{id} label"),
        group: "service".to_string(),
        region: region.map(str::to_string),
        ..Node::default()
    }
}

fn region(id: &str, collapsed: bool, regions: Vec<Region>) -> Region {
    Region {
        id: id.to_string(),
        label: format!("{id} area"),
        collapsed,
        regions,
        ..Region::default()
    }
}

fn mark(target: &str, name: &str, level: Level) -> Mark {
    Mark {
        target: Target {
            node: target.to_string(),
            port: None,
        },
        name: name.to_string(),
        detail: Some(format!("{name} detail")),
        level,
    }
}

/// An open region `zone` with an open region `shelf` inside, a collapsed region `vault` with an
/// open region `inner` inside, and a node `x` in none. Marks: a refusal and a piece of
/// information on `zone`, a refusal on `shelf`, on `vault` and on `inner`.
fn scopes() -> Page {
    let mut p = Page::new("scopes");
    p.regions = vec![
        region("zone", false, vec![region("shelf", false, Vec::new())]),
        region("vault", true, vec![region("inner", false, Vec::new())]),
    ];
    p.nodes = vec![
        node("x", None),
        node("z1", Some("zone")),
        node("s1", Some("shelf")),
        node("v1", Some("vault")),
        node("i1", Some("inner")),
    ];
    p.marks = vec![
        mark("zone", "scope_failed", Level::Refusal),
        mark("zone", "scope_note", Level::Info),
        mark("shelf", "shelf_failed", Level::Refusal),
        mark("vault", "vault_failed", Level::Refusal),
        mark("inner", "inner_failed", Level::Refusal),
    ];
    p
}

/// The SVG of the page as served, without the templates of the other states.
fn served(html: &str) -> &str {
    let start = html.find("<svg class=\"graph\"").expect("the graph");
    let end = start + html[start..].find("</svg>").expect("the graph ends") + 6;
    &html[start..end]
}

/// The SVG of the state in which region `id` is open.
fn state<'a>(html: &'a str, id: &str) -> &'a str {
    let key = format!("<template data-state=\"{id}\"");
    let start = html
        .find(&key)
        .unwrap_or_else(|| panic!("a template for {id}"));
    let open_end = start + html[start..].find('>').expect("tag ends") + 1;
    let end = open_end + html[open_end..].find("</template>").expect("template ends");
    &html[open_end..end]
}

/// The frame of region `id`: its group's opening tag up to the next frame or card.
fn frame<'a>(svg: &'a str, id: &str) -> &'a str {
    let key = format!("data-region=\"{id}\"");
    let at = svg
        .find(&key)
        .unwrap_or_else(|| panic!("the frame of {id}"));
    let open = svg[..at].rfind("<g ").expect("the frame's group");
    let rest = &svg[at..];
    let end = ["<g class=\"region", "<g class=\"node"]
        .iter()
        .filter_map(|m| rest.find(m))
        .min()
        .unwrap_or(rest.len());
    &svg[open..at + end]
}

/// The band of region `id`: its group up to the next band or the edges.
fn band<'a>(svg: &'a str, id: &str) -> &'a str {
    let key = format!("data-band=\"{id}\"");
    let at = svg.find(&key).unwrap_or_else(|| panic!("the band {id}"));
    let open = svg[..at].rfind("<g ").expect("the band's group");
    let rest = &svg[at..];
    let end = ["<g class=\"region band", "<g class=\"edges"]
        .iter()
        .filter_map(|m| rest.find(m))
        .min()
        .unwrap_or(rest.len());
    &svg[open..at + end]
}

/// The card of node `id`: its group's opening tag up to the next card.
fn card<'a>(svg: &'a str, id: &str) -> &'a str {
    let at = svg
        .find(&format!("data-part=\"{id}\""))
        .unwrap_or_else(|| panic!("{id} is drawn"));
    let open = svg[..at].rfind("<g ").expect("the card's group");
    let rest = &svg[at..];
    let end = rest.find("<g class=\"node").unwrap_or(rest.len());
    &svg[open..at + end]
}

/// The first rect of `svg` with class `class`, as `x y w h`.
fn rect(svg: &str, class: &str) -> [f64; 4] {
    let at = svg
        .find(&format!("<rect class=\"{class}\""))
        .unwrap_or_else(|| panic!("a rect {class}"));
    let tag = &svg[at..at + svg[at..].find("/>").expect("tag ends")];
    let num = |name: &str| -> f64 {
        let i = tag.find(&format!(" {name}=\"")).expect(name) + name.len() + 3;
        tag[i..i + tag[i..].find('"').unwrap()].parse().unwrap()
    };
    [num("x"), num("y"), num("width"), num("height")]
}

/// The rect of the chip whose title starts with `name`.
fn chip(svg: &str, name: &str) -> [f64; 4] {
    let at = svg
        .find(&format!("<title>{name}"))
        .unwrap_or_else(|| panic!("a chip {name}"));
    let tag = &svg[at..];
    let r = tag.find("<rect").unwrap();
    let t = &tag[r..r + tag[r..].find("/>").unwrap()];
    let num = |n: &str| -> f64 {
        let i = t.find(&format!(" {n}=\"")).expect(n) + n.len() + 3;
        t[i..i + t[i..].find('"').unwrap()].parse().unwrap()
    };
    [num("x"), num("y"), num("width"), num("height")]
}

fn within(inner: [f64; 4], outer: [f64; 4]) -> bool {
    inner[0] >= outer[0]
        && inner[1] >= outer[1]
        && inner[0] + inner[2] <= outer[0] + outer[2]
        && inner[1] + inner[3] <= outer[1] + outer[3]
}

#[test]
fn a_mark_on_an_open_region_is_a_chip_on_its_frame_label_and_a_refusal_rings_the_frame() {
    let html = render(&scopes());
    let svg = served(&html);
    let zone = frame(svg, "zone");
    assert!(zone.contains("class=\"region"), "{zone}");
    assert!(
        zone.contains(" refused"),
        "the frame is marked refused: {zone}"
    );
    assert!(
        zone.contains("<rect class=\"refusal-ring\""),
        "a ring: {zone}"
    );
    assert!(zone.contains("<g class=\"chip refusal\""), "{zone}");
    assert!(zone.contains("<title>scope_failed: scope_failed detail</title>"));
    assert!(zone.contains("<title>scope_note: scope_note detail</title>"));
    let bg = rect(zone, "region-bg");
    for name in ["scope_failed", "scope_note"] {
        let c = chip(zone, name);
        assert!(within(c, bg), "chip {name} {c:?} lies in the frame {bg:?}");
        assert!(
            c[1] < bg[1] + 24.0,
            "chip {name} sits on the label line: {c:?}"
        );
    }
    let shelf = frame(svg, "shelf");
    assert!(shelf.contains("<title>shelf_failed: shelf_failed detail</title>"));
    assert!(shelf.contains("<rect class=\"refusal-ring\""), "{shelf}");
    let x = card(svg, "x");
    assert!(
        !x.contains("refused"),
        "a node without a mark is not refused"
    );
}

#[test]
fn an_information_mark_alone_does_not_ring_its_region() {
    let mut p = scopes();
    p.marks = vec![mark("zone", "scope_note", Level::Info)];
    let html = render(&p);
    let zone = frame(served(&html), "zone");
    assert!(zone.contains("<title>scope_note: scope_note detail</title>"));
    assert!(!zone.contains("refusal-ring"), "{zone}");
    assert!(!zone.contains(" refused"), "{zone}");
}

#[test]
fn a_mark_on_a_collapsed_region_is_on_its_card_and_a_refusal_inside_counts() {
    let html = render(&scopes());
    let svg = served(&html);
    let vault = card(svg, "vault");
    assert!(vault.contains(" refused"), "the card is refused: {vault}");
    assert!(
        vault.contains("<title>vault_failed: vault_failed detail</title>"),
        "{vault}"
    );
    assert!(
        vault.contains("<title>refused inside: 1 refusal</title>"),
        "the refusal on inner counts inside: {vault}"
    );
    assert!(!svg.contains("inner_failed"), "inner is hidden in the card");
    let open = state(&html, "vault");
    let frame_v = frame(open, "vault");
    assert!(frame_v.contains("<title>vault_failed: vault_failed detail</title>"));
    assert!(
        frame_v.contains("<rect class=\"refusal-ring\""),
        "{frame_v}"
    );
    let inner = frame(open, "inner");
    assert!(inner.contains("<title>inner_failed: inner_failed detail</title>"));
    assert!(
        !open.contains("refused inside"),
        "nothing is hidden when vault is open"
    );
}

#[test]
fn a_mark_on_a_band_is_in_its_label_and_on_a_region_inside_on_that_frame() {
    let mut p = scopes();
    p.layout = Layout::Bands;
    let html = render(&p);
    let svg = served(&html);
    let zone = band(svg, "zone");
    assert!(zone.contains(" refused"), "the band is refused: {zone}");
    assert!(zone.contains("<rect class=\"refusal-ring\""), "{zone}");
    assert!(zone.contains("<title>scope_failed: scope_failed detail</title>"));
    assert!(zone.contains("<title>scope_note: scope_note detail</title>"));
    let bg = rect(zone, "region-bg");
    for name in ["scope_failed", "scope_note"] {
        let c = chip(zone, name);
        assert!(within(c, bg), "chip {name} {c:?} lies in the band {bg:?}");
    }
    let shelf = frame(svg, "shelf");
    assert!(shelf.contains("<title>shelf_failed: shelf_failed detail</title>"));
    let vault = card(svg, "vault");
    assert!(
        vault.contains("<title>vault_failed: vault_failed detail</title>"),
        "{vault}"
    );
    assert!(
        vault.contains("<title>refused inside: 1 refusal</title>"),
        "{vault}"
    );
}

#[test]
fn a_page_with_region_marks_reads_back_from_json() {
    let p = scopes();
    let text = serde_json::to_string(&p).unwrap();
    let back: Page = serde_json::from_str(&text).expect("region marks are accepted");
    assert_eq!(back, p);
}
