//! The page of the workstation architecture, with and without a simulation result.

mod bands;
mod collapse;
mod graph;
mod lanes;
mod model;
mod optim;
mod page;
mod references;
mod wrap;

use agenticspec_render::architecture::page;
use serde_json::{Value, json};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn workstation() -> Value {
    let path = root().join("architectures/workstation.yaml");
    let text = std::fs::read_to_string(&path).expect("architectures/workstation.yaml");
    serde_yaml_ng::from_str(&text).expect("the workstation architecture is YAML")
}

fn result() -> Value {
    serde_json::from_str(include_str!("fixtures/workstation-result.json")).expect("fixture JSON")
}

/// The text of the element that starts at the first `marker`, up to the next `</tr>`.
fn row<'a>(html: &'a str, marker: &str) -> &'a str {
    let start = html
        .find(marker)
        .unwrap_or_else(|| panic!("{marker} is on the page"));
    let rest = &html[start..];
    &rest[..rest.find("</tr>").expect("the row ends")]
}

#[test]
fn every_part_and_every_link_label_is_drawn() {
    let arch = workstation();
    let html = page(&arch, None);
    let parts = arch["parts"].as_object().expect("parts");
    assert_eq!(parts.len(), 9);
    for name in parts.keys() {
        assert!(
            html.contains(&format!(">{name}<")),
            "part {name} is drawn as text"
        );
        assert!(
            html.contains(&format!("data-part=\"{name}\"")),
            "part {name} is a node"
        );
    }
    for link in arch["links"].as_array().expect("links") {
        let carries = link["carries"].as_str().expect("carries");
        assert!(
            html.contains(&format!(">{carries}<")),
            "link label {carries} is drawn"
        );
    }
    assert!(
        html.contains("class=\"border-mark\""),
        "the border is marked"
    );
    assert!(html.contains(">×12<"), "the worker count is a badge");
}

#[test]
fn marked_places_are_chips_with_their_range() {
    let html = page(&workstation(), None);
    assert!(html.contains("data-place=\"parts.builds.capacity\""));
    assert!(html.contains("8…32"), "the list range of capacity");
    assert!(
        html.contains("2…24"),
        "the min/max range of the worker count"
    );
    assert!(html.contains("data-place=\"policies.question_default_after\""));
}

#[test]
fn a_result_shows_kpis_best_values_and_the_change_at_marked_places() {
    let html = page(&workstation(), Some(&result()));
    let best = &result()["best"]["kpis"];
    let expected = [
        ("tasks_done", "188"),
        ("owner_wait_share", "0.12"),
        ("cost_usd", "276.9"),
        ("queue_wait_p90_seconds", "240"),
        ("questions_defaulted", "21"),
        ("unreachable_parts", "0"),
    ];
    for (name, value) in expected {
        assert!(best.get(name).is_some(), "fixture has {name}");
        let tr = row(&html, &format!("data-kpi=\"{name}\""));
        assert!(tr.contains(&format!(">{name}<")), "KPI {name} is named");
        assert!(
            tr.contains(&format!(">{value}<")),
            "KPI {name} shows best {value}: {tr}"
        );
    }
    assert!(
        html.contains("data-place=\"parts.builds.capacity\" data-baseline=\"16\" data-best=\"24\"")
    );
    assert!(
        html.contains("class=\"trace\""),
        "the search trace is drawn"
    );
    assert!(
        html.contains("data-variant=\"v007\""),
        "the best variant is ranked"
    );
    assert!(
        row(&html, "data-variant=\"v006\"").contains("infeasible"),
        "an infeasible variant is marked"
    );
    assert!(
        html.contains("data-per-part=\"builds\""),
        "per-part KPIs of builds"
    );
}

#[test]
fn the_page_loads_nothing_from_outside() {
    for html in [
        page(&workstation(), None),
        page(&workstation(), Some(&result())),
    ] {
        assert!(html.starts_with("<!doctype html>"));
        assert!(!html.contains("http://"), "no http:// reference");
        assert!(!html.contains("https://"), "no https:// reference");
        assert!(!html.contains(" src="), "no loaded script or image");
    }
}

#[test]
fn the_output_is_deterministic() {
    assert_eq!(page(&workstation(), None), page(&workstation(), None));
    assert_eq!(
        page(&workstation(), Some(&result())),
        page(&workstation(), Some(&result()))
    );
}

#[test]
fn no_two_boxes_of_the_graph_overlap() {
    for html in [
        page(&workstation(), None),
        page(&workstation(), Some(&result())),
    ] {
        let mut boxes: Vec<(String, [f64; 4])> = Vec::new();
        for piece in html.split("data-box=\"").skip(1) {
            let text = &piece[..piece.find('"').expect("closed")];
            let v: Vec<f64> = text
                .split(' ')
                .map(|n| n.parse().expect("a number"))
                .collect();
            boxes.push((text.to_string(), [v[0], v[1], v[2], v[3]]));
        }
        assert!(
            boxes.len() >= 9 + 4,
            "nodes and edge labels carry a box (links with one label into one part share one): {}",
            boxes.len()
        );
        for (i, (a, ra)) in boxes.iter().enumerate() {
            for (b, rb) in &boxes[i + 1..] {
                let apart = ra[0] + ra[2] <= rb[0]
                    || rb[0] + rb[2] <= ra[0]
                    || ra[1] + ra[3] <= rb[1]
                    || rb[1] + rb[3] <= ra[1];
                assert!(apart, "boxes {a} and {b} overlap");
            }
        }
    }
}

#[test]
fn absent_and_partial_inputs_render() {
    let empty = page(&json!({}), None);
    assert!(empty.contains("</html>"));
    let partial = page(
        &workstation(),
        Some(&json!({ "best": { "kpis": { "tasks_done": 3 } } })),
    );
    assert!(row(&partial, "data-kpi=\"tasks_done\"").contains(">3<"));
    let hostile = page(
        &json!({ "architecture": "</script><b>x", "parts": { "a<b": { "block": "agent" } } }),
        None,
    );
    assert!(!hostile.contains("</script><b>"));
    assert!(!hostile.contains(">a<b<"));
}

/// The graph alone, for a host that draws it inline: its SVG without the page around it, its size,
/// and the parts and links it shows, each part with its properties.
#[test]
fn the_graph_alone_holds_the_svg_its_size_and_every_part_and_link() {
    let arch = workstation();
    let g = agenticspec_render::architecture::graph(&arch);
    let svg = g["svg"].as_str().expect("svg");
    assert!(svg.starts_with("<svg class=\"graph\""), "{}", &svg[..40]);
    assert!(svg.ends_with("</svg>"));
    assert!(!svg.contains("<html") && !svg.contains("data-theme-choice"));
    assert!(g["width"].as_f64().unwrap() > 100.0 && g["height"].as_f64().unwrap() > 80.0);
    let parts = g["parts"].as_array().expect("parts");
    assert_eq!(parts.len(), 9);
    let worker = parts.iter().find(|p| p["id"] == "worker").expect("worker");
    assert_eq!(worker["block"], "agent");
    assert_eq!(worker["count"], "12");
    assert!(worker["properties"].is_array());
    assert_eq!(
        g["links"].as_array().unwrap().len(),
        arch["links"].as_array().unwrap().len()
    );
    assert_eq!(g["name"], arch["architecture"]);
}

/// An outer document as the simulator loads it: the nested architecture beside its part.
fn nested() -> Value {
    json!({
        "format": "agenticspec-architecture/1",
        "architecture": "team",
        "parts": {
            "owner": { "block": "principal", "kind": "human" },
            "console": {
                "block": "architecture",
                "ref": "console.yaml",
                "digest": format!("sha256:{}", "a".repeat(64)),
                "inner": {
                    "architecture": "console",
                    "parts": {
                        "steward": { "block": "agent" },
                        "desk": { "block": "agent" },
                        "model": { "block": "model" }
                    },
                    "links": [
                        { "from": "steward", "to": "model", "carries": "model_call" },
                        { "from": "desk", "to": "model", "carries": "model_call" },
                        { "from": "steward", "to": "desk", "carries": "order" }
                    ],
                    "border": ["steward", "desk"]
                }
            },
            "worker": { "block": "agent", "count": 2 },
            "model": { "block": "model" }
        },
        "links": [
            { "from": "owner", "to": "console.steward", "carries": "prompt" },
            { "from": "console.desk", "to": "worker", "carries": "order" },
            { "from": "worker", "to": "model", "carries": "model_call" }
        ],
        "border": ["console.steward"]
    })
}

#[test]
fn a_nested_architecture_is_one_box_with_its_border_parts_as_ports() {
    let html = page(&nested(), None);
    assert_eq!(
        html.matches("class=\"node f-nested").count(),
        1,
        "one box for the nested architecture"
    );
    assert!(html.contains("data-part=\"console\""));
    assert!(
        html.contains(">console · 3 parts<"),
        "named with its part count"
    );
    for port in ["console.steward", "console.desk"] {
        assert!(
            html.contains(&format!("data-port=\"{port}\"")),
            "{port} is a port"
        );
    }
    assert!(
        !html.contains("data-part=\"console.steward\""),
        "an inner part is no box of its own"
    );
    assert!(
        html.contains("<tr data-from=\"owner\" data-to=\"console.steward\""),
        "a link to a port is listed"
    );
    // Both links that address a port reach the box: an edge labelled with the port it reaches.
    assert!(
        html.contains(">prompt · steward<"),
        "the prompt edge names its port"
    );
    assert!(
        html.contains(">order · desk<"),
        "the order edge names its port"
    );
}

#[test]
fn the_nested_box_overlaps_nothing() {
    let html = page(&nested(), None);
    let mut boxes: Vec<(String, [f64; 4])> = Vec::new();
    for piece in html.split("data-box=\"").skip(1) {
        let text = &piece[..piece.find('"').expect("closed")];
        let v: Vec<f64> = text
            .split(' ')
            .map(|n| n.parse().expect("a number"))
            .collect();
        boxes.push((text.to_string(), [v[0], v[1], v[2], v[3]]));
    }
    assert!(boxes.len() >= 4, "{}", boxes.len());
    for (i, (a, ra)) in boxes.iter().enumerate() {
        for (b, rb) in &boxes[i + 1..] {
            let apart = ra[0] + ra[2] <= rb[0]
                || rb[0] + rb[2] <= ra[0]
                || ra[1] + ra[3] <= rb[1]
                || rb[1] + rb[3] <= ra[1];
            assert!(apart, "boxes {a} and {b} overlap");
        }
    }
}

#[test]
fn a_nested_part_not_yet_resolved_still_renders_as_a_box() {
    let mut doc = nested();
    doc["parts"]["console"]
        .as_object_mut()
        .unwrap()
        .remove("inner");
    let html = page(&doc, None);
    assert!(html.contains("data-part=\"console\""));
    assert!(html.contains(">console.yaml<") || html.contains("console.yaml"));
    assert!(html.contains("data-port=\"console.steward\""));
}
