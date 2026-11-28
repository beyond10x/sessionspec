//! The control plane's Inbox follows a principal part's `kind`, which `blocks/settings.yaml` types
//! as `agentic.shared.PrincipalKind`: a part of every kind of that enum has the same Inbox, and
//! the kinds are read from the compiled blocks, the built-in copy as well as a root's.

use super::{Derivation, derived, inputs, root, scratch};
use serde_json::Value as Json;

const EXAMPLES: &str = "ui/views/examples";
const PRINCIPAL_KIND: &str = "agentic.shared.PrincipalKind";

fn inbox_sections_with_owner_kind(kind: &str) -> usize {
    let r = root();
    let dir = scratch(&format!("principal-kind-{kind}"));
    let text = std::fs::read_to_string(r.join("architectures/workstation.yaml"))
        .unwrap()
        .replace(
            "owner:    { block: principal, kind: human }",
            &format!("owner:    {{ block: principal, kind: {kind} }}"),
        );
    assert!(
        text.contains(&format!("kind: {kind} }}")),
        "the owner line was not rewritten"
    );
    let arch = dir.join("workstation.yaml");
    std::fs::write(&arch, text).unwrap();
    let out = dir.join("workstation.control-plane.ui.yaml");
    let mut i = inputs(
        &arch,
        Some(&format!("{EXAMPLES}/workstation.contract.yaml")),
        Some(r.join(EXAMPLES).join("ports.json")),
        out,
    );
    i.control_plane = true;
    let d: Derivation = derived(&i);
    let text = d
        .files
        .iter()
        .find(|(p, _)| p.to_string_lossy().ends_with(".layout.json"))
        .map(|(_, t)| t.clone())
        .expect("the layout is among the files");
    let doc: Json = serde_json::from_str(&text).unwrap();
    let inbox = doc["control"]["pages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["page"] == "control.inbox")
        .expect("the inbox page");
    inbox["frames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["sections"].as_array().unwrap().len())
        .sum()
}

/// The variants of `agentic.shared.PrincipalKind` in the compiled blocks built into the binary,
/// as an architecture writes them.
fn built_in_kinds() -> Vec<String> {
    let ir = agenticspec_ui::operator::built_in_model().expect("the built-in model");
    ir["types"][PRINCIPAL_KIND]["body"]["variants"]
        .as_array()
        .expect("PrincipalKind is an enum of the built-in model")
        .iter()
        .map(|v| agenticspec_ui::derive::snake(v.as_str().unwrap()))
        .collect()
}

#[test]
fn the_inbox_of_a_principal_part_does_not_depend_on_which_principal_kind_it_is() {
    let human = inbox_sections_with_owner_kind("human");
    assert!(human > 0, "the control: a human owner has an inbox");
    let kinds = built_in_kinds();
    for kind in ["organization", "agent", "service"] {
        assert!(
            kinds.iter().any(|k| k == kind),
            "{kind} is a PrincipalKind: {kinds:?}"
        );
    }
    for kind in kinds.iter().filter(|k| *k != "human") {
        assert_eq!(
            inbox_sections_with_owner_kind(kind),
            human,
            "an owner of kind `{kind}` (a value of {PRINCIPAL_KIND}, accepted by `sim check`) has no Inbox section"
        );
    }
}

#[test]
fn the_principal_kinds_are_the_variants_of_the_compiled_blocks() {
    let built_in = agenticspec_ui::operator::built_in_model().expect("the built-in model");
    let compiled = agenticspec_views::compile(&root().join("blocks")).expect("the blocks compile");
    let want: std::collections::BTreeSet<String> = built_in_kinds().into_iter().collect();
    assert_eq!(
        agenticspec_ui::derive::control::principal_kinds(built_in),
        want
    );
    assert_eq!(
        agenticspec_ui::derive::control::principal_kinds(&compiled),
        want
    );
    assert!(
        agenticspec_ui::derive::control::principal_kinds(&serde_json::json!({})).is_empty(),
        "a model without the enum names no principal kind"
    );
}
