//! agenticstudio-mcp: the agent channel of agenticstudio. An MCP server over stdio (JSON-RPC 2.0,
//! one message per line) whose four tools, `query`, `apply`, `check` and `simulate`, are tool
//! calls with caller `Agent` in a studio session on one draft: each request is sent as the agent
//! (`X-Studio-Caller: Agent`), which the studio records as the caller. The calls go to the tool-call
//! route of `agenticstudio serve` (`POST /api/sessions/<id>/calls`), so every client open on the
//! draft sees each change live on the draft's event channel.
//!
//! - `Backend`: how the server reaches the studio: `Http` to a running `agenticstudio serve`,
//!   `Local` to a studio in the same process.
//! - `Server`: the MCP messages, the tools and the session on the draft.

use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use agenticstudio::http::{CALLER_HEADER, Request};
use agenticstudio::routes::answer;
use agenticstudio::studio::Studio;
use serde_json::{Value, json};

/// The protocol versions this server speaks, newest first; it answers the client's when it is
/// one of them, else the newest.
pub const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

/// The studio a server drives when it names none.
pub const DEFAULT_STUDIO: &str = "http://127.0.0.1:7431";

/// How the server reaches the studio: one request, answered with its status and JSON body.
pub trait Backend {
    fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<(u16, Value), String>;
}

/// A running `agenticstudio serve`, over HTTP/1.1 on its host and port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Http {
    pub host: String,
    pub port: u16,
}

/// The longest one call may take, a simulate included.
pub const TIMEOUT: Duration = Duration::from_secs(120);

impl Http {
    /// `http://<host>:<port>[/]`; the port is 80 when none is given.
    pub fn parse(url: &str) -> Result<Http, String> {
        let rest = url
            .strip_prefix("http://")
            .ok_or_else(|| format!("{url}: the studio's address starts with http://"))?;
        let authority = rest.split('/').next().unwrap_or("");
        let (host, port) = match authority.rsplit_once(':') {
            Some((h, p)) => (
                h,
                p.parse::<u16>()
                    .map_err(|_| format!("{url}: {p} is no port"))?,
            ),
            None => (authority, 80),
        };
        if host.is_empty() {
            return Err(format!("{url}: no host"));
        }
        Ok(Http {
            host: host.to_string(),
            port,
        })
    }
}

impl Backend for Http {
    fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<(u16, Value), String> {
        let at = format!("{}:{}", self.host, self.port);
        let mut stream = TcpStream::connect(&at).map_err(|e| {
            format!(
                "cannot reach the studio at http://{at}/: {e}; start it with `agenticstudio serve`"
            )
        })?;
        let _ = stream.set_read_timeout(Some(TIMEOUT));
        let _ = stream.set_write_timeout(Some(TIMEOUT));
        let body = body.map(Value::to_string).unwrap_or_default();
        let head = format!(
            "{method} {path} HTTP/1.1\r\nHost: {at}\r\n{CALLER_HEADER}: Agent\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream
            .write_all(head.as_bytes())
            .and_then(|()| stream.write_all(body.as_bytes()))
            .map_err(|e| format!("the studio at http://{at}/: {e}"))?;
        let mut raw = Vec::new();
        stream
            .read_to_end(&mut raw)
            .map_err(|e| format!("the studio at http://{at}/: {e}"))?;
        let text = String::from_utf8_lossy(&raw);
        let (head, body) = text
            .split_once("\r\n\r\n")
            .ok_or_else(|| format!("the studio at http://{at}/ answered no HTTP"))?;
        let status = head
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or_else(|| format!("the studio at http://{at}/ answered no status"))?;
        let value = serde_json::from_str(body)
            .map_err(|e| format!("the studio at http://{at}/ answered no JSON ({status}): {e}"))?;
        Ok((status, value))
    }
}

/// A studio in the same process, answered by its routes without a socket.
#[derive(Clone)]
pub struct Local(pub Arc<Mutex<Studio>>);

impl Backend for Local {
    fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<&Value>,
    ) -> Result<(u16, Value), String> {
        let (path, query) = path.split_once('?').unwrap_or((path, ""));
        let request = Request {
            method: method.to_string(),
            path: path.to_string(),
            query: query.to_string(),
            body: body.map(|b| b.to_string().into_bytes()).unwrap_or_default(),
            caller: Some("Agent".to_string()),
        };
        let r = answer(&self.0, &request);
        let value = serde_json::from_slice(&r.body).map_err(|e| e.to_string())?;
        Ok((r.status, value))
    }
}

/// The draft a server works on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The draft of this id.
    Draft(String),
    /// A new draft of this architecture document (`architectures/<name>.yaml`), opened at the
    /// first call.
    Open(String),
    /// The draft opened last of those still open, at the first call.
    Newest,
}

const SELECTORS: &str = "A selector is a rule body in the query language: atoms joined by \
commas, variables in capitals, comparisons such as `C > 4`. The relations and their fields: \
part{id, block, count}: each part, its name, its block, its count (1 when the document gives \
none); setting{part, name, value}: each other key of a part (harness, role, acts_for, capacity, \
...), one fact per element of a list; link{index, from, to, carries, over}: each link, numbered \
from 0 in document order, over is null when absent; marker{path, value}: each marked place, \
such as parts.worker.count, with its value. A duration such as 10m is held in milliseconds and \
compares with duration literals.";

const VERBS: &str = "A patch is a list of rules, each ending with `.`: a transform, then `:-` and \
a selector; the transform runs once per answer of the selector, a transform without a selector \
once. The rules run in order, each on what the ones before it left, then the structural check \
runs on the result. Transforms: set(P, field, value) sets a field of part P (a part name), or \
of link P when P is a link index; on a marked place it sets the value and keeps the range. \
add(part{id: \"x\", block: \"agent\", ...}) adds a part (id and block required, every other \
field becomes a setting). add(link{from: A, to: B, carries: \"c\", over: C}) adds a link (over \
optional). A value is a string, a whole number, true, false, a duration such as 5m, or a \
variable the selector binds. remove, connect, replicate, wrap, prepend, append, mark and unmark \
are verbs of the language that are refused as not applied yet.";

/// The four tools, as `tools/list` answers them.
pub fn tools() -> Value {
    json!([
        {
            "name": "query",
            "title": "Query the draft",
            "description": format!(
                "Run a selector over the facts of the draft's current revision and answer the \
                 matching rows, one object per row keyed by the selector's variables. Changes \
                 nothing. {SELECTORS} Example: part{{id: P, block: \"agent\"}}."
            ),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "selector": {"type": "string", "description": "a rule body, such as part{id: P, block: \"agent\"}"}
                },
                "required": ["selector"],
                "additionalProperties": false
            },
            "annotations": {"readOnlyHint": true}
        },
        {
            "name": "apply",
            "title": "Apply a patch to the draft",
            "description": format!(
                "Apply a patch to the draft as its next revision, or with dry_run only answer \
                 what it would change. The answer carries the diff and the check result. If a \
                 rule or the check refuses, nothing changes and the answer names the rule's \
                 position (from 1) and the error, or the check's rule and the part it names. \
                 {VERBS} {SELECTORS} Example: set(P, count, 3) :- part{{id: P, block: \"agent\", \
                 count: 12}}."
            ),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "patch": {"type": "string", "description": "the rules of an agenticspec-patch/1 patch"},
                    "dry_run": {"type": "boolean", "description": "answer what would change and commit nothing", "default": false}
                },
                "required": ["patch"],
                "additionalProperties": false
            }
        },
        {
            "name": "check",
            "title": "Check the draft",
            "description": "Run the structural check on the draft's current revision: it passes, or each refusal names its rule, the part and why. Changes nothing.",
            "inputSchema": {"type": "object", "properties": {}, "additionalProperties": false},
            "annotations": {"readOnlyHint": true}
        },
        {
            "name": "simulate",
            "title": "Simulate the draft",
            "description": "Run the simulator on the draft's current revision at its marked values for `budget` seeds over `horizon` and answer the mean of each KPI with its unit, which direction is better and what it measures, and the KPIs per part. Changes nothing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "budget": {"type": "integer", "minimum": 1, "maximum": 16, "default": 4, "description": "seeds to run"},
                    "horizon": {"type": "string", "default": "24h", "description": "simulated time, such as 8h or 24h"}
                },
                "additionalProperties": false
            },
            "annotations": {"readOnlyHint": true}
        }
    ])
}

/// An MCP server on one draft of a studio.
pub struct Server<B: Backend> {
    backend: B,
    target: Target,
    draft: Option<String>,
    session: Option<String>,
}

fn rpc_error(id: &Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

fn result(id: &Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

/// A tool's result: its text, the answer as structured content, and whether it is an error.
fn tool_result(text: String, structured: Option<Value>, error: bool) -> Value {
    let mut r = json!({"content": [{"type": "text", "text": text}], "isError": error});
    if let Some(s) = structured {
        r["structuredContent"] = s;
    }
    r
}

/// The text an answer is told in: its summary, then what an agent reads next.
fn told(tool: &str, answer: &Value) -> String {
    let summary = answer["summary"].as_str().unwrap_or_default().to_string();
    let detail = match tool {
        "query" => json!({"rows": answer["rows"]}),
        "apply" => json!({
            "outcome": answer["outcome"],
            "revision": answer["revision"]["number"],
            "diff": answer["diff"],
            "refusal": answer["refusal"],
            "check": answer["check"],
        }),
        "check" => answer["check"].clone(),
        _ => return summary,
    };
    format!("{summary}\n{detail}")
}

impl<B: Backend> Server<B> {
    pub fn new(backend: B, target: Target) -> Self {
        Server {
            backend,
            target,
            draft: None,
            session: None,
        }
    }

    /// The session the calls are made in, once the first call started it.
    pub fn session(&self) -> Option<&str> {
        self.session.as_deref()
    }

    /// The draft the server works on, once the first call found it.
    pub fn draft(&self) -> Option<&str> {
        self.draft.as_deref()
    }

    fn studio_error(v: &Value, status: u16) -> String {
        match (v["error"].as_str(), v["message"].as_str()) {
            (Some(e), Some(m)) => format!("{e}: {m}"),
            _ => format!("the studio answered {status}: {v}"),
        }
    }

    fn find_draft(&mut self) -> Result<String, String> {
        if let Some(d) = &self.draft {
            return Ok(d.clone());
        }
        let id = match &self.target {
            Target::Draft(id) => id.clone(),
            Target::Open(source) => {
                let (status, v) = self.backend.request(
                    "POST",
                    "/api/drafts",
                    Some(&json!({"source": source})),
                )?;
                if status != 201 {
                    return Err(Self::studio_error(&v, status));
                }
                v["draft"]["draft_id"]
                    .as_str()
                    .ok_or("the studio answered no draft id")?
                    .to_string()
            }
            Target::Newest => {
                let (status, v) = self.backend.request("GET", "/api/drafts", None)?;
                if status != 200 {
                    return Err(Self::studio_error(&v, status));
                }
                v["drafts"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .rev()
                    .find(|d| d["state"] == "Open")
                    .and_then(|d| d["draft_id"].as_str())
                    .ok_or("NoDraft: no draft is open in the studio; open one on its page, or start agenticstudio-mcp with --open architectures/<name>.yaml")?
                    .to_string()
            }
        };
        self.draft = Some(id.clone());
        Ok(id)
    }

    fn start_session(&mut self) -> Result<String, String> {
        if let Some(s) = &self.session {
            return Ok(s.clone());
        }
        let draft = self.find_draft()?;
        let (status, v) =
            self.backend
                .request("POST", "/api/sessions", Some(&json!({"draft_id": draft})))?;
        if status != 201 {
            return Err(Self::studio_error(&v, status));
        }
        let id = v["session"]["session_id"]
            .as_str()
            .ok_or("the studio answered no session id")?
            .to_string();
        self.session = Some(id.clone());
        Ok(id)
    }

    /// Closes the session, if one was started.
    pub fn close(&mut self) {
        if let Some(s) = self.session.take() {
            let _ = self
                .backend
                .request("POST", &format!("/api/sessions/{s}/close"), None);
        }
    }

    /// One tool call in the session: the result `tools/call` answers.
    fn call(&mut self, name: &str, arguments: &Value) -> Value {
        let (arguments, dry_run) = match name {
            "query" => match arguments["selector"].as_str() {
                Some(s) => (json!({"selector": s}), false),
                None => {
                    return tool_result("query takes a selector".into(), None, true);
                }
            },
            "apply" => match arguments["patch"].as_str() {
                Some(p) => (
                    json!({"patch": p}),
                    arguments["dry_run"].as_bool().unwrap_or(false),
                ),
                None => {
                    return tool_result(
                        "apply takes a patch: its rules as text".into(),
                        None,
                        true,
                    );
                }
            },
            "check" => (json!({}), false),
            _ => {
                let mut a = json!({});
                for k in ["budget", "horizon"] {
                    if !arguments[k].is_null() {
                        a[k] = arguments[k].clone();
                    }
                }
                (a, false)
            }
        };
        let body = json!({"tool": name, "arguments": arguments, "dry_run": dry_run});
        let mut retried = false;
        loop {
            let session = match self.start_session() {
                Ok(s) => s,
                Err(e) => return tool_result(e, None, true),
            };
            let (status, v) = match self.backend.request(
                "POST",
                &format!("/api/sessions/{session}/calls"),
                Some(&body),
            ) {
                Ok(r) => r,
                Err(e) => return tool_result(e, None, true),
            };
            if status == 200 {
                let call = &v["call"];
                if call["state"] == "Answered" {
                    let answer = call["answer"].clone();
                    return tool_result(told(name, &answer), Some(answer), false);
                }
                let failure = call["failure"].as_str().unwrap_or("the call failed");
                return tool_result(format!("Failed: {failure}"), None, true);
            }
            let gone = matches!(v["error"].as_str(), Some("NoSuchSession" | "ClosedSession"));
            if gone && !retried {
                retried = true;
                self.session = None;
                continue;
            }
            return tool_result(Self::studio_error(&v, status), None, true);
        }
    }

    /// The answer to one message, or none for a notification or a response.
    pub fn handle(&mut self, line: &str) -> Option<Value> {
        let message: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(e) => {
                return Some(rpc_error(
                    &Value::Null,
                    -32700,
                    &format!("parse error: {e}"),
                ));
            }
        };
        let Some(method) = message["method"].as_str() else {
            if message.get("id").is_some() {
                // A response to a request of ours: this server sends none.
                return None;
            }
            return Some(rpc_error(&Value::Null, -32600, "no method"));
        };
        let id = message.get("id")?.clone();
        let params = &message["params"];
        Some(match method {
            "initialize" => {
                let asked = params["protocolVersion"].as_str().unwrap_or("");
                let version = PROTOCOL_VERSIONS
                    .iter()
                    .find(|v| **v == asked)
                    .unwrap_or(&PROTOCOL_VERSIONS[0]);
                result(
                    &id,
                    json!({
                        "protocolVersion": version,
                        "capabilities": {"tools": {"listChanged": false}},
                        "serverInfo": {"name": "agenticstudio-mcp", "version": env!("CARGO_PKG_VERSION")},
                        "instructions": "Drives one draft of agenticstudio, an architecture of agenticspec building blocks: query its facts with a selector, apply a patch (dry_run first to see the diff and the check), check it and simulate it. Every call is recorded in a studio session with caller Agent, and every client open on the draft sees each change live."
                    }),
                )
            }
            "ping" => result(&id, json!({})),
            "tools/list" => result(&id, json!({"tools": tools()})),
            "tools/call" => {
                let name = params["name"].as_str().unwrap_or("");
                if !["query", "apply", "check", "simulate"].contains(&name) {
                    return Some(rpc_error(
                        &id,
                        -32602,
                        &format!("no tool {name}; the tools are query, apply, check and simulate"),
                    ));
                }
                let arguments = match &params["arguments"] {
                    Value::Null => json!({}),
                    v => v.clone(),
                };
                result(&id, self.call(name, &arguments))
            }
            other => rpc_error(&id, -32601, &format!("no method {other}")),
        })
    }

    /// Answers every line of `input` on `output`, one line each, until `input` ends; then closes
    /// the session.
    pub fn run(&mut self, input: impl BufRead, mut output: impl Write) -> io::Result<()> {
        for line in input.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Some(answer) = self.handle(&line) {
                writeln!(output, "{answer}")?;
                output.flush()?;
            }
        }
        self.close();
        Ok(())
    }
}
