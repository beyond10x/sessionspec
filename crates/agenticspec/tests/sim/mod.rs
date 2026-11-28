//! `agenticspec sim`: the simulator's commands on the generated command line. What each command
//! computes is tested in the `agenticspec-sim` crate.

use std::path::PathBuf;
use std::process::{Command, Output};

use serde_json::Value;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("sim")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn sim(args: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agenticspec"))
        .arg("sim")
        .args(args)
        .current_dir(repo())
        .output()
        .unwrap()
}

fn result(out: &Output) -> Value {
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
fn check_prints_the_accepted_architecture() {
    let arch = repo().join("architectures/workstation.yaml");
    let accepted = result(&sim(&["check".as_ref(), arch.as_os_str()]));
    assert_eq!(accepted["architecture"], "workstation");
    assert_eq!(accepted["kpis"]["agents_total"], 13, "{accepted}");
    assert_eq!(accepted["kpis"]["capacity_ratio"], 0.75, "{accepted}");
    assert_eq!(
        accepted["single_points"],
        serde_json::json!(["gateway", "model"])
    );
}

#[test]
fn check_exits_1_and_names_the_refusal() {
    let dir = scratch("check-refused");
    let arch = dir.join("broken.yaml");
    std::fs::write(
        &arch,
        "format: agenticspec-architecture/1\narchitecture: broken\nparts:\n  you: { block: principal, kind: human }\nlinks:\n  - { from: you, to: nobody, carries: prompt }\n",
    )
    .unwrap();
    let out = sim(&["check".as_ref(), arch.as_os_str()]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.starts_with("architecture_refused: "), "{stderr}");
    assert!(stderr.contains("nobody"), "{stderr}");
}

#[test]
fn check_json_prints_the_document_of_an_accepted_architecture() {
    let arch = repo().join("architectures/author-reviewer.yaml");
    let out = sim(&["check".as_ref(), "--json".as_ref(), arch.as_os_str()]);
    let doc = result(&out);
    assert_eq!(doc["format"], "agenticspec-check/1");
    assert_eq!(doc["valid"], true);
    assert_eq!(doc["violations"], serde_json::json!([]));
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn check_json_prints_the_document_and_exits_1_when_refused() {
    let dir = scratch("check-json-refused");
    let arch = dir.join("broken.yaml");
    std::fs::write(
        &arch,
        "format: agenticspec-architecture/1\narchitecture: broken\nparts:\n  you: { block: principal, kind: human }\nlinks:\n  - { from: you, to: nobody, carries: prompt }\n",
    )
    .unwrap();
    let out = sim(&["check".as_ref(), "--json".as_ref(), arch.as_os_str()]);
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();
    assert_eq!(stdout.lines().count(), 1, "{stdout}");
    let doc: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(doc["valid"], false);
    assert_eq!(doc["violations"][0]["subject"], "you->nobody:prompt");
    assert_eq!(doc["counts"]["unknown_link_parts"], 1);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.starts_with("architecture_refused: "), "{stderr}");
    let again = sim(&["check".as_ref(), "--json".as_ref(), arch.as_os_str()]);
    assert_eq!(again.stdout, out.stdout);
}

#[test]
fn run_writes_the_result_with_the_threads_given() {
    let dir = scratch("run");
    let exp = dir.join("exp.yaml");
    std::fs::write(
        &exp,
        format!(
            "format: agenticspec-experiment/1\nexperiment: cli-run\narchitecture: {}\nprofile: {}\nhorizon: 1h\nseeds: 1\nseed: 1\noptimize: marked\nsearch:\n  mode: fit\n  budget: 2\n  restarts: 1\n  weights:\n    tasks_done: 1\n",
            repo().join("architectures/workstation.yaml").display(),
            repo().join("fields/coding/profiles/coding.yaml").display(),
        ),
    )
    .unwrap();
    let out_dir = dir.join("out");
    let ran = result(&sim(&[
        "run".as_ref(),
        exp.as_os_str(),
        "--out".as_ref(),
        out_dir.as_os_str(),
        "--threads".as_ref(),
        "1".as_ref(),
    ]));
    assert_eq!(ran["threads"], 1);
    assert_eq!(ran["seeds"], 1);
    assert!(out_dir.join("result.json").is_file());
    assert!(out_dir.join("index.html").is_file());
}

#[test]
fn render_writes_the_page() {
    let page = scratch("render").join("page.html");
    let arch = repo().join("architectures/workstation.yaml");
    let rendered = result(&sim(&[
        "render".as_ref(),
        arch.as_os_str(),
        "--out".as_ref(),
        page.as_os_str(),
    ]));
    assert_eq!(rendered["out"], page.display().to_string());
    assert!(page.is_file());
}

#[test]
fn ladder_check_accepts_the_committed_data_file_and_refuses_a_missing_one() {
    let current = result(&sim(&[
        "ladder".as_ref(),
        "ladder/ladder.yaml".as_ref(),
        "--out".as_ref(),
        "website/static/ladder/ladder.json".as_ref(),
        "--check".as_ref(),
    ]));
    assert_eq!(current["written"], false);
    let missing = scratch("ladder").join("ladder.json");
    let out = sim(&[
        "ladder".as_ref(),
        "ladder/ladder.yaml".as_ref(),
        "--out".as_ref(),
        missing.as_os_str(),
        "--check".as_ref(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("ladder_stale"), "{stderr}");
    assert!(stderr.contains("ladder:generate"), "{stderr}");
}

#[test]
fn scenario_check_and_run_print_their_results() {
    let scenario = repo().join("scenarios/workstation/question-answered.scenario.yaml");
    let checked = result(&sim(&[
        "scenario".as_ref(),
        "check".as_ref(),
        scenario.as_os_str(),
    ]));
    assert_eq!(checked["scenario"], "question-answered");
    let out = scratch("scenario-run");
    let ran = result(&sim(&[
        "scenario".as_ref(),
        "run".as_ref(),
        scenario.as_os_str(),
        "--out".as_ref(),
        out.as_os_str(),
    ]));
    assert_eq!(ran["verdict"], "held");
    assert_eq!(ran["predicted"], "question");
    assert!(out.join("scenario-result.json").is_file());
}

#[test]
fn scenario_run_exits_1_on_a_failed_expectation() {
    let dir = scratch("scenario-failed");
    let source = repo().join("scenarios/workstation/control-refused.scenario.yaml");
    let text = std::fs::read_to_string(&source)
        .unwrap()
        .replace("../../", &format!("{}/", repo().display()))
        .replace("rule: control_without_proof", "rule: input_without_link");
    let scenario = dir.join("failed.scenario.yaml");
    std::fs::write(&scenario, text).unwrap();
    let out = sim(&[
        "scenario".as_ref(),
        "run".as_ref(),
        scenario.as_os_str(),
        "--out".as_ref(),
        dir.join("out").as_os_str(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.starts_with("expectation_failed: "), "{stderr}");
    assert!(dir.join("out/scenario-result.json").is_file());
}

#[test]
fn corpus_run_prints_the_table_and_the_rows() {
    let corpus = repo().join("scenarios/workstation/workstation.corpus.yaml");
    let out = sim(&["corpus".as_ref(), "run".as_ref(), corpus.as_os_str()]);
    let rows = result(&out);
    assert_eq!(rows["entries"], 3);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("predicted"), "{stderr}");
    assert!(stderr.contains("control-refused"), "{stderr}");
}

#[test]
fn digest_prints_the_sha256_of_the_file_and_check_lists_the_nested_architecture() {
    let console = repo().join("architectures/console.yaml");
    let printed = result(&sim(&["digest".as_ref(), console.as_os_str()]));
    let digest = printed["digest"].as_str().expect("digest");
    assert!(
        digest.starts_with("sha256:") && digest.len() == 7 + 64,
        "{digest}"
    );
    let team = repo().join("architectures/team-with-console.yaml");
    let accepted = result(&sim(&["check".as_ref(), team.as_os_str()]));
    assert_eq!(accepted["nested"][0]["part"], "console");
    assert_eq!(accepted["nested"][0]["digest"], digest);
}

const BROKEN: &str = "format: agenticspec-architecture/1\narchitecture: broken\nparts:\n  you: { block: principal, kind: human }\nlinks:\n  - { from: you, to: nobody, carries: prompt }\n";

/// `sim score` with the committed coding profile, seed 7, `extra` and the documents after `--`.
fn score(extra: &[&str], documents: &[&str]) -> Output {
    let mut args: Vec<&std::ffi::OsStr> = vec![
        "score".as_ref(),
        "--profile".as_ref(),
        "fields/coding/profiles/coding.yaml".as_ref(),
        "--seed".as_ref(),
        "7".as_ref(),
    ];
    args.extend(extra.iter().map(std::ffi::OsStr::new));
    args.push("--".as_ref());
    args.extend(documents.iter().map(std::ffi::OsStr::new));
    sim(&args)
}

fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

#[test]
fn score_prints_the_kpis_of_one_document() {
    let out = score(&[], &["architectures/author-reviewer.yaml"]);
    let scored = result(&out);
    assert!(scored["kpis"]["tasks_done"].is_number(), "{scored}");
    assert!(scored["kpis"]["cost_usd"].is_number(), "{scored}");
    assert!(scored["per_part"]["reviewer"].is_object(), "{scored}");
    assert_eq!(
        scored.as_object().unwrap().keys().collect::<Vec<_>>(),
        ["kpis", "per_part"]
    );
    assert_eq!(stdout(&out).lines().count(), 1);

    let plain = sim(&[
        "score".as_ref(),
        "--profile".as_ref(),
        "fields/coding/profiles/coding.yaml".as_ref(),
        "--seed".as_ref(),
        "7".as_ref(),
        "architectures/author-reviewer.yaml".as_ref(),
    ]);
    assert_eq!(
        plain.stdout, out.stdout,
        "the path as the last argument, without --"
    );
}

#[test]
fn score_is_byte_equal_for_the_same_document_and_arguments() {
    let first = score(&[], &["architectures/solo.yaml"]);
    let again = score(&[], &["architectures/solo.yaml"]);
    assert_eq!(first.status.code(), Some(0));
    assert_eq!(first.stdout, again.stdout);
    let other_seed = sim(&[
        "score".as_ref(),
        "--profile".as_ref(),
        "fields/coding/profiles/coding.yaml".as_ref(),
        "--seed".as_ref(),
        "8".as_ref(),
        "architectures/solo.yaml".as_ref(),
    ]);
    assert_eq!(other_seed.status.code(), Some(0));
    assert_ne!(
        first.stdout, other_seed.stdout,
        "the seed is an argument of the run"
    );
}

#[test]
fn score_exits_3_with_the_error_of_a_refused_document() {
    let arch = scratch("score-refused").join("broken.yaml");
    std::fs::write(&arch, BROKEN).unwrap();
    let out = score(&[], &[arch.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(3));
    let text = stdout(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    let answer: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(answer["error"]["code"], "architecture_refused");
    assert!(
        answer["error"]["message"]
            .as_str()
            .unwrap()
            .contains("nobody"),
        "{answer}"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.starts_with("architecture_refused: "), "{stderr}");

    let missing = score(&[], &["architectures/missing.yaml"]);
    assert_eq!(missing.status.code(), Some(3));
    let answer: Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert_eq!(answer["error"]["code"], "document_unreadable");
}

#[test]
fn score_exits_1_with_the_error_of_a_profile_it_cannot_read() {
    let out = sim(&[
        "score".as_ref(),
        "--profile".as_ref(),
        "fields/coding/profiles/missing.yaml".as_ref(),
        "architectures/solo.yaml".as_ref(),
    ]);
    assert_eq!(out.status.code(), Some(1));
    let answer: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(answer["error"]["code"], "profile_refused");
}

#[test]
fn score_batch_answers_each_path_in_order_byte_equal_to_its_single_run() {
    let broken = scratch("score-batch").join("broken.yaml");
    std::fs::write(&broken, BROKEN).unwrap();
    let paths = [
        "architectures/solo.yaml",
        broken.to_str().unwrap(),
        "architectures/author-reviewer.yaml",
    ];
    let batch = score(&[], &paths);
    assert_eq!(
        batch.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&batch.stderr)
    );
    let singles: Vec<String> = paths
        .iter()
        .map(|p| stdout(&score(&[], &[p])).trim_end().to_owned())
        .collect();
    assert_eq!(
        stdout(&batch),
        format!("{{\"results\":[{}]}}\n", singles.join(","))
    );
    let answer: Value = serde_json::from_slice(&batch.stdout).unwrap();
    assert_eq!(
        answer["results"][1]["error"]["code"],
        "architecture_refused"
    );

    let one = score(&["--batch"], &[paths[0]]);
    assert_eq!(one.status.code(), Some(0));
    assert_eq!(stdout(&one), format!("{{\"results\":[{}]}}\n", singles[0]));
}
