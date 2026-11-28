//! What the architecture block (blocks/9-composition/architecture.yaml) refuses to build,
//! `agenticspec sim check` refuses too: one case per refusal of the block that a document can
//! write and the check once accepted. Each case names the block's command, outcome and error.

use super::common::{ARCH, yaml};
use agenticspec_sim::check::{GATE_KPIS, check};
use serde_json::{Value, json};

fn base() -> Value {
    yaml(ARCH)
}

fn refused(a: &Value) -> Vec<(String, String)> {
    check(a)
        .iter()
        .map(|r| (r.rule().to_string(), r.part().to_string()))
        .collect()
}

fn only(a: &Value, rule: &str, part: &str) {
    assert!(GATE_KPIS.contains(&rule), "{rule} is not a gate KPI");
    assert_eq!(refused(a), vec![(rule.to_string(), part.to_string())]);
}

fn push_link(a: &mut Value, link: Value) {
    a["links"].as_array_mut().unwrap().push(link);
}

#[test]
fn a_link_over_a_part_that_is_no_channel_is_refused() {
    // AddLink not-a-channel, NotAChannel.
    let mut a = base();
    a["links"][0]["over"] = json!("model");
    only(&a, "not_a_channel", "model");
}

#[test]
fn acts_for_a_part_that_is_no_principal_is_refused() {
    // SetActsFor not-a-principal, ActsForNotPrincipal.
    let mut a = base();
    a["parts"]["operator"]["acts_for"] = json!("model");
    only(&a, "acts_for_not_principal", "operator");
}

#[test]
fn acts_for_on_a_part_that_is_no_agent_is_refused() {
    // SetActsFor not-an-agent, ActsForNeedsAgent: blocks/settings.yaml declares `acts_for` on
    // agents only, so the check names it as a setting the block does not take.
    let mut a = base();
    a["parts"]["gateway"]["acts_for"] = json!("owner");
    only(&a, "setting_unknown", "gateway");
}

#[test]
fn a_link_from_a_part_to_itself_is_refused() {
    // AddLink self-link, SelfReference.
    let mut a = base();
    push_link(
        &mut a,
        json!({"from": "worker", "to": "worker", "carries": "handoff"}),
    );
    only(&a, "self_reference", "worker");
}

#[test]
fn per_naming_the_part_itself_is_refused() {
    // SetPer self-reference, SelfReference.
    let mut a = base();
    a["parts"]["tree"]["per"] = json!("tree");
    only(&a, "self_reference", "tree");
}

#[test]
fn acts_for_naming_the_part_itself_is_refused() {
    // SetActsFor self-reference, SelfReference: the block refuses it before it asks for a
    // principal.
    let mut a = base();
    a["parts"]["operator"]["acts_for"] = json!("operator");
    only(&a, "self_reference", "operator");
}

#[test]
fn the_same_link_twice_is_refused() {
    // AddLink exists, LinkExists: the same from, to and carries.
    let mut a = base();
    let again = a["links"][3].clone();
    push_link(&mut a, again);
    only(&a, "link_exists", "operator");
}

#[test]
fn the_same_ends_carrying_another_thing_are_two_links() {
    let mut a = base();
    push_link(
        &mut a,
        json!({"from": "operator", "to": "worker", "carries": "handoff"}),
    );
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
}

#[test]
fn a_gateway_that_reaches_no_model_is_refused() {
    // CommitRevision gateway-without-model, GatewayWithoutModel.
    let mut a = base();
    a["parts"]["spare"] = json!({"block": "gateway"});
    push_link(
        &mut a,
        json!({"from": "worker", "to": "spare", "carries": "model_call"}),
    );
    only(&a, "gateway_without_model", "spare");
}

#[test]
fn an_architecture_without_parts_is_refused() {
    // CommitRevision empty, EmptyArchitecture: the refusal names the architecture.
    let a = json!({
        "format": "agenticspec-architecture/1",
        "architecture": "empty",
        "parts": {},
        "links": [],
    });
    only(&a, "empty_architecture", "empty");
}
