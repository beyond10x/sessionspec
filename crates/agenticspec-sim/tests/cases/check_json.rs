//! `sim check --json`: the `agenticspec-check/1` document, one field per test, over a refused
//! architecture and an architecture of the catalog.

use super::common::{repo_root, scratch};
use agenticspec_sim::check::GATE_KPIS;
use agenticspec_sim::cli;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const REFUSED: &str = r#"
format: agenticspec-architecture/1
architecture: refused
parts:
  you:   { block: principal, kind: human }
  coder: { block: agent, harness: coding }
  stray: { block: pool, kind: build_slots, capacity: 1 }
links:
  - { from: you, to: coder,  carries: prompt }
  - { from: you, to: nobody, carries: prompt }
"#;

fn refused_path(test: &str) -> PathBuf {
    let dir = scratch(&format!("check-json-{test}"));
    let path = dir.join("refused.yaml");
    std::fs::write(&path, REFUSED).unwrap();
    path
}

fn catalog_path() -> PathBuf {
    repo_root().join("architectures/author-reviewer.yaml")
}

/// `sim check --json` with the default ports file and KPI catalogue.
fn json_of(path: &Path) -> Result<cli::Checked, cli::Refusal> {
    cli::check_json(&cli::CheckOptions {
        architecture: path,
        ports: None,
        kpis: Some(&repo_root().join("kpis/catalogue.yaml")),
    })
}

fn refused(test: &str) -> cli::Checked {
    json_of(&refused_path(test)).unwrap()
}

fn catalog() -> cli::Checked {
    json_of(&catalog_path()).unwrap()
}

#[test]
fn check_json_names_its_format() {
    assert_eq!(cli::CHECK_FORMAT, "agenticspec-check/1");
    assert_eq!(refused("format").document["format"], cli::CHECK_FORMAT);
    assert_eq!(catalog().document["format"], cli::CHECK_FORMAT);
}

#[test]
fn check_json_carries_the_summary_of_the_text_output() {
    let text = cli::check(&catalog_path()).unwrap();
    let doc = catalog().document;
    for key in ["architecture", "parts", "links"] {
        assert_eq!(doc[key], text[key], "{key}");
    }
    let doc = refused("summary").document;
    assert_eq!(
        (&doc["architecture"], &doc["parts"], &doc["links"]),
        (&json!("refused"), &json!(3), &json!(2))
    );
}

#[test]
fn check_json_valid_is_false_only_when_a_gate_is_violated() {
    assert_eq!(refused("valid").document["valid"], false);
    assert_eq!(catalog().document["valid"], true);
}

#[test]
fn check_json_lists_each_violation_sorted_with_its_subject() {
    assert_eq!(catalog().document["violations"], json!([]));
    assert_eq!(
        refused("violations").document["violations"],
        json!([
            {
                "rule": "agent_without_model_path",
                "gate": true,
                "subject": "coder",
                "part": "coder",
                "message": "an agent with no model_call path to a model"
            },
            {
                "rule": "unknown_link_parts",
                "gate": true,
                "subject": "you->nobody:prompt",
                "part": "nobody",
                "link": 1,
                "message": "link 1 names a part that does not exist"
            },
            {
                "rule": "unreachable_parts",
                "gate": true,
                "subject": "stray",
                "part": "stray",
                "message": "nothing links to this part"
            }
        ])
    );
}

#[test]
fn check_json_marks_every_rule_of_the_check_as_a_gate() {
    assert!(GATE_KPIS.iter().all(|r| cli::is_gate(r)));
    assert!(!cli::is_gate("no_such_rule"));
}

#[test]
fn check_json_counts_every_rule() {
    let counts = refused("counts").document["counts"].clone();
    let counts = counts.as_object().unwrap();
    let keys: Vec<&str> = counts.keys().map(String::as_str).collect();
    let mut rules = GATE_KPIS.to_vec();
    rules.sort_unstable();
    assert_eq!(keys, rules);
    for (rule, n) in counts {
        let want = match rule.as_str() {
            "agent_without_model_path" | "unknown_link_parts" | "unreachable_parts" => 1,
            _ => 0,
        };
        assert_eq!(n, &json!(want), "{rule}");
    }
    let clean = catalog().document["counts"].clone();
    let clean = clean.as_object().unwrap();
    assert_eq!(clean.len(), GATE_KPIS.len());
    assert!(clean.values().all(|n| n == 0));
}

#[test]
fn check_json_carries_no_other_field() {
    let keys = |doc: &Value| -> Vec<String> { doc.as_object().unwrap().keys().cloned().collect() };
    assert_eq!(
        keys(&refused("fields").document),
        [
            "architecture",
            "counts",
            "format",
            "links",
            "parts",
            "valid",
            "violations"
        ]
    );
    // An accepted architecture also carries what `sim check` prints.
    assert_eq!(
        keys(&catalog().document),
        [
            "architecture",
            "counts",
            "format",
            "ingresses",
            "kpis",
            "links",
            "nested",
            "parts",
            "requirements",
            "single_points",
            "valid",
            "violations"
        ]
    );
    let text = cli::check(&catalog_path()).unwrap();
    for key in keys(&text) {
        assert_eq!(catalog().document[&key], text[&key], "{key}");
    }
}

#[test]
fn check_json_refuses_with_the_text_outputs_refusal() {
    let text = cli::check(&refused_path("refusal-text")).unwrap_err();
    let json = refused("refusal").refusal.unwrap();
    assert_eq!(json, text);
    assert_eq!(json.code, "architecture_refused");
    assert_eq!(catalog().refusal, None);
}

#[test]
fn check_json_refuses_a_file_it_cannot_read_without_a_document() {
    let err = json_of(&scratch("check-json-missing").join("missing.yaml")).unwrap_err();
    assert_eq!(err.code, "document_unreadable");
}

#[test]
fn check_json_prints_the_same_bytes_on_every_run() {
    let once: Vec<String> = (0..3)
        .map(|_| refused("bytes").document.to_string())
        .collect();
    assert!(once.windows(2).all(|w| w[0] == w[1]), "{once:#?}");
    let reordered = REFUSED.replace(
        "  - { from: you, to: coder,  carries: prompt }\n  - { from: you, to: nobody, carries: prompt }\n",
        "  - { from: you, to: nobody, carries: prompt }\n  - { from: you, to: coder,  carries: prompt }\n",
    );
    assert_ne!(reordered, REFUSED);
    let path = scratch("check-json-reordered").join("refused.yaml");
    std::fs::write(&path, reordered).unwrap();
    let doc: Value = json_of(&path).unwrap().document;
    let rules: Vec<&str> = doc["violations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["rule"].as_str().unwrap())
        .collect();
    let mut sorted = rules.clone();
    sorted.sort_unstable();
    assert_eq!(rules, sorted);
}

#[test]
fn check_json_names_the_border_part_and_the_question_link_no_principal_reaches() {
    let dir = scratch("check-json-principal");
    let path = dir.join("unreached.yaml");
    std::fs::write(
        &path,
        r#"
format: agenticspec-architecture/1
architecture: unreached
parts:
  you:     { block: principal, kind: human }
  lead:    { block: agent, harness: coding }
  helper:  { block: agent, harness: coding }
  model:   { block: model, route: frontier }
links:
  - { from: you,    to: helper, carries: prompt }
  - { from: helper, to: lead,   carries: question }
  - { from: lead,   to: model,  carries: model_call }
  - { from: helper, to: model,  carries: model_call }
border: [lead]
"#,
    )
    .unwrap();
    let doc = json_of(&path).unwrap().document;
    assert_eq!(doc["valid"], false);
    assert_eq!(doc["counts"]["not_reached_by_principal"], 1);
    assert_eq!(doc["counts"]["question_without_answerer"], 1);
    let got: Vec<(&str, &str, &str)> = doc["violations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v["rule"].as_str().unwrap(),
                v["subject"].as_str().unwrap(),
                v["message"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            (
                "not_reached_by_principal",
                "lead",
                "on the border, and no principal part links to it and no grant link reaches it"
            ),
            (
                "question_without_answerer",
                "helper->lead:question",
                "link 1 sends a question to an agent that acts for no principal and forwards questions to no principal"
            ),
        ]
    );
}
