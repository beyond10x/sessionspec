//! Sessions, tool calls, the event channel of a draft and its revisions (story studio-live), on
//! the routes in process. The channel is read from its receiver; no socket is opened.

use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use agenticstudio::live::{self, Event};
use agenticstudio::routes::{events, lock};

use super::*;

const SET_3: &str = r#"set(P, count, 3) :- part{id: P, block: "agent", count: 12}."#;
const SET_5: &str = r#"set(P, count, 5) :- part{id: P, block: "agent", count: 3}."#;

fn start(s: &Mutex<Studio>, draft: &str) -> String {
    let (status, v) = call(s, "POST", "/api/sessions", Some(json!({"draft_id": draft})));
    assert_eq!(status, 201, "{v}");
    assert_eq!(v["session"]["state"], "Open");
    v["session"]["session_id"].as_str().unwrap().to_string()
}

fn tool(s: &Mutex<Studio>, session: &str, input: Value) -> (u16, Value) {
    tool_as(s, None, session, input)
}

/// A tool call sent as `caller` (the `X-Studio-Caller` header), or with no such header.
fn tool_as(s: &Mutex<Studio>, caller: Option<&str>, session: &str, input: Value) -> (u16, Value) {
    call_as(
        s,
        caller,
        "POST",
        &format!("/api/sessions/{session}/calls"),
        Some(input),
    )
}

#[test]
fn a_call_is_recorded_as_the_caller_its_request_is_sent_as() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    let check = json!({"tool": "check"});
    for (caller, status) in [
        (Some("Agent"), 200),
        (None, 200),
        (Some("user"), 200),
        (Some("Server"), 400),
    ] {
        let (got, v) = tool_as(&s, caller, &session, check.clone());
        assert_eq!(got, status, "{caller:?}: {v}");
    }
    let list = calls(&s, &session);
    let callers: Vec<&Value> = list.iter().map(|c| &c["caller"]).collect();
    assert_eq!(callers, [&json!("Agent"), &json!("User"), &json!("User")]);
}

#[test]
fn a_body_that_names_another_caller_is_refused_and_makes_no_call() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    for (caller, named) in [(None, "Agent"), (Some("Agent"), "User")] {
        let (status, v) = tool_as(
            &s,
            caller,
            &session,
            json!({"tool": "check", "caller": named}),
        );
        assert_eq!(
            (status, v["error"].as_str()),
            (403, Some("CallerNotSender")),
            "{caller:?} naming {named}: {v}"
        );
    }
    let (status, v) = call(
        &s,
        "POST",
        "/api/commands/studio.session.CallTool",
        Some(json!({"session_id": session, "tool": "Check", "caller": "Agent"})),
    );
    assert_eq!(
        (status, v["error"].as_str()),
        (403, Some("CallerNotSender")),
        "{v}"
    );
    assert!(calls(&s, &session).is_empty());

    let (status, v) = tool_as(
        &s,
        Some("Agent"),
        &session,
        json!({"tool": "check", "caller": "Agent"}),
    );
    assert_eq!(status, 200, "{v}");
    assert_eq!(calls(&s, &session)[0]["caller"], "Agent");
}

fn calls(s: &Mutex<Studio>, session: &str) -> Vec<Value> {
    let (status, v) = view(
        s,
        "studio.session.ToolCalls",
        &format!("session_id={session}"),
    );
    assert_eq!(status, 200, "{v}");
    rows(&v).clone()
}

fn subscribe(s: &Mutex<Studio>, draft: &str) -> Receiver<Event> {
    lock(s).subscribe(draft).unwrap()
}

/// The next event named `name` within `within`, skipping others.
fn next(rx: &Receiver<Event>, name: &str, within: Duration) -> Event {
    let until = Instant::now() + within;
    loop {
        let left = until.saturating_duration_since(Instant::now());
        match rx.recv_timeout(left) {
            Ok(e) if e.name == name => return e,
            Ok(_) => continue,
            Err(e) => panic!("no {name} within {within:?}: {e}"),
        }
    }
}

/// The story's first acceptance: two clients on the channel of one draft at revision 1; the
/// first applies a patch through a tool call; the second receives revision 2 within one second.
#[test]
fn a_tool_call_apply_reaches_a_second_client_as_revision_2_within_a_second() {
    let s = studio();
    let id = open_workstation(&s);
    let first = subscribe(&s, &id);
    let second = subscribe(&s, &id);
    assert_eq!(lock(&s).subscribers(&id), 2);
    let session = start(&s, &id);

    let (status, v) = tool(
        &s,
        &session,
        json!({"tool": "apply", "caller": "User", "arguments": {"patch": SET_3}}),
    );
    assert_eq!(status, 200, "{v}");
    assert_eq!(v["call"]["state"], "Answered", "{v}");
    assert_eq!(v["call"]["answer"]["outcome"], "Applied");

    let e = next(&second, live::DRAFT_REVISED, Duration::from_secs(1));
    assert_eq!(e.data["number"], 2, "{:?}", e.data);
    assert_eq!(e.data["draft_id"], id.as_str());
    assert_eq!(e.data["cause"], "Patch");
    assert_eq!(e.data["application"], v["call"]["application"]);
    let answered = next(&second, live::CALL_ANSWERED, Duration::from_secs(1));
    assert_eq!(answered.data["call_id"], v["call"]["call_id"]);
    assert_eq!(answered.data["tool"], "Apply");
    assert_eq!(
        next(&first, live::DRAFT_REVISED, Duration::from_secs(1)).data["number"],
        2
    );
}

/// The story's second acceptance: revisions 1, 2 and 3 with three digests; revision 1 reads as
/// opened, with `worker` at count 12.
#[test]
fn the_revisions_of_a_draft_are_listed_and_read() {
    let s = studio();
    let id = open_workstation(&s);
    assert_eq!(apply(&s, &id, SET_3, false).0, 200);
    assert_eq!(apply(&s, &id, SET_5, false).0, 200);

    let (status, v) = call(&s, "GET", &format!("/api/drafts/{id}/revisions"), None);
    assert_eq!(status, 200, "{v}");
    let list = v["revisions"].as_array().unwrap();
    let numbers: Vec<&Value> = list.iter().map(|r| &r["number"]).collect();
    assert_eq!(numbers, [&json!(1), &json!(2), &json!(3)]);
    let mut digests: Vec<&str> = list.iter().map(|r| r["digest"].as_str().unwrap()).collect();
    digests.dedup();
    assert_eq!(digests.len(), 3, "{v}");
    assert_eq!(list[0]["cause"], "Opened");
    assert_eq!(list[0]["application"], Value::Null);
    assert_eq!(list[1]["cause"], "Patch");
    let applications = &read(&s, &id)["applications"];
    assert_eq!(list[1]["application"], applications[0]["application_id"]);

    let (status, r1) = call(&s, "GET", &format!("/api/drafts/{id}/revisions/1"), None);
    assert_eq!(status, 200, "{r1}");
    assert_eq!(r1["document"]["parts"]["worker"]["count"]["value"], 12);
    assert_eq!(r1["revision"]["digest"], list[0]["digest"]);
    assert_eq!(r1["check"]["passed"], true);
    assert!(
        r1["graph"]["svg"]
            .as_str()
            .is_some_and(|g| g.contains("<svg")),
        "{}",
        r1["graph"]
    );
    let (_, r3) = call(&s, "GET", &format!("/api/drafts/{id}/revisions/3"), None);
    assert_eq!(r3["document"]["parts"]["worker"]["count"]["value"], 5);

    for bad in ["0", "4", "x"] {
        let (status, v) = call(
            &s,
            "GET",
            &format!("/api/drafts/{id}/revisions/{bad}"),
            None,
        );
        assert_eq!(
            (status, v["error"].as_str()),
            (404, Some("NoSuchRevision")),
            "{bad}: {v}"
        );
    }
    let (status, _) = call(&s, "GET", "/api/drafts/nope/revisions", None);
    assert_eq!(status, 404);
}

#[test]
fn a_session_records_each_call_and_refuses_calls_once_closed() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    assert!(calls(&s, &session).is_empty());

    let (status, v) = tool_as(
        &s,
        Some("Agent"),
        &session,
        json!({"tool": "query", "arguments": {"selector": "part{id: P, block: \"agent\"}"}}),
    );
    assert_eq!(status, 200, "{v}");
    let rows_of = |v: &Value| -> Vec<String> {
        v["call"]["answer"]["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["P"].as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(rows_of(&v), ["operator", "worker"]);
    let list = calls(&s, &session);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["tool"], "Query");
    assert_eq!(list[0]["caller"], "Agent");
    assert_eq!(list[0]["state"], "Answered");
    let answer: Value = serde_json::from_str(list[0]["answer"].as_str().unwrap()).unwrap();
    assert_eq!(answer["rows"].as_array().unwrap().len(), 2);

    let (status, v) = tool(&s, &session, json!({"tool": "check", "dry_run": true}));
    assert_eq!(
        (status, v["error"].as_str()),
        (422, Some("DryRunOutsideApply")),
        "{v}"
    );
    assert_eq!(
        calls(&s, &session).len(),
        1,
        "a refused call is not recorded"
    );

    let (status, v) = call(&s, "POST", &format!("/api/sessions/{session}/close"), None);
    assert_eq!(status, 200, "{v}");
    assert_eq!(v["session"]["state"], "Closed");
    let (status, v) = tool(&s, &session, json!({"tool": "check"}));
    assert_eq!(
        (status, v["error"].as_str()),
        (409, Some("ClosedSession")),
        "{v}"
    );
    let (status, _) = call(&s, "POST", &format!("/api/sessions/{session}/close"), None);
    assert_eq!(status, 409);

    let (status, v) = tool(&s, "nope", json!({"tool": "check"}));
    assert_eq!(
        (status, v["error"].as_str()),
        (404, Some("NoSuchSession")),
        "{v}"
    );
    let (status, _) = call(
        &s,
        "POST",
        "/api/sessions",
        Some(json!({"draft_id": "nope"})),
    );
    assert_eq!(status, 404);
    let (status, v) = tool(&s, &session, json!({"tool": "frobnicate"}));
    assert_eq!(status, 400, "{v}");

    let (status, v) = view(&s, "studio.session.Sessions", "");
    assert_eq!(status, 200);
    assert_eq!(rows(&v).len(), 1);
}

/// `studio.session.CallTool` answers `dry-run-outside-apply` before it reads the session: a dry
/// run of any tool but apply is refused as such on an unknown and on a closed session too.
#[test]
fn a_dry_run_outside_apply_is_refused_before_the_session_is_read() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    let (status, _) = call(&s, "POST", &format!("/api/sessions/{session}/close"), None);
    assert_eq!(status, 200);
    for on in ["nope", session.as_str()] {
        let (status, v) = tool(&s, on, json!({"tool": "check", "dry_run": true}));
        assert_eq!(
            (status, v["error"].as_str()),
            (422, Some("DryRunOutsideApply")),
            "on session {on}: {v}"
        );
    }
}

#[test]
fn a_call_that_cannot_be_carried_out_fails_and_is_published() {
    let s = studio();
    let id = open_workstation(&s);
    let rx = subscribe(&s, &id);
    let session = start(&s, &id);
    let (status, v) = tool(
        &s,
        &session,
        json!({"tool": "query", "arguments": {"selector": "nopart{id: P}"}}),
    );
    assert_eq!(status, 200, "{v}");
    assert_eq!(v["call"]["state"], "Failed");
    assert!(
        v["call"]["failure"]
            .as_str()
            .unwrap()
            .starts_with("InvalidSelector"),
        "{v}"
    );
    assert_eq!(v["call"]["caller"], "User");
    let e = next(&rx, live::CALL_FAILED, Duration::from_secs(1));
    assert_eq!(e.data["call_id"], v["call"]["call_id"]);

    let (_, v) = tool(&s, &session, json!({"tool": "apply", "arguments": {}}));
    assert_eq!(v["call"]["state"], "Failed", "{v}");
}

#[test]
fn check_and_a_refused_apply_are_answered_and_explained() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    let (_, v) = tool(&s, &session, json!({"tool": "check"}));
    assert_eq!(v["call"]["answer"]["check"]["passed"], true, "{v}");
    assert_eq!(v["call"]["answer"]["summary"], "The check passes.");

    let unknown =
        "set(P, count, 2) :- part{id: P, block: \"agent\"}.\nfrobnicate(P) :- part{id: P}.";
    let (_, v) = tool(
        &s,
        &session,
        json!({"tool": "apply", "arguments": {"patch": unknown}}),
    );
    let a = &v["call"]["answer"];
    assert_eq!(v["call"]["state"], "Answered");
    assert_eq!(a["outcome"], "Refused");
    assert_eq!(a["refusal"]["rule"], 2);
    assert_eq!(a["refusal"]["error"], "UnknownVerb");
    assert!(
        a["summary"]
            .as_str()
            .unwrap()
            .starts_with("Refused at rule 2: UnknownVerb"),
        "{a}"
    );

    let broken = r#"set(P, count, 0) :- part{id: P, block: "agent", count: 12}."#;
    let (_, v) = tool(
        &s,
        &session,
        json!({"tool": "apply", "arguments": {"patch": broken}}),
    );
    assert_eq!(v["call"]["answer"]["outcome"], "Refused", "{v}");
    assert_eq!(read(&s, &id)["revision"]["number"], 1);
}

#[test]
fn simulate_answers_each_kpi_with_what_it_measures() {
    let s = studio();
    let id = open_workstation(&s);
    let session = start(&s, &id);
    let started = Instant::now();
    let (_, v) = tool(
        &s,
        &session,
        json!({"tool": "simulate", "arguments": {"budget": 1, "horizon": "2h"}}),
    );
    let took = started.elapsed();
    let a = &v["call"]["answer"];
    assert_eq!(v["call"]["state"], "Answered", "{v}");
    assert_eq!(a["budget"], 1);
    let kpis = a["kpis"].as_array().unwrap();
    let wait = kpis
        .iter()
        .find(|k| k["id"] == "owner_wait_share")
        .expect("owner_wait_share");
    assert_eq!(wait["direction"], "lower");
    assert!(
        wait["summary"].as_str().is_some_and(|s| !s.is_empty()),
        "{wait}"
    );
    assert!(
        a["summary"]
            .as_str()
            .unwrap()
            .contains("owner_wait_share = "),
        "{a}"
    );
    assert!(
        a["per_part"].as_object().is_some_and(|p| !p.is_empty()),
        "{a}"
    );
    assert!(took < Duration::from_secs(20), "simulate took {took:?}");

    let (_, v) = tool(
        &s,
        &session,
        json!({"tool": "simulate", "arguments": {"budget": 99}}),
    );
    assert_eq!(v["call"]["state"], "Failed");
    assert!(
        v["call"]["failure"]
            .as_str()
            .unwrap()
            .starts_with("InvalidBudget"),
        "{v}"
    );
    let (_, v) = tool(
        &s,
        &session,
        json!({"tool": "simulate", "arguments": {"profile": "../fields/coding/profiles/coding.yaml"}}),
    );
    assert!(
        v["call"]["failure"]
            .as_str()
            .unwrap()
            .starts_with("NoSuchProfile"),
        "{v}"
    );
}

/// The page's patch form and the script route are calls of the draft's editor session.
#[test]
fn the_page_and_the_patch_route_call_in_the_drafts_editor_session() {
    let s = studio();
    let id = open_workstation(&s);
    let (status, _) = command(
        &s,
        "studio.session.CallTool",
        json!({"draft_id": id, "arguments": SET_3, "dry_run": true}),
    );
    assert_eq!(status, 200);
    assert_eq!(apply(&s, &id, SET_3, false).0, 200);
    let (_, v) = view(&s, "studio.session.Sessions", "");
    let sessions = rows(&v);
    assert_eq!(sessions.len(), 1, "{v}");
    assert_eq!(sessions[0]["draft_id"], id.as_str());
    let list = calls(&s, sessions[0]["session_id"].as_str().unwrap());
    assert_eq!(list.len(), 2);
    assert!(
        list.iter()
            .all(|c| c["tool"] == "Apply" && c["caller"] == "User")
    );
    assert_eq!(
        (&list[0]["dry_run"], &list[1]["dry_run"]),
        (&json!(true), &json!(false))
    );
    assert!(list[1]["application"].is_string(), "{}", list[1]);

    let session = sessions[0]["session_id"].as_str().unwrap().to_string();
    let (status, v) = command(
        &s,
        "studio.session.CallTool",
        json!({"session_id": session, "tool": "Check", "caller": "User", "dry_run": false}),
    );
    assert_eq!(status, 200, "{v}");
    assert_eq!(v["row"]["state"], "Answered");
    assert_eq!(v["row"]["answer"]["check"]["passed"], true);
}

/// A scratch root with `architectures/local.yaml`, a copy of the workstation document.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("studio-live-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("architectures")).unwrap();
    std::fs::copy(
        root().join("architectures/workstation.yaml"),
        dir.join("architectures/local.yaml"),
    )
    .unwrap();
    dir
}

/// The owner edits the file a draft was opened from: the change becomes a new revision that
/// every client on the draft receives.
#[test]
fn a_source_changed_on_disk_becomes_a_revision_every_client_sees() {
    let dir = scratch("reload");
    let file = dir.join("architectures/local.yaml");
    let s = Mutex::new(Studio::new(&dir));
    let (status, v) = call(
        &s,
        "POST",
        "/api/drafts",
        Some(json!({"source": "architectures/local.yaml"})),
    );
    assert_eq!(status, 201, "{v}");
    let id = v["draft"]["draft_id"].as_str().unwrap().to_string();
    let rx = subscribe(&s, &id);
    assert!(
        lock(&s).reload_sources().is_empty(),
        "an unchanged file is no revision"
    );

    let text = std::fs::read_to_string(&file).unwrap();
    let edited = text.replacen("count: { value: 12,", "count: { value: 7,", 1);
    assert_ne!(text, edited);
    std::fs::write(&file, &edited).unwrap();
    assert_eq!(lock(&s).reload_sources(), [(id.clone(), 2)]);
    let e = next(&rx, live::DRAFT_REVISED, Duration::from_secs(1));
    assert_eq!(
        (&e.data["number"], &e.data["cause"]),
        (&json!(2), &json!("SourceChanged"))
    );
    assert_eq!(part(&read(&s, &id), "worker")["count"], 7);
    let (_, v) = call(&s, "GET", &format!("/api/drafts/{id}/revisions"), None);
    assert_eq!(v["revisions"][1]["cause"], "SourceChanged");

    std::fs::write(&file, format!("{edited}\n# a comment\n")).unwrap();
    assert!(
        lock(&s).reload_sources().is_empty(),
        "the same architecture is no revision"
    );
    std::fs::write(&file, "format: something-else\n").unwrap();
    assert!(
        lock(&s).reload_sources().is_empty(),
        "no architecture is no revision"
    );
    std::fs::remove_file(&file).unwrap();
    assert!(lock(&s).reload_sources().is_empty());
    std::fs::write(&file, &text).unwrap();
    assert_eq!(lock(&s).reload_sources(), [(id.clone(), 3)]);
    assert_eq!(part(&read(&s, &id), "worker")["count"], 12);
}

#[test]
fn the_channel_is_asked_for_by_draft_and_streamed_as_server_sent_events() {
    let s = studio();
    let id = open_workstation(&s);
    let get = |path: &str, query: &str| Request {
        method: "GET".into(),
        path: path.into(),
        query: query.into(),
        body: Vec::new(),
        caller: None,
    };
    assert!(events(&s, &get(&format!("/api/drafts/{id}/events"), "")).is_some_and(|r| r.is_ok()));
    let by_channel = get("/api/channels/draft_events", &format!("draft_id={id}"));
    let Some(Ok((rx, hello))) = events(&s, &by_channel) else {
        panic!("no subscription")
    };
    assert_eq!(hello, format!("draft {id} at revision 1"));
    match events(&s, &get("/api/drafts/nope/events", "")) {
        Some(Err(r)) => assert_eq!(r.status, 404),
        _ => panic!("an unknown draft has no channel"),
    }
    assert!(events(&s, &get(&format!("/api/drafts/{id}"), "")).is_none());
    let (status, v) = call(&s, "GET", &format!("/api/drafts/{id}/events"), None);
    assert_eq!((status, v["error"].as_str()), (400, Some("StreamOnly")));

    assert_eq!(apply(&s, &id, SET_3, false).0, 200);
    // The stream ends when the studio drops the channel's senders: here, with the studio.
    drop(s);
    let mut out = Vec::new();
    live::stream(&mut out, &hello, &rx, Duration::from_secs(5)).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.starts_with(live::HEAD), "{text}");
    assert!(
        text.contains(&format!(": draft {id} at revision 1\n\n")),
        "{text}"
    );
    assert!(
        text.contains("event: studio.draft.DraftRevised\ndata: {"),
        "{text}"
    );
    assert!(
        text.contains("event: studio.session.ToolCallAnswered\ndata: {"),
        "{text}"
    );
    let revised = text.find("DraftRevised").unwrap();
    let answered = text.find("ToolCallAnswered").unwrap();
    assert!(
        revised < answered,
        "the revision comes before the answer: {text}"
    );
}
