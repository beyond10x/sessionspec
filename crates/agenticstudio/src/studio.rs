//! The drafts of one studio, held in memory: each draft with the revisions of its architecture,
//! the patches applied to it and every application of a patch, applied or refused; the
//! sessions on the drafts and every tool call of a session; and the event channel of each draft
//! (`live`), on which each new revision and each answered or failed call is published.

use std::path::{Component, Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::time::{SystemTime, UNIX_EPOCH};

use agenticspec_patch::{Change, Patch, Refused};
use agenticspec_sim::check::Refusal;
use serde_json::{Number, Value, json};
use sha2::{Digest as _, Sha256};

use crate::live::{self, Event, Hub};
use crate::tools;
use crate::types::architecture::{
    self as a, ArchitectureId, ArchitectureRevisionId, Digest, EssPresence as APresence,
    PatchApplicationId, PatchApplicationsRow, PatchId, PatchRule, PatchVerb, PatchesRow,
    RevisionsRow,
};
use crate::types::session::{
    EssPresence as CPresence, StudioDraftDraftRevised, StudioDraftRevisionCause,
    StudioSessionCaller as Caller, StudioSessionSessionState as SessionState,
    StudioSessionSessionsRow as SessionsRow, StudioSessionTool as Tool,
    StudioSessionToolCallState as CallState, StudioSessionToolCallsRow as ToolCallsRow,
    StudioSharedDraftId, StudioSharedPatchApplicationRef, StudioSharedRevisionRef,
    StudioSharedSessionId, StudioSharedToolCallId,
};
use crate::types::studio::{
    ArchitectureRef, DraftId, DraftsRow, EssPresence as SPresence, Name, RevisionRef, State,
};

pub use crate::types::session::{
    StudioSessionCaller, StudioSessionSessionsRow, StudioSessionTool, StudioSessionToolCallsRow,
};

/// The directory under the root that holds the architecture documents a draft opens.
pub const ARCHITECTURES: &str = "architectures";
/// The format an architecture document names.
pub const ARCHITECTURE_FORMAT: &str = "agenticspec-architecture/1";

/// Why the studio refused a request; the name is the error a client reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudioError {
    /// `studio.draft.NoSuchDraft`.
    NoSuchDraft(String),
    /// `studio.draft.DraftNameTaken`.
    DraftNameTaken(String),
    /// The source is no architecture document under `architectures/`.
    NoSuchSource(String),
    /// The source is not an `agenticspec-architecture/1` document.
    NotAnArchitecture(String),
    /// `studio.session.NoSuchSession`.
    NoSuchSession(String),
    /// `studio.session.ClosedSession`.
    ClosedSession(String),
    /// `studio.session.DryRunOutsideApply`.
    DryRunOutsideApply,
    /// The draft has no revision of this number.
    NoSuchRevision(String, usize),
}

impl StudioError {
    pub fn name(&self) -> &'static str {
        match self {
            StudioError::NoSuchDraft(_) => "NoSuchDraft",
            StudioError::DraftNameTaken(_) => "DraftNameTaken",
            StudioError::NoSuchSource(_) => "NoSuchSource",
            StudioError::NotAnArchitecture(_) => "NotAnArchitecture",
            StudioError::NoSuchSession(_) => "NoSuchSession",
            StudioError::ClosedSession(_) => "ClosedSession",
            StudioError::DryRunOutsideApply => "DryRunOutsideApply",
            StudioError::NoSuchRevision(..) => "NoSuchRevision",
        }
    }

    pub fn message(&self) -> String {
        match self {
            StudioError::NoSuchDraft(id) => format!("no draft has the id {id}"),
            StudioError::DraftNameTaken(name) => format!("an open draft is named {name}"),
            StudioError::NoSuchSource(s) => {
                format!("{s} is no architecture document under {ARCHITECTURES}/")
            }
            StudioError::NotAnArchitecture(m) => m.clone(),
            StudioError::NoSuchSession(id) => format!("no session has the id {id}"),
            StudioError::ClosedSession(id) => format!("the session {id} is closed"),
            StudioError::DryRunOutsideApply => "only an apply runs dry".into(),
            StudioError::NoSuchRevision(id, n) => format!("the draft {id} has no revision {n}"),
        }
    }
}

/// What made a revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    /// The draft was opened at it.
    Opened,
    /// The patch application of this id committed it.
    Patch(String),
    /// The draft's source document changed on disk and was read again.
    SourceChanged,
}

impl Cause {
    pub fn name(&self) -> &'static str {
        match self {
            Cause::Opened => "Opened",
            Cause::Patch(_) => "Patch",
            Cause::SourceChanged => "SourceChanged",
        }
    }

    /// The patch application that committed the revision, if a patch did.
    pub fn application(&self) -> Option<&str> {
        match self {
            Cause::Patch(id) => Some(id),
            _ => None,
        }
    }
}

/// One revision of a draft's architecture, the document it froze, and what made it.
#[derive(Debug, Clone)]
pub struct Revision {
    pub row: RevisionsRow,
    pub document: Value,
    pub cause: Cause,
}

#[derive(Debug, Clone)]
pub struct Draft {
    pub row: DraftsRow,
    /// In order; the last is the current one.
    pub revisions: Vec<Revision>,
    pub patches: Vec<PatchesRow>,
    pub applications: Vec<PatchApplicationsRow>,
    /// The SHA-256 of the source file's bytes as last read; a change on disk is a new revision.
    pub source_digest: Option<String>,
}

impl Draft {
    pub fn current(&self) -> &Revision {
        // A draft is opened with its first revision and never loses one.
        &self.revisions[self.revisions.len() - 1]
    }
}

/// What applying a patch to a draft did.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// None when a rule's verb is no verb of `agenticspec-patch/1`: such a patch cannot be
    /// recorded, so neither can its application.
    pub patch: Option<PatchesRow>,
    pub application: Option<PatchApplicationsRow>,
    pub diff: Vec<Change>,
    /// The parts and links an applied patch changed; 0 for a refusal.
    pub changed_rows: usize,
    pub check: Vec<Refusal>,
    pub refusal: Option<Refused>,
    /// The revision an applied patch committed; none for a dry run or a refusal.
    pub revision: Option<RevisionsRow>,
}

/// A tool call of a session, as recorded, with its answer as JSON when it was answered.
#[derive(Debug, Clone)]
pub struct Call {
    pub row: ToolCallsRow,
    pub answer: Option<Value>,
}

pub struct Studio {
    root: PathBuf,
    drafts: Vec<Draft>,
    sessions: Vec<SessionsRow>,
    calls: Vec<ToolCallsRow>,
    /// The session each draft's page makes its calls in, `(draft, session)`, started on its first
    /// call.
    editors: Vec<(String, String)>,
    hub: Hub,
    seed: u128,
    minted: u64,
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A tool by its name, in any case: `query`, `apply`, `check`, `simulate`.
pub fn tool(name: &str) -> Option<Tool> {
    Some(match name.to_ascii_lowercase().as_str() {
        "query" => Tool::Query,
        "apply" => Tool::Apply,
        "check" => Tool::Check,
        "simulate" => Tool::Simulate,
        _ => return None,
    })
}

/// A caller by its name, in any case: `user` or `agent`.
pub fn caller(name: &str) -> Option<Caller> {
    Some(match name.to_ascii_lowercase().as_str() {
        "user" => Caller::User,
        "agent" => Caller::Agent,
        _ => return None,
    })
}

/// The name of a value of a generated enum, as it is written on the wire.
pub fn wire<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_value(v)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

fn number(n: usize) -> Number {
    Number::from(n as u64)
}

fn verb(name: &str) -> Option<PatchVerb> {
    Some(match name {
        "set" => PatchVerb::Set,
        "add" => PatchVerb::Add,
        "remove" => PatchVerb::Remove,
        "connect" => PatchVerb::Connect,
        "replicate" => PatchVerb::Replicate,
        "wrap" => PatchVerb::Wrap,
        "prepend" => PatchVerb::Prepend,
        "append" => PatchVerb::Append,
        "mark" => PatchVerb::Mark,
        "unmark" => PatchVerb::Unmark,
        _ => return None,
    })
}

impl Studio {
    /// A studio over the architecture documents under `root/architectures`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        Studio {
            root: root.into(),
            drafts: Vec::new(),
            sessions: Vec::new(),
            calls: Vec::new(),
            editors: Vec::new(),
            hub: Hub::default(),
            seed,
            minted: 0,
        }
    }

    /// The directory whose `architectures/` holds the documents a draft opens.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// A new identifier in the form of a UUID, unique within this process.
    fn mint(&mut self, kind: &str) -> String {
        self.minted += 1;
        let hash = Sha256::digest(format!("{kind}:{}:{}", self.seed, self.minted).as_bytes());
        let mut b: [u8; 16] = [0; 16];
        b.copy_from_slice(&hash[..16]);
        b[6] = (b[6] & 0x0f) | 0x80;
        b[8] = (b[8] & 0x3f) | 0x80;
        let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
        format!(
            "{}-{}-{}-{}-{}",
            &hex[..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..]
        )
    }

    /// The architecture documents under `architectures/`, by their path from the root, sorted.
    pub fn architectures(&self) -> Vec<String> {
        fn walk(dir: &Path, rel: &str, out: &mut Vec<String>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().into_owned();
                let path = e.path();
                let rel = format!("{rel}/{name}");
                if path.is_dir() {
                    walk(&path, &rel, out);
                } else if name.ends_with(".yaml") {
                    out.push(rel);
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.root.join(ARCHITECTURES), ARCHITECTURES, &mut out);
        out.sort();
        out
    }

    pub fn drafts(&self) -> &[Draft] {
        &self.drafts
    }

    pub fn draft(&self, id: &str) -> Result<&Draft, StudioError> {
        self.drafts
            .iter()
            .find(|d| d.row.draft_id.0 == id)
            .ok_or_else(|| StudioError::NoSuchDraft(id.to_string()))
    }

    fn name_taken(&self, name: &str) -> bool {
        self.drafts
            .iter()
            .any(|d| d.row.name.0 == name && d.row.state == State::Open)
    }

    /// Opens a draft of the architecture document `source` (a path under `architectures/`) at
    /// revision 1. Without a name the draft is named after its architecture, numbered when
    /// that name is taken.
    pub fn open(&mut self, source: &str, name: Option<&str>) -> Result<&Draft, StudioError> {
        let rel = Path::new(source);
        let inside = rel.components().all(|c| matches!(c, Component::Normal(_)))
            && rel.starts_with(ARCHITECTURES)
            && source.ends_with(".yaml");
        let path = self.root.join(rel);
        if !inside || !path.is_file() {
            return Err(StudioError::NoSuchSource(source.to_string()));
        }
        let bytes = std::fs::read(&path)
            .map_err(|e| StudioError::NotAnArchitecture(format!("{source}: {e}")))?;
        let document = read_architecture(source, &bytes).map_err(StudioError::NotAnArchitecture)?;
        let name = match name {
            Some(n) if self.name_taken(n) => return Err(StudioError::DraftNameTaken(n.into())),
            Some(n) => n.to_string(),
            None => {
                let base = document["architecture"]
                    .as_str()
                    .map(str::to_string)
                    .or_else(|| rel.file_stem().map(|s| s.to_string_lossy().into_owned()))
                    .unwrap_or_else(|| "draft".into());
                let mut n = base.clone();
                let mut i = 1;
                while self.name_taken(&n) {
                    i += 1;
                    n = format!("{base} {i}");
                }
                n
            }
        };
        let architecture = self.mint("architecture");
        let revision = self.mint("revision");
        let draft_id = self.mint("draft");
        let first = Revision {
            row: RevisionsRow {
                revision_id: ArchitectureRevisionId(revision.clone()),
                architecture_id: ArchitectureId(architecture.clone()),
                number: number(1),
                digest: Digest(agenticspec_patch::digest(&document)),
            },
            document,
            cause: Cause::Opened,
        };
        self.drafts.push(Draft {
            row: DraftsRow {
                draft_id: DraftId(draft_id),
                name: Name(name),
                architecture: ArchitectureRef(architecture),
                source: SPresence::Present(source.to_string()),
                opened_at_revision: SPresence::Present(RevisionRef(revision)),
                state: State::Open,
            },
            revisions: vec![first],
            patches: Vec::new(),
            applications: Vec::new(),
            source_digest: Some(sha256(&bytes)),
        });
        let last = self.drafts.len() - 1;
        Ok(&self.drafts[last])
    }

    /// Applies `patch` to the draft's current revision. Applied and not a dry run: a new
    /// revision. Otherwise the draft keeps its revision; the application is recorded either way
    /// when the patch can be.
    pub fn apply(
        &mut self,
        id: &str,
        patch: &Patch,
        dry_run: bool,
    ) -> Result<Outcome, StudioError> {
        let index = self
            .drafts
            .iter()
            .position(|d| d.row.draft_id.0 == id)
            .ok_or_else(|| StudioError::NoSuchDraft(id.to_string()))?;
        let current = self.drafts[index].current().clone();
        let result = agenticspec_patch::apply(&current.document, patch);

        let rules: Option<Vec<PatchRule>> = patch
            .rules
            .iter()
            .map(|r| {
                let v = verb(agenticspec_patch::verb_of(r)?)?;
                Some(PatchRule {
                    verb: v,
                    text: r.clone(),
                })
            })
            .collect();
        let patch_row = match rules {
            Some(rules) => {
                let row = PatchesRow {
                    patch_id: PatchId(self.mint("patch")),
                    name: patch.name.as_ref().map_or(APresence::Absent, |n| {
                        APresence::Present(a::Name(n.clone()))
                    }),
                    rules,
                    digest: Digest(patch.digest()),
                };
                Some(row)
            }
            None => None,
        };
        let architecture_id = ArchitectureId(self.drafts[index].row.architecture.0.clone());
        let mut outcome = Outcome {
            patch: patch_row.clone(),
            application: None,
            diff: Vec::new(),
            changed_rows: 0,
            check: Vec::new(),
            refusal: None,
            revision: None,
        };
        let mut application = patch_row.as_ref().map(|p| PatchApplicationsRow {
            application_id: PatchApplicationId(String::new()),
            architecture_id: architecture_id.clone(),
            patch_id: p.patch_id.clone(),
            dry_run,
            revision_id: APresence::Absent,
            changed_rows: APresence::Absent,
            rule_index: APresence::Absent,
            error: APresence::Absent,
            state: a::State::Requested,
        });
        match result {
            Ok(applied) => {
                outcome.diff = applied.diff;
                outcome.changed_rows = applied.changed_rows;
                if let Some(app) = application.as_mut() {
                    app.changed_rows = APresence::Present(number(applied.changed_rows));
                    app.state = a::State::Applied;
                }
                if !dry_run {
                    let number_next = self.drafts[index].revisions.len() + 1;
                    let row = RevisionsRow {
                        revision_id: ArchitectureRevisionId(self.mint("revision")),
                        architecture_id,
                        number: number(number_next),
                        digest: Digest(agenticspec_patch::digest(&applied.document)),
                    };
                    if let Some(app) = application.as_mut() {
                        app.revision_id = APresence::Present(row.revision_id.clone());
                    }
                    self.drafts[index].revisions.push(Revision {
                        row: row.clone(),
                        document: applied.document,
                        // The application's id is minted below, once it is recorded.
                        cause: Cause::Patch(String::new()),
                    });
                    outcome.revision = Some(row);
                }
            }
            Err(refused) => {
                outcome.diff = refused.diff.clone();
                outcome.check = refused.check.clone();
                if let Some(app) = application.as_mut() {
                    app.rule_index = refused
                        .rule
                        .map_or(APresence::Absent, |n| APresence::Present(number(n)));
                    app.error = APresence::Present(refused.error.name().to_string());
                    app.state = a::State::Refused;
                }
                outcome.refusal = Some(refused);
            }
        }
        if let (Some(mut app), Some(p)) = (application, patch_row) {
            app.application_id = PatchApplicationId(self.mint("application"));
            let draft = &mut self.drafts[index];
            if outcome.revision.is_some()
                && let Some(last) = draft.revisions.last_mut()
            {
                last.cause = Cause::Patch(app.application_id.0.clone());
            }
            draft.patches.push(p);
            draft.applications.push(app.clone());
            outcome.application = Some(app);
        }
        if outcome.revision.is_some() {
            self.publish_revision(index);
        }
        Ok(outcome)
    }

    /// Publishes the current revision of the draft at `index` on its channel.
    fn publish_revision(&mut self, index: usize) {
        let draft = &self.drafts[index];
        let current = draft.current();
        let event = StudioDraftDraftRevised {
            draft_id: StudioSharedDraftId(draft.row.draft_id.0.clone()),
            revision: StudioSharedRevisionRef(current.row.revision_id.0.clone()),
            number: current.row.number.clone(),
            cause: match current.cause {
                Cause::SourceChanged => StudioDraftRevisionCause::SourceChanged,
                _ => StudioDraftRevisionCause::Patch,
            },
        };
        let mut data = serde_json::to_value(&event).unwrap_or(Value::Null);
        data["digest"] = Value::from(current.row.digest.0.as_str());
        data["application"] = current.cause.application().map_or(Value::Null, Value::from);
        let id = draft.row.draft_id.0.clone();
        self.hub.publish(
            &id,
            &Event {
                name: live::DRAFT_REVISED,
                data,
            },
        );
    }

    /// A new subscriber of the event channel of draft `id`.
    pub fn subscribe(&mut self, id: &str) -> Result<Receiver<Event>, StudioError> {
        self.draft(id)?;
        Ok(self.hub.subscribe(id))
    }

    /// How many clients are subscribed to the channel of draft `id`.
    pub fn subscribers(&self, id: &str) -> usize {
        self.hub.subscribers(id)
    }

    /// Revision `number` (from 1) of draft `id`.
    pub fn revision(&self, id: &str, number: usize) -> Result<&Revision, StudioError> {
        let d = self.draft(id)?;
        number
            .checked_sub(1)
            .and_then(|i| d.revisions.get(i))
            .ok_or_else(|| StudioError::NoSuchRevision(id.to_string(), number))
    }

    /// Reads the source document of every open draft again. A draft whose file changed on disk
    /// into another architecture moves to a new revision of it, published on its channel. A file
    /// that is gone or that is no architecture document is left until it is one again. Answers
    /// the drafts that moved, with their new revision number.
    pub fn reload_sources(&mut self) -> Vec<(String, usize)> {
        let mut moved = Vec::new();
        for index in 0..self.drafts.len() {
            let draft = &self.drafts[index];
            if draft.row.state != State::Open {
                continue;
            }
            let SPresence::Present(source) = &draft.row.source else {
                continue;
            };
            let source = source.clone();
            let Ok(bytes) = std::fs::read(self.root.join(&source)) else {
                continue;
            };
            let digest = sha256(&bytes);
            if draft.source_digest.as_deref() == Some(digest.as_str()) {
                continue;
            }
            self.drafts[index].source_digest = Some(digest);
            let Ok(document) = read_architecture(&source, &bytes) else {
                continue;
            };
            let doc_digest = agenticspec_patch::digest(&document);
            if self.drafts[index].current().row.digest.0 == doc_digest {
                continue;
            }
            let number_next = self.drafts[index].revisions.len() + 1;
            let row = RevisionsRow {
                revision_id: ArchitectureRevisionId(self.mint("revision")),
                architecture_id: ArchitectureId(self.drafts[index].row.architecture.0.clone()),
                number: number(number_next),
                digest: Digest(doc_digest),
            };
            self.drafts[index].revisions.push(Revision {
                row,
                document,
                cause: Cause::SourceChanged,
            });
            self.publish_revision(index);
            moved.push((self.drafts[index].row.draft_id.0.clone(), number_next));
        }
        moved
    }

    pub fn sessions(&self) -> &[SessionsRow] {
        &self.sessions
    }

    pub fn session(&self, id: &str) -> Result<&SessionsRow, StudioError> {
        self.sessions
            .iter()
            .find(|s| s.session_id.0 == id)
            .ok_or_else(|| StudioError::NoSuchSession(id.to_string()))
    }

    /// The tool calls of session `id`, in the order they were made.
    pub fn calls(&self, id: &str) -> Result<Vec<&ToolCallsRow>, StudioError> {
        self.session(id)?;
        Ok(self.calls.iter().filter(|c| c.session_id.0 == id).collect())
    }

    /// `studio.session.StartSession`: a new open session on draft `draft_id`.
    pub fn start_session(&mut self, draft_id: &str) -> Result<&SessionsRow, StudioError> {
        self.draft(draft_id)?;
        let row = SessionsRow {
            session_id: StudioSharedSessionId(self.mint("session")),
            draft_id: StudioSharedDraftId(draft_id.to_string()),
            state: SessionState::Open,
        };
        self.sessions.push(row);
        let last = self.sessions.len() - 1;
        Ok(&self.sessions[last])
    }

    /// `studio.session.CloseSession`.
    pub fn close_session(&mut self, id: &str) -> Result<&SessionsRow, StudioError> {
        let i = self
            .sessions
            .iter()
            .position(|s| s.session_id.0 == id)
            .ok_or_else(|| StudioError::NoSuchSession(id.to_string()))?;
        if self.sessions[i].state == SessionState::Closed {
            return Err(StudioError::ClosedSession(id.to_string()));
        }
        self.sessions[i].state = SessionState::Closed;
        Ok(&self.sessions[i])
    }

    /// The session the page of draft `draft_id` makes its calls in; started on its first call.
    pub fn editor_session(&mut self, draft_id: &str) -> Result<String, StudioError> {
        self.draft(draft_id)?;
        if let Some((_, s)) = self.editors.iter().find(|(d, _)| d == draft_id)
            && self
                .session(s)
                .is_ok_and(|row| row.state == SessionState::Open)
        {
            return Ok(s.clone());
        }
        let id = self.start_session(draft_id)?.session_id.0.clone();
        self.editors.retain(|(d, _)| d != draft_id);
        self.editors.push((draft_id.to_string(), id.clone()));
        Ok(id)
    }

    /// `studio.session.CallTool`, then `AnswerToolCall` or `FailToolCall`: makes the call in
    /// session `session_id` on its draft and answers it. `arguments` is the call's input:
    /// `{"selector"}` for a query, `{"patch", "name"?}` for an apply (the patch as text or as a
    /// patch document), nothing for a check, `{"budget"?, "horizon"?, "profile"?}` for a
    /// simulate. An apply that the patch refuses is answered; a call fails only when it could not
    /// be carried out. The answered or failed call is published on the draft's channel.
    pub fn call(
        &mut self,
        session_id: &str,
        tool: Tool,
        caller: Caller,
        arguments: &Value,
        dry_run: bool,
    ) -> Result<Call, StudioError> {
        if dry_run && tool != Tool::Apply {
            return Err(StudioError::DryRunOutsideApply);
        }
        let session = self.session(session_id)?;
        if session.state == SessionState::Closed {
            return Err(StudioError::ClosedSession(session_id.to_string()));
        }
        let draft_id = session.draft_id.0.clone();
        let index = self
            .drafts
            .iter()
            .position(|d| d.row.draft_id.0 == draft_id)
            .ok_or_else(|| StudioError::NoSuchDraft(draft_id.clone()))?;
        let call_id = self.mint("call");
        let arguments_text = if arguments.is_null() {
            "{}".to_string()
        } else {
            arguments.to_string()
        };
        self.calls.push(ToolCallsRow {
            call_id: StudioSharedToolCallId(call_id.clone()),
            session_id: StudioSharedSessionId(session_id.to_string()),
            tool: tool.clone(),
            caller: caller.clone(),
            arguments: arguments_text,
            dry_run,
            application: CPresence::Absent,
            evaluation_id: CPresence::Absent,
            answer: CPresence::Absent,
            failure: CPresence::Absent,
            state: CallState::Running,
        });
        let result: Result<(Value, Option<String>), String> = match tool {
            Tool::Query => match arguments["selector"].as_str() {
                Some(selector) => tools::query(&self.drafts[index].current().document, selector)
                    .map(|a| (a, None)),
                None => {
                    Err("InvalidArguments: a query takes {\"selector\": \"<rule body>\"}".into())
                }
            },
            Tool::Check => Ok((tools::check(&self.drafts[index].current().document), None)),
            Tool::Simulate => {
                let current = self.drafts[index].current();
                let n = current.row.number.as_u64().unwrap_or(0) as usize;
                tools::simulate(&self.root, n, &current.document, arguments).map(|a| (a, None))
            }
            Tool::Apply => self.apply_call(&draft_id, arguments, dry_run),
        };
        let i = self
            .calls
            .iter()
            .position(|c| c.call_id.0 == call_id)
            .unwrap_or(self.calls.len() - 1);
        let (event, answer) = match result {
            Ok((answer, application)) => {
                let row = &mut self.calls[i];
                row.answer = CPresence::Present(answer.to_string());
                if let Some(app) = application {
                    row.application = CPresence::Present(StudioSharedPatchApplicationRef(app));
                }
                row.state = CallState::Answered;
                let data = json!({
                    "call_id": call_id,
                    "session_id": session_id,
                    "draft_id": draft_id,
                    "tool": wire(&row.tool),
                    "caller": wire(&row.caller),
                    "application": serde_json::to_value(&row.application).unwrap_or(Value::Null),
                    "summary": answer["summary"].as_str().and_then(|s| s.lines().next()),
                });
                (
                    Event {
                        name: live::CALL_ANSWERED,
                        data,
                    },
                    Some(answer),
                )
            }
            Err(failure) => {
                let row = &mut self.calls[i];
                row.failure = CPresence::Present(failure.clone());
                row.state = CallState::Failed;
                let data = json!({
                    "call_id": call_id,
                    "session_id": session_id,
                    "draft_id": draft_id,
                    "tool": wire(&row.tool),
                    "caller": wire(&row.caller),
                    "failure": failure,
                });
                (
                    Event {
                        name: live::CALL_FAILED,
                        data,
                    },
                    None,
                )
            }
        };
        self.hub.publish(&draft_id, &event);
        Ok(Call {
            row: self.calls[i].clone(),
            answer,
        })
    }

    /// An apply in a call: the answer and the patch application it recorded.
    fn apply_call(
        &mut self,
        draft_id: &str,
        arguments: &Value,
        dry_run: bool,
    ) -> Result<(Value, Option<String>), String> {
        let patch = match &arguments["patch"] {
            Value::String(text) => Patch::from_text(text).map(|mut p| {
                p.name = arguments["name"].as_str().map(str::to_string);
                p
            }),
            v @ Value::Object(_) => Patch::from_document(v),
            _ => {
                return Err(
                    "InvalidArguments: an apply takes {\"patch\": \"<rules>\"} or a patch document as patch"
                        .into(),
                );
            }
        };
        let patch = match patch {
            Ok(p) => p,
            Err(refused) => return Ok((tools::unreadable(&refused), None)),
        };
        let outcome = self
            .apply(draft_id, &patch, dry_run)
            .map_err(|e| format!("{}: {}", e.name(), e.message()))?;
        let application = outcome
            .application
            .as_ref()
            .map(|a| a.application_id.0.clone());
        Ok((tools::apply(&outcome), application))
    }
}

/// The architecture document in `bytes`, read from `source`.
fn read_architecture(source: &str, bytes: &[u8]) -> Result<Value, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("{source}: {e}"))?;
    let document = agenticspec_sim::doc::parse_yaml(text).map_err(|e| format!("{source}: {e}"))?;
    if document["format"].as_str() != Some(ARCHITECTURE_FORMAT) {
        return Err(format!("{source} is not an {ARCHITECTURE_FORMAT} document"));
    }
    Ok(document)
}
