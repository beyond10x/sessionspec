//! Every reference of a page names something the page declares: a port's `node` names a node, a
//! mark's target names a node (and a port the page declares on it), a node's `region` names a
//! region or a node. The renderer skips a reference that names nothing (`graph.rs` drops a port
//! or a mark whose node is no part, and draws a node outside every region), so what an author
//! declared would never be drawn where it was declared and no check would say why. Reading a page
//! from JSON refuses it instead, naming the code and the reference. Each case pairs a page that
//! resolves (accepted) with the one reference that must be refused.

use agenticspec_render::page::{Page, Port};
use serde_json::{Value, json};

fn base() -> Value {
    json!({
        "format": "agenticspec-render-model/1",
        "title": "t",
        "regions": [{ "id": "zone", "label": "zone", "regions": [{ "id": "shelf", "label": "shelf" }] }],
        "nodes": [
            { "id": "pay", "label": "pay", "group": "service" },
            { "id": "db", "label": "db", "group": "store", "region": "zone" },
            { "id": "log", "label": "log", "group": "store", "region": "pay" },
            { "id": "disk", "label": "disk", "group": "store", "region": "shelf" }
        ],
        "ports": [{ "node": "pay", "name": "charge", "kind": "http", "direction": "in" }],
        "edges": [],
        "marks": [
            { "target": { "node": "pay" }, "name": "slow" },
            { "target": { "node": "pay", "port": "charge" }, "name": "no_retry" }
        ]
    })
}

fn refusal(v: Value) -> String {
    match serde_json::from_value::<Page>(v.clone()) {
        Err(e) => {
            let text = e.to_string();
            let page: Page = serde_json::from_value(base()).unwrap();
            let unchecked = Page {
                ports: serde_json::from_value(v["ports"].clone()).unwrap(),
                marks: serde_json::from_value(v["marks"].clone()).unwrap(),
                nodes: serde_json::from_value(v["nodes"].clone()).unwrap(),
                ..page
            };
            let checked = unchecked
                .check()
                .expect_err("Page::check refuses the same page");
            assert!(text.contains(&checked), "reading runs Page::check: {text}");
            text
        }
        Ok(_) => panic!("accepted: {v}"),
    }
}

#[test]
fn a_page_whose_references_all_resolve_is_accepted() {
    serde_json::from_value::<Page>(base()).expect("every reference resolves");
}

#[test]
fn a_port_whose_node_names_no_node_is_refused_by_the_page_check() {
    serde_json::from_value::<Page>(base()).expect("a port of a node parses");
    let mut v = base();
    v["ports"] = json!([{ "node": "ghost", "name": "in", "kind": "http", "direction": "in" }]);
    v["marks"] = json!([]);
    match serde_json::from_value::<Page>(v.clone()) {
        Err(e) => assert!(e.to_string().contains("ghost"), "{e}"),
        Ok(_) => panic!(
            "a port of node `ghost`, which no node is, is accepted; the renderer drops it unseen"
        ),
    }
    let err = refusal(v);
    assert!(err.contains("port_node_unknown"), "{err}");
    assert!(
        err.contains("the port \"ghost.in\""),
        "names the port: {err}"
    );
}

#[test]
fn a_port_built_in_code_whose_node_names_no_node_fails_the_check() {
    let mut page: Page = serde_json::from_value(base()).unwrap();
    page.ports.push(Port {
        node: "ghost".to_string(),
        name: "in".to_string(),
        kind: "http".to_string(),
        ..Port::default()
    });
    let err = page.check().expect_err("a port of no node");
    assert!(err.starts_with("port_node_unknown"), "{err}");
}

#[test]
fn a_mark_whose_target_names_no_node_or_no_port_of_it_is_refused() {
    for (target, named) in [
        (json!({ "node": "ghost" }), "\"ghost\""),
        (json!({ "node": "pay", "port": "refund" }), "\"pay.refund\""),
        (json!({ "node": "db", "port": "charge" }), "\"db.charge\""),
    ] {
        let mut v = base();
        v["marks"]
            .as_array_mut()
            .unwrap()
            .push(json!({ "target": target, "name": "m" }));
        let err = refusal(v);
        assert!(err.contains("mark_target_unknown"), "{err}");
        assert!(
            err.contains(&format!("the mark \"m\" on {named}")),
            "names the mark: {err}"
        );
    }
}

#[test]
fn a_node_whose_region_names_no_region_and_no_node_is_refused() {
    let mut v = base();
    v["nodes"]
        .as_array_mut()
        .unwrap()
        .push(json!({ "id": "cache", "label": "cache", "group": "store", "region": "nowhere" }));
    let err = refusal(v);
    assert!(err.contains("node_region_unknown"), "{err}");
    assert!(
        err.contains("the node \"cache\": its region \"nowhere\""),
        "names the node and the region: {err}"
    );
}
