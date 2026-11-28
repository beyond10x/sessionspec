// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! Ephemeral generated server. Durable storage and production authentication remain ports.
use clap::Parser;

#[derive(Parser)]
struct Options {
#[arg(long, default_value = "127.0.0.1:8080")]
listen: String,
#[arg(long, default_value = "none", value_parser = ["none", "actor-header"])]
callers: String,
#[arg(long = "static")]
static_root: Option<std::path::PathBuf>,
}

fn main() -> std::process::ExitCode {
let options = Options::parse();
let ports = agenticspec_server::memory::MemoryPorts::default();
let mut system = agenticspec_system::System::new(session_ir::SessionIr::new(agenticspec_types::behaviour::Generated::new(ports.clone())), session_view::SessionView::new(agenticspec_types::behaviour::Generated::new(ports.clone())));
let mode = options.callers;
eprintln!("{}", if mode == "actor-header" { r#"{"format":"ess/1","callers":"actor-header","demonstration":true}"# } else { r#"{"format":"ess/1","callers":"none"}"# });
match agenticspec_server::session_ir::serve_with_static(&mut system, &options.listen, |request| { if mode != "actor-header" { return None; } let mut authorization = request.headers.iter().filter(|(key, _)| key.eq_ignore_ascii_case("Authorization")); let header = authorization.next()?.1.strip_prefix("Actor ")?; if authorization.next().is_some() { return None; } let actor = match header { "AgentLoop" => "agenticspec.session.AgentLoop",
"Delegate" => "agenticspec.session.Delegate",
"HarnessRuntime" => "agenticspec.session.HarnessRuntime",
"Human" => "agenticspec.session.Human",
"PeerSession" => "agenticspec.session.PeerSession",
"PermissionAuthority" => "agenticspec.session.PermissionAuthority",
"Redactor" => "agenticspec.session.Redactor",
"ToolProvider" => "agenticspec.session.ToolProvider",
"Viewer" => "agenticspec.view.Viewer",
 other => other }; agenticspec_types::actor::Actor::ALL.iter().find(|candidate| candidate.name() == actor).map(|actor| agenticspec_types::actor::Caller { actor: *actor }) }, options.static_root.as_deref()) { Ok(()) => std::process::ExitCode::SUCCESS, Err(error) => { eprintln!("{error}"); std::process::ExitCode::FAILURE } }
}
