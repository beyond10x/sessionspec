//! The scratch directories of a unit (`scratch: [{mount, size}]` on a unit `packing.units` lists
//! or on a part of the unit, R118): a writable directory that lives as long as one instance of
//! the unit, carried per unit as `scratch` with no volume claim; refused when it is the mount of
//! another directory of the unit or one holds the other (`unit_mount_collision`), and when it is
//! of the wrong form, a relative path among them (`unit_scratch_invalid`); `deploy check` applies
//! the same rules to a bundle read back.

use super::common::{repo_root, scratch};
use agenticspec_sim::cli::{self, BundleOptions};
use agenticspec_sim::doc::parse_yaml;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The architecture whose runtime part `flow` keeps durable state.
fn architecture() -> PathBuf {
    repo_root().join(
        "crates/agenticspec-sim/tests/cases/deploy/durable_state_unplaceable.architecture.yaml",
    )
}

/// An image of the registry, pinned by digest `n`.
fn image(n: u32) -> Value {
    json!({ "image": format!("registry.example/image-{n}"), "digest": format!("sha256:{n:064x}") })
}

/// A binding with the runtime in a service unit of its own with its image, its state mounted at
/// `/var/lib/flow`, and a scratch directory at `/run/flow`; every crossing has a transport and
/// every destination is declared.
fn base() -> Value {
    json!({
        "format": "agenticspec-binding/1",
        "binding": "scratch",
        "architecture": "durable-state",
        "transports": { "run": "http", "model_call": "http" },
        "runtimes": { "core": image(1) },
        "parts": {
            "lead": { "egress": ["flow", "gateway"] },
            "flow": {
                "egress": ["gateway"],
                "volumes": [{ "name": "flow-state", "mount": "/var/lib/flow" }],
            },
            "gateway": { "egress": ["model"] },
        },
        "packing": {
            "units": [
                { "name": "core", "kind": "core", "parts": ["lead", "gateway"] },
                {
                    "name": "flow",
                    "kind": "service",
                    "parts": ["flow"],
                    "image": image(2),
                    "scratch": [{ "mount": "/run/flow", "size": "64Mi" }],
                },
            ],
            "external": ["owner", "model"],
        },
    })
}

fn write(dir: &Path, name: &str, text: &str) -> PathBuf {
    let file = dir.join(name);
    std::fs::write(&file, text).unwrap();
    file
}

/// `deploy bundle` of the architecture under `binding`: the bundle's text, or the refusal.
fn bundle_text(dir: &Path, binding: &Value) -> Result<String, cli::Refusal> {
    let root = repo_root();
    let file = write(dir, "b.yaml", &binding.to_string());
    let out = cli::deploy_bundle(&BundleOptions {
        architecture: &architecture(),
        binding: &file,
        out: None,
        ports: Some(&root.join("blocks/ports.json")),
        accepts: Some(&root.join("bundles/cloudspec-accepts.yaml")),
        kpis: Some(&root.join("kpis/catalogue.yaml")),
    })?;
    Ok(out["bundle"].as_str().unwrap().to_owned())
}

fn admitted(dir: &Path, binding: &Value) -> Value {
    let text = bundle_text(dir, binding).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    parse_yaml(&text).unwrap()
}

fn unit<'a>(v: &'a Value, name: &str) -> &'a Value {
    v["units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["name"] == name)
        .unwrap_or_else(|| panic!("no unit {name}"))
}

fn names(r: &cli::Refusal) -> BTreeSet<String> {
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    r.detail
        .split("; ")
        .map(|f| f.split(':').next().unwrap_or("").to_owned())
        .collect()
}

fn refused_by(r: &cli::Refusal, name: &str) {
    assert_eq!(names(r), BTreeSet::from([name.to_owned()]), "{}", r.detail);
}

fn check(dir: &Path, v: &Value) -> Result<(), cli::Refusal> {
    let accepts = repo_root().join("bundles/cloudspec-accepts.yaml");
    let file = write(
        dir,
        "bundle.yaml",
        &agenticspec_sim::deploy::bundle::write(v),
    );
    cli::deploy_check(&file, Some(&accepts)).map(|_| ())
}

#[test]
fn a_scratch_directory_is_carried_into_its_unit_with_no_volume_claim() {
    let dir = scratch("deploy-scratch-carried");
    let mut b = base();
    // A part of a unit may give one too; the unit carries both sorted by mount.
    b["parts"]["flow"]["scratch"] = json!([{ "mount": "/tmp" }]);
    b["parts"]["lead"]["scratch"] = json!([{ "mount": "/var/cache/lead" }]);
    let text = bundle_text(&dir, &b).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let v = parse_yaml(&text).unwrap();
    let flow = unit(&v, "flow");
    assert_eq!(
        flow["scratch"],
        json!([{ "mount": "/run/flow", "size": "64Mi" }, { "mount": "/tmp" }])
    );
    assert_eq!(
        flow["claims"]["volumes"],
        json!([{
            "name": "flow-state",
            "access": "read_write_once",
            "per_instance": false,
            "mount": "/var/lib/flow",
        }])
    );
    assert_eq!(
        unit(&v, "core")["scratch"],
        json!([{ "mount": "/var/cache/lead" }])
    );
    check(&dir, &v).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    // Without scratch a unit carries no `scratch` key.
    let mut plain = base();
    plain["packing"]["units"][1]
        .as_object_mut()
        .unwrap()
        .remove("scratch");
    plain["binding"] = json!("plain");
    let text = bundle_text(&dir, &plain).unwrap();
    assert!(!text.contains("scratch"), "{text}");
}

#[test]
fn a_scratch_directory_at_or_around_another_mount_of_its_unit_is_refused() {
    let dir = scratch("deploy-scratch-collision");
    // The state's own mount, a directory inside it, one that holds it, and the root's child that
    // holds both; then two scratch directories of one path from two sources.
    for (mount, words) in [
        ("/var/lib/flow", "is the mount of"),
        ("/var/lib/flow/run", "lies inside the mount of"),
        ("/var/lib", "holds the mount of"),
    ] {
        let mut b = base();
        b["packing"]["units"][1]["scratch"] = json!([{ "mount": mount }]);
        let r = bundle_text(&dir, &b).unwrap_err();
        refused_by(&r, "unit_mount_collision");
        assert!(
            r.detail.contains(&format!(
                "unit flow: scratch[0] of packing: unit flow {words} volume flow-state (parts: flow)"
            )),
            "{mount}: {}",
            r.detail
        );
        assert!(!r.detail.contains(mount), "{}", r.detail);
    }
    let mut b = base();
    b["parts"]["flow"]["scratch"] = json!([{ "mount": "/run/flow" }]);
    let r = bundle_text(&dir, &b).unwrap_err();
    refused_by(&r, "unit_mount_collision");
    assert!(
        r.detail.contains(
            "unit flow: scratch[0] of packing: unit flow is the mount of scratch[0] of parts: flow"
        ),
        "{}",
        r.detail
    );
    // A name that only starts like the other is no collision: paths compare by segment.
    let mut b = base();
    b["packing"]["units"][1]["scratch"] = json!([{ "mount": "/var/lib/flowx" }]);
    admitted(&dir, &b);
}

#[test]
fn a_relative_scratch_path_and_one_of_the_wrong_form_are_refused_as_unit_scratch_invalid() {
    let dir = scratch("deploy-scratch-invalid");
    for (scratch, says) in [
        (
            json!([{ "mount": "run/flow" }]),
            "mount is an absolute path",
        ),
        (
            json!([{ "mount": "/run/../flow" }]),
            "mount is an absolute path",
        ),
        (json!([{ "mount": "/" }]), "mount is an absolute path"),
        (
            json!([{ "mount": "/run/flow/" }]),
            "mount is an absolute path",
        ),
        (json!([{ "size": "64Mi" }]), "needs a `mount:`"),
        (
            json!([{ "mount": "/run/flow", "size": 64 }]),
            "size is text",
        ),
        (
            json!([{ "mount": "/run/flow", "medium": "memory" }]),
            "unknown key `medium`",
        ),
        (json!("/run/flow"), "scratch is a list of {mount, size}"),
    ] {
        let mut b = base();
        b["packing"]["units"][1]["scratch"] = scratch.clone();
        let r = bundle_text(&dir, &b).unwrap_err();
        refused_by(&r, "unit_scratch_invalid");
        assert!(r.detail.contains(says), "{scratch}: {}", r.detail);
        assert!(r.detail.contains("packing: unit flow"), "{}", r.detail);
    }
    // A part in no unit has no directory to give.
    let mut b = base();
    b["parts"]["owner"] = json!({ "scratch": [{ "mount": "/run/owner" }] });
    let r = bundle_text(&dir, &b).unwrap_err();
    refused_by(&r, "unit_scratch_invalid");
    assert!(
        r.detail
            .contains("parts: owner: scratch is for a part of a unit, and owner is in no unit"),
        "{}",
        r.detail
    );
}

#[test]
fn deploy_check_refuses_a_scratch_directory_changed_after_writing() {
    let dir = scratch("deploy-scratch-check");
    let text = bundle_text(&dir, &base()).unwrap();
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit); 4] = [
        // The carried directory moved onto the state's mount, the binding with it.
        ("unit_mount_collision", |v| {
            v["binding"]["document"]["packing"]["units"][1]["scratch"][0]["mount"] =
                json!("/var/lib/flow");
            for u in v["units"].as_array_mut().unwrap() {
                if u["name"] == "flow" {
                    u["scratch"][0]["mount"] = json!("/var/lib/flow/tmp");
                }
            }
        }),
        // The state's carried mount moved under the directory.
        ("unit_mount_collision", |v| {
            for u in v["units"].as_array_mut().unwrap() {
                if u["name"] == "flow" {
                    u["claims"]["volumes"][0]["mount"] = json!("/run/flow/state");
                }
            }
        }),
        // A relative path.
        ("unit_scratch_invalid", |v| {
            for u in v["units"].as_array_mut().unwrap() {
                if u["name"] == "flow" {
                    u["scratch"][0]["mount"] = json!("run/flow");
                }
            }
        }),
        // Dropped: not the scratch its binding resolves.
        ("unit_scratch_invalid", |v| {
            for u in v["units"].as_array_mut().unwrap() {
                if u["name"] == "flow" {
                    u.as_object_mut().unwrap().remove("scratch");
                }
            }
        }),
    ];
    let v: Value = parse_yaml(&text).unwrap();
    check(&dir, &v).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    for (name, edit) in cases {
        let mut v: Value = parse_yaml(&text).unwrap();
        edit(&mut v);
        v["binding"]["digest"] = json!(agenticspec_sim::deploy::digest(&v["binding"]["document"]));
        v["digest"] = json!(agenticspec_sim::deploy::bundle::recompute(&v));
        let r = check(&dir, &v).unwrap_err();
        assert!(names(&r).contains(name), "{name}: {}", r.detail);
    }
}
