//! A setting whose block refuses a value refuses it in `sim check` too: the table's bound is the
//! block's guard. `agentic.architecture` refuses a part of fewer than one copy (`InvalidCount`),
//! `agentic.resource` a pool without a unit (`ZeroCapacity`) and a claim without a time limit of at
//! least one second (`NoTimeLimit`).

use super::common::{ARCH, yaml};
use agenticspec_sim::check::check;
use serde_json::json;

#[test]
fn settings_the_blocks_refuse_are_refused_by_the_check() {
    for (part, key, value) in [
        ("model", "count", json!(0)),
        ("builds", "capacity", json!(0)),
        ("builds", "job_limit", json!(0)),
        ("builds", "job_limit", json!("0s")),
        ("builds", "job_limit", json!(0.5)),
    ] {
        let mut a = yaml(ARCH);
        a["parts"][part][key] = value.clone();
        let rules: Vec<(&str, String)> = check(&a)
            .iter()
            .map(|r| (r.rule(), r.part().to_string()))
            .collect();
        assert_eq!(
            rules,
            vec![("setting_invalid", part.to_string())],
            "{part}.{key} = {value}"
        );
    }
}

#[test]
fn the_smallest_values_the_blocks_take_pass_the_check() {
    for (part, key, value) in [
        ("model", "count", json!(1)),
        ("builds", "capacity", json!(1)),
        ("builds", "job_limit", json!(1)),
        ("builds", "job_limit", json!("1s")),
    ] {
        let mut a = yaml(ARCH);
        a["parts"][part][key] = value.clone();
        assert_eq!(check(&a), vec![], "{part}.{key} = {value}");
    }
}
