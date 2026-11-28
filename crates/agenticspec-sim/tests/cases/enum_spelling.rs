//! A setting whose type is a block's enum takes the word (`shared_read_only`) or the variant as
//! the block writes it (`SharedReadOnly`). Both are one value: every reader past the check (the
//! simulator, the bundle, admission, the measures of the whole) sees the word.

use super::common::{experiment, repo_root, scratch, yaml};
use agenticspec_sim::check::check;
use agenticspec_sim::cli::{self, BundleOptions};
use agenticspec_sim::doc::{load_yaml, parse_yaml};
use agenticspec_sim::settings::{self, Kind};
use agenticspec_sim::{Experiment, nest};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn rules(a: &Value) -> Vec<String> {
    check(a).iter().map(ToString::to_string).collect()
}

/// `shared_read_only` as the block writes the variant: `SharedReadOnly`.
fn variant(word: &str) -> String {
    word.split('_')
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_ascii_uppercase().to_string() + c.as_str())
                .unwrap_or_default()
        })
        .collect()
}

/// Every enum setting of a block's enum, every one of its words: written as the variant, as the
/// value of a marked place and among its candidates, a document reads back with the word.
#[test]
fn every_enum_setting_written_as_its_variant_reads_as_its_word() {
    let mut seen = 0;
    for b in &settings::table().blocks {
        for s in &b.settings {
            let Kind::Enum {
                values,
                from: Some(_),
            } = &s.ty.kind
            else {
                continue;
            };
            for w in values {
                let v = variant(w);
                assert_eq!(settings::snake(&v), *w, "{}.{}", b.kind, s.name);
                let doc = json!({"parts": {
                    "plain": {"block": b.kind, s.name.as_str(): v},
                    "marked": {"block": b.kind, s.name.as_str(): {"value": v, "vary": [v]}},
                }});
                let read = nest::flatten(&doc);
                assert_eq!(
                    read["parts"]["plain"][&s.name],
                    json!(w),
                    "{}.{}",
                    b.kind,
                    s.name
                );
                assert_eq!(
                    read["parts"]["marked"][&s.name],
                    json!({"value": w, "vary": [w]}),
                    "{}.{}",
                    b.kind,
                    s.name
                );
                seen += 1;
            }
        }
    }
    assert!(seen > 20, "{seen} enum words");
    // A word of `values` is no variant of a block's enum and stays as written.
    let doc = json!({"parts": {"w": {"block": "agent", "partition": "Scope"}}});
    assert_eq!(nest::flatten(&doc)["parts"]["w"]["partition"], "Scope");
}

/// `deploy bundle` of `doc` (written into a scratch file) under the repository's
/// local-workstation binding.
fn workstation_bundle(name: &str, doc: &Value) -> Result<Value, cli::Refusal> {
    let root = repo_root();
    let dir = scratch(name);
    let file = dir.join("workstation.yaml");
    std::fs::write(&file, serde_yaml_ng::to_string(doc).unwrap()).unwrap();
    let ports = root.join("blocks/ports.json");
    let accepts = root.join("bundles/cloudspec-accepts.yaml");
    cli::deploy_bundle(&BundleOptions {
        architecture: &file,
        binding: &root.join("bundles/bindings/local-workstation.yaml"),
        out: None,
        ports: Some(&ports),
        accepts: Some(&accepts),
        kpis: Some(&root.join("kpis/catalogue.yaml")),
    })
    .map(|answer| yaml(answer["bundle"].as_str().expect("the bundle's text")))
}

fn workstation() -> Value {
    load_yaml(Path::new(
        &repo_root().join("architectures/workstation.yaml"),
    ))
    .unwrap()
}

#[test]
fn a_sandbox_isolation_written_as_the_variant_is_held_to_the_target_like_the_word() {
    // The local binding gives a unit a container; a micro VM is more than that.
    let mut word = workstation();
    word["parts"]["box"]["isolation"] = json!("micro_vm");
    let mut variant = workstation();
    variant["parts"]["box"]["isolation"] = json!("MicroVm");
    assert_eq!(rules(&word), Vec::<String>::new());
    assert_eq!(rules(&variant), Vec::<String>::new());
    let s = workstation_bundle("enum-isolation-word", &word).unwrap_err();
    assert!(s.detail.contains("isolation_unmet"), "{}", s.detail);
    match workstation_bundle("enum-isolation-variant", &variant) {
        Err(r) => assert!(r.detail.contains("isolation_unmet"), "{}", r.detail),
        Ok(b) => panic!(
            "isolation MicroVm admitted on a container target, unit isolation {}",
            b["units"][0]["isolation"]
        ),
    }
}

#[test]
fn a_workspace_sharing_written_as_the_variant_is_mounted_like_the_word() {
    let mut word = workstation();
    word["parts"]["tree"]["sharing"] = json!("shared_read_only");
    let mut variant = workstation();
    variant["parts"]["tree"]["sharing"] = json!("SharedReadOnly");
    assert_eq!(rules(&word), Vec::<String>::new());
    assert_eq!(rules(&variant), Vec::<String>::new());
    let access = |b: &Value| b["units"][0]["claims"]["volumes"][0]["access"].clone();
    let s = workstation_bundle("enum-sharing-word", &word).unwrap();
    let v = workstation_bundle("enum-sharing-variant", &variant).unwrap();
    assert_eq!(access(&s), json!("read_only_many"));
    assert_eq!(
        access(&v),
        json!("read_only_many"),
        "a SharedReadOnly workspace is mounted {}",
        access(&v)
    );
}

fn architecture_with_variant_spelling() -> PathBuf {
    let text = std::fs::read_to_string(
        repo_root()
            .join("crates/agenticspec-sim/tests/cases/deploy/volume_access.architecture.yaml"),
    )
    .unwrap()
    .replace("shared_read_only", "SharedReadOnly")
    .replace("shared_read_write", "SharedReadWrite");
    let dir = scratch("enum-spelling-access");
    let file = dir.join("volume-access.yaml");
    std::fs::write(&file, text).unwrap();
    file
}

fn image(n: u32) -> Value {
    json!({ "image": format!("registry.example/image-{n}"), "digest": format!("sha256:{n:064x}") })
}

fn access_binding() -> Value {
    json!({
        "format": "agenticspec-binding/1",
        "binding": "access",
        "architecture": "volume-access",
        "transports": { "run": "http", "model_call": "http", "checkout": "http" },
        "runtimes": { "core": image(1) },
        "parts": {
            "lead": {
                "egress": ["flow", "gateway"],
                "volumes": [
                    { "name": "board", "mount": "/board" },
                    { "name": "base", "mount": "/base" },
                ],
            },
            "flow": {
                "egress": ["gateway"],
                "volumes": [{ "name": "notes", "mount": "/notes" }],
            },
            "gateway": { "egress": ["model"] },
        },
        "packing": {
            "units": [
                { "name": "core", "kind": "core", "parts": ["lead", "gateway", "board", "base"] },
                { "name": "flow", "kind": "service", "parts": ["flow", "notes"], "replicas": 1, "image": image(2) },
            ],
            "external": ["owner", "model"],
        },
    })
}

fn access_bundle(architecture: &Path) -> Value {
    let root = repo_root();
    let dir = scratch("enum-spelling-access-bundle");
    let file = dir.join("b.yaml");
    std::fs::write(&file, access_binding().to_string()).unwrap();
    let out = cli::deploy_bundle(&BundleOptions {
        architecture,
        binding: &file,
        out: None,
        ports: Some(&root.join("blocks/ports.json")),
        accepts: Some(&root.join("bundles/cloudspec-accepts.yaml")),
        kpis: Some(&root.join("kpis/catalogue.yaml")),
    })
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    parse_yaml(out["bundle"].as_str().unwrap()).unwrap()
}

#[test]
fn a_shared_read_only_workspace_written_as_its_variant_is_read_only_in_the_bundle() {
    let arch = architecture_with_variant_spelling();
    cli::check(&arch)
        .unwrap_or_else(|r| panic!("`sim check` refuses the document: {}: {}", r.code, r.detail));
    let v = access_bundle(&arch);
    let core = v["units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["name"] == "core")
        .unwrap();
    let base = core["claims"]["volumes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "base")
        .unwrap();
    assert_eq!(
        base["access"], "read_only_many",
        "`base` is `sharing: SharedReadOnly`; the bundle claims it as {}",
        base["access"]
    );
}

/// A principal's `kind` written `Human` picks the profile's entry `human`.
#[test]
fn a_principal_kind_written_as_its_variant_picks_the_profile_entry_of_its_word() {
    let e = experiment("fit", 1, 256);
    let mut arch = e.architecture.clone();
    arch["parts"]["owner"]["kind"] = json!("Human");
    assert_eq!(check(&arch).len(), 0, "the check accepts kind: Human");
    let loaded = Experiment::from_docs(e.doc.clone(), arch, e.profile.clone())
        .unwrap_or_else(|err| panic!("the simulator refuses kind `Human`: {err}"));
    assert_eq!(loaded.architecture["parts"]["owner"]["kind"], "human");
}

/// A trigger written with the variants of its block's enums (`OnSchedule`, `Queue`) passes
/// `sim check`, whose rules of triggers read the words.
#[test]
fn a_trigger_written_with_variants_passes_sim_check() {
    let text = std::fs::read_to_string(repo_root().join("architectures/scheduled-loops.yaml"))
        .unwrap()
        .replace("kind: on_schedule", "kind: OnSchedule")
        .replace("policy: skip", "policy: Queue");
    assert!(text.contains("OnSchedule") && text.contains("Queue"));
    let file = scratch("enum-spelling-trigger").join("scheduled-loops.yaml");
    std::fs::write(&file, text).unwrap();
    cli::check(&file).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
}
