//! The UI units of a bundle: the operator UI the binding asks for under `ui:` as a `service`
//! unit with static code (the archive `agenticspec ui build --archive` writes, carried in
//! `files`), the ingress export it serves, its upstream, and the refusals of `deploy bundle` and
//! `deploy check` about it.

use super::common::{repo_root, scratch};
use agenticspec_sim::cli::{self, BundleOptions, ContractOptions};
use agenticspec_sim::deploy::bundle;
use agenticspec_sim::deploy::ui;
use agenticspec_sim::doc::{load_yaml, parse_yaml};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const FIXTURE: &str = "bundles/workstation.operator-ui.yaml";

fn fixture() -> Value {
    load_yaml(&repo_root().join(FIXTURE)).unwrap()
}

fn unit<'a>(v: &'a Value, name: &str) -> &'a Value {
    v["units"]
        .as_array()
        .unwrap()
        .iter()
        .find(|u| u["name"] == name)
        .unwrap_or_else(|| panic!("no unit {name}"))
}

fn entry<'a>(v: &'a Value, ingress: &str, protocol: &str) -> &'a Value {
    v["ingress"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["ingress"] == ingress && e["protocol"] == protocol)
        .unwrap_or_else(|| panic!("no {ingress} {protocol}"))
}

/// `deploy bundle` of the workstation under a binding written to `dir` with `text`.
fn bundle_of(dir: &Path, text: &str) -> Result<Value, cli::Refusal> {
    let root = repo_root();
    let file = dir.join("b.yaml");
    std::fs::write(&file, text).unwrap();
    let answer = cli::deploy_bundle(&BundleOptions {
        architecture: &root.join("architectures/workstation.yaml"),
        binding: &file,
        out: None,
        ports: Some(&root.join("blocks/ports.json")),
        accepts: Some(&root.join("bundles/cloudspec-accepts.yaml")),
        kpis: Some(&root.join("kpis/catalogue.yaml")),
    })?;
    Ok(parse_yaml(answer["bundle"].as_str().unwrap()).unwrap())
}

/// A binding that refines `base` (relative to the repository root) with `extra`.
fn refining(base: &str, extra: &str) -> String {
    format!(
        "format: agenticspec-binding/1\nbinding: odd\narchitecture: workstation\nrefines: {}\n{extra}",
        repo_root().join(base).display()
    )
}

/// The operator UI fixture changed by `edit`, its digests recomputed, through `deploy check`.
fn check_tampered(dir: &Path, edit: impl FnOnce(&mut Value)) -> Result<Value, cli::Refusal> {
    let mut v = fixture();
    edit(&mut v);
    v["digest"] = json!(bundle::recompute(&v));
    let file: PathBuf = dir.join("bundle.yaml");
    std::fs::write(&file, bundle::write(&v)).unwrap();
    cli::deploy_check(
        &file,
        Some(&repo_root().join("bundles/cloudspec-accepts.yaml")),
    )
}

fn names(detail: &str) -> Vec<String> {
    detail
        .split("; ")
        .map(|f| f.split(':').next().unwrap_or("").to_owned())
        .collect()
}

#[test]
fn the_operator_ui_is_a_service_unit_with_static_code_carried_in_files() {
    let v = fixture();
    let u = unit(&v, "operator-ui");
    assert_eq!(u["kind"], "service");
    assert_eq!(u["roles"], json!(["operator-ui"]));
    assert_eq!(u["parts"], json!([]));
    assert_eq!(u["egress"], json!([]));
    assert_eq!(u["secrets"], json!([]));
    assert_eq!(u["code"]["kind"], "static");
    assert_eq!(u["code"]["archive"], "operator-ui.tar");
    let digest = u["code"]["digest"].as_str().unwrap();
    assert!(
        agenticspec_sim::deploy::binding::is_digest(digest),
        "{digest}"
    );
    assert_eq!(
        u["checks"]["ready"],
        json!({ "kind": "http", "path": "/", "port": 8080 })
    );
    let file = &v["files"]["operator-ui.tar"];
    assert_eq!(file["digest"], digest);
    let bytes = ui::unbase64(file["base64"].as_str().unwrap()).unwrap();
    assert_eq!(file["size"], bytes.len());
    assert_eq!(agenticspec_sim::deploy::sha256(&bytes), digest);
    // The members are written in their order, `files` before `digest`.
    let text = std::fs::read_to_string(repo_root().join(FIXTURE)).unwrap();
    let keys: Vec<&str> = text
        .lines()
        .filter(|l| !l.starts_with([' ', '-']))
        .map(|l| l.split(':').next().unwrap())
        .collect();
    assert_eq!(&keys[keys.len() - 3..], ["runtimes", "files", "digest"]);
    // Without `ui:` the bundle has no UI unit and no `files`.
    let split = load_yaml(&repo_root().join("bundles/workstation.split.yaml")).unwrap();
    assert!(split.get("files").is_none());
    assert!(
        split["units"]
            .as_array()
            .unwrap()
            .iter()
            .all(|u| u["code"]["kind"] != "static")
    );
}

/// `ui: {control-plane: {port: 8090}}` adds the control-plane UI beside the operator UI: a
/// service unit of its own with static code, listening on its own port, the same upstream.
#[test]
fn the_control_plane_ui_is_a_second_service_unit_beside_the_operator_ui() {
    let v = fixture();
    let u = unit(&v, "control-plane-ui");
    assert_eq!(u["kind"], "service");
    assert_eq!(u["roles"], json!(["control-plane-ui"]));
    assert_eq!(u["parts"], json!([]));
    assert_eq!(u["egress"], json!([]));
    assert_eq!(u["code"]["kind"], "static");
    assert_eq!(u["code"]["archive"], "control-plane-ui.tar");
    assert_ne!(
        u["code"]["digest"],
        unit(&v, "operator-ui")["code"]["digest"]
    );
    for check in ["ready", "alive"] {
        assert_eq!(
            u["checks"][check],
            json!({ "kind": "http", "path": "/", "port": 8090 }),
            "{check}"
        );
    }
    assert_eq!(
        u["upstream"],
        json!({ "unit": "core", "port": 8080, "path": "/api/" })
    );
    let file = &v["files"]["control-plane-ui.tar"];
    assert_eq!(file["digest"], u["code"]["digest"]);
    // The operator UI serves the `ui` export when both are asked.
    assert_eq!(entry(&v, "operator", "ui")["unit"], "operator-ui");
}

/// Asked alone, the control-plane UI serves the `ui` export of the operator ingress.
#[test]
fn the_control_plane_ui_alone_serves_the_ui_export() {
    let dir = scratch("deploy-ui-control-alone");
    let text = refining("bundles/bindings/split.yaml", "ui:\n  control-plane: {}\n");
    let v = bundle_of(&dir, &text).unwrap();
    assert!(
        v["units"]
            .as_array()
            .unwrap()
            .iter()
            .all(|u| u["name"] != "operator-ui")
    );
    let e = entry(&v, "operator", "ui");
    assert_eq!(e["unit"], "control-plane-ui");
    assert_eq!(e["port"], 8080);
    assert_eq!(
        unit(&v, "control-plane-ui")["checks"]["ready"]["port"],
        8080
    );
}

#[test]
fn the_ui_export_of_the_operator_ingress_names_the_ui_unit_and_the_rest_stay_in_core() {
    let v = fixture();
    assert_eq!(
        entry(&v, "operator", "ui"),
        &json!({ "ingress": "operator", "protocol": "ui", "audience": "control.operator", "authenticated": true, "unit": "operator-ui", "part": "control", "port": 8080, "bind": "all" })
    );
    for protocol in ["http", "event_stream", "tool"] {
        assert_eq!(
            entry(&v, "operator", protocol)["unit"],
            "core",
            "{protocol}"
        );
    }
}

#[test]
fn the_upstream_is_the_unit_serving_the_api_unless_the_binding_names_one() {
    let v = fixture();
    assert_eq!(
        unit(&v, "operator-ui")["upstream"],
        json!({ "unit": "core", "port": 8080, "path": "/api/" })
    );
    let dir = scratch("deploy-ui-upstream");
    let named = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {unit: gateway, port: 8081}\n",
        ),
    )
    .unwrap();
    assert_eq!(
        unit(&named, "operator-ui")["upstream"],
        json!({ "unit": "gateway", "port": 8081, "path": "/api/" })
    );
    // The gateway serves no export of the operator ingress, so it has no port unless named.
    let unported = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {unit: gateway}\n",
        ),
    )
    .unwrap();
    assert_eq!(
        unit(&unported, "operator-ui")["upstream"],
        json!({ "unit": "gateway", "path": "/api/" })
    );
    // The single packing gives no export a port: `ui.operator` gives the UI's.
    let single = bundle_of(
        &dir,
        &refining(
            "crates/agenticspec-sim/tests/cases/deploy/egress/workstation.yaml",
            "ui:\n  operator: {port: 3000, bind: all}\n",
        ),
    )
    .unwrap();
    let e = entry(&single, "operator", "ui");
    assert_eq!(
        (&e["unit"], &e["port"], &e["bind"]),
        (&json!("operator-ui"), &json!(3000), &json!("all"))
    );
    assert_eq!(
        unit(&single, "operator-ui")["upstream"],
        json!({ "unit": "core", "path": "/api/" })
    );
}

#[test]
fn the_archive_digest_is_the_one_ui_build_archive_writes_for_the_same_inputs() {
    let root = repo_root();
    let dir = scratch("deploy-ui-archive");
    let architecture = root.join("architectures/workstation.yaml");
    let contract = dir.join("workstation.contract.yaml");
    cli::contract(
        &ContractOptions {
            architecture: &architecture,
            ports: Some(&root.join("blocks/ports.json")),
            out: Some(&contract),
            check: None,
            suite: None,
            trace: None,
            kpis: Some(&root.join("kpis/catalogue.yaml")),
        },
        &mut std::io::sink(),
    )
    .unwrap();
    let v = fixture();
    for (name, control_plane) in [("operator-ui", false), ("control-plane-ui", true)] {
        let files = agenticspec_ui::operator::build(&agenticspec_ui::operator::Inputs {
            root: root.clone(),
            architecture: architecture.clone(),
            contract: contract.clone(),
            api_base: agenticspec_ui::operator::API_BASE.to_string(),
            control_plane,
            prediction: None,
        })
        .unwrap_or_else(|e| panic!("{e:?}"));
        let tar = agenticspec_ui::operator::archive(&files).unwrap();
        assert_eq!(
            unit(&v, name)["code"]["digest"],
            format!("sha256:{}", agenticspec_ui::operator::sha256(&tar)),
            "{name}"
        );
        let archive = format!("{name}.tar");
        assert_eq!(
            ui::unbase64(v["files"][&archive]["base64"].as_str().unwrap()).unwrap(),
            tar,
            "{name}"
        );
    }
}

#[test]
fn the_binding_gives_the_ui_unit_its_claims_and_check_ports_by_its_unit_name() {
    let fixture = fixture();
    let u = unit(&fixture, "operator-ui");
    assert_eq!(
        u["claims"],
        json!({ "cpu_millis": 500, "memory_mib": 512, "accelerators": 0, "volumes": [] })
    );
    let dir = scratch("deploy-ui-claims");
    let v = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "claims:\n  operator-ui: {cpu_millis: 100, memory_mib: 64}\nchecks:\n  operator-ui:\n    ready: {port: 9000}\n",
        ),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let u = unit(&v, "operator-ui");
    assert_eq!(
        u["claims"],
        json!({ "cpu_millis": 100, "memory_mib": 64, "accelerators": 0, "volumes": [] })
    );
    assert_eq!(
        u["checks"],
        json!({
            "ready": { "kind": "http", "path": "/", "port": 9000 },
            "alive": { "kind": "http", "path": "/", "port": 8080 },
        })
    );
    // The UI unit is a service unit: it has no startup check, and without `ui:` it is no unit.
    for (base, extra, says) in [
        (
            "bundles/bindings/operator-ui.yaml",
            "checks:\n  operator-ui:\n    startup: {port: 9000}\n",
            "checks: operator-ui: a service unit has no startup check",
        ),
        (
            "bundles/bindings/split.yaml",
            "claims:\n  operator-ui: {cpu_millis: 100}\n",
            "claims: `operator-ui` is not a unit of the packing",
        ),
    ] {
        let r = bundle_of(&dir, &refining(base, extra)).unwrap_err();
        assert_eq!(r.code, "binding_invalid", "{extra}: {}", r.detail);
        assert!(r.detail.contains(says), "{extra}: {}", r.detail);
    }
}

#[test]
fn deploy_bundle_refuses_an_upstream_that_names_no_unit_and_a_ui_of_the_wrong_form() {
    let dir = scratch("deploy-ui-refused");
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {unit: adapter}\n",
        ),
    )
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["ui_upstream_unknown"], "{}", r.detail);
    for (extra, says) in [
        ("ui:\n  panel: {}\n", "ui: panel: needs `source`"),
        ("ui:\n  Console: {}\n", "`Console` is neither a built-in UI"),
        (
            "ui:\n  panel: {source: {archive: c.tar}, export: operator.ui}\n",
            "source is {ess_ui: <file>} or {archive: <file>, digest: sha256:<hex>}",
        ),
        (
            "ui:\n  panel: {source: {ess_ui: c.yaml}, export: ui}\n",
            "export is `<ingress>.<protocol>`",
        ),
        (
            "ui:\n  operator: {source: {ess_ui: c.yaml}}\n",
            "unknown key `source`",
        ),
        (
            "ui:\n  gateway: {source: {ess_ui: c.yaml}, export: operator.ui}\n",
            "the packing has a unit gateway",
        ),
        (
            "ui:\n  operator: {upstream: {host: x}}\n",
            "unknown key `host`",
        ),
        ("ui:\n  operator: {port: 0}\n", "port is a whole number"),
        ("ui:\n  operator: {port: 9000}\n", "give it in one place"),
        (
            "ingress:\n  operator:\n    exports:\n      ui: {part: gateway}\n",
            "the operator UI's unit serves this export",
        ),
    ] {
        let r = bundle_of(&dir, &refining("bundles/bindings/operator-ui.yaml", extra)).unwrap_err();
        assert_eq!(r.code, "binding_invalid", "{extra}: {}", r.detail);
        assert!(r.detail.contains(says), "{extra}: {}", r.detail);
    }
}

#[test]
fn deploy_check_admits_the_ui_bundle_and_refuses_a_changed_archive_or_an_unknown_upstream() {
    let dir = scratch("deploy-ui-check");
    check_tampered(&dir, |_| {}).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let ui_unit = |v: &mut Value| -> usize {
        v["units"]
            .as_array()
            .unwrap()
            .iter()
            .position(|u| u["name"] == "operator-ui")
            .unwrap()
    };
    let r = check_tampered(&dir, |v| {
        let b = v["files"]["operator-ui.tar"]["base64"].as_str().unwrap();
        let changed = format!("AAAA{}", &b[4..]);
        v["files"]["operator-ui.tar"]["base64"] = json!(changed);
    })
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["ui_archive_mismatch"], "{}", r.detail);
    let r = check_tampered(&dir, |v| {
        v["files"]
            .as_object_mut()
            .unwrap()
            .remove("operator-ui.tar");
    })
    .unwrap_err();
    assert_eq!(names(&r.detail), ["ui_archive_mismatch"], "{}", r.detail);
    let r = check_tampered(&dir, |v| {
        let i = ui_unit(v);
        v["units"][i]["upstream"]["unit"] = json!("adapter");
    })
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["ui_upstream_unknown"], "{}", r.detail);
}

/// The `config.json` of the archive of the UI unit `name` carried in the bundle `v`.
fn config_of(v: &Value, name: &str) -> Value {
    let archive = format!("{name}.tar");
    let tar = ui::unbase64(v["files"][&archive]["base64"].as_str().unwrap()).unwrap();
    let mut at = 0;
    while at + 512 <= tar.len() && tar[at] != 0 {
        let h = &tar[at..at + 512];
        let end = h[..100].iter().position(|b| *b == 0).unwrap_or(100);
        let file = std::str::from_utf8(&h[..end]).unwrap();
        let size = std::str::from_utf8(&h[124..136]).unwrap();
        let size = usize::from_str_radix(size.trim_matches(['\0', ' ']), 8).unwrap();
        if file == "config.json" {
            return serde_json::from_slice(&tar[at + 512..at + 512 + size]).unwrap();
        }
        at += 512 + size.div_ceil(512) * 512;
    }
    panic!("no config.json in {archive}");
}

/// Without `ui.<ui>.upstream.path` a UI reads its upstream's adapter at `/api/`: the unit's
/// `upstream.path` and the `api_base` of the built archive's `config.json`.
#[test]
fn the_upstream_path_is_api_by_default_in_the_unit_and_in_config_json() {
    let v = fixture();
    for name in ["operator-ui", "control-plane-ui"] {
        assert_eq!(unit(&v, name)["upstream"]["path"], "/api/", "{name}");
        assert_eq!(config_of(&v, name)["api_base"], "/api/", "{name}");
    }
    assert_eq!(unit(&v, "console")["upstream"]["path"], "/api/");
}

/// `ui.<ui>.upstream.path` is written into the unit's `upstream.path` and, for a UI the binary
/// builds, into the `api_base` of its `config.json`; the bundle passes `deploy check`.
#[test]
fn a_binding_upstream_path_is_carried_into_the_unit_and_config_json() {
    let dir = scratch("deploy-ui-upstream-path");
    let v = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {path: /ingress/operator/}\n  control-plane:\n    upstream: {path: /control/api/}\n  console:\n    upstream: {path: /console/api/}\n",
        ),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    for (name, path) in [
        ("operator-ui", "/ingress/operator/"),
        ("control-plane-ui", "/control/api/"),
    ] {
        assert_eq!(
            unit(&v, name)["upstream"],
            json!({ "unit": "core", "port": 8080, "path": path }),
            "{name}"
        );
        assert_eq!(config_of(&v, name)["api_base"], path, "{name}");
    }
    // The archive of a UI of the consumer's own is carried as it is; its unit names the path.
    assert_eq!(unit(&v, "console")["upstream"]["path"], "/console/api/");
    assert_eq!(
        v["files"]["console.tar"]["digest"],
        fixture()["files"]["console.tar"]["digest"]
    );
    assert_ne!(
        v["files"]["operator-ui.tar"]["digest"],
        fixture()["files"]["operator-ui.tar"]["digest"]
    );
    let file = dir.join("bundle.yaml");
    std::fs::write(&file, bundle::write(&v)).unwrap();
    cli::deploy_check(
        &file,
        Some(&repo_root().join("bundles/cloudspec-accepts.yaml")),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
}

/// A path that does not start with `/` is refused with `ui_upstream_path_invalid` by `deploy
/// bundle` and, in a written bundle, by `deploy check`; a path that is no string is a binding of
/// the wrong form.
#[test]
fn an_upstream_path_not_starting_with_a_slash_is_refused() {
    let dir = scratch("deploy-ui-upstream-path-invalid");
    for (key, path) in [
        ("operator", "api/"),
        ("control-plane", "\"\""),
        ("console", "http://adapter/api/"),
    ] {
        let r = bundle_of(
            &dir,
            &refining(
                "bundles/bindings/operator-ui.yaml",
                &format!("ui:\n  {key}:\n    upstream: {{path: {path}}}\n"),
            ),
        )
        .unwrap_err();
        assert_eq!(r.code, "not_admitted", "{key}: {}", r.detail);
        assert_eq!(
            names(&r.detail),
            ["ui_upstream_path_invalid"],
            "{key}: {}",
            r.detail
        );
        assert!(r.detail.contains(&format!("ui: {key}:")), "{}", r.detail);
    }
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {path: 3}\n",
        ),
    )
    .unwrap_err();
    assert_eq!(r.code, "binding_invalid", "{}", r.detail);
    assert!(r.detail.contains("upstream: path is"), "{}", r.detail);
    for path in [json!("api/"), json!(""), json!(3)] {
        let r = check_tampered(&dir, |v| {
            let i = v["units"]
                .as_array()
                .unwrap()
                .iter()
                .position(|u| u["name"] == "operator-ui")
                .unwrap();
            v["units"][i]["upstream"]["path"] = path.clone();
        })
        .unwrap_err();
        assert_eq!(r.code, "not_admitted", "{path}: {}", r.detail);
        assert_eq!(
            names(&r.detail),
            ["ui_upstream_path_invalid"],
            "{path}: {}",
            r.detail
        );
    }
}

#[test]
fn base64_reads_back_what_it_writes() {
    for bytes in [&b""[..], b"a", b"ab", b"abc", b"abcd", &[0, 255, 128, 7, 9]] {
        let text = ui::base64(bytes);
        assert_eq!(text.len() % 4, 0);
        assert_eq!(ui::unbase64(&text).as_deref(), Some(bytes), "{text}");
    }
    assert_eq!(ui::base64(b"foobar"), "Zm9vYmFy");
    assert_eq!(ui::base64(b"fo"), "Zm8=");
    for bad in ["Zm8", "Zm=8", "Z===", "Zm8=Zm8=", "Zm!="] {
        assert_eq!(ui::unbase64(bad), None, "{bad}");
    }
}

/// The archive of the consumer's own UI in the fixture binding.
const OWN_ARCHIVE: &str = "bundles/bindings/console-ui.tar";

/// `ui.<name>: {source, export, ...}` of a UI of the consumer's own, its archive by absolute path.
fn own_ui(name: &str, source: &str, export: &str) -> String {
    format!("ui:\n  {name}:\n    source: {source}\n    export: {export}\n")
}

fn archive_source(digest: &str) -> String {
    format!(
        "{{archive: {}, digest: \"{digest}\"}}",
        repo_root().join(OWN_ARCHIVE).display()
    )
}

fn own_digest() -> String {
    agenticspec_sim::deploy::sha256(&std::fs::read(repo_root().join(OWN_ARCHIVE)).unwrap())
}

/// `ui.console: {source: {archive, digest}, export: operator.ui, port: 8091}`: a unit `console`,
/// `roles: [ui]`, whose code is the consumer's archive, carried byte for byte in `files`, on its
/// own port beside the operator UI, which keeps the export.
#[test]
fn a_ui_of_the_consumers_own_is_a_service_unit_carrying_its_archive() {
    let v = fixture();
    let u = unit(&v, "console");
    assert_eq!(u["kind"], "service");
    assert_eq!(u["roles"], json!(["ui"]));
    assert_eq!(u["parts"], json!([]));
    assert_eq!(u["egress"], json!([]));
    assert_eq!(
        u["code"],
        json!({ "kind": "static", "archive": "console.tar", "digest": own_digest() })
    );
    for check in ["ready", "alive"] {
        assert_eq!(
            u["checks"][check],
            json!({ "kind": "http", "path": "/", "port": 8091 }),
            "{check}"
        );
    }
    assert_eq!(
        u["upstream"],
        json!({ "unit": "core", "port": 8080, "path": "/api/" })
    );
    let file = &v["files"]["console.tar"];
    assert_eq!(file["digest"], own_digest());
    assert_eq!(
        ui::unbase64(file["base64"].as_str().unwrap()).unwrap(),
        std::fs::read(repo_root().join(OWN_ARCHIVE)).unwrap()
    );
    assert_eq!(entry(&v, "operator", "ui")["unit"], "operator-ui");
    // Asked alone, it serves its export; the binding's file name is read beside the binding.
    let dir = scratch("deploy-ui-own-alone");
    let text = refining(
        "bundles/bindings/split.yaml",
        &own_ui("console", &archive_source(&own_digest()), "operator.ui"),
    );
    let alone = bundle_of(&dir, &text).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let e = entry(&alone, "operator", "ui");
    assert_eq!(
        (&e["unit"], &e["part"]),
        (&json!("console"), &json!("control"))
    );
    assert_eq!(unit(&alone, "console")["checks"]["ready"]["port"], 8080);
    // The fixture bundle, with its `console` unit, passes `deploy check`.
    check_tampered(&dir, |_| {}).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
}

/// An archive whose bytes are not the digest the binding names is refused at bundle time, and a
/// carried archive changed afterwards at check time, both `ui_archive_mismatch`.
#[test]
fn an_archive_that_is_not_its_digest_is_refused() {
    let dir = scratch("deploy-ui-own-mismatch");
    let wrong = format!("sha256:{}", "0".repeat(64));
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/split.yaml",
            &own_ui("console", &archive_source(&wrong), "operator.ui"),
        ),
    )
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["ui_archive_mismatch"], "{}", r.detail);
    assert!(r.detail.contains(&own_digest()), "{}", r.detail);
    let r = check_tampered(&dir, |v| {
        let b = v["files"]["console.tar"]["base64"].as_str().unwrap();
        let changed = format!("AAAA{}", &b[4..]);
        v["files"]["console.tar"]["base64"] = json!(changed);
    })
    .unwrap_err();
    assert_eq!(names(&r.detail), ["ui_archive_mismatch"], "{}", r.detail);
    // A file that cannot be read does not build.
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/split.yaml",
            &own_ui(
                "console",
                &format!("{{archive: missing.tar, digest: \"{}\"}}", own_digest()),
                "operator.ui",
            ),
        ),
    )
    .unwrap_err();
    assert_eq!(names(&r.detail), ["ui_build_refused"], "{}", r.detail);
}

/// `{ess_ui: <file>}` is refused with `ui_build_needs_toolchain`: the pages `ui generate` writes
/// need node and vite to build; the refusal names the archive form.
#[test]
fn an_ess_ui_source_is_refused_naming_the_archive_form() {
    let dir = scratch("deploy-ui-own-ess-ui");
    let doc = repo_root().join("ui/views/examples/workstation.ui.yaml");
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/split.yaml",
            &own_ui(
                "console",
                &format!("{{ess_ui: {}}}", doc.display()),
                "operator.ui",
            ),
        ),
    )
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(
        names(&r.detail),
        ["ui_build_needs_toolchain"],
        "{}",
        r.detail
    );
    for says in ["node and vite", "archive: <file>, digest: sha256:<hex>"] {
        assert!(r.detail.contains(says), "{says}: {}", r.detail);
    }
}

/// An export no ingress of the architecture has is refused with `ui_export_unknown`.
#[test]
fn a_ui_of_an_export_the_architecture_has_not_is_refused() {
    let dir = scratch("deploy-ui-own-export");
    for export in ["operator.console", "nowhere.ui"] {
        let r = bundle_of(
            &dir,
            &refining(
                "bundles/bindings/split.yaml",
                &own_ui("console", &archive_source(&own_digest()), export),
            ),
        )
        .unwrap_err();
        assert_eq!(r.code, "not_admitted", "{export}: {}", r.detail);
        assert_eq!(
            names(&r.detail),
            ["ui_export_unknown"],
            "{export}: {}",
            r.detail
        );
        assert!(r.detail.contains(export), "{}", r.detail);
    }
}

/// The built-in sources give the same units, files and ingress with or without a UI of the
/// consumer's own beside them.
#[test]
fn the_built_in_sources_are_unchanged_by_a_ui_of_the_consumers_own() {
    let dir = scratch("deploy-ui-own-built-in");
    let without = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/split.yaml",
            "ui:\n  operator: {}\n  control-plane: {port: 8090}\n",
        ),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let with = fixture();
    for name in ["operator-ui", "control-plane-ui"] {
        assert_eq!(unit(&with, name), unit(&without, name), "{name}");
        let archive = format!("{name}.tar");
        assert_eq!(
            with["files"][&archive], without["files"][&archive],
            "{name}"
        );
    }
    assert_eq!(with["ingress"], without["ingress"]);
}

/// A binding that gives the operator UI the client secret `ui-client-token`, which the part `chat`
/// of the core unit declares.
const WITH_SECRET: &str = "ui:\n  operator:\n    upstream: {secret: ui-client-token}\nparts:\n  chat:\n    secrets: [ui-client-token]\n";

/// A written bundle (a bundle `v`) through `deploy check`, its digest recomputed.
fn check_bundle(dir: &Path, mut v: Value) -> Result<Value, cli::Refusal> {
    v["binding"]["digest"] = json!(agenticspec_sim::deploy::digest(&v["binding"]["document"]));
    v["digest"] = json!(bundle::recompute(&v));
    let file: PathBuf = dir.join("bundle.yaml");
    std::fs::write(&file, bundle::write(&v)).unwrap();
    cli::deploy_check(
        &file,
        Some(&repo_root().join("bundles/cloudspec-accepts.yaml")),
    )
}

/// `ui.<ui>.upstream.secret` names the credential the UI presents to its upstream: the UI unit
/// carries it in `upstream.secret` and in `secrets`, the upstream unit holds it through a part,
/// and `deploy check` admits the bundle. A UI that names none keeps its unit.
#[test]
fn an_upstream_secret_is_carried_in_the_ui_unit_and_held_by_the_upstream_unit() {
    let dir = scratch("deploy-ui-upstream-secret");
    let v = bundle_of(
        &dir,
        &refining("bundles/bindings/operator-ui.yaml", WITH_SECRET),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let u = unit(&v, "operator-ui");
    assert_eq!(
        u["upstream"],
        json!({ "unit": "core", "port": 8080, "path": "/api/", "secret": "ui-client-token" })
    );
    assert_eq!(u["secrets"], json!(["ui-client-token"]));
    assert_eq!(unit(&v, "core")["secrets"], json!(["ui-client-token"]));
    assert_eq!(unit(&v, "control-plane-ui")["secrets"], json!([]));
    assert!(unit(&v, "control-plane-ui")["upstream"]["secret"].is_null());
    check_bundle(&dir, v).unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
}

/// No part of the upstream unit declares the secret: `secret_undeclared`, naming the UI and the
/// unit.
#[test]
fn an_upstream_secret_no_part_of_the_upstream_unit_declares_is_refused() {
    let dir = scratch("deploy-ui-upstream-secret-undeclared");
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {secret: ui-client-token}\n",
        ),
    )
    .unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["secret_undeclared"], "{}", r.detail);
    assert!(r.detail.contains("ui: operator"), "{}", r.detail);
    assert!(r.detail.contains("unit core"), "{}", r.detail);
    assert!(r.detail.contains("ui-client-token"), "{}", r.detail);
    // A part of another unit declaring it is no help.
    let r = bundle_of(
        &dir,
        &refining(
            "bundles/bindings/operator-ui.yaml",
            "ui:\n  operator:\n    upstream: {secret: ui-client-token}\nparts:\n  gateway:\n    secrets: [model-api-key, ui-client-token]\n",
        ),
    )
    .unwrap_err();
    assert_eq!(names(&r.detail), ["secret_undeclared"], "{}", r.detail);
}

/// A value that is no secret name is refused as `secret_value`, and the finding does not repeat
/// it.
#[test]
fn an_upstream_secret_that_is_no_name_is_refused_without_echoing_it() {
    let dir = scratch("deploy-ui-upstream-secret-value");
    let token: &str = &format!("sk{}{}", "0123456789abcdef".repeat(2), "0123456789");
    for secret in [token, "9871", "Token"] {
        let r = bundle_of(
            &dir,
            &refining(
                "bundles/bindings/operator-ui.yaml",
                &format!("ui:\n  operator:\n    upstream: {{secret: {secret}}}\n"),
            ),
        )
        .unwrap_err();
        assert_eq!(r.code, "not_admitted", "{secret}: {}", r.detail);
        assert_eq!(names(&r.detail), ["secret_value"], "{secret}: {}", r.detail);
        assert!(!r.detail.contains(secret), "{}", r.detail);
        assert!(r.detail.contains("ui: operator"), "{}", r.detail);
    }
}

/// A written bundle whose UI unit's `upstream.secret` is not in its `secrets`, or names a secret
/// its upstream unit does not hold, is refused with `secret_undeclared`; one that is no name,
/// with `secret_value`.
#[test]
fn deploy_check_refuses_an_upstream_secret_the_unit_or_its_upstream_does_not_hold() {
    let dir = scratch("deploy-ui-upstream-secret-check");
    let base = bundle_of(
        &dir,
        &refining("bundles/bindings/operator-ui.yaml", WITH_SECRET),
    )
    .unwrap_or_else(|r| panic!("{}: {}", r.code, r.detail));
    let at = |v: &Value, name: &str| {
        v["units"]
            .as_array()
            .unwrap()
            .iter()
            .position(|u| u["name"] == name)
            .unwrap()
    };
    let mut v = base.clone();
    let i = at(&v, "operator-ui");
    v["units"][i]["secrets"] = json!([]);
    let r = check_bundle(&dir, v).unwrap_err();
    assert_eq!(r.code, "not_admitted", "{}", r.detail);
    assert_eq!(names(&r.detail), ["secret_undeclared"], "{}", r.detail);
    let mut v = base.clone();
    let i = at(&v, "core");
    v["units"][i]["secrets"] = json!([]);
    v["binding"]["document"]["parts"]["chat"]["secrets"] = json!([]);
    let r = check_bundle(&dir, v).unwrap_err();
    assert_eq!(names(&r.detail), ["secret_undeclared"], "{}", r.detail);
    assert!(r.detail.contains("ui-client-token"), "{}", r.detail);
    let token: &str = &format!("sk{}{}", "0123456789abcdef".repeat(2), "0123456789");
    let mut v = base.clone();
    let i = at(&v, "operator-ui");
    v["units"][i]["upstream"]["secret"] = json!(token);
    let r = check_bundle(&dir, v).unwrap_err();
    assert!(
        names(&r.detail).iter().any(|n| n == "secret_value"),
        "{}",
        r.detail
    );
    assert!(!r.detail.contains(token), "{}", r.detail);
}
