//! The routes of `agenticstudio serve`, answered from the studio without a socket.
//!
//! | method | path | answer |
//! |---|---|---|
//! | GET | `/api/views/<view>`, POST `/api/commands/<command>` | the routes of the generated app (`app`) |
//! | GET | any path outside `/api` | a file of the built app (`app::file`); `/` is its `index.html` |
//! | GET | `/api/architectures` | the architecture documents a draft opens |
//! | GET | `/api/drafts` | the drafts |
//! | POST | `/api/drafts` | open a draft: `{"source": "architectures/<name>.yaml", "name"?}` |
//! | GET | `/api/drafts/<id>` | the draft: its document, revision, parts, check and applications |
//! | GET | `/api/drafts/<id>/graph` | the graph page of its current revision |
//! | POST | `/api/drafts/<id>/patches` | apply a patch: `{"text", "dry_run"?}` or a patch document, a call of the draft's editor session |
//! | GET | `/api/drafts/<id>/revisions` | its revisions: number, digest, cause and the application that committed each |
//! | GET | `/api/drafts/<id>/revisions/<n>` | revision `n`: its document, graph and check |
//! | GET | `/api/drafts/<id>/events`, `/api/channels/draft_events?draft_id=<id>` | the draft's event channel (server-sent events; `events`) |
//! | GET | `/api/sessions` | the sessions |
//! | POST | `/api/sessions` | start a session on a draft: `{"draft_id"}` |
//! | POST | `/api/sessions/<id>/close` | close it |
//! | GET | `/api/sessions/<id>/calls` | its tool calls (the `ToolCalls` view) |
//! | POST | `/api/sessions/<id>/calls` | make a tool call: `{"tool", "arguments"?, "dry_run"?}`, by the caller the `X-Studio-Caller` header names (`User` when none) |

use std::path::Path;
use std::sync::mpsc::Receiver;
use std::sync::{Mutex, MutexGuard};

use agenticspec_records::sq::facts::Val;
use agenticspec_sim::check;
use serde_json::{Value, json};

use crate::http::{CALLER_HEADER, HTML, Request, Response};
use crate::live::Event;
use crate::studio::{self as st, Call, Draft, Revision, Studio, StudioError};
use crate::tools::check_json;
use crate::types::session::StudioSessionCaller as Caller;

/// The studio behind its lock; a lock a panicking thread left is taken as it is.
pub fn lock(studio: &Mutex<Studio>) -> MutexGuard<'_, Studio> {
    studio.lock().unwrap_or_else(|e| e.into_inner())
}

pub(crate) fn error(status: u16, name: &str, message: &str) -> Response {
    Response::json(status, &json!({"error": name, "message": message}))
}

pub(crate) fn studio_error(e: &StudioError) -> Response {
    let status = match e {
        StudioError::NoSuchDraft(_)
        | StudioError::NoSuchSource(_)
        | StudioError::NoSuchSession(_)
        | StudioError::NoSuchRevision(..) => 404,
        StudioError::DraftNameTaken(_) | StudioError::ClosedSession(_) => 409,
        StudioError::NotAnArchitecture(_) | StudioError::DryRunOutsideApply => 422,
    };
    error(status, e.name(), &e.message())
}

pub(crate) fn to_json<T: serde::Serialize>(v: &T) -> Value {
    serde_json::to_value(v).unwrap_or(Value::Null)
}

pub(crate) fn val(v: &Val) -> Value {
    match v {
        Val::Null => Value::Null,
        Val::Bool(b) => Value::from(*b),
        Val::Int(n) => Value::from(*n),
        Val::Str(s) => Value::from(s.as_ref()),
    }
}

/// A draft as the page reads it.
pub fn draft_json(d: &Draft) -> Value {
    let current = d.current();
    let facts = agenticspec_patch::facts(&current.document);
    let id = &d.row.draft_id.0;
    json!({
        "draft": to_json(&d.row),
        "revision": to_json(&current.row),
        "document": current.document,
        "parts": facts
            .parts
            .iter()
            .map(|p| json!({"id": p.id, "block": val(&p.block), "count": p.count}))
            .collect::<Vec<_>>(),
        "check": check_json(&check::check(&current.document)),
        "graph": format!("/api/drafts/{id}/graph"),
        "applications": d.applications.iter().map(to_json).collect::<Vec<_>>(),
    })
}

/// One revision as the revision list names it.
pub fn revision_json(r: &Revision) -> Value {
    json!({
        "number": r.row.number,
        "digest": r.row.digest.0,
        "revision_id": r.row.revision_id.0,
        "cause": r.cause.name(),
        "application": r.cause.application(),
    })
}

/// A tool call as a route answers it: its row, with the answer as JSON.
pub fn call_json(c: &Call) -> Value {
    let mut v = to_json(&c.row);
    v["answer"] = c.answer.clone().unwrap_or(Value::Null);
    v
}

fn body(request: &Request) -> Result<Value, Response> {
    let v: Value = serde_json::from_slice(&request.body)
        .map_err(|e| error(400, "InvalidRequest", &format!("the body is no JSON: {e}")))?;
    if v.is_object() {
        Ok(v)
    } else {
        Err(error(400, "InvalidRequest", "the body is a JSON object"))
    }
}

fn open(studio: &Mutex<Studio>, request: &Request) -> Response {
    let v = match body(request) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some(source) = v["source"].as_str() else {
        return error(
            400,
            "InvalidRequest",
            "open takes {\"source\": \"architectures/<name>.yaml\"}",
        );
    };
    let name = match &v["name"] {
        Value::Null => None,
        Value::String(n) if !n.trim().is_empty() => Some(n.trim()),
        _ => {
            return error(
                400,
                "InvalidRequest",
                "a draft's name is a non-empty string",
            );
        }
    };
    let mut s = lock(studio);
    match s.open(source, name) {
        Ok(d) => Response::json(201, &draft_json(d)),
        Err(e) => studio_error(&e),
    }
}

/// `POST /api/drafts/<id>/patches`: an apply in the draft's editor session, by the caller the
/// request is sent as.
fn apply(studio: &Mutex<Studio>, id: &str, request: &Request) -> Response {
    let caller = match sender(request) {
        Ok(c) => c,
        Err(r) => return r,
    };
    let v = match body(request) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let dry_run = v["dry_run"].as_bool().unwrap_or(false);
    let arguments = if v.get("format").is_some() {
        json!({"patch": v})
    } else if let Some(text) = v["text"].as_str() {
        json!({"patch": text, "name": v["name"]})
    } else {
        return error(
            400,
            "InvalidRequest",
            "apply takes {\"text\": \"<rules>\"} or a patch document",
        );
    };
    let mut s = lock(studio);
    let session = match s.editor_session(id) {
        Ok(session) => session,
        Err(e) => return studio_error(&e),
    };
    let call = match s.call(
        &session,
        st::tool("apply").expect("a tool"),
        caller,
        &arguments,
        dry_run,
    ) {
        Ok(c) => c,
        Err(e) => return studio_error(&e),
    };
    let Some(mut answer) = call.answer else {
        let failure = serde_json::to_value(&call.row.failure).unwrap_or(Value::Null);
        return error(
            422,
            "CallFailed",
            failure.as_str().unwrap_or("the call failed"),
        );
    };
    let status = if answer["refusal"].is_null() {
        200
    } else {
        422
    };
    if !answer["patch"].is_null() {
        answer["draft"] = s.draft(id).map(draft_json).unwrap_or(Value::Null);
    }
    answer["call"] = Value::from(call.row.call_id.0.as_str());
    Response::json(status, &answer)
}

fn start_session(studio: &Mutex<Studio>, request: &Request) -> Response {
    let v = match body(request) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some(draft) = v["draft_id"].as_str() else {
        return error(400, "InvalidRequest", "a session takes {\"draft_id\"}");
    };
    match lock(studio).start_session(draft) {
        Ok(row) => Response::json(201, &json!({"session": to_json(row)})),
        Err(e) => studio_error(&e),
    }
}

/// The caller a request is sent as: its `X-Studio-Caller` header, `User` when it names none.
/// A call records this caller and no other (`studio.session.CallTool` sets `caller` from the
/// sending actor's `kind`, not from its input).
pub(crate) fn sender(request: &Request) -> Result<Caller, Response> {
    match request.caller.as_deref() {
        None => Ok(Caller::User),
        Some(name) => st::caller(name).ok_or_else(|| {
            error(
                400,
                "InvalidRequest",
                &format!("the {CALLER_HEADER} header names User or Agent"),
            )
        }),
    }
}

/// The tool call a request names, `{"tool", "arguments"?, "dry_run"?}`, made by the caller the
/// request is sent as ([`sender`]). A body that names another caller is refused with
/// `CallerNotSender` and makes no call; one that names the sender itself is taken.
pub fn call(studio: &Mutex<Studio>, session: &str, request: &Request, input: &Value) -> Response {
    let caller = match sender(request) {
        Ok(c) => c,
        Err(r) => return r,
    };
    if !input["caller"].is_null()
        && input["caller"].as_str().and_then(st::caller) != Some(caller.clone())
    {
        return error(
            403,
            "CallerNotSender",
            &format!(
                "a call is made by the caller it is sent as ({CALLER_HEADER}: {}), and its body named another",
                st::wire(&caller)
            ),
        );
    }
    let Some(tool) = input["tool"].as_str().and_then(st::tool) else {
        return error(
            400,
            "InvalidRequest",
            "a call names its tool: query, apply, check or simulate",
        );
    };
    let arguments = match &input["arguments"] {
        Value::Null => json!({}),
        v @ Value::Object(_) => v.clone(),
        _ => {
            return error(
                400,
                "InvalidRequest",
                "a call's arguments are a JSON object",
            );
        }
    };
    let dry_run = input["dry_run"].as_bool().unwrap_or(false);
    match lock(studio).call(session, tool, caller, &arguments, dry_run) {
        Ok(c) => Response::json(200, &json!({"call": call_json(&c)})),
        Err(e) => studio_error(&e),
    }
}

fn revisions(studio: &Mutex<Studio>, id: &str) -> Response {
    match lock(studio).draft(id) {
        Ok(d) => Response::json(
            200,
            &json!({
                "draft_id": id,
                "revisions": d.revisions.iter().map(revision_json).collect::<Vec<_>>(),
            }),
        ),
        Err(e) => studio_error(&e),
    }
}

fn revision(studio: &Mutex<Studio>, id: &str, number: &str) -> Response {
    let Ok(n) = number.parse::<usize>() else {
        return error(
            404,
            "NoSuchRevision",
            &format!("{number} is no revision number"),
        );
    };
    match lock(studio).revision(id, n) {
        Ok(r) => Response::json(
            200,
            &json!({
                "revision": revision_json(r),
                "document": r.document,
                "graph": agenticspec_render::architecture::graph(&r.document),
                "check": check_json(&check::check(&r.document)),
            }),
        ),
        Err(e) => studio_error(&e),
    }
}

/// The draft whose event channel a request asks for, or none when it asks for no channel.
fn events_draft(request: &Request) -> Option<String> {
    if request.method != "GET" {
        return None;
    }
    let segments: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    match segments.as_slice() {
        ["api", "drafts", id, "events"] => Some((*id).to_string()),
        ["api", "channels", "draft_events"] => {
            Some(crate::app::param(&request.query, "draft_id").unwrap_or_default())
        }
        _ => None,
    }
}

/// A subscription to the event channel a request asks for: the receiver and the first comment
/// of the stream; an error response when the draft does not exist; none when the request asks
/// for no channel. `agenticstudio serve` streams the receiver on the connection (`live::stream`).
pub fn events(
    studio: &Mutex<Studio>,
    request: &Request,
) -> Option<Result<(Receiver<Event>, String), Response>> {
    let id = events_draft(request)?;
    let mut s = lock(studio);
    Some(match s.subscribe(&id) {
        Ok(rx) => {
            let n = s.draft(&id).map_or(0, |d| d.revisions.len());
            Ok((rx, format!("draft {id} at revision {n}")))
        }
        Err(e) => Err(studio_error(&e)),
    })
}

/// The answer to one request: an `/api` route, or a file of the built app under `www`.
pub fn serve(studio: &Mutex<Studio>, www: &Path, request: &Request) -> Response {
    let api = request.path == "/api" || request.path.starts_with("/api/");
    if !api && request.method == "GET" {
        return crate::app::file(www, &request.path);
    }
    answer(studio, request)
}

/// The answer to one `/api` request.
pub fn answer(studio: &Mutex<Studio>, request: &Request) -> Response {
    let method = request.method.as_str();
    let segments: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    match (method, segments.as_slice()) {
        ("GET", ["api", "views", name]) => crate::app::view(studio, name, request),
        ("POST", ["api", "commands", name]) => crate::app::command(studio, name, request),
        ("GET", ["api", "architectures"]) => {
            let list: Vec<Value> = lock(studio)
                .architectures()
                .into_iter()
                .map(|source| {
                    let name = source
                        .trim_start_matches("architectures/")
                        .trim_end_matches(".yaml")
                        .to_string();
                    json!({"name": name, "source": source})
                })
                .collect();
            Response::json(200, &json!({"architectures": list}))
        }
        ("GET", ["api", "drafts"]) => {
            let rows: Vec<Value> = lock(studio)
                .drafts()
                .iter()
                .map(|d| to_json(&d.row))
                .collect();
            Response::json(200, &json!({"drafts": rows}))
        }
        ("POST", ["api", "drafts"]) => open(studio, request),
        ("GET", ["api", "drafts", id]) => match lock(studio).draft(id) {
            Ok(d) => Response::json(200, &draft_json(d)),
            Err(e) => studio_error(&e),
        },
        ("GET", ["api", "drafts", id, "graph"]) => match lock(studio).draft(id) {
            Ok(d) => Response::new(
                200,
                HTML,
                agenticspec_render::architecture::page(&d.current().document, None),
            ),
            Err(e) => studio_error(&e),
        },
        ("POST", ["api", "drafts", id, "patches"]) => apply(studio, id, request),
        ("GET", ["api", "drafts", id, "revisions"]) => revisions(studio, id),
        ("GET", ["api", "drafts", id, "revisions", n]) => revision(studio, id, n),
        ("GET", ["api", "drafts", _, "events"] | ["api", "channels", "draft_events"]) => error(
            400,
            "StreamOnly",
            "the event channel is a server-sent-events stream of agenticstudio serve",
        ),
        ("GET", ["api", "sessions"]) => {
            let rows: Vec<Value> = lock(studio).sessions().iter().map(to_json).collect();
            Response::json(200, &json!({"sessions": rows}))
        }
        ("POST", ["api", "sessions"]) => start_session(studio, request),
        ("POST", ["api", "sessions", id, "close"]) => match lock(studio).close_session(id) {
            Ok(row) => Response::json(200, &json!({"session": to_json(row)})),
            Err(e) => studio_error(&e),
        },
        ("GET", ["api", "sessions", id, "calls"]) => match lock(studio).calls(id) {
            Ok(rows) => Response::json(
                200,
                &json!({"rows": rows.into_iter().map(to_json).collect::<Vec<_>>()}),
            ),
            Err(e) => studio_error(&e),
        },
        ("POST", ["api", "sessions", id, "calls"]) => match body(request) {
            Ok(v) => call(studio, id, request, &v),
            Err(r) => r,
        },
        (_, ["api", "architectures"] | ["api", "drafts"] | ["api", "drafts", _])
        | (
            _,
            [
                "api",
                "drafts",
                _,
                "graph" | "patches" | "revisions" | "events",
            ],
        )
        | (_, ["api", "drafts", _, "revisions", _])
        | (_, ["api", "sessions"] | ["api", "sessions", _, "close" | "calls"])
        | (_, ["api", "views" | "commands", _]) => error(
            405,
            "MethodNotAllowed",
            &format!("{method} is not allowed on {}", request.path),
        ),
        _ => error(404, "NoSuchRoute", &format!("no route {}", request.path)),
    }
}
