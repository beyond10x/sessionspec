//! `sim score`: one simulation of an architecture as written, its KPIs as JSON. The command line
//! (standard output, exit codes, batch) is tested in the `agenticspec` crate.

use super::common::{ARCH, PROFILE, repo_root, scratch};
use agenticspec_sim::cli::{self, ScoreOptions};
use std::path::{Path, PathBuf};

fn committed() -> (PathBuf, PathBuf) {
    (
        repo_root().join("architectures/author-reviewer.yaml"),
        repo_root().join("fields/coding/profiles/coding.yaml"),
    )
}

fn options(profile: &Path, seed: u64) -> ScoreOptions<'_> {
    ScoreOptions {
        profile,
        seed: Some(seed),
        horizon: None,
    }
}

#[test]
fn score_equals_the_baseline_sim_run_reports() {
    let (arch, profile) = committed();
    let dir = scratch("score-baseline");
    std::fs::write(
        dir.join("exp.yaml"),
        format!(
            "format: agenticspec-experiment/1\nexperiment: score-baseline\narchitecture: {}\nprofile: {}\nseeds: 1\nseed: 7\noptimize: marked\nsearch:\n  mode: fit\n  budget: 1\n  restarts: 0\n  weights:\n    tasks_done: 1\n",
            arch.display(),
            profile.display(),
        ),
    )
    .unwrap();
    // The result `sim run` writes, taken before it is written: serde_json's text parser may
    // read a float one unit in the last place off, so result.json read back is not the result.
    let experiment = agenticspec_sim::Experiment::load(&dir.join("exp.yaml")).unwrap();
    let result = agenticspec_sim::run(&experiment, 1).unwrap();
    assert_eq!(result["variants"].as_array().unwrap().len(), 1, "no search");
    let baseline = &result["baseline"];

    let scored = cli::Scorer::new(&options(&profile, 7))
        .unwrap()
        .score(&arch)
        .unwrap();
    let kpis = scored["kpis"].as_object().unwrap();
    let want = baseline["kpis"].as_object().unwrap();
    assert_eq!(
        kpis.keys().collect::<Vec<_>>(),
        want.keys().collect::<Vec<_>>()
    );
    for (id, v) in want {
        assert_eq!(kpis[id], v["value"], "{id}");
    }
    let per_part = scored["per_part"].as_object().unwrap();
    let want = baseline["per_part"].as_object().unwrap();
    assert_eq!(
        per_part.keys().collect::<Vec<_>>(),
        want.keys().collect::<Vec<_>>()
    );
    for (part, row) in want {
        for (id, v) in row.as_object().unwrap() {
            assert_eq!(per_part[part][id], v["value"], "{part}.{id}");
        }
    }
}

/// Every KPI id of `kpis/catalogue.yaml`.
fn catalogue_ids() -> Vec<String> {
    let catalogue =
        agenticspec_sim::doc::load_yaml(&repo_root().join("kpis/catalogue.yaml")).unwrap();
    catalogue["kpis"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|k| k["id"].as_str().map(str::to_owned))
        .collect()
}

#[test]
fn score_keys_its_kpis_by_catalogue_id_or_declared_name_a_number_each() {
    let dir = scratch("score-ids");
    std::fs::write(dir.join("small.yaml"), ARCH).unwrap();
    std::fs::write(dir.join("profile.yaml"), PROFILE).unwrap();
    let scored = cli::Scorer::new(&options(&dir.join("profile.yaml"), 1))
        .unwrap()
        .score(&dir.join("small.yaml"))
        .unwrap();
    let ids = catalogue_ids();
    // A KPI the catalogue does not list keeps the name the simulator or the check declares.
    let declared = |id: &str| {
        agenticspec_sim::sim::RUN_KPIS.contains(&id)
            || agenticspec_sim::check::GATE_KPIS.contains(&id)
    };
    let kpis = scored["kpis"].as_object().unwrap();
    assert!(kpis.contains_key("tasks_done"), "{scored}");
    assert!(kpis.contains_key("agents_total"), "{scored}");
    assert!(kpis.contains_key("unknown_link_parts"), "{scored}");
    for (id, v) in kpis {
        assert!(
            ids.contains(id) || declared(id),
            "{id} is neither a KPI of the catalogue nor a name of the simulator or the check"
        );
        assert!(v.is_number(), "{id}: {v}");
    }
    let worker = scored["per_part"]["worker"].as_object().unwrap();
    assert!(worker.contains_key("tasks_done"), "{scored}");
    for (id, v) in worker {
        assert!(v.is_number(), "worker.{id}: {v}");
    }
}

#[test]
fn score_refuses_a_document_sim_check_refuses() {
    let dir = scratch("score-refused");
    std::fs::write(
        dir.join("broken.yaml"),
        ARCH.replace(
            "to: builds,   carries: claim",
            "to: build,   carries: claim",
        ),
    )
    .unwrap();
    std::fs::write(dir.join("profile.yaml"), PROFILE).unwrap();
    let scorer = cli::Scorer::new(&options(&dir.join("profile.yaml"), 1)).unwrap();
    let err = scorer.score(&dir.join("broken.yaml")).unwrap_err();
    assert_eq!(err.code, "architecture_refused");
    assert_eq!(
        cli::check(&dir.join("broken.yaml")).unwrap_err(),
        err,
        "the refusal sim check answers"
    );
    assert_eq!(
        scorer.score(&dir.join("missing.yaml")).unwrap_err().code,
        "document_unreadable"
    );
    assert_eq!(
        cli::score_error(&err),
        serde_json::json!({"error": {"code": "architecture_refused", "message": err.detail}})
    );
}

#[test]
fn score_refuses_a_profile_it_cannot_read_as_its_arguments() {
    let dir = scratch("score-profile");
    let err = cli::Scorer::new(&options(&dir.join("missing.yaml"), 1))
        .err()
        .unwrap();
    assert_eq!(err.code, "profile_refused");
    assert!(!cli::score_refuses_document(err.code));
    assert!(cli::score_refuses_document("architecture_refused"));
}
