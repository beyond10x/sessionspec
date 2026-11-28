//! The settings table (`blocks/settings.yaml`): one refusal case per rule and per type kind, the
//! table against the part kinds, and every committed architecture checked clean.

use super::common::{ARCH, repo_root, yaml};
use agenticspec_sim::check::{GATE_KPIS, Refusal, check};
use agenticspec_sim::settings::{self, Kind, Narrows};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn refused(arch: &Value) -> Vec<(String, String)> {
    check(arch)
        .iter()
        .map(|r| (r.rule().to_string(), r.part().to_string()))
        .collect()
}

/// `part.setting = v` on the small architecture, and its one refusal, which must be of `rule`.
fn one(part: &str, setting: &str, v: Value, rule: &str) -> Refusal {
    let mut a = yaml(ARCH);
    a["parts"][part][setting] = v.clone();
    let r = check(&a);
    assert_eq!(
        r.iter().map(|r| (r.rule(), r.part())).collect::<Vec<_>>(),
        vec![(rule, part)],
        "{part}.{setting} = {v}: {r:?}"
    );
    r.into_iter().next().unwrap()
}

/// `part.setting = v` on the small architecture is accepted.
fn fine(part: &str, setting: &str, v: Value) {
    let mut a = yaml(ARCH);
    a["parts"][part][setting] = v.clone();
    assert_eq!(check(&a), Vec::<Refusal>::new(), "{part}.{setting} = {v}");
}

#[test]
fn the_settings_rules_are_gate_kpis() {
    for rule in ["setting_unknown", "setting_invalid"] {
        assert!(GATE_KPIS.contains(&rule), "{rule} is not a gate KPI");
    }
}

#[test]
fn a_setting_the_block_does_not_declare_is_refused() {
    let r = one("box", "flavour", json!("strong"), "setting_unknown");
    let line = r.to_string();
    for want in [
        "box",
        "flavour",
        "sandbox",
        "isolation",
        "egress",
        "count",
        "per",
    ] {
        assert!(line.contains(want), "{want} missing in {line}");
    }
    // A setting of another block is no setting of this one.
    one("model", "capacity", json!(4), "setting_unknown");
    // A part of a pack block takes only the settings of every part.
    let mut a = yaml(ARCH);
    a["parts"]["forge"] = json!({"block": "scm.forge", "kind": "hosted"});
    a["links"]
        .as_array_mut()
        .unwrap()
        .push(json!({"from": "operator", "to": "forge", "carries": "push"}));
    assert_eq!(
        refused(&a),
        vec![("setting_unknown".into(), "forge".into())]
    );
}

/// The refusal names the part, the setting and the expected type.
fn invalid(part: &str, setting: &str, v: Value, expected: &str) {
    let r = one(part, setting, v.clone(), "setting_invalid");
    let line = r.to_string();
    for want in [part, setting, expected] {
        assert!(
            line.contains(want),
            "{part}.{setting} = {v}: `{want}` missing in {line}"
        );
    }
}

#[test]
fn an_integer_setting_takes_a_whole_number_in_range() {
    invalid(
        "builds",
        "capacity",
        json!("many"),
        "an integer of at least 1 (slots)",
    );
    invalid("builds", "capacity", json!(2.5), "an integer");
    invalid("builds", "capacity", json!(-1), "an integer of at least 1");
    invalid("worker", "count", json!(-2), "an integer of at least 1");
    fine("builds", "capacity", json!(1));
}

#[test]
fn a_number_setting_takes_a_number_in_range() {
    invalid(
        "gateway",
        "rate_limit_per_minute",
        json!("fast"),
        "a number of at least 0 (calls per minute)",
    );
    invalid(
        "gateway",
        "rate_limit_per_minute",
        json!(-5),
        "a number of at least 0",
    );
    invalid(
        "worker",
        "false_accept",
        json!(1.5),
        "a number from 0 to 1 (share)",
    );
    fine("worker", "false_accept", json!(0.25));
}

#[test]
fn an_enum_setting_takes_one_of_its_words() {
    invalid(
        "box",
        "isolation",
        json!("airtight"),
        "one of none, process, container, user_kernel, micro_vm",
    );
    invalid(
        "chat",
        "distribution",
        json!(3),
        "one of direct, topic, queue, board",
    );
    fine("box", "isolation", json!("micro_vm"));
    // A block's enum may also be written as the block writes the variant.
    fine("box", "isolation", json!("MicroVm"));
    // Words of `values` are taken as written only.
    invalid("worker", "partition", json!("Scope"), "one of scope");
}

#[test]
fn a_duration_setting_takes_a_duration_or_seconds() {
    invalid(
        "builds",
        "job_limit",
        json!("soon"),
        "a duration of at least 1s",
    );
    invalid(
        "builds",
        "job_limit",
        json!("-10m"),
        "a duration of at least 1s",
    );
    fine("builds", "job_limit", json!("1h"));
    fine("builds", "job_limit", json!(600));
}

#[test]
fn a_string_setting_takes_a_text() {
    invalid("model", "route", json!(5), "a text");
    invalid("operator", "role", json!(["operator"]), "a text");
}

#[test]
fn a_part_setting_takes_the_name_of_a_part() {
    invalid("tree", "per", json!(3), "the name of a part");
    invalid(
        "operator",
        "acts_for",
        json!(["owner"]),
        "the name of a part",
    );
    invalid("box", "egress", json!([1]), "a list of names of parts");
    fine("box", "egress", json!("gateway"));
}

#[test]
fn a_shares_setting_maps_parts_to_shares() {
    invalid(
        "worker",
        "model_share",
        json!({"model": -1}),
        "a map from model parts to shares",
    );
    invalid(
        "worker",
        "model_share",
        json!(0.5),
        "a map from model parts to shares",
    );
    fine("worker", "model_share", json!({"model": 1}));
}

#[test]
fn a_runtime_part_may_keep_durable_state_off_by_default() {
    let s = settings::table()
        .setting("runtime", "durable_state")
        .unwrap();
    assert_eq!(s.ty.kind, Kind::Boolean);
    assert_eq!(s.default, Some(json!(false)));
    assert_eq!(s.narrows, Narrows::None);
    assert!(!s.summary.is_empty());
    let path = repo_root().join(
        "crates/agenticspec-sim/tests/cases/deploy/durable_state_unplaceable.architecture.yaml",
    );
    let mut a = agenticspec_sim::doc::load_yaml(&path).unwrap();
    assert_eq!(check(&a), Vec::<Refusal>::new());
    a["parts"]["flow"]["durable_state"] = json!("yes");
    assert_eq!(
        refused(&a),
        vec![("setting_invalid".to_string(), "flow".to_string())]
    );
    a["parts"]["lead"]["durable_state"] = json!(true);
    a["parts"]["flow"]["durable_state"] = json!(true);
    assert_eq!(
        refused(&a),
        vec![("setting_unknown".to_string(), "lead".to_string())]
    );
}

#[test]
fn a_runtime_part_with_durable_state_may_name_the_outside_writers_of_it() {
    let s = settings::table()
        .setting("runtime", "state_writers")
        .unwrap();
    assert_eq!(s.ty.kind, Kind::String { list: true });
    assert_eq!(s.ty.describe(), "a non-empty list of texts");
    assert_eq!(s.ty.name(), "string list");
    assert_eq!(s.default, None);
    assert_eq!(s.narrows, Narrows::None);
    assert!(!s.summary.is_empty());
    // A plain text setting is unchanged.
    let kind = settings::table().setting("runtime", "kind").unwrap();
    assert_eq!(kind.ty.kind, Kind::String { list: false });
    assert_eq!(kind.ty.describe(), "a text");
    assert!(kind.ty.accepts(&json!("")).is_ok());

    let path = repo_root().join(
        "crates/agenticspec-sim/tests/cases/deploy/durable_state_unplaceable.architecture.yaml",
    );
    let mut a = agenticspec_sim::doc::load_yaml(&path).unwrap();
    a["parts"]["flow"]["state_writers"] = json!(["backup", "indexer"]);
    assert_eq!(check(&a), Vec::<Refusal>::new());
    for bad in [
        json!([]),
        json!("backup"),
        json!([""]),
        json!(["backup", 1]),
        json!({"writers": ["backup"]}),
        json!(true),
    ] {
        a["parts"]["flow"]["state_writers"] = bad.clone();
        let r = check(&a);
        assert_eq!(
            refused(&a),
            vec![("setting_invalid".to_string(), "flow".to_string())],
            "{bad}"
        );
        assert!(
            r[0].to_string().contains(&format!(
                "setting `state_writers` is {bad}, expected a non-empty list of texts"
            )),
            "{}",
            r[0]
        );
    }
    // Writers of a state the part does not keep are refused: `durable_state` absent or false.
    a["parts"]["flow"]["state_writers"] = json!(["backup"]);
    for durable in [None, Some(json!(false))] {
        let flow = a["parts"]["flow"].as_object_mut().unwrap();
        match &durable {
            None => flow.remove("durable_state"),
            Some(d) => flow.insert("durable_state".into(), d.clone()),
        };
        let r = check(&a);
        assert_eq!(
            refused(&a),
            vec![("setting_invalid".to_string(), "flow".to_string())],
            "durable_state {durable:?}"
        );
        assert!(
            r[0].to_string().contains(
                "setting `state_writers` is [\"backup\"], expected writers of durable state, on a part with `durable_state: true`"
            ),
            "{}",
            r[0]
        );
    }
    // Only a runtime part keeps durable state.
    a["parts"]["flow"]["durable_state"] = json!(true);
    a["parts"]["lead"]["state_writers"] = json!(["backup"]);
    assert_eq!(
        refused(&a),
        vec![("setting_unknown".to_string(), "lead".to_string())]
    );
}

#[test]
fn a_string_list_setting_takes_a_non_empty_list_of_texts() {
    let t = settings::parse(
        "format: agenticspec-settings/1\nevery_part: []\nblocks:\n  - kind: gate\n    domain: agentic.gate\n    settings:\n      - {name: keys, type: string, list: true, summary: The keys that open the gate.}\n",
    )
    .unwrap();
    let keys = t.setting("gate", "keys").unwrap();
    assert_eq!(keys.ty.kind, Kind::String { list: true });
    assert!(keys.ty.accepts(&json!(["brass", "iron"])).is_ok());
    for bad in [json!([]), json!("brass"), json!([""]), json!([1])] {
        assert_eq!(
            keys.ty.accepts(&bad),
            Err("a non-empty list of texts".into()),
            "{bad}"
        );
    }
}

#[test]
fn a_boolean_setting_takes_true_or_false() {
    // The type read from a table of its own, beside the runtime's `durable_state`.
    let t = settings::parse(
        "format: agenticspec-settings/1\nevery_part: []\nblocks:\n  - kind: gate\n    domain: agentic.gate\n    settings:\n      - {name: open, type: boolean, default: false, summary: Whether the gate is open.}\n",
    )
    .unwrap();
    let open = t.setting("gate", "open").unwrap();
    assert_eq!(open.ty.kind, Kind::Boolean);
    assert!(open.ty.accepts(&json!(true)).is_ok());
    assert_eq!(open.ty.accepts(&json!("yes")), Err("true or false".into()));
}

#[test]
fn a_marked_place_is_judged_by_its_value() {
    invalid(
        "builds",
        "capacity",
        json!({"value": "many", "vary": [1, 2]}),
        "an integer",
    );
}

#[test]
fn a_table_that_does_not_hold_is_refused() {
    let head = "format: agenticspec-settings/1\nevery_part: []\nblocks:\n  - kind: k\n    domain: agentic.k\n    settings:\n";
    for (body, want) in [
        ("      - {name: a, type: colour}\n", "unknown type"),
        (
            "      - {name: a, type: enum, enum: agentic.nowhere.Kind}\n",
            "no block declares",
        ),
        (
            "      - {name: a, type: integer, min: 0, default: -1}\n",
            "default",
        ),
        (
            "      - {name: a, type: string, colour: red}\n",
            "unknown key",
        ),
        (
            "      - {name: a, type: string}\n      - {name: a, type: string}\n",
            "twice",
        ),
    ] {
        let e = settings::parse(&format!("{head}{body}")).unwrap_err();
        assert!(e.contains(want), "{body}: {e}");
    }
}

#[test]
fn every_part_kind_has_its_settings_and_every_entry_is_a_part_kind() {
    let t = settings::table();
    let kinds = agenticspec_sim::check::part_kinds();
    for k in kinds {
        assert!(
            t.block(k).is_some(),
            "part kind `{k}` has no entry in blocks/settings.yaml"
        );
    }
    for b in &t.blocks {
        assert!(kinds.contains(&b.kind), "`{}` is no part kind", b.kind);
    }
    // The contract copies what the table marks: a pool's claim port takes its three settings.
    assert_eq!(
        t.narrowing("pool", "resource.claim"),
        vec!["kind", "capacity", "job_limit"]
    );
    assert_eq!(t.narrowing("agent", "model.inference"), Vec::<&str>::new());
    assert!(
        t.blocks
            .iter()
            .flat_map(|b| &b.settings)
            .any(|s| s.narrows == Narrows::All)
    );
}

fn yaml_files(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yaml"))
        .collect();
    out.sort();
    out
}

/// Every committed architecture document: under `architectures/` and its ladder, the
/// architectures the fields list and the catalog's downloads.
fn committed() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = yaml_files(&root.join("architectures"));
    out.extend(yaml_files(&root.join("architectures/ladder")));
    for field in std::fs::read_dir(root.join("fields")).unwrap().flatten() {
        let manifest = field.path().join("field.yaml");
        let Ok(text) = std::fs::read_to_string(&manifest) else {
            continue;
        };
        for a in yaml(&text)["architectures"]
            .as_array()
            .into_iter()
            .flatten()
        {
            out.push(field.path().join(a.as_str().unwrap()));
        }
    }
    for entry in std::fs::read_dir(root.join("website/static/catalog"))
        .unwrap()
        .flatten()
    {
        out.extend(
            yaml_files(&entry.path())
                .into_iter()
                .filter(|p| std::fs::read_to_string(p).is_ok_and(|t| t.contains("\nparts:"))),
        );
    }
    out
}

#[test]
fn every_committed_architecture_checks_clean() {
    let files = committed();
    assert!(files.len() > 30, "{}", files.len());
    for path in files {
        let a = agenticspec_sim::nest::load(&path)
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let refusals: Vec<String> = check(&agenticspec_sim::nest::flatten(&a))
            .iter()
            .map(ToString::to_string)
            .collect();
        assert!(refusals.is_empty(), "{}: {refusals:?}", path.display());
    }
}

/// The table and the simulator: every setting `sim::PART_READS` says the simulator reads from a
/// part of a block is a setting the table declares for that block, so `sim check` accepts what
/// `sim run` simulates.
#[test]
fn every_setting_the_simulator_reads_is_declared_for_its_block() {
    let t = settings::table();
    let mut missing = Vec::new();
    for (block, keys) in agenticspec_sim::sim::PART_READS {
        assert!(
            t.block(block).is_some(),
            "the table lists no block `{block}`"
        );
        let names = t.names(block);
        for key in *keys {
            if !names.contains(key) {
                missing.push(format!("{block}.{key}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "the simulator reads settings blocks/settings.yaml does not declare: {missing:?}"
    );
}

/// The sources of the simulator (`src/sim.rs` and `src/sim/`), the body of `fn setting` left out.
fn simulator_sources() -> Vec<(String, String)> {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = vec![src.join("sim.rs")];
    let mut sub: Vec<PathBuf> = std::fs::read_dir(src.join("sim"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    sub.sort();
    files.extend(sub);
    files
        .into_iter()
        .map(|f| {
            let text = std::fs::read_to_string(&f).unwrap();
            let text = match text.find("fn setting<'a>") {
                Some(start) => {
                    let end = start + text[start..].find("\n}\n").expect("end of fn setting");
                    format!("{}{}", &text[..start], &text[end..])
                }
                None => text,
            };
            (f.display().to_string(), text)
        })
        .collect()
}

/// `PART_READS` cannot fall behind the code: the simulator reads a part's setting only through
/// `setting` (which asserts in a debug build that the part's block lists the key), every key it
/// names there is listed, and every listed key is read.
#[test]
fn the_simulator_reads_a_part_only_through_setting() {
    let raw = [
        // `p["kind"]`, `p[key]`, `p.get("kind")`: a part read past `setting`.
        regex::Regex::new(r#"\bp\s*\[\s*("(\w+)"|[a-z_]+)\s*\]|\bp\.get\("#).unwrap(),
        // `part(n)["route"]`, `arch["parts"][n]["route"]`.
        regex::Regex::new(r#"\bpart\([^)]*\)\s*\[|\["parts"\]\s*\[[^\]]+\]\s*\["#).unwrap(),
    ];
    let named = regex::Regex::new(r#"\bsetting\([^,]+,\s*"(\w+)"\s*\)"#).unwrap();
    let listed: BTreeSet<&str> = agenticspec_sim::sim::PART_READS
        .iter()
        .flat_map(|(_, keys)| keys.iter().copied())
        .collect();
    let mut read = BTreeSet::new();
    let mut past = Vec::new();
    for (file, text) in simulator_sources() {
        for (n, line) in text.lines().enumerate() {
            for re in &raw {
                for m in re.captures_iter(line) {
                    if m.get(2).map(|k| k.as_str()) != Some("block") {
                        past.push(format!("{file}:{}: {}", n + 1, line.trim()));
                    }
                }
            }
            for m in named.captures_iter(line) {
                read.insert(m[1].to_string());
            }
        }
    }
    assert!(
        past.is_empty(),
        "read a part through sim::setting, and list the key in PART_READS:\n{}",
        past.join("\n")
    );
    let unlisted: Vec<&String> = read
        .iter()
        .filter(|k| !listed.contains(k.as_str()))
        .collect();
    assert!(
        unlisted.is_empty(),
        "read but not in PART_READS: {unlisted:?}"
    );
    // Read through `check`, not `setting`: an agent's `role` (also read through `setting`) and a
    // channel's `distribution`.
    let stale: Vec<&&str> = listed
        .iter()
        .filter(|k| !read.contains(**k) && **k != "distribution")
        .collect();
    assert!(
        stale.is_empty(),
        "listed in PART_READS, never read: {stale:?}"
    );
}

/// The settings refusals of `arch` alone, as (rule, part, setting).
fn setting_refusals(arch: &Value) -> Vec<(String, String, String)> {
    check(arch)
        .into_iter()
        .filter_map(|r| match r {
            Refusal::SettingUnknown { part, setting, .. } => {
                Some(("setting_unknown".to_string(), part, setting))
            }
            Refusal::SettingInvalid { part, setting, .. } => {
                Some(("setting_invalid".to_string(), part, setting))
            }
            _ => None,
        })
        .collect()
}

/// The small architecture with part `name` added as `part`.
fn with_part(name: &str, part: Value) -> Value {
    let mut a = yaml(ARCH);
    a["parts"][name] = part;
    a
}

#[test]
fn a_part_carries_notes_the_check_does_not_type() {
    let notes = json!({
        "file": "screens/inbox.vue",
        "audience": ["owner", "reviewer"],
        "seats": 5,
        "model": {"use": "triage", "fallback": null},
        "vary": [1, 2],
    });
    // On a part of a core block, of a block whose settings are few, and of a pack block.
    fine("model", "notes", notes.clone());
    fine("chat", "notes", json!({}));
    let a = with_part(
        "forge",
        json!({"block": "scm.forge", "notes": notes.clone()}),
    );
    assert_eq!(setting_refusals(&a), Vec::new());
    // Notes are a map; any other value is refused as a value outside its type.
    let r = one("model", "notes", json!("free text"), "setting_invalid");
    assert!(r.to_string().contains("a map"), "{r}");
    // A number in the notes is no place of the search, even under `optimize: all`, and a `vary`
    // key in them marks nothing.
    let mut a = yaml(ARCH);
    a["parts"]["model"]["notes"] = notes;
    let places = agenticspec_sim::marked::collect(&a, true).unwrap();
    assert!(
        places.iter().all(|m| !m.path.contains(".notes")),
        "{:?}",
        places.iter().map(|m| &m.path).collect::<Vec<_>>()
    );
    let resolved = agenticspec_sim::marked::resolve(&a, &Default::default()).unwrap();
    assert_eq!(resolved["parts"]["model"]["notes"]["vary"], json!([1, 2]));
    // The simulator reads no note.
    assert!(
        agenticspec_sim::sim::PART_READS
            .iter()
            .all(|(_, keys)| !keys.contains(&"notes"))
    );
    // Every part takes them, so every block page lists them.
    let s = settings::table().setting("question", "notes").unwrap();
    assert_eq!(s.ty.name(), "notes");
    assert_eq!(s.narrows, Narrows::None);
}

/// The typed settings of story R97, each accepted on a part of its block.
#[test]
fn the_r97_settings_are_accepted_on_their_blocks() {
    let cases: &[(&str, Value, &str, Value)] = &[
        ("chat", json!({"block": "channel"}), "kind", json!("screen")),
        ("builds", json!({"block": "pool"}), "kind", json!("queue")),
        ("builds", json!({"block": "pool"}), "order", json!("fifo")),
        (
            "builds",
            json!({"block": "pool"}),
            "order",
            json!("priority"),
        ),
        (
            "model",
            json!({"block": "model"}),
            "hosting",
            json!("vendor"),
        ),
        (
            "model",
            json!({"block": "model"}),
            "hosting",
            json!("local"),
        ),
        (
            "model",
            json!({"block": "model"}),
            "billing",
            json!("subscription"),
        ),
        (
            "model",
            json!({"block": "model"}),
            "billing",
            json!("metered"),
        ),
        ("model", json!({"block": "model"}), "billing", json!("none")),
        (
            "guard",
            json!({"block": "authority"}),
            "per_day_usd",
            json!(20),
        ),
        (
            "guard",
            json!({"block": "authority"}),
            "per_month_usd",
            json!(400.5),
        ),
        (
            "guard",
            json!({"block": "authority"}),
            "route",
            json!("stop"),
        ),
        ("host", json!({"block": "runtime"}), "per_day_usd", json!(0)),
        (
            "host",
            json!({"block": "runtime"}),
            "per_month_usd",
            json!(100),
        ),
        ("host", json!({"block": "runtime"}), "route", json!("ask")),
        (
            "host",
            json!({"block": "runtime"}),
            "route",
            json!("notify"),
        ),
        (
            "ask",
            json!({"block": "question"}),
            "default",
            json!("approve"),
        ),
        (
            "ask",
            json!({"block": "question"}),
            "default_after",
            json!("15m"),
        ),
        (
            "ask",
            json!({"block": "question"}),
            "default_after",
            json!(900),
        ),
    ];
    for (name, part, setting, v) in cases {
        let mut p = part.clone();
        p[*setting] = v.clone();
        let mut a = yaml(ARCH);
        let base = &a["parts"][*name];
        if base["block"] == p["block"] {
            a["parts"][*name][*setting] = v.clone();
        } else {
            a["parts"][*name] = p;
        }
        assert_eq!(setting_refusals(&a), Vec::new(), "{name}.{setting} = {v}");
    }
}

/// A wrong value of each new typed setting is refused with what was expected.
#[test]
fn the_r97_settings_refuse_a_value_outside_their_type() {
    let cases: &[(&str, Value, &str, Value, &str)] = &[
        (
            "chat",
            json!({"block": "channel"}),
            "kind",
            json!("page"),
            "screen",
        ),
        (
            "builds",
            json!({"block": "pool"}),
            "kind",
            json!("line"),
            "queue",
        ),
        (
            "builds",
            json!({"block": "pool"}),
            "order",
            json!("random"),
            "one of fifo, lifo, priority",
        ),
        (
            "model",
            json!({"block": "model"}),
            "hosting",
            json!("self"),
            "one of vendor, managed, local",
        ),
        (
            "model",
            json!({"block": "model"}),
            "billing",
            json!("free"),
            "one of subscription, metered, none",
        ),
        (
            "guard",
            json!({"block": "authority"}),
            "per_day_usd",
            json!(-1),
            "a number of at least 0 (USD)",
        ),
        (
            "host",
            json!({"block": "runtime"}),
            "per_month_usd",
            json!("lots"),
            "a number of at least 0 (USD)",
        ),
        (
            "guard",
            json!({"block": "authority"}),
            "route",
            json!("explode"),
            "one of stop, ask, notify",
        ),
        (
            "host",
            json!({"block": "runtime"}),
            "route",
            json!(3),
            "one of stop, ask, notify",
        ),
        (
            "ask",
            json!({"block": "question"}),
            "default",
            json!(1),
            "a text",
        ),
        (
            "ask",
            json!({"block": "question"}),
            "default_after",
            json!("later"),
            "a duration of at least 0s",
        ),
    ];
    for (name, part, setting, v, expected) in cases {
        let mut a = yaml(ARCH);
        if a["parts"][*name]["block"] == part["block"] {
            a["parts"][*name][*setting] = v.clone();
        } else {
            let mut p = part.clone();
            p[*setting] = v.clone();
            a["parts"][*name] = p;
        }
        assert_eq!(
            setting_refusals(&a),
            vec![(
                "setting_invalid".to_string(),
                name.to_string(),
                setting.to_string()
            )],
            "{name}.{setting} = {v}"
        );
        let line = check(&a)
            .iter()
            .find(|r| r.rule() == "setting_invalid")
            .unwrap()
            .to_string();
        assert!(line.contains(expected), "{name}.{setting} = {v}: {line}");
    }
    // A spend cap is no setting of a model part, nor a pool order one of a channel.
    let mut a = yaml(ARCH);
    a["parts"]["model"]["per_day_usd"] = json!(5);
    a["parts"]["chat"]["order"] = json!("fifo");
    let mut got = setting_refusals(&a);
    got.sort();
    assert_eq!(
        got,
        vec![
            ("setting_unknown".into(), "chat".into(), "order".into()),
            (
                "setting_unknown".into(),
                "model".into(),
                "per_day_usd".into()
            ),
        ]
    );
}
