// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! The `session-view` component of `agenticspec` v1, on the wire.
//!
//! The specification says this component's callers are not deployed with it, so its surface
//! exists on a wire. Which wire is derived rather than chosen: the one contract this model
//! projects for a command surface is the `OpenAPI` document, and an `OpenAPI` document is an
//! HTTP contract. The document is beside this file, served verbatim at `/openapi.json`.

use crate::{entry, http, json, wire};

/// The contract this surface answers, byte for byte as `generated/` commits it.
///
/// Embedded rather than rebuilt at run time: a server that regenerated its own contract could
/// publish one the repository never reviewed.
pub const OPENAPI: &str = include_str!("session-view.openapi.json");

/// The prose the same model produced, byte for byte as the documentation projection wrote it.
pub const DOCS: &str = include_str!("session-view.docs.md");

/// Every route this surface answers, in path order.
///
/// The same set the `OpenAPI` document declares, plus the two documents about the surface
/// itself, which no specification construct names and nothing can therefore derive. A path
/// absent from this table is answered with `404`, including one the document declares and this
/// table forgot — which is the failure a table computed twice would hide.
pub const ROUTES: &[(&str, &str)] = &[
    ("GET", "/docs"),
    ("GET", "/openapi.json"),
    ("GET", "/view/views/agent-turns"),
    ("GET", "/view/views/figures"),
    ("GET", "/view/views/session"),
    ("GET", "/view/views/session-agents"),
    ("GET", "/view/views/session-checks"),
    ("GET", "/view/views/session-event-counts"),
    ("GET", "/view/views/session-events"),
    ("GET", "/view/views/session-list"),
    ("GET", "/view/views/session-timeline"),
    ("GET", "/view/views/session-turns"),
];

/// What this process says about itself as it starts, before it answers anything.
///
/// Three lines of JSON on standard output, in this order, every member of them derived from the
/// specification — except `runtime`, which is appended by the emitted code below and holds what
/// is true of *this process*: the language it was synthesised into, and the address it bound.
/// Everything outside `runtime` is the same in every language this plan is emitted into, and
/// `cargo xtask synth --check` starts both and compares them.
pub const STARTUP: &[&str] = &[
    "{\"log\":\"ess/1\",\"event\":\"system.starting\",\"system\":\"agenticspec\",\"version\":\"v1\",\"model_digest\":\"0ce0451bd95a1b3b6d050210c9abf868b7b9eabf8ab66c02dd02917b29c355fc\",\"contract_digest\":\"ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862\",\"components\":[\"session-ir\",\"session-view\"],\"capabilities\":{\"generated\":234,\"obligations\":3,\"refused\":0}",
    "{\"log\":\"ess/1\",\"event\":\"surface.serving\",\"component\":\"session-view\",\"reached_by\":\"network\",\"transport\":\"http/1.1\",\"routes\":12,\"paths\":[{\"method\":\"GET\",\"path\":\"/docs\",\"serves\":\"documentation\",\"name\":\"docs\"},{\"method\":\"GET\",\"path\":\"/openapi.json\",\"serves\":\"contract\",\"name\":\"openapi\"},{\"method\":\"GET\",\"path\":\"/view/views/agent-turns\",\"serves\":\"view\",\"name\":\"agenticspec.view.AgentTurns\"},{\"method\":\"GET\",\"path\":\"/view/views/figures\",\"serves\":\"view\",\"name\":\"agenticspec.view.OverviewFigures\"},{\"method\":\"GET\",\"path\":\"/view/views/session\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionById\"},{\"method\":\"GET\",\"path\":\"/view/views/session-agents\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionAgents\"},{\"method\":\"GET\",\"path\":\"/view/views/session-checks\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionChecks\"},{\"method\":\"GET\",\"path\":\"/view/views/session-event-counts\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionEventCounts\"},{\"method\":\"GET\",\"path\":\"/view/views/session-events\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionEvents\"},{\"method\":\"GET\",\"path\":\"/view/views/session-list\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionList\"},{\"method\":\"GET\",\"path\":\"/view/views/session-timeline\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionTimeline\"},{\"method\":\"GET\",\"path\":\"/view/views/session-turns\",\"serves\":\"view\",\"name\":\"agenticspec.view.SessionTurns\"}]",
    "{\"log\":\"ess/1\",\"event\":\"system.ready\",\"system\":\"agenticspec\",\"surfaces\":2",
];

/// Writes the startup record, with this process's own facts closing each line.
fn announce(address: &std::net::SocketAddr) {
    for facts in STARTUP {
        let mut line = String::from(*facts);
        line.push_str(",\"runtime\":{\"address\":");
        json::push_text(&mut line, &address.to_string());
        line.push_str(",\"language\":\"rust\",\"port\":");
        json::push_integer(&mut line, i64::from(address.port()));
        line.push_str("}}");
        println!("{line}");
    }
}

/// Serves `session-view` at `address`, and does not return while it can answer.
///
/// `address` may name port `0`, which binds an ephemeral port; the startup record says which one
/// was taken, because a caller that cannot learn the port cannot make a request.
///
/// It chooses no realization. Every command reaches the port, and a port over unimplemented
/// obligations answers the typed refusal this surface reports as `501` — the honest empty
/// state rather than a server that pretends.
///
/// `authenticate` is the realization's: it says who each request was sent by, or `None`, and
/// [`dispatch`] checks that caller's grant before the command runs. Nothing here reads an
/// actor from the request itself.
///
/// One connection at a time: each is dropped after [`http::READ_TIMEOUT`] without a byte, or
/// [`http::WRITE_TIMEOUT`] of a stalled write, and whatever fails on one connection — a caller
/// that hung up before reading its answer, a reset, a failed accept — ends that connection only.
///
/// # Errors
///
/// What binding the address refuses: the address is taken, or the port is privileged.
pub fn serve<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, address: &str, authenticate: impl Fn(&http::Request) -> Option<agenticspec_types::actor::Caller>) -> std::io::Result<()>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{ serve_with_static(system, address, authenticate, None) }

/// Serves the declared surface, with files only for paths outside its route table.
///
/// # Errors
/// Returns a listener or static-root error before announcing readiness.
pub fn serve_with_static<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, address: &str, authenticate: impl Fn(&http::Request) -> Option<agenticspec_types::actor::Caller>, static_root: Option<&std::path::Path>) -> std::io::Result<()>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let static_root = static_root.map(std::fs::canonicalize).transpose()?;
    if static_root.as_ref().is_some_and(|root| !root.is_dir()) { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "static root is not a directory")); }
    let listener = std::net::TcpListener::bind(address)?;
    announce(&listener.local_addr()?);
    for connection in listener.incoming() {
        // What fails on one connection ends that connection, never this loop: a caller that
        // gave up before it was accepted, or before it read its answer, is that caller's affair.
        let Ok(connection) = connection else {
            // An accept the listener itself failed (no descriptor left) would fail again at
            // once: pause before the next.
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        };
        let _ = connection.set_read_timeout(Some(http::READ_TIMEOUT));
        let _ = connection.set_write_timeout(Some(http::WRITE_TIMEOUT));
        let mut reader = std::io::BufReader::new(connection);
        let (answer, refused) = match http::read(&mut reader) {
            Ok(request) => {
                if !ROUTES.iter().any(|(_, path)| *path == request.path) {
                    if let Some(root) = &static_root {
                        let _ = crate::static_assets::answer(reader.get_mut(), root, &request);
                        continue;
                    }
                }
                (dispatch(system, authenticate(&request).as_ref(), &request), false)
            },
            Err(refusal) => (refusal, true),
        };
        let mut stream = reader.into_inner();
        if http::write(&mut stream, &answer).is_ok() && refused {
            http::linger(&mut stream);
        }
    }
    Ok(())
}

/// Answers one request.
///
/// A path this table does not hold is a `404` naming where the whole table is published; a
/// path it holds under a different method is a `405` naming the one it answers. Neither is a
/// status the contract declares, and neither should be: both are facts about a transport rather
/// than about any command.
///
/// Public so a caller can hand it a request it built itself: [`serve`] is this function behind a
/// socket, and nothing else.
///
/// `caller` is who the realization authenticated the request as, or `None`. Every command
/// checks its grant before it runs, and answers the standard refusal when the caller is none
/// or is an actor the specification does not grant the command.
/// A view some actor's grant names checks the reader's grant the same way before it is read.
pub fn dispatch<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, request: &http::Request) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    dispatch_with_events(system, caller, request).0
}

/// Answers one request as [`dispatch`] does, and hands back every event published while it
/// ran, in log order: what the command published, what the bindings that reacted to it
/// published in turn, and what a held-back delivery retried during the call published. An
/// event already on the system's log when the call started is not the call's and is never
/// in the list. The list is empty where no command ran — a refusal before the port, a view,
/// a document or a path no route matches — and holds what was published where delivering
/// it failed after the command took effect. An event a component's port published through a
/// direct call is in that component's outbox until a pump collects it, so one no pump has
/// collected yet is listed as this call's: call `pump` after calling a port directly.
pub fn dispatch_with_events<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, request: &http::Request) -> (http::Response, Vec<agenticspec_system::SystemEvent>)
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let mut taken = Vec::new();
    let answer = route_request(system, caller, request, &mut taken);
    (answer, taken)
}

/// The route match behind [`dispatch_with_events`]: each command pushes what it took off the
/// log onto `taken`.
fn route_request<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, request: &http::Request, _taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match request.path.as_str() {
        "/docs" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::Response::new(200, http::MARKDOWN, DOCS)
        }
        "/openapi.json" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::Response::new(200, http::JSON, OPENAPI)
        }
        "/view/views/agent-turns" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.AgentTurns") {
                return not_granted(actor);
            }
            serve_agenticspec_view_agent_turns(system, &request.query)
        }
        "/view/views/figures" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.OverviewFigures") {
                return not_granted(actor);
            }
            serve_agenticspec_view_overview_figures(system, &request.query)
        }
        "/view/views/session" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionById") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_by_id(system, &request.query)
        }
        "/view/views/session-agents" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionAgents") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_agents(system, &request.query)
        }
        "/view/views/session-checks" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionChecks") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_checks(system, &request.query)
        }
        "/view/views/session-event-counts" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionEventCounts") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_event_counts(system, &request.query)
        }
        "/view/views/session-events" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionEvents") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_events(system, &request.query)
        }
        "/view/views/session-list" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionList") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_list(system, &request.query)
        }
        "/view/views/session-timeline" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionTimeline") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_timeline(system, &request.query)
        }
        "/view/views/session-turns" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            if let Err(actor) = admit_read(caller, "agenticspec.view.SessionTurns") {
                return not_granted(actor);
            }
            serve_agenticspec_view_session_turns(system, &request.query)
        }
        other => http::Response::refusal(
            404,
            &format!("`{other}` is not a path this surface declares; `GET /openapi.json` publishes every one that is"),
        ),
    }
}

/// Runs one command or view of `session-view` by its qualified name, with no transport.
///
/// The same decoding, refusals and rendering the HTTP routes use — each route and this function call
/// one `run_*` function — so a conformance runner or an in-process caller drives the system
/// without a socket and without a dispatch table of its own. `Ok` is the declared outcome, as the
/// route's body renders it; a view without parameters ignores `input`, as its `GET` route ignores a body, and a view with
/// parameters reads them from `input`, an object keyed by their wire names, as its route reads
/// them from the query string.
///
/// # Errors
///
/// [`entry::Refused::Unknown`] naming `name` when this surface declares no command or view
/// by it; [`entry::Refused::Input`] when `input` is not the command's declared input (the
/// route's `400`); [`entry::Refused::Unmet`] when the port reports an unmet obligation, and
/// [`entry::Refused::Undelivered`] when the command took effect and delivering what it published
/// failed (the route's `501`, with `committed` `false` and `true`).
/// [`entry::Refused::NotGranted`] when `caller` — who the realization authenticated the call
/// as — is none, or is an actor the specification does not grant the command; checked before
/// the command runs (the route's `403`).
/// A view some actor's grant names answers the same refusal, checked before it is read.
///
/// `input` is held to what the route's reader admits: nested no deeper, every number spelled
/// as JSON spells one, or [`entry::Refused::Input`]. The answer is read back at any depth; one holding a
/// `Json` value the realization built with a number JSON does not spell is not JSON, and is
/// [`entry::Refused::Undelivered`] for a command, whose effect stands, and [`entry::Refused::Unmet`]
/// for a view.
pub fn handle<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, name: &str, input: json::Value) -> Result<json::Value, entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    handle_with_events(system, caller, name, input).0
}

/// Runs one command or view as [`handle`] does, and hands back every event published while it
/// ran, in log order: what the command published, what the bindings that reacted to it
/// published in turn, and what a held-back delivery retried during the call published. An
/// event already on the system's log when the call started is not the call's and is never
/// in the list. The list is empty where no command ran — an unknown name, a refusal before
/// the port, a view — and holds what was published where delivering it failed after the
/// command took effect. An event a component's port published through a direct call is in
/// that component's outbox until a pump collects it, so one no pump has collected yet is
/// listed as this call's: call `pump` after calling a port directly.
pub fn handle_with_events<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, name: &str, input: json::Value) -> (Result<json::Value, entry::Refused>, Vec<agenticspec_system::SystemEvent>)
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let taken = Vec::new();
    let checked = |input: &json::Value| {
        json::admitted(input).map_err(|expected| {
            entry::Refused::Input(format!("the input is not JSON: expected {expected}"))
        })
    };
    let answered = match name {
        "agenticspec.view.AgentTurns" => match admit_read(caller, "agenticspec.view.AgentTurns") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_agent_turns(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.OverviewFigures" => match admit_read(caller, "agenticspec.view.OverviewFigures") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_overview_figures(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionAgents" => match admit_read(caller, "agenticspec.view.SessionAgents") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_agents(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionById" => match admit_read(caller, "agenticspec.view.SessionById") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_by_id(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionChecks" => match admit_read(caller, "agenticspec.view.SessionChecks") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_checks(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionEventCounts" => match admit_read(caller, "agenticspec.view.SessionEventCounts") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_event_counts(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionEvents" => match admit_read(caller, "agenticspec.view.SessionEvents") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_events(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionList" => match admit_read(caller, "agenticspec.view.SessionList") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_list(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionTimeline" => match admit_read(caller, "agenticspec.view.SessionTimeline") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_timeline(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.view.SessionTurns" => match admit_read(caller, "agenticspec.view.SessionTurns") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_view_session_turns(system, &input)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        other => return (Err(entry::Refused::Unknown(other.to_owned())), taken),
    };
    let body = match answered {
        Ok((_, body)) => body,
        Err(refused) => return (Err(refused), taken),
    };
    let read = entry::read(&body).map_err(|error| {
        entry::Refused::Unmet(format!(
            "unmet obligation: view_query `{name}` answered rows this surface cannot read back: {error}"
        ))
    });
    (read, taken)
}

/// Nothing, where `caller` may invoke `command`; otherwise the actor the standard refusal names,
/// `None` where the request was authenticated as no actor.
///
/// Checked before the command runs, on the caller the realization authenticated the request
/// as and never on anything the request says about itself. Public, so code that drives the system
/// in process checks the grant exactly as every route does.
pub fn admit(caller: Option<&agenticspec_types::actor::Caller>, command: &str) -> Result<(), Option<&'static str>> {
    match caller {
        Some(caller) if caller.may(command) => Ok(()),
        Some(caller) => Err(Some(caller.actor.name())),
        None => Err(None),
    }
}

/// The standard refusal for an actor no grant admits, as the contract declares it: `403`,
/// `{"refused": "not granted", "actor": <name or null>}`.
fn not_granted(actor: Option<&str>) -> http::Response {
    let mut body = String::from("{");
    json::member(&mut body, "refused");
    json::push_text(&mut body, "not granted");
    json::member(&mut body, "actor");
    match actor {
        Some(actor) => json::push_text(&mut body, actor),
        None => body.push_str("null"),
    }
    body.push('}');
    http::Response::new(403, http::JSON, body)
}

/// Nothing, where `caller` may read `view`, a view some actor's grant names; otherwise the actor
/// the standard refusal names, `None` where the request was authenticated as no actor.
///
/// Checked before the view is read, on the caller the realization authenticated the request as.
/// Public, so code that reads the system in process checks the grant exactly as every route does.
pub fn admit_read(caller: Option<&agenticspec_types::actor::Caller>, view: &str) -> Result<(), Option<&'static str>> {
    match caller {
        Some(caller) if caller.may_read(view) => Ok(()),
        Some(caller) => Err(Some(caller.actor.name())),
        None => Err(None),
    }
}

/// `GET` `agenticspec.view.AgentTurns`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_agent_turns<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text), ("agent", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_agent_turns(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.AgentTurns` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_agent_turns<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1) = match wire::decode_params_agenticspec_view_agent_turns(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.agent_turns(param0, param1) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_agent_turns(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.OverviewFigures`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_overview_figures<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("since", http::Scalar::Text), ("bucket_seconds", http::Scalar::Integer)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_overview_figures(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.OverviewFigures` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_overview_figures<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1) = match wire::decode_params_agenticspec_view_overview_figures(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.overview_figures(param0, param1) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_overview_figures(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionById`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_by_id<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_by_id(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionById` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_by_id<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0,) = match wire::decode_params_agenticspec_view_session_by_id(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_by_id(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_by_id(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionAgents`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_agents<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_agents(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionAgents` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_agents<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0,) = match wire::decode_params_agenticspec_view_session_agents(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_agents(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_agents(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionChecks`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_checks<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_checks(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionChecks` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_checks<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0,) = match wire::decode_params_agenticspec_view_session_checks(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_checks(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_checks(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionEventCounts`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_event_counts<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text), ("agent", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_event_counts(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionEventCounts` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_event_counts<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1) = match wire::decode_params_agenticspec_view_session_event_counts(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_event_counts(param0, param1) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_event_counts(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionEvents`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_events<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text), ("agent", http::Scalar::Text), ("kind", http::Scalar::Text), ("notable", http::Scalar::Boolean), ("before", http::Scalar::Integer)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_events(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionEvents` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_events<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1, param2, param3, param4) = match wire::decode_params_agenticspec_view_session_events(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_events(param0, param1, param2, param3, param4) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_events(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionList`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_list<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("since", http::Scalar::Text), ("search", http::Scalar::Text), ("limit", http::Scalar::Integer), ("offset", http::Scalar::Integer)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_list(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionList` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_list<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1, param2, param3) = match wire::decode_params_agenticspec_view_session_list(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_list(param0, param1, param2, param3) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_list(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionTimeline`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_timeline<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text), ("slot_seconds", http::Scalar::Integer)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_timeline(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionTimeline` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_timeline<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0, param1) = match wire::decode_params_agenticspec_view_session_timeline(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_timeline(param0, param1) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_timeline(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.view.SessionTurns`: reads the declared parameters from the query string by their wire names, runs the
/// port, answers its rows. A key the view does not declare is ignored.
fn serve_agenticspec_view_session_turns<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, query: &str) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match http::query_object(query, &[("session_id", http::Scalar::Text)], "query") {
        Ok(value) => http::answer(run_agenticspec_view_session_turns(system, &value)),
        Err(refused) => http::Response::refusal(400, &refused),
    }
}

/// `GET` `agenticspec.view.SessionTurns` at `eventual` consistency: every row the owed projection holds for the declared
/// parameters, read from `value`, an object keyed by their wire names.
///
/// The one path the `GET` route and [`handle`] share. A decoding failure is located under
/// `query`, as the route reports it.
fn run_agenticspec_view_session_turns<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let (param0,) = match wire::decode_params_agenticspec_view_session_turns(value, "query") {
        Ok(params) => params,
        Err(error) => return Err(entry::Refused::Input(format!("{error}"))),
    };
    match system.session_view.session_turns(param0) {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_view_session_turns(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}
