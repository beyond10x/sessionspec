//! What the architecture block (blocks/9-composition/architecture.yaml) refuses to build,
//! `agenticspec sim check` refuses too: one case per refusal of the block that a document can
//! write and the check once accepted. Each case names the block's command, outcome and error.
//! The border and question rules hold only in an architecture that has a principal part; their
//! admitted cases are here too.

use super::common::{ARCH, repo_root, yaml};
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

fn document(path: &str) -> Value {
    let path = repo_root().join(path);
    yaml(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
}

fn flattened(path: &str) -> Value {
    let a = agenticspec_sim::nest::load(&repo_root().join(path)).unwrap();
    agenticspec_sim::nest::flatten(&a)
}

fn sorted(a: &Value) -> Vec<(String, String)> {
    let mut r = refused(a);
    r.sort();
    r
}

/// The link of `a` from `from` to `to` carrying `carries`.
fn link<'a>(a: &'a mut Value, from: &str, to: &str, carries: &str) -> &'a mut Value {
    a["links"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["from"] == from && l["to"] == to && l["carries"] == carries)
        .unwrap_or_else(|| panic!("link {from} -> {to} carrying {carries}"))
}

#[test]
fn a_border_part_no_principal_reaches_is_refused() {
    // SetBorder not-reached, NotReachedByPrincipal: the owner links to the operator only.
    let mut a = base();
    a["border"] = json!(["operator", "worker"]);
    only(&a, "not_reached_by_principal", "worker");
}

#[test]
fn a_border_part_reached_through_a_grant_is_admitted() {
    // The owner's grant reaches the worker through an authority part.
    let mut a = base();
    a["border"] = json!(["operator", "worker"]);
    a["parts"]["grant"] = json!({"block": "authority", "kind": "grant"});
    push_link(
        &mut a,
        json!({"from": "owner", "to": "grant", "carries": "grant"}),
    );
    push_link(
        &mut a,
        json!({"from": "grant", "to": "worker", "carries": "grant"}),
    );
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
}

#[test]
fn the_auto_merger_is_reached_through_its_grant() {
    let mut a = document("fields/coding/architectures/auto-merger.yaml");
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
    // Without the grant's link to the merger nothing a principal holds reaches it.
    *link(&mut a, "merge_grant", "merger", "grant") =
        json!({"from": "merge_grant", "to": "merger", "carries": "context"});
    only(&a, "not_reached_by_principal", "merger");
}

#[test]
fn a_question_to_an_agent_that_forwards_to_no_principal_is_refused() {
    // CommitRevision question-without-answerer, QuestionWithoutAnswerer: the operator acts for
    // nobody and its link to the owner carries no questions.
    let mut a = base();
    a["parts"]["operator"]
        .as_object_mut()
        .unwrap()
        .remove("acts_for");
    link(&mut a, "operator", "owner", "question")["carries"] = json!("report");
    only(&a, "question_without_answerer", "operator");
}

#[test]
fn a_question_to_an_agent_that_forwards_to_a_principal_is_admitted() {
    // The operator acts for nobody and forwards questions to the owner, as the simulator does.
    let mut a = base();
    a["parts"]["operator"]
        .as_object_mut()
        .unwrap()
        .remove("acts_for");
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
}

#[test]
fn a_console_with_no_principal_is_admitted() {
    // Made to be nested: its steward is on the border and answers the helper with no principal
    // in the document.
    let a = document("architectures/console.yaml");
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
}

#[test]
fn a_console_with_a_principal_holds_both_rules() {
    // Once the document has a principal part, the steward is on the border unreached and the
    // helper's questions reach no principal.
    let mut a = document("architectures/console.yaml");
    a["parts"]["owner"] = json!({"block": "principal", "kind": "human"});
    push_link(
        &mut a,
        json!({"from": "owner", "to": "helper", "carries": "prompt"}),
    );
    assert_eq!(
        sorted(&a),
        vec![
            ("not_reached_by_principal".into(), "steward".into()),
            ("question_without_answerer".into(), "steward".into()),
        ]
    );
}

#[test]
fn the_team_with_console_flattened_is_admitted() {
    let mut a = flattened("architectures/team-with-console.yaml");
    assert_eq!(refused(&a), Vec::<(String, String)>::new());
    // The steward forwards the questions of the workers and of the console's helper to the
    // owner; without that link both reach no principal.
    link(&mut a, "console.steward", "owner", "question")["carries"] = json!("report");
    assert_eq!(
        sorted(&a),
        vec![
            ("question_without_answerer".into(), "console.steward".into()),
            ("question_without_answerer".into(), "console.steward".into()),
        ]
    );
}
