//! A gateway's `rate_limit_per_minute` is a number (`blocks/settings.yaml`): a rate below one call
//! a minute admits calls, spaced by the rate, and a fractional rate above one admits as many calls
//! as it says.

use super::common::experiment;
use agenticspec_sim::check::check;
use agenticspec_sim::{Experiment, run};
use serde_json::{Value, json};

/// The refusals of the check and the baseline's `tasks_done` and turns with the gateway's
/// rate set to `rate`.
fn with_rate(rate: Value) -> (usize, f64, f64) {
    let e = experiment("fit", 1, 256);
    let mut arch = e.architecture.clone();
    arch["parts"]["gateway"]["rate_limit_per_minute"] = rate;
    let refusals = check(&arch).len();
    let e = Experiment::from_docs(e.doc.clone(), arch, e.profile.clone()).unwrap();
    let v = run(&e, 2).unwrap();
    let kpi = |k: &str| {
        v["baseline"]["kpis"][k]["value"]
            .as_f64()
            .unwrap_or(f64::NAN)
    };
    (refusals, kpi("tasks_done"), kpi("turns"))
}

#[test]
fn a_fractional_rate_limit_the_check_accepts_admits_calls_in_the_simulator() {
    let (refused, done, _) = with_rate(json!(0.5));
    assert_eq!(refused, 0, "the check accepts rate_limit_per_minute: 0.5");
    assert!(
        done > 0.0,
        "rate_limit_per_minute: 0.5 admits no call (tasks_done = {done})"
    );
    // No call at all only at a rate of 0.
    let (_, none, _) = with_rate(json!(0));
    assert_eq!(none, 0.0);
}

#[test]
fn a_saturated_gateway_takes_turns_in_the_order_of_its_rate() {
    let (_, _, half) = with_rate(json!(0.5));
    let (_, _, one) = with_rate(json!(1));
    let (_, _, one_and_half) = with_rate(json!(1.5));
    let (_, _, two) = with_rate(json!(2));
    assert!(half < one, "0.5/min: {half} turns, 1/min: {one}");
    assert!(one < one_and_half, "1/min: {one}, 1.5/min: {one_and_half}");
    assert!(one_and_half < two, "1.5/min: {one_and_half}, 2/min: {two}");
}
