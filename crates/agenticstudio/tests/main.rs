//! Tests of agenticstudio. One binary (workspace rule). The routes are answered in process, with
//! no socket; the architectures are the documents under `architectures/`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use agenticstudio::http::Request;
use agenticstudio::routes::answer;
use agenticstudio::studio::Studio;
use serde_json::{Value, json};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn studio() -> Mutex<Studio> {
    Mutex::new(Studio::new(root()))
}

fn call_raw(
    s: &Mutex<Studio>,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (u16, String, String) {
    call_raw_as(s, None, method, path, body)
}

/// A request sent as `caller` (its `X-Studio-Caller` header), or with no such header.
fn call_raw_as(
    s: &Mutex<Studio>,
    caller: Option<&str>,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (u16, String, String) {
    let request = Request {
        method: method.into(),
        path: path.into(),
        query: String::new(),
        body: body.map(|b| b.to_string().into_bytes()).unwrap_or_default(),
        caller: caller.map(str::to_string),
    };
    let r = answer(s, &request);
    (
        r.status,
        r.content_type.to_string(),
        String::from_utf8(r.body).unwrap(),
    )
}

fn call(s: &Mutex<Studio>, method: &str, path: &str, body: Option<Value>) -> (u16, Value) {
    call_as(s, None, method, path, body)
}

fn call_as(
    s: &Mutex<Studio>,
    caller: Option<&str>,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (u16, Value) {
    let (status, ctype, body) = call_raw_as(s, caller, method, path, body);
    assert!(ctype.starts_with("application/json"), "{ctype}: {body}");
    (status, serde_json::from_str(&body).unwrap())
}

/// Opens a draft of `architectures/workstation.yaml` and returns its id.
fn open_workstation(s: &Mutex<Studio>) -> String {
    let (status, v) = call(
        s,
        "POST",
        "/api/drafts",
        Some(json!({"source": "architectures/workstation.yaml"})),
    );
    assert_eq!(status, 201, "{v}");
    v["draft"]["draft_id"].as_str().unwrap().to_string()
}

fn read(s: &Mutex<Studio>, id: &str) -> Value {
    let (status, v) = call(s, "GET", &format!("/api/drafts/{id}"), None);
    assert_eq!(status, 200, "{v}");
    v
}

fn part<'a>(draft: &'a Value, id: &str) -> &'a Value {
    draft["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == id)
        .unwrap_or_else(|| panic!("no part {id} in {draft}"))
}

fn apply(s: &Mutex<Studio>, id: &str, text: &str, dry_run: bool) -> (u16, Value) {
    call(
        s,
        "POST",
        &format!("/api/drafts/{id}/patches"),
        Some(json!({"text": text, "dry_run": dry_run})),
    )
}

mod live;

mod acceptance {
    use super::*;

    /// Both bullets of the story's acceptance, in order, on the routes.
    #[test]
    fn a_set_patch_makes_revision_2_and_an_unknown_verb_changes_nothing() {
        let s = studio();
        let id = open_workstation(&s);
        let before = read(&s, &id);
        assert_eq!(before["revision"]["number"], 1);
        assert_eq!(part(&before, "worker")["count"], 12);

        let (status, v) = apply(
            &s,
            &id,
            r#"set(P, count, 3) :- part{id: P, block: "agent", count: 12}."#,
            false,
        );
        assert_eq!(status, 200, "{v}");
        let after = read(&s, &id);
        assert_eq!(after["revision"]["number"], 2);
        assert_eq!(part(&after, "worker")["count"], 3);
        assert_eq!(part(&after, "operator"), part(&before, "operator"));
        assert_eq!(after["check"]["passed"], true);

        let digest = after["revision"]["digest"].clone();
        let (status, v) = apply(
            &s,
            &id,
            "set(P, count, 2) :- part{id: P, block: \"agent\"}.\nfrobnicate(P) :- part{id: P}.",
            false,
        );
        assert_eq!(status, 422, "{v}");
        assert_eq!(v["refusal"]["rule"], 2);
        assert_eq!(v["refusal"]["error"], "UnknownVerb");
        let still = read(&s, &id);
        assert_eq!(still["revision"]["number"], 2);
        assert_eq!(still["revision"]["digest"], digest);
        assert_eq!(part(&still, "worker")["count"], 3);
    }
}

mod routes {
    use super::*;
    use agenticstudio::types::architecture::{PatchApplicationsRow, RevisionsRow};
    use agenticstudio::types::studio::DraftsRow;

    #[test]
    fn the_architectures_are_listed_by_their_source() {
        let (status, v) = call(&studio(), "GET", "/api/architectures", None);
        assert_eq!(status, 200);
        let sources: Vec<&str> = v["architectures"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["source"].as_str().unwrap())
            .collect();
        assert!(
            sources.contains(&"architectures/workstation.yaml"),
            "{sources:?}"
        );
        assert!(
            sources.windows(2).all(|w| w[0] < w[1]),
            "sorted: {sources:?}"
        );
    }

    #[test]
    fn a_draft_opens_at_revision_1_with_its_digest_check_and_graph() {
        let s = studio();
        let id = open_workstation(&s);
        let v = read(&s, &id);
        let draft: DraftsRow = serde_json::from_value(v["draft"].clone()).unwrap();
        assert_eq!(draft.name.0, "workstation");
        let revision: RevisionsRow = serde_json::from_value(v["revision"].clone()).unwrap();
        assert_eq!(revision.digest.0, agenticspec_patch::digest(&v["document"]));
        assert_eq!(revision.digest.0.len(), 64);
        assert_eq!(v["check"], json!({"passed": true, "refusals": []}));
        assert_eq!(v["parts"].as_array().unwrap().len(), 9);
        let graph = v["graph"].as_str().unwrap();
        let (status, ctype, html) = call_raw(&s, "GET", graph, None);
        assert_eq!(status, 200);
        assert!(ctype.starts_with("text/html"));
        assert!(html.contains("worker") && html.contains("<svg"));
    }

    #[test]
    fn a_draft_name_is_unique_among_open_drafts() {
        let s = studio();
        let a = read(&s, &open_workstation(&s));
        let b = read(&s, &open_workstation(&s));
        assert_ne!(a["draft"]["name"], b["draft"]["name"]);
        let (status, v) = call(
            &s,
            "POST",
            "/api/drafts",
            Some(json!({"source": "architectures/workstation.yaml", "name": "workstation"})),
        );
        assert_eq!(status, 409);
        assert_eq!(v["error"], "DraftNameTaken");
        let (status, v) = call(&s, "GET", "/api/drafts", None);
        assert_eq!(status, 200);
        assert_eq!(v["drafts"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn a_source_outside_the_architectures_is_refused() {
        let s = studio();
        for source in [
            "architectures/none.yaml",
            "../Cargo.toml",
            "Cargo.toml",
            "architectures/../Cargo.toml",
        ] {
            let (status, v) = call(&s, "POST", "/api/drafts", Some(json!({"source": source})));
            assert_eq!(status, 404, "{source}: {v}");
            assert_eq!(v["error"], "NoSuchSource");
        }
        let (status, v) = call(&s, "POST", "/api/drafts", Some(json!({"name": 3})));
        assert_eq!(status, 400, "{v}");
    }

    #[test]
    fn an_unknown_draft_or_route_is_refused() {
        let s = studio();
        let (status, v) = call(&s, "GET", "/api/drafts/nope", None);
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchDraft")));
        let (status, v) = apply(&s, "nope", "add(part{id: \"x\", block: \"model\"}).", false);
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchDraft")));
        let (status, _, _) = call_raw(&s, "GET", "/api/nothing", None);
        assert_eq!(status, 404);
        let (status, _, _) = call_raw(&s, "DELETE", "/api/drafts", None);
        assert_eq!(status, 405);
    }

    #[test]
    fn a_dry_run_answers_the_diff_and_check_and_makes_no_revision() {
        let s = studio();
        let id = open_workstation(&s);
        let (status, v) = apply(&s, &id, "set(P, count, 4) :- part{id: P, count: 12}.", true);
        assert_eq!(status, 200, "{v}");
        assert_eq!(v["diff"][0]["path"], "parts.worker.count.value");
        assert_eq!(v["check"]["passed"], true);
        let row: PatchApplicationsRow = serde_json::from_value(v["application"].clone()).unwrap();
        assert!(row.dry_run);
        assert!(row.revision_id.is_absent());
        assert_eq!(read(&s, &id)["revision"]["number"], 1);
    }

    #[test]
    fn every_application_is_recorded_with_its_patch() {
        let s = studio();
        let id = open_workstation(&s);
        let (status, v) = apply(
            &s,
            &id,
            "add(part{id: \"lonely\", block: \"model\"}).",
            false,
        );
        assert_eq!(status, 422, "{v}");
        assert_eq!(v["refusal"]["error"], "CheckRefused");
        assert_eq!(v["refusal"]["rule"], Value::Null);
        assert_eq!(v["check"]["passed"], false);
        assert_eq!(v["check"]["refusals"][0]["rule"], "unreachable_parts");
        assert_eq!(v["application"]["state"], "Refused");
        assert_eq!(v["application"]["error"], "CheckRefused");
        assert!(v["application"].get("rule_index").is_none());
        assert_eq!(v["patch"]["rules"][0]["verb"], "Add");

        let (status, v) = apply(
            &s,
            &id,
            "set(P, count, 2) :- part{id: P, count: 12}.",
            false,
        );
        assert_eq!(status, 200, "{v}");
        assert_eq!(v["application"]["state"], "Applied");
        assert_eq!(v["application"]["changed_rows"], 1);
        assert_eq!(
            v["application"]["revision_id"],
            v["revision"]["revision_id"]
        );

        let draft = read(&s, &id);
        let states: Vec<&str> = draft["applications"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["state"].as_str().unwrap())
            .collect();
        assert_eq!(states, ["Refused", "Applied"]);
    }

    #[test]
    fn a_patch_whose_verb_is_no_patch_verb_is_refused_unrecorded() {
        let s = studio();
        let id = open_workstation(&s);
        let (status, v) = apply(&s, &id, "frobnicate(P) :- part{id: P}.", false);
        assert_eq!(status, 422);
        assert_eq!(v["refusal"]["rule"], 1);
        assert_eq!(v["application"], Value::Null);
        assert!(read(&s, &id)["applications"].as_array().unwrap().is_empty());
    }

    #[test]
    fn a_patch_document_applies_like_its_text() {
        let s = studio();
        let id = open_workstation(&s);
        let doc = json!({
            "format": "agenticspec-patch/1",
            "name": "fewer workers",
            "rules": ["set(P, count, 6) :- part{id: P, block: \"agent\", count: 12}."],
        });
        let (status, v) = call(&s, "POST", &format!("/api/drafts/{id}/patches"), Some(doc));
        assert_eq!(status, 200, "{v}");
        assert_eq!(v["patch"]["name"], "fewer workers");
        assert_eq!(part(&read(&s, &id), "worker")["count"], 6);
    }
}

/// A read of the generated app: `GET /api/views/<view>?<params>`, answered `{"rows": [...]}`.
fn view(s: &Mutex<Studio>, name: &str, query: &str) -> (u16, Value) {
    let request = Request {
        method: "GET".into(),
        path: format!("/api/views/{name}"),
        query: query.into(),
        body: Vec::new(),
        caller: None,
    };
    let r = answer(s, &request);
    (r.status, serde_json::from_slice(&r.body).unwrap())
}

fn rows(v: &Value) -> &Vec<Value> {
    v["rows"]
        .as_array()
        .unwrap_or_else(|| panic!("no rows in {v}"))
}

fn command(s: &Mutex<Studio>, name: &str, input: Value) -> (u16, Value) {
    call(s, "POST", &format!("/api/commands/{name}"), Some(input))
}

/// The views and commands `studio/studio.ui.yaml` names, as the generated client calls them.
mod views {
    use super::*;

    #[test]
    fn the_architectures_and_drafts_are_rows() {
        let s = studio();
        let (status, v) = view(&s, "studio.Architectures", "");
        assert_eq!(status, 200, "{v}");
        assert!(
            rows(&v).contains(
                &json!({"name": "workstation", "source": "architectures/workstation.yaml"})
            ),
            "{v}"
        );
        assert!(
            rows(&v).iter().any(|r| r["name"] == "ladder/09-factory"),
            "{v}"
        );
        let id = open_workstation(&s);
        let (status, v) = view(&s, "studio.draft.Drafts", "");
        assert_eq!(status, 200, "{v}");
        assert_eq!(rows(&v).len(), 1);
        assert_eq!(rows(&v)[0]["draft_id"], id.as_str());
        assert_eq!(rows(&v)[0]["source"], "architectures/workstation.yaml");
    }

    #[test]
    fn a_draft_answers_its_summary_parts_check_and_graph() {
        let s = studio();
        let id = open_workstation(&s);
        let q = format!("draft_id={id}");

        let (status, v) = view(&s, "studio.DraftSummary", &q);
        assert_eq!(status, 200, "{v}");
        let row = &rows(&v)[0];
        assert_eq!(row["draft_id"], id.as_str());
        assert_eq!(row["name"], "workstation");
        assert_eq!(row["revision"], 1);
        assert_eq!(row["digest"].as_str().unwrap().len(), 64);
        assert_eq!(row["passed"], true);
        assert!(
            row["description"]
                .as_str()
                .unwrap()
                .starts_with("One workstation")
        );

        let (_, v) = view(&s, "studio.DraftParts", &q);
        assert_eq!(rows(&v).len(), 9);
        assert!(rows(&v).contains(&json!({"id": "worker", "block": "agent", "count": 12})));

        let (_, v) = view(&s, "studio.DraftRefusals", &q);
        assert!(rows(&v).is_empty(), "{v}");

        let (status, v) = view(&s, "studio.DraftGraph", &q);
        assert_eq!(status, 200);
        let g = &rows(&v)[0];
        assert!(g["svg"].as_str().unwrap().starts_with("<svg"));
        assert!(g["width"].as_f64().unwrap() > 0.0 && g["height"].as_f64().unwrap() > 0.0);
        assert_eq!(g["parts"].as_array().unwrap().len(), 9);
        assert_eq!(g["draft_id"], id.as_str());
        assert_eq!(g["revision"], 1);
    }

    #[test]
    fn an_unknown_view_or_draft_is_refused() {
        let s = studio();
        let (status, v) = view(&s, "studio.Nothing", "");
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchView")));
        let (status, v) = view(&s, "studio.DraftSummary", "draft_id=nope");
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchDraft")));
        let (status, v) = view(&s, "studio.DraftParts", "");
        assert_eq!((status, v["error"].as_str()), (400, Some("InvalidRequest")));
    }

    #[test]
    fn open_draft_answers_the_draft_row() {
        let s = studio();
        let (status, v) = command(
            &s,
            "studio.draft.OpenDraft",
            json!({"source": "architectures/solo.yaml"}),
        );
        assert_eq!(status, 201, "{v}");
        assert_eq!(v["row"]["name"], "solo");
        assert_eq!(v["row"]["state"], "Open");
        let (status, v) = command(
            &s,
            "studio.draft.OpenDraft",
            json!({"source": "Cargo.toml"}),
        );
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchSource")));
    }

    #[test]
    fn call_tool_applies_dry_runs_and_refuses_a_patch_with_its_row() {
        let s = studio();
        let id = open_workstation(&s);
        let (status, v) = command(
            &s,
            "studio.session.CallTool",
            json!({"draft_id": id, "arguments": "set(P, count, 4) :- part{id: P, count: 12}.", "dry_run": true}),
        );
        assert_eq!(status, 200, "{v}");
        assert_eq!(v["row"]["outcome"], "DryRun");
        assert_eq!(
            v["row"]["diff"],
            json!(["parts.worker.count.value: 12 → 4"])
        );
        assert_eq!(read(&s, &id)["revision"]["number"], 1);

        let (status, v) = command(
            &s,
            "studio.session.CallTool",
            json!({"draft_id": id, "arguments": "set(P, count, 3) :- part{id: P, count: 12}.", "dry_run": false}),
        );
        assert_eq!(status, 200, "{v}");
        assert_eq!(v["row"]["outcome"], "Applied");
        assert_eq!(v["row"]["revision"], 2);
        assert_eq!(v["row"]["message"], "Applied as revision 2, 1 row changed.");

        let (status, v) = command(
            &s,
            "studio.session.CallTool",
            json!({"draft_id": id, "arguments": "set(P, count, 2) :- part{id: P}.\nfrobnicate(P) :- part{id: P}."}),
        );
        assert_eq!(status, 422, "{v}");
        assert_eq!(v["row"]["outcome"], "Refused");
        assert_eq!(v["row"]["rule"], 2);
        assert_eq!(v["row"]["error"], "UnknownVerb");
        assert!(
            v["row"]["message"]
                .as_str()
                .unwrap()
                .starts_with("Refused at rule 2: UnknownVerb"),
            "{v}"
        );
        assert_eq!(read(&s, &id)["revision"]["number"], 2);

        let (status, v) = command(
            &s,
            "studio.session.CallTool",
            json!({"draft_id": id, "arguments": "add(part{id: \"lonely\", block: \"model\"})."}),
        );
        assert_eq!(status, 422, "{v}");
        assert_eq!(v["row"]["error"], "CheckRefused");
        assert!(
            v["row"]["message"]
                .as_str()
                .unwrap()
                .contains("unreachable_parts"),
            "{v}"
        );
    }

    #[test]
    fn an_unknown_command_or_draft_is_refused() {
        let s = studio();
        let (status, v) = command(&s, "studio.Nothing", json!({}));
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchCommand")));
        let (status, v) = command(
            &s,
            "studio.session.CallTool",
            json!({"draft_id": "nope", "arguments": "frobnicate(P) :- part{id: P}."}),
        );
        assert_eq!((status, v["error"].as_str()), (404, Some("NoSuchDraft")));
        let (status, v) = command(&s, "studio.session.CallTool", json!({"draft_id": "x"}));
        assert_eq!((status, v["error"].as_str()), (400, Some("InvalidRequest")));
    }
}

/// The built app (`ui/studio`, written to the www directory) is served as static files.
mod static_files {
    use super::*;
    use agenticstudio::routes::serve;

    fn get(s: &Mutex<Studio>, www: &Path, path: &str) -> (u16, String, Vec<u8>) {
        let request = Request {
            method: "GET".into(),
            path: path.into(),
            query: String::new(),
            body: Vec::new(),
            caller: None,
        };
        let r = serve(s, www, &request);
        (r.status, r.content_type.to_string(), r.body)
    }

    fn www(name: &str) -> PathBuf {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("assets")).unwrap();
        std::fs::write(
            dir.join("index.html"),
            "<!doctype html><div id=\"app\"></div>",
        )
        .unwrap();
        std::fs::write(dir.join("assets/app-1.js"), "export {}").unwrap();
        std::fs::write(dir.join("assets/app-1.css"), "body{}").unwrap();
        dir
    }

    #[test]
    fn the_app_and_its_assets_are_served_with_their_type() {
        let s = studio();
        let www = www("www-served");
        let (status, ctype, body) = get(&s, &www, "/");
        assert_eq!(status, 200);
        assert!(ctype.starts_with("text/html"));
        assert!(String::from_utf8(body).unwrap().contains("id=\"app\""));
        let (status, ctype, _) = get(&s, &www, "/assets/app-1.js");
        assert_eq!(
            (status, ctype.as_str()),
            (200, "text/javascript; charset=utf-8")
        );
        let (status, ctype, _) = get(&s, &www, "/assets/app-1.css");
        assert_eq!((status, ctype.as_str()), (200, "text/css; charset=utf-8"));
        let (status, ctype, _) = get(&s, &www, "/api/architectures");
        assert_eq!((status, ctype.as_str()), (200, "application/json"));
    }

    #[test]
    fn no_path_leaves_the_www_directory() {
        let s = studio();
        let www = www("www-guarded");
        for path in [
            "/../Cargo.toml",
            "/assets/../../Cargo.toml",
            "/assets/%2e%2e/%2e%2e/Cargo.toml",
            "/assets/missing.js",
            "/assets",
        ] {
            let (status, _, _) = get(&s, &www, path);
            assert_eq!(status, 404, "{path}");
        }
    }

    #[test]
    fn without_a_built_app_the_root_says_how_to_build_it() {
        let s = studio();
        let www = Path::new(env!("CARGO_TARGET_TMPDIR")).join("www-absent");
        let (status, _, body) = get(&s, &www, "/");
        assert_eq!(status, 404);
        assert!(
            String::from_utf8(body)
                .unwrap()
                .contains("task studio:build")
        );
    }
}
