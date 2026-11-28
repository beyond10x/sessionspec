//! A part's `model_share` is refused by `sim check` wherever `sim run` refuses it: a key that names
//! no model part the part reaches over `model_call`, and shares that give every reached model 0
//! (an empty map among them).

use super::common::{PROFILE, yaml};
use agenticspec_sim::check::check;
use agenticspec_sim::{Experiment, run};
use serde_json::{Value, json};

const EXPERIMENT: &str = r#"
format: agenticspec-experiment/1
experiment: shares
architecture: shares.yaml
profile: shares-profile.yaml
horizon: 8h
seeds: 2
seed: 11
optimize: marked
search:
  mode: exhaustive
  limit: 16
  weights: { tasks_done: 1, cost_usd: -0.01 }
"#;

const TWO_MODELS: &str = r#"
format: agenticspec-architecture/1
architecture: two-models
parts:
  owner:   { block: principal, kind: human }
  worker:  { block: agent, count: 2, model_share: { big: 3, small: 1 } }
  gateway: { block: gateway }
  big:     { block: model, route: frontier }
  small:   { block: model, route: fast }
links:
  - { from: owner,   to: worker,  carries: prompt }
  - { from: worker,  to: gateway, carries: model_call }
  - { from: gateway, to: big,     carries: model_call }
  - { from: gateway, to: small,   carries: model_call }
"#;

fn profile() -> Value {
    let mut p = yaml(PROFILE);
    p["prices"]["fast"] = json!({
        "input": 1.0, "cache_read": 0.1, "cache_write": 1.25, "output": 5.0, "per": "1M_tokens_usd"
    });
    p
}

/// The simulator's run path on `arch`: its baseline, or the refusal that stops the run.
fn run_path(arch: &Value) -> Result<Value, String> {
    let e = Experiment::from_docs(yaml(EXPERIMENT), arch.clone(), profile())?;
    Ok(run(&e, 2)?["baseline"].clone())
}

#[test]
fn the_static_check_refuses_a_model_share_the_run_refuses() {
    let mut a = yaml(TWO_MODELS);
    assert!(run_path(&a).is_ok(), "the unchanged fixture runs");
    assert!(check(&a).is_empty(), "and checks clean");
    let mut accepted = Vec::new();
    for shares in [
        json!({ "big": 1, "elsewhere": 1 }),
        json!({ "big": 0, "small": 0 }),
        json!({}),
    ] {
        a["parts"]["worker"]["model_share"] = shares.clone();
        let err = run_path(&a)
            .err()
            .unwrap_or_else(|| panic!("the simulator takes model_share {shares}"));
        if check(&a).is_empty() {
            accepted.push(format!("{shares} => run refuses: {err}"));
        }
    }
    assert_eq!(
        accepted,
        Vec::<String>::new(),
        "sim check accepts model_share the simulator refuses"
    );
}

fn share_refusals(a: &Value) -> Vec<String> {
    check(a)
        .iter()
        .filter(|r| r.rule() == "setting_invalid")
        .map(ToString::to_string)
        .collect()
}

#[test]
fn a_model_share_naming_no_part_or_no_reached_model_is_refused() {
    let mut a = yaml(super::common::ARCH);
    for shares in [
        json!({"ghost": 1}),
        // A part, but no model.
        json!({"model": 1, "gateway": 1}),
    ] {
        a["parts"]["worker"]["model_share"] = shares.clone();
        let r = share_refusals(&a);
        assert_eq!(r.len(), 1, "{shares}: {r:?}");
        assert!(
            r[0].contains("worker") && r[0].contains("model_share"),
            "{}",
            r[0]
        );
    }
    // A model part this part does not reach: `spare` is reached by nobody's model_call.
    a["parts"]["spare"] = json!({"block": "model", "route": "frontier"});
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "operator", "to": "spare", "carries": "model_call"}));
    a["parts"]["worker"]["model_share"] = json!({"model": 1, "spare": 1});
    assert_eq!(share_refusals(&a).len(), 1, "{:?}", share_refusals(&a));
    // What the simulator takes, the check takes: a reached model with a share, one at 0 beside it,
    // and a marked share.
    a["parts"]["operator"]["model_share"] =
        json!({"model": 0, "spare": {"value": 2, "vary": [1, 2]}});
    a["parts"]["worker"]["model_share"] = json!({"model": 1});
    assert_eq!(share_refusals(&a), Vec::<String>::new());
}
