//! A secret as a read-only file of a unit (`secret_files: [{secret, key, path}]` on a unit
//! `packing.units` lists or on a part of the unit, R120): carried in the unit's `config` beside
//! `env` and `args` by the secret's name, never a value; the name must be one a part of the unit
//! declares (`secret_undeclared`, `secret_value`), the entry of the right form with an absolute
//! path (`unit_config_invalid`), and its path no other mount of the unit, nor around or inside one
//! (`unit_mount_collision`, shared with the scratch directories of R118); `deploy check` applies
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
/// `/var/lib/flow`, a scratch directory at `/run/flow`, and the secret `flow-token` its part
/// declares given as the file `/etc/flow/token`; every crossing has a transport and every
/// destination is declared.
fn base() -> Value {
    json!({
        "format": "agenticspec-binding/1",
        "binding": "files",
        "architecture": "durable-state",
        "transports": { "run": "http", "model_call": "http" },
        "runtimes": { "core": image(1) },
        "parts": {
            "lead": { "egress": ["flow", "gateway"] },
            "flow": {
                "egress": ["gateway"],
                "secrets": ["flow-token"],
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
                    "scratch": [{ "mount": "/run/flow" }],
                    "secret_files": [
                        { "secret": "flow-token", "key": "token", "path": "/etc/flow/token" },
                    ],
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
fn a_secret_file_is_carried_in_its_unit_config_by_the_secret_name() {
    let dir = scratch("deploy-secret-files-carried");
    let mut b = base();
    // A part of the unit may give one too, and env beside it; the files sort by path.
    b["parts"]["flow"]["secret_files"] = json!([{ "secret": "flow-token", "path": "/etc/flow/a" }]);
    b["parts"]["flow"]["env"] = json!({ "FLOW_MODE": "run" });
    let text = bundle_text(&dir, &b).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let v = parse_yaml(&text).unwrap();
    assert_eq!(
        unit(&v, "flow")["config"],
        json!({
            "env": { "FLOW_MODE": "run" },
            "secret_files": [
                { "path": "/etc/flow/a", "secret": "flow-token" },
                { "path": "/etc/flow/token", "secret": "flow-token", "key": "token" },
            ],
        })
    );
    assert!(unit(&v, "core").get("config").is_none());
    check(&dir, &v).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    // Without secret files a unit's config has none, and a unit with nothing else none at all.
    let mut plain = base();
    plain["packing"]["units"][1]
        .as_object_mut()
        .unwrap()
        .remove("secret_files");
    let v = parse_yaml(&bundle_text(&dir, &plain).unwrap()).unwrap();
    assert!(unit(&v, "flow").get("config").is_none(), "{v}");
}

#[test]
fn a_secret_file_of_a_secret_no_part_of_its_unit_declares_is_refused() {
    let dir = scratch("deploy-secret-files-undeclared");
    let mut b = base();
    b["packing"]["units"][1]["secret_files"][0]["secret"] = json!("other-token");
    let r = bundle_text(&dir, &b).unwrap_err();
    refused_by(&r, "secret_undeclared");
    assert!(
        r.detail.contains(
            "packing: unit flow: secret_files[0] refers to secret other-token, and no part of unit flow declares it"
        ),
        "{}",
        r.detail
    );
    // A value where the name belongs is refused by where it is, never echoed.
    let mut b = base();
    b["packing"]["units"][1]["secret_files"][0]["secret"] = json!("sk-Abc123=");
    let r = bundle_text(&dir, &b).unwrap_err();
    refused_by(&r, "secret_value");
    assert!(
        r.detail
            .contains("packing: unit flow: secret_files[0]: secret is not a secret name"),
        "{}",
        r.detail
    );
    assert!(!r.detail.contains("sk-Abc123"), "{}", r.detail);
}

#[test]
fn a_secret_file_of_the_wrong_form_or_at_a_relative_path_is_refused_as_unit_config_invalid() {
    let dir = scratch("deploy-secret-files-invalid");
    for (entry, says) in [
        (
            json!({ "secret": "flow-token", "path": "etc/flow/token" }),
            "path is an absolute path",
        ),
        (json!({ "secret": "flow-token" }), "needs a `path:`"),
        (json!({ "path": "/etc/flow/token" }), "needs a `secret:`"),
        (
            json!({ "secret": "flow-token", "path": "/etc/flow/token", "key": "a b" }),
            "key is a key of the secret",
        ),
        (
            json!({ "secret": "flow-token", "path": "/etc/flow/token", "mode": "0400" }),
            "unknown key `mode`",
        ),
    ] {
        let mut b = base();
        b["packing"]["units"][1]["secret_files"] = json!([entry]);
        let r = bundle_text(&dir, &b).unwrap_err();
        refused_by(&r, "unit_config_invalid");
        assert!(
            r.detail.contains("packing: unit flow: secret_files[0]") && r.detail.contains(says),
            "{entry}: {}",
            r.detail
        );
    }
    let mut b = base();
    b["packing"]["units"][1]["secret_files"] = json!({ "secret": "flow-token" });
    refused_by(&bundle_text(&dir, &b).unwrap_err(), "unit_config_invalid");
}

#[test]
fn a_secret_file_at_or_around_another_mount_of_its_unit_is_refused() {
    let dir = scratch("deploy-secret-files-collision");
    // Two entries of one path, one inside the state's volume, one inside the scratch directory.
    for (path, says) in [
        (
            "/etc/flow/token",
            "secret_files[0] of packing: unit flow is the mount of secret_files[1] of packing: unit flow",
        ),
        (
            "/var/lib/flow/token",
            "secret_files[1] of packing: unit flow lies inside the mount of volume flow-state (parts: flow)",
        ),
        (
            "/run/flow/token",
            "secret_files[1] of packing: unit flow lies inside the mount of scratch[0] of packing: unit flow",
        ),
    ] {
        let mut b = base();
        b["packing"]["units"][1]["secret_files"]
            .as_array_mut()
            .unwrap()
            .push(json!({ "secret": "flow-token", "path": path }));
        let r = bundle_text(&dir, &b).unwrap_err();
        refused_by(&r, "unit_mount_collision");
        assert!(
            r.detail.contains(&format!("unit flow: {says}")),
            "{path}: {}",
            r.detail
        );
        assert!(!r.detail.contains(path), "{}", r.detail);
    }
}

#[test]
fn deploy_check_refuses_a_secret_file_changed_after_writing() {
    let dir = scratch("deploy-secret-files-check");
    let text = bundle_text(&dir, &base()).unwrap();
    let v: Value = parse_yaml(&text).unwrap();
    check(&dir, &v).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    type Edit = fn(&mut Value);
    let cases: [(&str, Edit); 3] = [
        ("unit_mount_collision", |f| {
            f["path"] = json!("/var/lib/flow/token")
        }),
        ("secret_undeclared", |f| f["secret"] = json!("other-token")),
        ("unit_config_invalid", |f| f["path"] = json!("etc/token")),
    ];
    for (name, edit) in cases {
        let mut v: Value = parse_yaml(&text).unwrap();
        for u in v["units"].as_array_mut().unwrap() {
            if u["name"] == "flow" {
                edit(&mut u["config"]["secret_files"][0]);
            }
        }
        v["digest"] = json!(agenticspec_sim::deploy::bundle::recompute(&v));
        let r = check(&dir, &v).unwrap_err();
        assert!(names(&r).contains(name), "{name}: {}", r.detail);
    }
}
