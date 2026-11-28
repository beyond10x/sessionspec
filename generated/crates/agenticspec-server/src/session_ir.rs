// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! The `session-ir` component of `agenticspec` v1, on the wire.
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
pub const OPENAPI: &str = include_str!("session-ir.openapi.json");

/// The prose the same model produced, byte for byte as the documentation projection wrote it.
pub const DOCS: &str = include_str!("session-ir.docs.md");

/// Every route this surface answers, in path order.
///
/// The same set the `OpenAPI` document declares, plus the two documents about the surface
/// itself, which no specification construct names and nothing can therefore derive. A path
/// absent from this table is answered with `404`, including one the document declares and this
/// table forgot — which is the failure a table computed twice would hide.
pub const ROUTES: &[(&str, &str)] = &[
    ("GET", "/docs"),
    ("GET", "/openapi.json"),
    ("POST", "/session/commands/compact-context"),
    ("POST", "/session/commands/complete-tool-call"),
    ("POST", "/session/commands/decide-tool-call"),
    ("POST", "/session/commands/end-agent"),
    ("POST", "/session/commands/end-session"),
    ("POST", "/session/commands/end-turn"),
    ("POST", "/session/commands/open-session"),
    ("POST", "/session/commands/record-exposure"),
    ("POST", "/session/commands/record-model-turn"),
    ("POST", "/session/commands/request-tool-call"),
    ("POST", "/session/commands/start-agent"),
    ("POST", "/session/commands/submit-prompt"),
    ("GET", "/session/views/agents"),
    ("GET", "/session/views/exposures"),
    ("GET", "/session/views/sessions"),
    ("GET", "/session/views/tool-calls"),
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
    "{\"log\":\"ess/1\",\"event\":\"surface.serving\",\"component\":\"session-ir\",\"reached_by\":\"network\",\"transport\":\"http/1.1\",\"routes\":18,\"paths\":[{\"method\":\"GET\",\"path\":\"/docs\",\"serves\":\"documentation\",\"name\":\"docs\"},{\"method\":\"GET\",\"path\":\"/openapi.json\",\"serves\":\"contract\",\"name\":\"openapi\"},{\"method\":\"POST\",\"path\":\"/session/commands/compact-context\",\"serves\":\"command\",\"name\":\"agenticspec.session.CompactContext\"},{\"method\":\"POST\",\"path\":\"/session/commands/complete-tool-call\",\"serves\":\"command\",\"name\":\"agenticspec.session.CompleteToolCall\"},{\"method\":\"POST\",\"path\":\"/session/commands/decide-tool-call\",\"serves\":\"command\",\"name\":\"agenticspec.session.DecideToolCall\"},{\"method\":\"POST\",\"path\":\"/session/commands/end-agent\",\"serves\":\"command\",\"name\":\"agenticspec.session.EndAgent\"},{\"method\":\"POST\",\"path\":\"/session/commands/end-session\",\"serves\":\"command\",\"name\":\"agenticspec.session.EndSession\"},{\"method\":\"POST\",\"path\":\"/session/commands/end-turn\",\"serves\":\"command\",\"name\":\"agenticspec.session.EndTurn\"},{\"method\":\"POST\",\"path\":\"/session/commands/open-session\",\"serves\":\"command\",\"name\":\"agenticspec.session.OpenSession\"},{\"method\":\"POST\",\"path\":\"/session/commands/record-exposure\",\"serves\":\"command\",\"name\":\"agenticspec.session.RecordExposure\"},{\"method\":\"POST\",\"path\":\"/session/commands/record-model-turn\",\"serves\":\"command\",\"name\":\"agenticspec.session.RecordModelTurn\"},{\"method\":\"POST\",\"path\":\"/session/commands/request-tool-call\",\"serves\":\"command\",\"name\":\"agenticspec.session.RequestToolCall\"},{\"method\":\"POST\",\"path\":\"/session/commands/start-agent\",\"serves\":\"command\",\"name\":\"agenticspec.session.StartAgent\"},{\"method\":\"POST\",\"path\":\"/session/commands/submit-prompt\",\"serves\":\"command\",\"name\":\"agenticspec.session.SubmitPrompt\"},{\"method\":\"GET\",\"path\":\"/session/views/agents\",\"serves\":\"view\",\"name\":\"agenticspec.session.Agents\"},{\"method\":\"GET\",\"path\":\"/session/views/exposures\",\"serves\":\"view\",\"name\":\"agenticspec.session.Exposures\"},{\"method\":\"GET\",\"path\":\"/session/views/sessions\",\"serves\":\"view\",\"name\":\"agenticspec.session.Sessions\"},{\"method\":\"GET\",\"path\":\"/session/views/tool-calls\",\"serves\":\"view\",\"name\":\"agenticspec.session.ToolCalls\"}]",
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

/// Serves `session-ir` at `address`, and does not return while it can answer.
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
fn route_request<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, caller: Option<&agenticspec_types::actor::Caller>, request: &http::Request, taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
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
        "/session/commands/compact-context" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.CompactContext") {
                return not_granted(actor);
            }
            serve_agenticspec_session_compact_context(system, &request.body, taken)
        }
        "/session/commands/complete-tool-call" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.CompleteToolCall") {
                return not_granted(actor);
            }
            serve_agenticspec_session_complete_tool_call(system, &request.body, taken)
        }
        "/session/commands/decide-tool-call" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.DecideToolCall") {
                return not_granted(actor);
            }
            serve_agenticspec_session_decide_tool_call(system, &request.body, taken)
        }
        "/session/commands/end-agent" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.EndAgent") {
                return not_granted(actor);
            }
            serve_agenticspec_session_end_agent(system, &request.body, taken)
        }
        "/session/commands/end-session" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.EndSession") {
                return not_granted(actor);
            }
            serve_agenticspec_session_end_session(system, &request.body, taken)
        }
        "/session/commands/end-turn" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.EndTurn") {
                return not_granted(actor);
            }
            serve_agenticspec_session_end_turn(system, &request.body, taken)
        }
        "/session/commands/open-session" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.OpenSession") {
                return not_granted(actor);
            }
            serve_agenticspec_session_open_session(system, &request.body, taken)
        }
        "/session/commands/record-exposure" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.RecordExposure") {
                return not_granted(actor);
            }
            serve_agenticspec_session_record_exposure(system, &request.body, taken)
        }
        "/session/commands/record-model-turn" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.RecordModelTurn") {
                return not_granted(actor);
            }
            serve_agenticspec_session_record_model_turn(system, &request.body, taken)
        }
        "/session/commands/request-tool-call" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.RequestToolCall") {
                return not_granted(actor);
            }
            serve_agenticspec_session_request_tool_call(system, &request.body, taken)
        }
        "/session/commands/start-agent" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.StartAgent") {
                return not_granted(actor);
            }
            serve_agenticspec_session_start_agent(system, &request.body, taken)
        }
        "/session/commands/submit-prompt" => {
            if request.method != "POST" {
                return http::method_not_allowed("POST");
            }
            if let Err(actor) = admit(caller, "agenticspec.session.SubmitPrompt") {
                return not_granted(actor);
            }
            serve_agenticspec_session_submit_prompt(system, &request.body, taken)
        }
        "/session/views/agents" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::answer(run_agenticspec_session_agents(system))
        }
        "/session/views/exposures" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::answer(run_agenticspec_session_exposures(system))
        }
        "/session/views/sessions" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::answer(run_agenticspec_session_sessions(system))
        }
        "/session/views/tool-calls" => {
            if request.method != "GET" {
                return http::method_not_allowed("GET");
            }
            http::answer(run_agenticspec_session_tool_calls(system))
        }
        other => http::Response::refusal(
            404,
            &format!("`{other}` is not a path this surface declares; `GET /openapi.json` publishes every one that is"),
        ),
    }
}

/// Runs one command or view of `session-ir` by its qualified name, with no transport.
///
/// The same decoding, refusals and rendering the HTTP routes use — each route and this function call
/// one `run_*` function — so a conformance runner or an in-process caller drives the system
/// without a socket and without a dispatch table of its own. `Ok` is the declared outcome, as the
/// route's body renders it; a view ignores `input`, as its `GET` route ignores a body.
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
    let mut taken = Vec::new();
    let checked = |input: &json::Value| {
        json::admitted(input).map_err(|expected| {
            entry::Refused::Input(format!("the input is not JSON: expected {expected}"))
        })
    };
    let answered = match name {
        "agenticspec.session.Agents" => run_agenticspec_session_agents(system),
        "agenticspec.session.CompactContext" => match admit(caller, "agenticspec.session.CompactContext") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_compact_context(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.CompleteToolCall" => match admit(caller, "agenticspec.session.CompleteToolCall") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_complete_tool_call(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.DecideToolCall" => match admit(caller, "agenticspec.session.DecideToolCall") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_decide_tool_call(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.EndAgent" => match admit(caller, "agenticspec.session.EndAgent") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_end_agent(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.EndSession" => match admit(caller, "agenticspec.session.EndSession") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_end_session(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.EndTurn" => match admit(caller, "agenticspec.session.EndTurn") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_end_turn(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.Exposures" => run_agenticspec_session_exposures(system),
        "agenticspec.session.OpenSession" => match admit(caller, "agenticspec.session.OpenSession") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_open_session(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.RecordExposure" => match admit(caller, "agenticspec.session.RecordExposure") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_record_exposure(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.RecordModelTurn" => match admit(caller, "agenticspec.session.RecordModelTurn") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_record_model_turn(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.RequestToolCall" => match admit(caller, "agenticspec.session.RequestToolCall") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_request_tool_call(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.Sessions" => run_agenticspec_session_sessions(system),
        "agenticspec.session.StartAgent" => match admit(caller, "agenticspec.session.StartAgent") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_start_agent(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.SubmitPrompt" => match admit(caller, "agenticspec.session.SubmitPrompt") {
            Ok(()) => checked(&input).and_then(|()| run_agenticspec_session_submit_prompt(system, &input, &mut taken)),
            Err(actor) => Err(entry::Refused::NotGranted(actor.map(str::to_owned))),
        },
        "agenticspec.session.ToolCalls" => run_agenticspec_session_tool_calls(system),
        other => return (Err(entry::Refused::Unknown(other.to_owned())), taken),
    };
    let body = match answered {
        Ok((_, body)) => body,
        Err(refused) => return (Err(refused), taken),
    };
    let read = entry::read(&body).map_err(|error| {
        if matches!(name, "agenticspec.session.CompactContext" | "agenticspec.session.CompleteToolCall" | "agenticspec.session.DecideToolCall" | "agenticspec.session.EndAgent" | "agenticspec.session.EndSession" | "agenticspec.session.EndTurn" | "agenticspec.session.OpenSession" | "agenticspec.session.RecordExposure" | "agenticspec.session.RecordModelTurn" | "agenticspec.session.RequestToolCall" | "agenticspec.session.StartAgent" | "agenticspec.session.SubmitPrompt") {
            entry::Refused::Undelivered(format!(
                "delivering what the command published: its answer is not JSON this surface reads back: {error}"
            ))
        } else {
            entry::Refused::Unmet(format!(
                "unmet obligation: view_query `{name}` answered rows this surface cannot read back: {error}"
            ))
        }
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

/// `POST` `agenticspec.session.CompactContext`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_compact_context<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_compact_context(system, &value, taken))
}

/// `agenticspec.session.CompactContext` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_compact_context<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_compact_context(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.compact_context(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_compact_context(&outcome))
}

/// One declared outcome of `agenticspec.session.CompactContext`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_compact_context(outcome: &agenticspec_types::session::CompactContextOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::CompactContextOutcome::Compacted { context_compacted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "compacted");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ContextCompacted");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_context_compacted(context_compacted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::CompactContextOutcome::NoSuchAgent { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-agent");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.UnknownAgent");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_unknown_agent(error, &mut body);
            404
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.CompleteToolCall`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_complete_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_complete_tool_call(system, &value, taken))
}

/// `agenticspec.session.CompleteToolCall` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_complete_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_complete_tool_call(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.complete_tool_call(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_complete_tool_call(&outcome))
}

/// One declared outcome of `agenticspec.session.CompleteToolCall`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_complete_tool_call(outcome: &agenticspec_types::session::CompleteToolCallOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::CompleteToolCallOutcome::Succeeded { tool_call_completed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "succeeded");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ToolCallCompleted");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_tool_call_completed(tool_call_completed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::CompleteToolCallOutcome::Failed { tool_call_completed, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "failed");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ToolCallCompleted");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_tool_call_completed(tool_call_completed, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::CompleteToolCallOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.ToolCallStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_tool_call_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::CompleteToolCallOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.ToolCallStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.DecideToolCall`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_decide_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_decide_tool_call(system, &value, taken))
}

/// `agenticspec.session.DecideToolCall` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_decide_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_decide_tool_call(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.decide_tool_call(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_decide_tool_call(&outcome))
}

/// One declared outcome of `agenticspec.session.DecideToolCall`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_decide_tool_call(outcome: &agenticspec_types::session::DecideToolCallOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::DecideToolCallOutcome::Accepted { tool_call_decided, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "accepted");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ToolCallDecided");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_tool_call_decided(tool_call_decided, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::DecideToolCallOutcome::Refused { tool_call_decided, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "refused");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ToolCallDecided");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_tool_call_decided(tool_call_decided, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::DecideToolCallOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.ToolCallStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_tool_call_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::DecideToolCallOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.ToolCallStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.EndAgent`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_end_agent<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_end_agent(system, &value, taken))
}

/// `agenticspec.session.EndAgent` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_end_agent<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_end_agent(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.end_agent(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_end_agent(&outcome))
}

/// One declared outcome of `agenticspec.session.EndAgent`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_end_agent(outcome: &agenticspec_types::session::EndAgentOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::EndAgentOutcome::Ended { agent_ended, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "ended");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.AgentEnded");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_agent_ended(agent_ended, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::EndAgentOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_agent_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::EndAgentOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.EndSession`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_end_session<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_end_session(system, &value, taken))
}

/// `agenticspec.session.EndSession` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_end_session<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_end_session(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.end_session(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_end_session(&outcome))
}

/// One declared outcome of `agenticspec.session.EndSession`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_end_session(outcome: &agenticspec_types::session::EndSessionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::EndSessionOutcome::Ended { session_ended, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "ended");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.SessionEnded");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_session_ended(session_ended, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::EndSessionOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.SessionStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_session_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::EndSessionOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.SessionStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.EndTurn`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_end_turn<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_end_turn(system, &value, taken))
}

/// `agenticspec.session.EndTurn` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_end_turn<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_end_turn(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.end_turn(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_end_turn(&outcome))
}

/// One declared outcome of `agenticspec.session.EndTurn`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_end_turn(outcome: &agenticspec_types::session::EndTurnOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::EndTurnOutcome::Ended { turn_ended, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "ended");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.TurnEnded");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_turn_ended(turn_ended, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::EndTurnOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_agent_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::EndTurnOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.OpenSession`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_open_session<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_open_session(system, &value, taken))
}

/// `agenticspec.session.OpenSession` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_open_session<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_open_session(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.open_session(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_open_session(&outcome))
}

/// One declared outcome of `agenticspec.session.OpenSession`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_open_session(outcome: &agenticspec_types::session::OpenSessionOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::OpenSessionOutcome::Opened { session_opened, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "opened");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.SessionOpened");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_session_opened(session_opened, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.RecordExposure`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_record_exposure<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_record_exposure(system, &value, taken))
}

/// `agenticspec.session.RecordExposure` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_record_exposure<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_record_exposure(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.record_exposure(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_record_exposure(&outcome))
}

/// One declared outcome of `agenticspec.session.RecordExposure`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_record_exposure(outcome: &agenticspec_types::session::RecordExposureOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::RecordExposureOutcome::Recorded { exposure_detected, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "recorded");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ExposureDetected");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_exposure_detected(exposure_detected, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.RecordModelTurn`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_record_model_turn<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_record_model_turn(system, &value, taken))
}

/// `agenticspec.session.RecordModelTurn` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_record_model_turn<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_record_model_turn(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.record_model_turn(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_record_model_turn(&outcome))
}

/// One declared outcome of `agenticspec.session.RecordModelTurn`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_record_model_turn(outcome: &agenticspec_types::session::RecordModelTurnOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::RecordModelTurnOutcome::Recorded { model_turn_recorded, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "recorded");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ModelTurnRecorded");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_model_turn_recorded(model_turn_recorded, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::RecordModelTurnOutcome::NoSuchAgent { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "no-such-agent");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.UnknownAgent");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_unknown_agent(error, &mut body);
            404
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.RequestToolCall`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_request_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_request_tool_call(system, &value, taken))
}

/// `agenticspec.session.RequestToolCall` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_request_tool_call<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_request_tool_call(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.request_tool_call(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_request_tool_call(&outcome))
}

/// One declared outcome of `agenticspec.session.RequestToolCall`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_request_tool_call(outcome: &agenticspec_types::session::RequestToolCallOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::RequestToolCallOutcome::Requested { tool_call_requested, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "requested");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.ToolCallRequested");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_tool_call_requested(tool_call_requested, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.StartAgent`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_start_agent<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_start_agent(system, &value, taken))
}

/// `agenticspec.session.StartAgent` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_start_agent<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_start_agent(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.start_agent(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_start_agent(&outcome))
}

/// One declared outcome of `agenticspec.session.StartAgent`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_start_agent(outcome: &agenticspec_types::session::StartAgentOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::StartAgentOutcome::Started { agent_started, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "started");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.AgentStarted");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_agent_started(agent_started, &mut body);
            body.push('}');
            body.push(']');
            202
        }
    };
    body.push('}');
    (status, body)
}

/// `POST` `agenticspec.session.SubmitPrompt`: reads the declared input, runs the port, answers the declared outcome.
fn serve_agenticspec_session_submit_prompt<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, body: &[u8], taken: &mut Vec<agenticspec_system::SystemEvent>) -> http::Response
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    let text = match std::str::from_utf8(body) {
        Ok(text) => text,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not UTF-8: {error}"));
        }
    };
    let value = match json::parse(text) {
        Ok(value) => value,
        Err(error) => {
            return http::Response::refusal(400, &format!("the body is not JSON: {error}"));
        }
    };
    http::answer(run_agenticspec_session_submit_prompt(system, &value, taken))
}

/// `agenticspec.session.SubmitPrompt` from its input as a JSON value: decode, run the port, render the declared outcome.
///
/// The one path the `POST` route and [`handle`] share. A decoding failure is located under
/// `body`, as the route reports it. Every event published while the call runs is pushed onto
/// `taken`, in log order, whatever it answers; what was on the log before it is taken off
/// the log but not pushed.
fn run_agenticspec_session_submit_prompt<SessionIrBehaviors, SessionViewBehaviors>(system: &mut agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>, value: &json::Value, taken: &mut Vec<agenticspec_system::SystemEvent>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    // What is on the log before this call is not what this call published: it is delivered
    // and taken with the rest, and never pushed onto `taken`.
    let before = system.published().len();
    let input = match wire::decode_command_agenticspec_session_submit_prompt(value, "body") {
        Ok(input) => input,
        Err(error) => {
            // `400` and not `422`: this is a body the schema decides, which is the difference
            // between fixing a value and fixing a serialiser.
            return Err(entry::Refused::Input(format!("{error}")));
        }
    };
    let outcome = match system.session_ir.submit_prompt(input) {
        Ok(outcome) => outcome,
        Err(unmet) => return Err(entry::Refused::Unmet(format!("{unmet}"))),
    };
    // Deliver what this command published to every binding that reacts to it, then take it
    // off the log: a long-running server keeps nothing from one request to the next.
    let delivered = system.pump();
    taken.extend(system.take_published().into_iter().skip(before));
    if let Err(failure) = delivered {
        return Err(entry::Refused::Undelivered(format!("delivering what the command published: {failure}")));
    }
    Ok(answer_agenticspec_session_submit_prompt(&outcome))
}

/// One declared outcome of `agenticspec.session.SubmitPrompt`: the branch that was taken, every event it published in
/// publication order, the declared error where there is one, and that error's own payload —
/// with the status the contract declares for that branch.
fn answer_agenticspec_session_submit_prompt(outcome: &agenticspec_types::session::SubmitPromptOutcome) -> (u16, String) {
    let mut body = String::from("{");
    let status = match outcome {
        agenticspec_types::session::SubmitPromptOutcome::Submitted { prompt_submitted, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "submitted");
            json::member(&mut body, "published");
            body.push('[');
            body.push('{');
            json::member(&mut body, "event");
            json::push_text(&mut body, "agenticspec.session.PromptSubmitted");
            json::member(&mut body, "payload");
            wire::encode_event_agenticspec_session_prompt_submitted(prompt_submitted, &mut body);
            body.push('}');
            body.push(']');
            202
        }
        agenticspec_types::session::SubmitPromptOutcome::WrongState { error, .. } => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push('[');
            body.push(']');
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            json::member(&mut body, "payload");
            wire::encode_error_agenticspec_session_agent_state_conflict(error, &mut body);
            409
        }
        agenticspec_types::session::SubmitPromptOutcome::WrongStateUnknownInstance => {
            json::member(&mut body, "outcome");
            json::push_text(&mut body, "wrong-state");
            json::member(&mut body, "published");
            body.push_str("[]");
            json::member(&mut body, "error");
            json::push_text(&mut body, "agenticspec.session.AgentStateConflict");
            409
        }
    };
    body.push('}');
    (status, body)
}

/// `GET` `agenticspec.session.Agents` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agenticspec_session_agents<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match system.session_ir.agents() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_session_agents(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.session.Exposures` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agenticspec_session_exposures<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match system.session_ir.exposures() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_session_exposures(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.session.Sessions` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agenticspec_session_sessions<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match system.session_ir.sessions() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_session_sessions(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}

/// `GET` `agenticspec.session.ToolCalls` at `read_your_writes` consistency: every row the owed projection holds.
///
/// The one path the `GET` route and [`handle`] share.
fn run_agenticspec_session_tool_calls<SessionIrBehaviors, SessionViewBehaviors>(system: &agenticspec_system::System<SessionIrBehaviors, SessionViewBehaviors>) -> Result<(u16, String), entry::Refused>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    match system.session_ir.tool_calls() {
        Ok(rows) => {
            let mut body = String::from("{");
            json::member(&mut body, "rows");
            body.push('[');
            for (position, row) in rows.iter().enumerate() {
                if position > 0 {
                    body.push(',');
                }
                wire::encode_view_agenticspec_session_tool_calls(row, &mut body);
            }
            body.push(']');
            body.push('}');
            Ok((200, body))
        }
        Err(unmet) => Err(entry::Refused::Unmet(format!("{unmet}"))),
    }
}
