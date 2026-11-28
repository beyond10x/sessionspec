//! One case per refusal of the structural check, and the accepted documents.

use super::common::{ARCH, repo_root, yaml};
use agenticspec_sim::check::{GATE_KPIS, Refusal, check};
use serde_json::{Value, json};

fn refused(arch: &Value) -> Vec<(String, String)> {
    check(arch)
        .iter()
        .map(|r| (r.rule().to_string(), r.part().to_string()))
        .collect()
}

fn base() -> Value {
    yaml(ARCH)
}

#[test]
fn the_small_architecture_is_accepted() {
    assert_eq!(check(&base()), Vec::<Refusal>::new());
}

#[test]
fn the_workstation_architecture_is_accepted() {
    let text = std::fs::read_to_string(repo_root().join("architectures/workstation.yaml")).unwrap();
    assert_eq!(check(&yaml(&text)), Vec::<Refusal>::new());
}

#[test]
fn a_link_to_an_unknown_part_is_refused() {
    let mut a = base();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "worker", "to": "ghost", "carries": "claim"}));
    assert_eq!(
        refused(&a),
        vec![("unknown_link_parts".into(), "ghost".into())]
    );
}

#[test]
fn a_link_over_an_unknown_channel_is_refused() {
    let mut a = base();
    a["links"][0]["over"] = json!("radio");
    assert_eq!(
        refused(&a),
        vec![("unknown_link_parts".into(), "radio".into())]
    );
}

#[test]
fn per_naming_no_part_is_refused() {
    let mut a = base();
    a["parts"]["tree"]["per"] = json!("builder");
    assert_eq!(
        refused(&a),
        vec![("per_without_part".into(), "tree".into())]
    );
}

#[test]
fn acts_for_naming_no_part_is_refused() {
    let mut a = base();
    a["parts"]["operator"]["acts_for"] = json!("boss");
    assert_eq!(
        refused(&a),
        vec![("acts_for_without_part".into(), "operator".into())]
    );
}

#[test]
fn egress_naming_no_part_is_refused() {
    let mut a = base();
    a["parts"]["box"]["egress"] = json!(["gateway", "internet"]);
    assert_eq!(
        refused(&a),
        vec![("egress_without_part".into(), "box".into())]
    );
}

#[test]
fn an_agent_without_a_model_call_path_is_refused() {
    let mut a = base();
    let links = a["links"].as_array_mut().unwrap();
    links.retain(|l| !(l["from"] == "worker" && l["carries"] == "model_call"));
    assert_eq!(
        refused(&a),
        vec![("agent_without_model_path".into(), "worker".into())]
    );
}

#[test]
fn a_model_call_path_that_ends_before_a_model_is_refused_for_every_agent() {
    let mut a = base();
    let links = a["links"].as_array_mut().unwrap();
    links.retain(|l| !(l["from"] == "gateway"));
    let mut got = refused(&a);
    got.sort();
    assert_eq!(
        got,
        vec![
            ("agent_without_model_path".into(), "operator".into()),
            ("agent_without_model_path".into(), "worker".into()),
            ("gateway_without_model".into(), "gateway".into()),
            ("unreachable_parts".into(), "model".into()),
        ]
    );
}

#[test]
fn a_question_to_a_part_that_cannot_answer_is_refused() {
    let mut a = base();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "worker", "to": "builds", "carries": "question"}));
    assert_eq!(
        refused(&a),
        vec![("question_without_answerer".into(), "builds".into())]
    );
}

#[test]
fn a_border_naming_no_part_is_refused() {
    let mut a = base();
    a["border"] = json!(["operator", "firewall"]);
    assert_eq!(
        refused(&a),
        vec![("border_without_part".into(), "firewall".into())]
    );
}

#[test]
fn a_part_nothing_reaches_is_refused() {
    let mut a = base();
    a["parts"]["spare"] = json!({"block": "pool", "kind": "build_slots", "capacity": 4});
    assert_eq!(
        refused(&a),
        vec![("unreachable_parts".into(), "spare".into())]
    );
}

#[test]
fn every_rule_is_a_gate_kpi() {
    for rule in [
        "unknown_link_parts",
        "per_without_part",
        "acts_for_without_part",
        "egress_without_part",
        "agent_without_model_path",
        "question_without_answerer",
        "border_without_part",
        "unreachable_parts",
    ] {
        assert!(GATE_KPIS.contains(&rule), "{rule} is not a gate KPI");
    }
}

fn with_reviewer(a: &mut Value, role: Option<&str>) {
    let mut reviewer = json!({"block": "agent", "harness": "coding"});
    if let Some(r) = role {
        reviewer["role"] = json!(r);
    }
    a["parts"]["reviewer"] = reviewer;
    let links = a["links"].as_array_mut().unwrap();
    links.push(json!({"from": "worker", "to": "reviewer", "carries": "review"}));
    links.push(json!({"from": "reviewer", "to": "gateway", "carries": "model_call"}));
}

#[test]
fn a_review_link_to_a_reviewer_is_accepted() {
    let mut a = base();
    with_reviewer(&mut a, Some("reviewer"));
    assert_eq!(check(&a), Vec::<Refusal>::new());
}

#[test]
fn a_review_link_to_a_part_that_is_not_a_reviewer_is_refused() {
    let mut a = base();
    with_reviewer(&mut a, None);
    assert_eq!(
        refused(&a),
        vec![("review_without_reviewer".into(), "reviewer".into())]
    );
    let mut b = base();
    b["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "worker", "to": "builds", "carries": "review"}));
    assert_eq!(
        refused(&b),
        vec![("review_without_reviewer".into(), "builds".into())]
    );
    assert!(GATE_KPIS.contains(&"review_without_reviewer"));
}

#[test]
fn every_architecture_file_is_accepted() {
    let dir = repo_root().join("architectures");
    let mut names = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "yaml") {
            // A nested architecture is resolved from its file and checked flattened.
            let a = agenticspec_sim::nest::load(&path)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let a = agenticspec_sim::nest::flatten(&a);
            assert_eq!(check(&a), Vec::<Refusal>::new(), "{}", path.display());
            names.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }
    for want in [
        "workstation.yaml",
        "solo.yaml",
        "parallel-unaware.yaml",
        "handoff.yaml",
        "author-reviewer.yaml",
        "scope-partition.yaml",
        "planner-executor.yaml",
        "scheduled-loops.yaml",
        "shared-board.yaml",
        "orchestrator-workers.yaml",
        "team-with-console.yaml",
        "console.yaml",
    ] {
        assert!(names.iter().any(|n| n == want), "{want} missing");
    }
}

/// The variants of an enum of `agentic.architecture`, in snake case, read from the block.
fn domain_variants(name: &str) -> Vec<String> {
    let text = std::fs::read_to_string(repo_root().join("blocks/9-composition/architecture.yaml"))
        .unwrap();
    let doc = yaml(&text);
    let t = doc["types"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == name)
        .unwrap_or_else(|| panic!("{name} is declared"));
    t["variants"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let mut out = String::new();
            for (i, c) in v.as_str().unwrap().chars().enumerate() {
                if c.is_ascii_uppercase() && i > 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            }
            out
        })
        .collect()
}

#[test]
fn the_checker_and_the_domain_agree_on_part_kinds_and_carries() {
    assert_eq!(
        agenticspec_sim::check::part_kinds(),
        domain_variants("agentic.architecture.PartKind")
    );
    assert_eq!(
        agenticspec_sim::check::carries(),
        domain_variants("agentic.architecture.Carries")
    );
    assert!(
        agenticspec_sim::check::part_kinds()
            .iter()
            .any(|k| k == "architecture")
    );
}

#[test]
fn every_block_and_carries_written_under_architectures_is_declared() {
    let root = repo_root().join("architectures");
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for dir in [root.clone(), root.join("ladder")] {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "yaml") {
                files.push(path);
            }
        }
    }
    assert!(files.len() > 15, "{}", files.len());
    for path in files {
        let a = yaml(&std::fs::read_to_string(&path).unwrap());
        let unknown: Vec<String> = check(&a)
            .iter()
            .filter(|r| matches!(r.rule(), "unknown_blocks" | "unknown_carries"))
            .map(ToString::to_string)
            .collect();
        assert!(unknown.is_empty(), "{}: {unknown:?}", path.display());
    }
}

#[test]
fn a_block_the_domain_does_not_declare_is_refused() {
    let mut a = base();
    a["parts"]["robot"] = json!({"block": "robot"});
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "operator", "to": "robot", "carries": "order"}));
    assert_eq!(refused(&a), vec![("unknown_blocks".into(), "robot".into())]);
}

#[test]
fn a_carries_the_domain_does_not_declare_is_refused() {
    let mut a = base();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "worker", "to": "operator", "carries": "telepathy"}));
    let r = check(&a);
    assert_eq!(
        refused(&a),
        vec![("unknown_carries".into(), "worker".into())]
    );
    assert!(r[0].to_string().contains("telepathy"), "{}", r[0]);
}

/// A field's architecture places the blocks of its packs (`scm.change`) and its links to them
/// carry what the pack names; a link between parts of the core still carries only the core's
/// vocabulary.
#[test]
fn a_pack_block_is_a_part_and_a_link_to_it_carries_what_the_pack_names() {
    assert!(
        agenticspec_sim::check::pack_blocks()
            .iter()
            .any(|b| b == "scm.change")
    );
    let mut a = base();
    a["parts"]["changes"] = json!({"block": "scm.change"});
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "operator", "to": "changes", "carries": "merge"}));
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "worker", "to": "operator", "carries": "merge"}));
    assert_eq!(
        refused(&a),
        vec![("unknown_carries".into(), "worker".into())]
    );
    let field = repo_root().join("fields/coding/architectures/auto-merger.yaml");
    let doc = yaml(&std::fs::read_to_string(field).unwrap());
    assert_eq!(check(&doc), Vec::<Refusal>::new());
}

#[test]
fn the_kind_rules_are_gate_kpis() {
    for rule in ["unknown_blocks", "unknown_carries"] {
        assert!(GATE_KPIS.contains(&rule), "{rule} is not a gate KPI");
    }
}

/// The scheduled sweeper of the swarm cases, as a document to break.
fn scheduled() -> Value {
    yaml(super::swarm::SCHEDULED)
}

#[test]
fn the_trigger_architectures_are_accepted() {
    assert_eq!(check(&scheduled()), Vec::<Refusal>::new());
    assert_eq!(check(&yaml(super::swarm::SHARED)), Vec::<Refusal>::new());
}

#[test]
fn a_trigger_link_from_a_part_that_is_not_a_trigger_is_refused() {
    let mut a = scheduled();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "owner", "to": "sweeper", "carries": "trigger"}));
    assert_eq!(
        refused(&a),
        vec![("trigger_source_not_trigger".into(), "owner".into())]
    );
}

#[test]
fn a_schedule_may_wake_an_agent_without_a_trigger_part() {
    let mut a = scheduled();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "hourly", "to": "sweeper", "carries": "trigger"}));
    assert_eq!(check(&a), Vec::<Refusal>::new());
}

#[test]
fn a_trigger_link_to_a_part_that_is_not_an_agent_is_refused() {
    let mut a = scheduled();
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "wake", "to": "model", "carries": "trigger"}));
    assert_eq!(
        refused(&a),
        vec![("trigger_target_not_agent".into(), "model".into())]
    );
}

#[test]
fn a_trigger_without_its_source_is_refused() {
    let mut a = scheduled();
    a["parts"]["wake"]["schedule"] = json!("nightly");
    assert_eq!(
        refused(&a),
        vec![
            ("trigger_without_source".into(), "wake".into()),
            ("unreachable_parts".into(), "hourly".into()),
        ]
    );
    let mut b = yaml(super::swarm::SHARED);
    b["parts"]["inbox"]["distribution"] = json!("direct");
    assert_eq!(
        refused(&b),
        vec![("trigger_without_source".into(), "on_post".into())],
        "a direct channel has no subscribers"
    );
    let mut c = scheduled();
    c["parts"]["wake"]["kind"] = json!("on_event");
    assert_eq!(
        refused(&c),
        vec![("trigger_without_source".into(), "wake".into())],
        "an on_event trigger names its event"
    );
}

#[test]
fn more_than_one_run_under_skip_or_queue_is_refused() {
    for (policy, max, want) in [
        ("skip", 2, true),
        ("queue", 3, true),
        ("parallel", 3, false),
        ("parallel", 0, true),
        ("sometimes", 1, true),
    ] {
        let mut a = scheduled();
        a["parts"]["wake"]["policy"] = json!(policy);
        a["parts"]["wake"]["max_parallel"] = json!(max);
        let r = refused(&a);
        let concurrency = ("invalid_concurrency".to_string(), "wake".to_string());
        // A policy that is no ConcurrencyPolicy is also a value outside the setting's type.
        let typed = ("setting_invalid".to_string(), "wake".to_string());
        let expected = match (want, policy) {
            (false, _) => vec![],
            (true, "sometimes") => vec![typed, concurrency],
            (true, _) => vec![concurrency],
        };
        assert_eq!(r, expected, "{policy} {max}");
    }
}

#[test]
fn a_schedule_without_a_positive_interval_is_refused() {
    let mut a = scheduled();
    a["parts"]["hourly"]
        .as_object_mut()
        .unwrap()
        .remove("every");
    assert_eq!(
        refused(&a),
        vec![("invalid_schedule".into(), "hourly".into())]
    );
}

#[test]
fn swarm_settings_on_a_part_that_is_not_an_agent_are_refused() {
    let mut a = scheduled();
    a["parts"]["model"]["coordination"] = json!("mesh");
    assert_eq!(
        refused(&a),
        vec![
            ("setting_unknown".into(), "model".into()),
            ("swarm_needs_agent".into(), "model".into())
        ]
    );
    let mut b = scheduled();
    b["parts"]["sweeper"]["coordination"] = json!("hive");
    assert_eq!(
        refused(&b),
        vec![
            ("setting_invalid".into(), "sweeper".into()),
            ("swarm_needs_agent".into(), "sweeper".into())
        ],
        "an unknown coordination"
    );
}

#[test]
fn the_trigger_rules_are_gate_kpis() {
    for rule in [
        "trigger_source_not_trigger",
        "trigger_target_not_agent",
        "trigger_without_source",
        "invalid_concurrency",
        "invalid_schedule",
        "swarm_needs_agent",
    ] {
        assert!(GATE_KPIS.contains(&rule), "{rule} is not a gate KPI");
    }
}
