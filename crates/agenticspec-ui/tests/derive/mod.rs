//! `agenticspec ui derive`: an architecture, its contract and its `view:` key into one ess-ui/1
//! document. The fixtures are the committed examples of `ui/views/examples/` and the two small
//! documents of `tests/derive/`.

use std::path::{Path, PathBuf};

use agenticspec_ui::derive::{self, Derivation, Failure, Inputs};
use serde_yaml_ng::Value;

mod control;
mod ingress;
mod principal_kinds;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("derive")
        .join(name);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn inputs(arch: &Path, contract: Option<&str>, ports: Option<PathBuf>, out: PathBuf) -> Inputs {
    let r = root();
    Inputs {
        root: Some(r.clone()),
        architecture: arch.to_path_buf(),
        contract: contract.map(|c| r.join(c)),
        ports,
        out,
        control_plane: false,
        prediction: None,
    }
}

fn workstation_inputs(out: PathBuf, ports: Option<PathBuf>) -> Inputs {
    let r = root();
    inputs(
        &r.join("architectures/workstation.yaml"),
        Some("ui/views/examples/workstation.contract.yaml"),
        Some(ports.unwrap_or_else(|| r.join("ui/views/examples/ports.json"))),
        out,
    )
}

fn derived(i: &Inputs) -> Derivation {
    match agenticspec_ui::derive_ui(i) {
        Ok(d) => d,
        Err(Failure::Unreadable(e)) => panic!("unreadable: {e}"),
        Err(Failure::Refused(r)) => panic!("refused: {r:?}"),
    }
}

fn refused(i: &Inputs) -> Vec<String> {
    match agenticspec_ui::derive_ui(i) {
        Ok(_) => panic!("derived"),
        Err(Failure::Unreadable(e)) => panic!("unreadable: {e}"),
        Err(Failure::Refused(r)) => r.iter().map(|r| r.path.clone()).collect(),
    }
}

fn document(d: &Derivation, out: &Path) -> (String, Value) {
    let text = d
        .files
        .iter()
        .find(|(p, _)| p == out)
        .map(|(_, t)| t.clone())
        .expect("the document is among the files");
    let doc = serde_yaml_ng::from_str(&text).unwrap();
    (text, doc)
}

fn workstation() -> (String, Value) {
    let out = root().join("ui/views/examples/workstation.ui.yaml");
    let d = derived(&workstation_inputs(out.clone(), None));
    document(&d, &out)
}

fn sections<'a>(doc: &'a Value, page: &str) -> &'a Vec<Value> {
    doc["pages"][page]["sections"]
        .as_sequence()
        .unwrap_or_else(|| panic!("page {page} has sections"))
}

fn names(list: &[Value]) -> Vec<String> {
    list.iter()
        .map(|s| s["name"].as_str().unwrap_or_default().to_string())
        .collect()
}

fn components(doc: &Value, page: &str) -> Vec<String> {
    sections(doc, page)
        .iter()
        .filter_map(|s| s["component"].as_str().map(str::to_string))
        .collect()
}

/// Every row action and header action of the document: (page, action).
fn actions(doc: &Value) -> Vec<(String, Value)> {
    let mut out = Vec::new();
    for (name, page) in doc["pages"].as_mapping().unwrap() {
        let name = name.as_str().unwrap().to_string();
        for a in page["header"]["actions"]
            .as_sequence()
            .into_iter()
            .flatten()
        {
            out.push((name.clone(), a.clone()));
        }
        for s in page["sections"].as_sequence().into_iter().flatten() {
            for a in s["row_actions"].as_sequence().into_iter().flatten() {
                out.push((
                    format!("{name}/{}", s["name"].as_str().unwrap_or_default()),
                    a.clone(),
                ));
            }
        }
    }
    out
}

#[test]
fn the_committed_examples_are_current_and_derived_byte_for_byte_again() {
    let r = root();
    let ws = workstation_inputs(r.join("ui/views/examples/workstation.ui.yaml"), None);
    let large = inputs(
        &r.join("crates/agenticspec-render/tests/fixtures/large-37.yaml"),
        None,
        None,
        r.join("ui/views/examples/large-37.ui.yaml"),
    );
    for i in [ws, large] {
        let a = derived(&i);
        let b = derived(&i);
        assert_eq!(a.files, b.files, "deterministic");
        let stale = derive::differences(&a);
        assert!(stale.is_empty(), "stale: {stale:?}; run task ui:derive");
    }
}

#[test]
fn every_copied_widget_equals_its_panel_in_blocks_ui_yaml() {
    let panels: Value = serde_yaml_ng::from_str(
        &std::fs::read_to_string(root().join("ui/views/blocks.ui.yaml")).unwrap(),
    )
    .unwrap();
    let (_, doc) = workstation();
    let widgets = doc["widgets"].as_mapping().unwrap();
    assert!(widgets.len() > 10);
    for (name, w) in widgets {
        assert_eq!(&panels["widgets"][name], w, "{name:?}");
    }
}

#[test]
fn two_derived_documents_generate_the_same_panel_files_as_the_gallery() {
    let generate_file = |rel: &str| {
        let text = std::fs::read_to_string(root().join(rel)).unwrap();
        agenticspec_ui::generate(&text).unwrap_or_else(|r| panic!("{rel}: {r:?}"))
    };
    let gallery = generate_file("ui/views/blocks.ui.yaml");
    let mut shared = 0;
    let ws = generate_file("ui/views/examples/workstation.ui.yaml");
    let large = generate_file("ui/views/examples/large-37.ui.yaml");
    for files in [&ws, &large] {
        for (path, text) in files.iter().filter(|(p, _)| p.starts_with("widgets/")) {
            assert_eq!(gallery.get(path), Some(text), "{path}");
            if ws.get(path) == large.get(path) {
                shared += 1;
            }
        }
        assert_eq!(files.get("runtime.ts"), gallery.get("runtime.ts"));
    }
    assert!(shared > 0, "the two documents share a panel");
}

#[test]
fn the_overview_orders_nested_first_reads_reach_by_architecture_and_shows_one_level() {
    let dir = scratch("overview");
    let out = dir.join("parent.ui.yaml");
    let arch = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/derive/parent.yaml");
    let d = derived(&inputs(&arch, None, None, out.clone()));
    let (_, doc) = document(&d, &out);
    let s = sections(&doc, "overview");
    assert_eq!(names(s), ["parts", "reach", "graph", "settings"]);
    assert_eq!(
        s[1]["reads"]["view"].as_str(),
        Some("agentic.architecture.Reach")
    );
    assert_eq!(
        s[1]["reads"]["params"]["architecture_id"].as_str(),
        Some("'parent'")
    );
    assert_eq!(
        s[2]["reads"]["params"]["architecture"].as_str(),
        Some("'parent'")
    );
    assert!(
        s[3]["host"].as_str().is_some() && s[3]["does"].is_null(),
        "read-only settings"
    );
    let parts = d
        .files
        .iter()
        .find(|(p, _)| p.ends_with("parent.fixtures/parts.json"))
        .map(|(_, t)| serde_json::from_str::<serde_json::Value>(t).unwrap())
        .unwrap();
    let rows = parts["rows"].as_array().unwrap();
    let top = &rows[0]["architecture_id"];
    let order: Vec<&str> = rows
        .iter()
        .filter(|r| &r["architecture_id"] == top)
        .map(|r| r["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        order,
        ["team", "lead", "chat", "owner"],
        "nested, then block, then name"
    );
}

#[test]
fn a_nested_architecture_has_its_own_overview_and_section_and_takes_an_instance() {
    let dir = scratch("nested");
    let out = dir.join("parent.ui.yaml");
    let arch = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/derive/parent.yaml");
    let d = derived(&inputs(&arch, None, None, out.clone()));
    let (_, doc) = document(&d, &out);
    let nav = doc["navigation"]["sections"].as_sequence().unwrap();
    let team = nav
        .iter()
        .find(|s| s["name"].as_str() == Some("nested_team"))
        .expect("a navigation section of the nested level");
    assert_eq!(team["pages"][0].as_str(), Some("team.overview"));
    for page in ["team.overview", "part.team.coder", "part.team.tree"] {
        assert_eq!(
            doc["pages"][page]["state"]["instance"]["store"].as_str(),
            Some("url"),
            "{page} takes an instance"
        );
    }
    assert!(doc["pages"]["part.lead"]["state"]["instance"].is_null());
    assert_eq!(d.channels, 2, "a channel per level");
}

#[test]
fn the_operator_page_lists_its_instances_first_and_the_harness_in_a_collapsed_frame() {
    let (_, doc) = workstation();
    let page = &doc["pages"]["part.operator"];
    let comps = components(&doc, "part.operator");
    assert_eq!(comps[0], "agentic.agent.Agents");
    for v in ["HarnessSessions", "Turns", "ToolCalls"] {
        assert!(
            comps.contains(&format!("agentic.harness.{v}")),
            "{v}: {comps:?}"
        );
    }
    let place = page["layout"]["areas"]["place"].as_mapping().unwrap();
    let harness = place
        .values()
        .find(|p| p["title"].as_str() == Some("Harness"))
        .expect("a frame of the harness");
    assert_eq!(harness["frame"].as_str(), Some("shared"));
    assert_eq!(harness["collapsed"].as_bool(), Some(true));
    let grid = page["layout"]["areas"]["grid"].as_sequence().unwrap();
    assert_eq!(grid[0][0].as_str(), Some("area_instances"));
}

#[test]
fn a_part_run_once_per_worker_is_shown_on_the_worker_page() {
    let (_, doc) = workstation();
    let place = &doc["pages"]["part.worker"]["layout"]["areas"]["place"]["area_per"];
    let secs: Vec<&str> = place["sections"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap())
        .collect();
    assert!(secs.iter().any(|s| s.contains("workspace")), "{secs:?}");
    assert!(secs.iter().any(|s| s.contains("sandbox")), "{secs:?}");
    assert_eq!(place["title"].as_str(), Some("One per worker: box, tree"));
}

#[test]
fn the_channel_carries_the_events_of_the_page_domains_resumed_per_architecture() {
    let (_, doc) = workstation();
    let ch = &doc["channels"]["level"];
    assert_eq!(ch["resume"].as_str(), Some("from_last_seen"));
    assert_eq!(ch["scope"].as_str(), Some("param"));
    assert_eq!(ch["session"]["per"].as_str(), Some("architecture_id"));
    let events: Vec<&str> = ch["carries"]["events"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|e| e.as_str().unwrap())
        .collect();
    assert!(events.iter().any(|e| e.starts_with("agentic.harness.")));
    assert!(events.iter().any(|e| e.starts_with("agentic.workspace.")));
    assert!(
        !events.iter().any(|e| e.starts_with("agentic.work.")),
        "no domain of no page"
    );
}

#[test]
fn principal_ports_become_row_actions_and_a_border_link_a_header_action() {
    let (_, doc) = workstation();
    let all = actions(&doc);
    let on = |cmd: &str| -> Vec<String> {
        all.iter()
            .filter(|(_, a)| a["does"].as_str() == Some(cmd))
            .map(|(p, _)| p.clone())
            .collect()
    };
    assert!(
        on("agentic.question.Answer")
            .iter()
            .all(|p| p.contains("open_questions"))
    );
    assert!(!on("agentic.question.Answer").is_empty());
    // The workstation's contract gives the owner no `authority.decision` port: no decision to
    // approve or reject is the owner's.
    for c in [
        "agentic.authority.ApproveDecision",
        "agentic.authority.RejectDecision",
    ] {
        assert!(on(c).is_empty(), "{c}");
    }
    assert_eq!(on("agentic.channel.SendMessage"), ["part.operator"]);
    assert!(
        all.iter()
            .all(|(_, a)| !a["does"].as_str().unwrap_or("").contains("Inference"))
    );
    for (p, a) in &all {
        let does = a["does"].as_str().unwrap();
        assert_eq!(
            a["visible"].as_str(),
            Some(format!("actor.may({does})").as_str()),
            "{p}"
        );
    }
}

#[test]
fn a_renamed_intent_command_is_the_command_the_row_action_calls() {
    let dir = scratch("renamed");
    let ports = std::fs::read_to_string(root().join("ui/views/examples/ports.json"))
        .unwrap()
        .replace("agentic.question.Answer", "agentic.question.Withdraw");
    let file = dir.join("ports.json");
    std::fs::write(&file, ports).unwrap();
    let out = dir.join("workstation.ui.yaml");
    let i = workstation_inputs(out.clone(), Some(file));
    let d = derived(&i);
    let (_, doc) = document(&d, &out);
    let calls: Vec<(String, String)> = actions(&doc)
        .iter()
        .filter_map(|(p, a)| a["does"].as_str().map(|c| (p.clone(), c.to_string())))
        .collect();
    assert!(
        calls
            .iter()
            .any(|(p, c)| c == "agentic.question.Withdraw" && p.contains("open_questions")),
        "{calls:?}"
    );
    assert!(
        !calls.iter().any(|(_, c)| c == "agentic.question.Answer"),
        "{calls:?}"
    );
}

#[test]
fn the_view_key_labels_a_part() {
    let (text, doc) = workstation();
    assert_eq!(
        doc["pages"]["part.worker"]["nav"]["label"].as_str(),
        Some("coding agents")
    );
    assert!(text.contains("label: People"));
}

fn with_view(name: &str, view: &str) -> Inputs {
    let dir = scratch(name);
    let src = std::fs::read_to_string(root().join("architectures/workstation.yaml")).unwrap();
    let cut = src.find("\nview:").map_or(src.len(), |i| i + 1);
    let arch = dir.join("workstation.yaml");
    std::fs::write(&arch, format!("{}{view}", &src[..cut])).unwrap();
    inputs(&arch, None, None, dir.join("workstation.ui.yaml"))
}

#[test]
fn a_view_key_naming_an_unknown_part_is_refused_by_its_node_path() {
    let i = with_view(
        "unknown",
        "view:\n  labels:\n    parts:\n      nope: Nope\n",
    );
    assert_eq!(refused(&i), ["view/labels/parts/nope"]);
}

#[test]
fn a_view_key_holding_a_filter_is_refused_by_its_node_path() {
    let i = with_view(
        "filter",
        "view:\n  labels:\n    parts:\n      worker: { label: x, filter: \"row.a\" }\n",
    );
    let paths = refused(&i);
    assert!(
        paths
            .iter()
            .any(|p| p.starts_with("view/labels/parts/worker")),
        "{paths:?}"
    );
}
