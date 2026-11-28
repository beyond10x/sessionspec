//! Nested architectures: a part of block `architecture` refers to another architecture document
//! by path and digest, links address its border parts as `<part>.<border part>`, the check
//! resolves the references and the simulator flattens them.

use super::common::{PROFILE, repo_root, scratch, yaml};
use agenticspec_sim::nest::{self, Error};
use agenticspec_sim::{Experiment, cli, run};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const INNER: &str = r#"
format: agenticspec-architecture/1
architecture: console
parts:
  steward: { block: agent, harness: coding }
  helper:  { block: agent, harness: coding, count: { value: 1, vary: [1, 2] } }
  model:   { block: model, route: frontier }
links:
  - { from: steward, to: helper, carries: order }
  - { from: helper, to: steward, carries: question }
  - { from: steward, to: model, carries: model_call }
  - { from: helper, to: model, carries: model_call }
border: [steward]
"#;

const OUTER: &str = r#"
format: agenticspec-architecture/1
architecture: team
parts:
  owner:   { block: principal, kind: human }
  console: { block: architecture, ref: REF, digest: DIGEST }
  worker:  { block: agent, harness: coding, count: 2 }
  model:   { block: model, route: frontier }
links:
  - { from: owner, to: console.steward, carries: prompt }
  - { from: console.steward, to: owner, carries: question }
  - { from: console.steward, to: worker, carries: order }
  - { from: worker, to: console.steward, carries: question }
  - { from: worker, to: model, carries: model_call }
border: [console.steward]
policies:
  question_default_after: 15m
"#;

/// The outer document flattened by hand: what the simulator is said to simulate.
const FLAT: &str = r#"
format: agenticspec-architecture/1
architecture: team
parts:
  owner:           { block: principal, kind: human }
  console.steward: { block: agent, harness: coding }
  console.helper:  { block: agent, harness: coding, count: 1 }
  console.model:   { block: model, route: frontier }
  worker:          { block: agent, harness: coding, count: 2 }
  model:           { block: model, route: frontier }
links:
  - { from: owner, to: console.steward, carries: prompt }
  - { from: console.steward, to: owner, carries: question }
  - { from: console.steward, to: worker, carries: order }
  - { from: worker, to: console.steward, carries: question }
  - { from: worker, to: model, carries: model_call }
  - { from: console.steward, to: console.helper, carries: order }
  - { from: console.helper, to: console.steward, carries: question }
  - { from: console.steward, to: console.model, carries: model_call }
  - { from: console.helper, to: console.model, carries: model_call }
border: [console.steward]
policies:
  question_default_after: 15m
"#;

const EXPERIMENT: &str = r#"
format: agenticspec-experiment/1
experiment: nested
architecture: ARCH
profile: profile.yaml
horizon: 4h
seeds: 2
seed: 11
search:
  mode: fit
  budget: 1
  weights:
    tasks_done: 1
"#;

/// Writes `console.yaml` and `team.yaml` (pinned to the console's digest, or to `digest`).
fn pair(name: &str, inner: &str, outer_edit: impl Fn(String) -> String) -> PathBuf {
    let dir = scratch(&format!("nest-{name}"));
    std::fs::write(dir.join("console.yaml"), inner).unwrap();
    let d = nest::digest(&yaml(inner));
    let outer = outer_edit(OUTER.replace("REF", "console.yaml").replace("DIGEST", &d));
    std::fs::write(dir.join("team.yaml"), outer).unwrap();
    dir.join("team.yaml")
}

fn refusals(path: &Path) -> Vec<(String, String, String)> {
    match nest::load(path) {
        Err(Error::Refused(list)) => list
            .iter()
            .map(|r| (r.rule().to_string(), r.part().to_string(), r.to_string()))
            .collect(),
        Err(Error::Unreadable(e)) => panic!("unreadable: {e}"),
        Ok(_) => Vec::new(),
    }
}

fn rules(path: &Path) -> Vec<(String, String)> {
    refusals(path)
        .into_iter()
        .map(|(rule, part, _)| (rule, part))
        .collect()
}

#[test]
fn the_digest_is_the_sha256_of_the_canonical_parts_links_and_border() {
    // `{"parts":{"a":[true,"x"],"b":1}}`: keys sorted, no whitespace, only the digested keys.
    let want = "sha256:5eb69c97d120faa33d15056c2a1db2c271c6751f8cb10aac249a2a0e82983414";
    assert_eq!(
        nest::digest(&json!({"summary": "left out", "parts": {"b": 1, "a": [true, "x"]}})),
        want
    );
    let dir = scratch("nest-digest");
    std::fs::write(
        dir.join("a.yaml"),
        "# a comment\nsummary: left out\nparts:\n  b: 1\n  a:\n    - true\n    - x\n",
    )
    .unwrap();
    let out = cli::digest(&dir.join("a.yaml")).unwrap();
    assert_eq!(out["digest"], want);
    assert_eq!(
        cli::digest(&dir.join("missing.yaml")).unwrap_err().code,
        "document_unreadable"
    );
}

#[test]
fn a_reformatted_nested_file_keeps_its_pin() {
    let reformatted = format!(
        "# the same console, written in block style with its keys in another order\n{}",
        r#"
architecture: console
format: agenticspec-architecture/1
border:
  - steward
links:
  - from: steward
    to: helper
    carries: order
  - { carries: question, from: helper, to: steward }
  - { from: steward, to: model, carries: model_call }
  - { from: helper, to: model, carries: model_call }
parts:
  model:   { route: frontier, block: model }
  steward:
    harness: coding
    block: agent
  helper:  { block: agent, harness: coding, count: { vary: [1, 2], value: 1 } }
"#
    );
    assert_eq!(
        nest::digest(&yaml(&reformatted)),
        nest::digest(&yaml(INNER))
    );
    let team = pair("reformatted", INNER, |o| o);
    std::fs::write(team.parent().unwrap().join("console.yaml"), &reformatted).unwrap();
    assert_eq!(rules(&team), Vec::<(String, String)>::new());
}

#[test]
fn every_declared_kind_is_accepted_and_a_made_up_one_is_refused() {
    let dir = scratch("nest-kinds");
    std::fs::write(dir.join("console.yaml"), INNER).unwrap();
    let doc = format!(
        r#"
format: agenticspec-architecture/1
architecture: kinds
parts:
  owner:   {{ block: principal, kind: human }}
  agent:   {{ block: agent, harness: coding }}
  model:   {{ block: model }}
  ask:     {{ block: question }}
  flow:    {{ block: process }}
  nightly: {{ block: schedule, every: 24h }}
  grants:  {{ block: authority }}
  spec:    {{ block: work }}
  deputy:  {{ block: delegation }}
  console: {{ block: architecture, ref: console.yaml, digest: {} }}
links:
  - {{ from: owner, to: agent, carries: prompt }}
  - {{ from: agent, to: model, carries: model_call }}
  - {{ from: agent, to: ask, carries: record }}
  - {{ from: owner, to: ask, carries: read }}
  - {{ from: owner, to: agent, carries: answer }}
  - {{ from: agent, to: flow, carries: record }}
  - {{ from: agent, to: spec, carries: read }}
  - {{ from: nightly, to: agent, carries: trigger }}
  - {{ from: grants, to: agent, carries: grant }}
  - {{ from: owner, to: deputy, carries: grant }}
  - {{ from: agent, to: console.steward, carries: order }}
  - {{ from: console.steward, to: owner, carries: question }}
"#,
        nest::digest(&yaml(INNER))
    );
    std::fs::write(dir.join("kinds.yaml"), &doc).unwrap();
    let out = cli::check(&dir.join("kinds.yaml")).unwrap();
    assert_eq!(out["parts"], 10);
    std::fs::write(
        dir.join("made-up.yaml"),
        doc.replace("block: process", "block: gizmo")
            .replace("to: ask, carries: read", "to: ask, carries: glance"),
    )
    .unwrap();
    let err = cli::check(&dir.join("made-up.yaml")).unwrap_err();
    assert_eq!(err.code, "architecture_refused");
    assert!(
        err.detail.contains("unknown_blocks: part `flow`"),
        "{}",
        err.detail
    );
    assert!(
        err.detail.contains("unknown_carries: part `owner`"),
        "{}",
        err.detail
    );
    assert!(err.detail.contains("glance"), "{}", err.detail);
}

#[test]
fn a_pinned_nested_architecture_is_accepted_and_listed() {
    let team = pair("accepted", INNER, |o| o);
    assert_eq!(rules(&team), Vec::<(String, String)>::new());
    let out = cli::check(&team).unwrap();
    assert_eq!(out["architecture"], "team");
    assert_eq!(out["parts"], 4);
    assert_eq!(
        out["nested"],
        json!([{
            "part": "console",
            "architecture": "console",
            "reference": "console.yaml",
            "digest": nest::digest(&yaml(INNER)),
            "parts": 3,
            "links": 4,
        }])
    );
}

#[test]
fn a_missing_file_is_refused() {
    let team = pair("missing", INNER, |o| {
        o.replace("ref: console.yaml", "ref: gone.yaml")
    });
    assert_eq!(
        rules(&team),
        vec![("nested_missing".into(), "console".into())]
    );
    let err = cli::check(&team).unwrap_err();
    assert_eq!(err.code, "architecture_refused");
    assert!(err.detail.contains("gone.yaml"), "{}", err.detail);
}

#[test]
fn a_digest_mismatch_names_expected_and_actual() {
    let wrong = format!("sha256:{}", "0".repeat(64));
    let team = pair("mismatch", INNER, |o| {
        let pinned = nest::digest(&yaml(INNER));
        o.replace(&pinned, &wrong)
    });
    let found = refusals(&team);
    assert_eq!(found.len(), 1, "{found:?}");
    let (rule, part, text) = &found[0];
    assert_eq!(
        (rule.as_str(), part.as_str()),
        ("nested_digest_mismatch", "console")
    );
    assert!(text.contains(&format!("expected {wrong}")), "{text}");
    assert!(
        text.contains(&format!("actual {}", nest::digest(&yaml(INNER)))),
        "{text}"
    );
}

#[test]
fn a_cycle_of_references_is_refused() {
    let dir = scratch("nest-cycle");
    // b refers back to a; a pins b. The cycle is found before b's pin of a is compared.
    let b = format!(
        "format: agenticspec-architecture/1\narchitecture: b\nparts:\n  back: {{ block: architecture, ref: a.yaml, digest: sha256:{} }}\n  agent: {{ block: agent, harness: coding }}\n  model: {{ block: model }}\nlinks:\n  - {{ from: agent, to: model, carries: model_call }}\n  - {{ from: agent, to: back.agent, carries: order }}\nborder: [agent]\n",
        "1".repeat(64)
    );
    std::fs::write(dir.join("b.yaml"), &b).unwrap();
    let a = format!(
        "format: agenticspec-architecture/1\narchitecture: a\nparts:\n  owner: {{ block: principal, kind: human }}\n  inner: {{ block: architecture, ref: b.yaml, digest: {} }}\nlinks:\n  - {{ from: owner, to: inner.agent, carries: prompt }}\n",
        nest::digest(&yaml(&b))
    );
    std::fs::write(dir.join("a.yaml"), &a).unwrap();
    let found = refusals(&dir.join("a.yaml"));
    assert_eq!(found.len(), 1, "{found:?}");
    let (rule, part, text) = &found[0];
    assert_eq!(
        (rule.as_str(), part.as_str()),
        ("nested_cycle", "inner.back")
    );
    assert!(text.contains("a.yaml -> b.yaml -> a.yaml"), "{text}");
}

#[test]
fn an_inner_part_off_the_border_is_not_addressable() {
    let team = pair("off-border", INNER, |o| {
        o.replace(
            "{ from: worker, to: model, carries: model_call }",
            "{ from: worker, to: model, carries: model_call }\n  - { from: worker, to: console.helper, carries: order }",
        )
    });
    let found = refusals(&team);
    assert_eq!(
        found
            .iter()
            .map(|(r, p, _)| (r.as_str(), p.as_str()))
            .collect::<Vec<_>>(),
        vec![("nested_not_in_border", "console.helper")]
    );
    assert!(found[0].2.contains("border of `console`"), "{}", found[0].2);
    let ghost = pair("ghost", INNER, |o| {
        o.replace("border: [console.steward]", "border: [console.ghost]")
    });
    assert_eq!(
        rules(&ghost),
        vec![("nested_not_in_border".into(), "console.ghost".into())]
    );
}

#[test]
fn the_inner_document_runs_its_own_check() {
    let broken = INNER.replace("border: [steward]", "border: [steward, nobody]");
    let team = pair("inner-refused", &broken, |o| o);
    let found = refusals(&team);
    assert_eq!(found.len(), 1, "{found:?}");
    let (rule, part, text) = &found[0];
    assert_eq!(
        (rule.as_str(), part.as_str()),
        ("nested_refused", "console")
    );
    assert!(text.contains("border_without_part"), "{text}");
}

#[test]
fn a_reference_without_a_digest_is_refused() {
    let team = pair("no-digest", INNER, |o| {
        let pinned = nest::digest(&yaml(INNER));
        o.replace(&format!(", digest: {pinned}"), "")
    });
    assert_eq!(
        rules(&team),
        vec![("nested_reference_invalid".into(), "console".into())]
    );
    let malformed = pair("bad-digest", INNER, |o| {
        let pinned = nest::digest(&yaml(INNER));
        o.replace(&pinned, "md5:abc")
    });
    assert_eq!(
        rules(&malformed),
        vec![("nested_reference_invalid".into(), "console".into())]
    );
}

#[test]
fn flattening_prefixes_inner_parts_and_wires_links_through_the_border() {
    let team = pair("flatten", INNER, |o| o);
    let annotated = nest::load(&team).unwrap();
    let flat = nest::flatten(&annotated);
    let mut parts: Vec<&str> = flat["parts"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    parts.sort_unstable();
    assert_eq!(
        parts,
        vec![
            "console.helper",
            "console.model",
            "console.steward",
            "model",
            "owner",
            "worker"
        ]
    );
    // Markers of the nested document are fixed at their value: the outer experiment does not
    // search them.
    assert_eq!(flat["parts"]["console.helper"]["count"], json!(1));
    let links: Vec<(String, String)> = flat["links"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| {
            (
                l["from"].as_str().unwrap().to_string(),
                l["to"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert!(links.contains(&("owner".into(), "console.steward".into())));
    assert!(links.contains(&("console.steward".into(), "console.model".into())));
    assert_eq!(flat["nested"][0]["border"], json!(["console.steward"]));
    let hand = yaml(FLAT);
    assert_eq!(flat["links"], hand["links"]);
    for (name, p) in hand["parts"].as_object().unwrap() {
        assert_eq!(&flat["parts"][name], p, "part {name}");
    }
    assert_eq!(
        agenticspec_sim::check::check(&flat),
        Vec::<agenticspec_sim::check::Refusal>::new()
    );
}

#[test]
fn a_nested_architecture_simulates_as_its_flattened_document() {
    // The inner steward keeps its `role`, and the profile reaches it through that role.
    let steward = "steward: { block: agent, harness: coding }";
    let with_role = "steward: { block: agent, harness: coding, role: steward }";
    let inner = INNER.replace(steward, with_role);
    let team = pair("simulate", &inner, |o| o);
    let dir = team.parent().unwrap();
    std::fs::write(
        dir.join("flat.yaml"),
        FLAT.replace(
            &format!("console.{steward}"),
            &format!("console.{with_role}"),
        ),
    )
    .unwrap();
    std::fs::write(dir.join("profile.yaml"), rich_profile()).unwrap();
    std::fs::write(
        dir.join("nested.yaml"),
        EXPERIMENT.replace("ARCH", "team.yaml"),
    )
    .unwrap();
    std::fs::write(
        dir.join("flat-exp.yaml"),
        EXPERIMENT.replace("ARCH", "flat.yaml"),
    )
    .unwrap();
    let nested = Experiment::load(&dir.join("nested.yaml")).unwrap();
    assert!(nested.marked.is_empty(), "inner markers are not searched");
    let flat = Experiment::load(&dir.join("flat-exp.yaml")).unwrap();
    let a = run(&nested, 2).unwrap();
    let b = run(&flat, 2).unwrap();
    assert_eq!(a["baseline"]["kpis"], b["baseline"]["kpis"]);
    assert_eq!(a["baseline"]["per_part"], b["baseline"]["per_part"]);
    assert!(
        a["baseline"]["kpis"]["tasks_done"]["value"]
            .as_f64()
            .unwrap()
            > 0.0
    );
    // The result keeps the outer document as written, the nested one beside its part.
    assert_eq!(
        a["architecture"]["parts"]["console"]["inner"]["architecture"],
        "console"
    );
}

#[test]
fn an_experiment_from_documents_needs_the_nested_file_loaded() {
    let raw = yaml(
        &OUTER
            .replace("REF", "console.yaml")
            .replace("DIGEST", &nest::digest(&json!({"parts": {}}))),
    );
    let err = Experiment::from_docs(
        yaml(&EXPERIMENT.replace("ARCH", "team.yaml")),
        raw,
        yaml(PROFILE),
    )
    .unwrap_err();
    assert!(err.contains("console"), "{err}");
}

#[test]
fn the_public_example_pair_passes_the_check() {
    let path = repo_root().join("architectures/team-with-console.yaml");
    let out = cli::check(&path).unwrap();
    assert_eq!(out["architecture"], "team-with-console");
    let nested = &out["nested"][0];
    assert_eq!(nested["reference"], "console.yaml");
    let text = std::fs::read_to_string(repo_root().join("architectures/console.yaml")).unwrap();
    assert_eq!(nested["digest"], Value::from(nest::digest(&yaml(&text))));
}

/// The common profile with a `steward` role (a coding agent that also answers questions), a
/// `notify` role for a runtime and a second priced route, `fast`.
fn rich_profile() -> String {
    let profile = PROFILE
        .replace(
            "principals:\n",
            "  steward:\n    task_turns:          { dist: lognormal, median: 10, p90: 30 }\n    turn_seconds:        { dist: lognormal, median: 10, p90: 30 }\n    tokens_per_turn:     { input: 2000, cache_read: 20000, cache_write: 2000, output: 400 }\n    answer_seconds:      { dist: lognormal, median: 30, p90: 120 }\n  notify:\n    task_turns: 1\n    turn_seconds: 5\nprincipals:\n",
        )
        .replace(
            "prices:\n",
            "prices:\n  fast: { input: 1.0, cache_read: 0.1, cache_write: 1.25, output: 5.0, per: 1M_tokens_usd }\n",
        );
    assert!(profile.contains("notify:") && profile.contains("fast:"));
    profile
}

/// A console whose steward splits its model calls over two models and whose helper's shipped
/// tasks wake a notifier runtime through an `on_event` trigger with a `source`.
const RICH_INNER: &str = r#"
format: agenticspec-architecture/1
architecture: console
parts:
  steward:  { block: agent, harness: coding, role: steward, model_share: { model: 1, fast: 3 } }
  helper:   { block: agent, harness: coding }
  model:    { block: model, route: frontier }
  fast:     { block: model, route: fast }
  on_done:  { block: trigger, kind: on_event, event_name: agentic.work.TaskCompleted, source: helper, policy: parallel, max_parallel: 2, task_template: notice }
  notifier: { block: runtime, role: notify, count: 2 }
links:
  - { from: steward, to: helper, carries: order }
  - { from: helper, to: steward, carries: question }
  - { from: steward, to: model, carries: model_call }
  - { from: steward, to: fast, carries: model_call }
  - { from: helper, to: model, carries: model_call }
  - { from: on_done, to: notifier, carries: trigger }
border: [steward]
"#;

/// An outer trigger that fires each time the console's border part answers a question.
const RICH_OUTER_TRIGGER: &str = "  on_steward: { block: trigger, kind: on_event, event_name: agentic.question.QuestionAnswered, source: SOURCE, policy: parallel, max_parallel: 2, task_template: notice }\n  bell: { block: runtime, role: notify }\n";

fn rich_outer(o: String, source: &str) -> String {
    o.replace(
        "  model:   { block: model, route: frontier }\n",
        &format!(
            "  model:   {{ block: model, route: frontier }}\n{}",
            RICH_OUTER_TRIGGER.replace("SOURCE", source)
        ),
    )
    .replace(
        "border: [console.steward]",
        "  - { from: on_steward, to: bell, carries: trigger }\nborder: [console.steward]",
    )
}

#[test]
fn flattening_rewrites_model_shares_and_trigger_sources() {
    let team = pair("rich", RICH_INNER, |o| rich_outer(o, "console.steward"));
    let flat = nest::flatten(&nest::load(&team).unwrap());
    let parts = &flat["parts"];
    assert_eq!(parts["console.steward"]["role"], "steward");
    assert_eq!(
        parts["console.steward"]["model_share"],
        json!({ "console.model": 1, "console.fast": 3 })
    );
    assert_eq!(parts["console.on_done"]["source"], "console.helper");
    assert_eq!(parts["console.notifier"]["role"], "notify");
    assert_eq!(parts["on_steward"]["source"], "console.steward");
    assert_eq!(
        agenticspec_sim::check::check(&flat),
        Vec::<agenticspec_sim::check::Refusal>::new()
    );

    // The simulator runs the flattened shares and triggers: the second model is called and both
    // triggers fire.
    let dir = team.parent().unwrap();
    std::fs::write(dir.join("profile.yaml"), rich_profile()).unwrap();
    std::fs::write(
        dir.join("rich.yaml"),
        EXPERIMENT.replace("ARCH", "team.yaml"),
    )
    .unwrap();
    let e = Experiment::load(&dir.join("rich.yaml")).unwrap();
    let b = &run(&e, 2).unwrap()["baseline"];
    let part = |name: &str, kpi: &str| b["per_part"][name][kpi]["value"].as_f64().unwrap_or(0.0);
    assert!(part("console.fast", "calls") > 0.0, "{}", b["per_part"]);
    assert!(
        part("console.on_done", "firings") > 0.0,
        "{}",
        b["per_part"]
    );
    assert!(part("on_steward", "firings") > 0.0, "{}", b["per_part"]);
}

#[test]
fn a_trigger_source_off_the_nested_border_is_refused() {
    let team = pair("rich-off-border", RICH_INNER, |o| {
        rich_outer(o, "console.helper")
    });
    assert_eq!(
        rules(&team),
        vec![("nested_not_in_border".into(), "console.helper".into())]
    );
}
