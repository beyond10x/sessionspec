// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! Session — `agenticspec.session`.
//!
//! An agent session of any harness, as actors issuing commands. An adapter lifts a vendor transcript into these commands; replaying them yields the events and the state the KPIs read. Content is never held: a prompt, a tool input or a tool output is a digest and a length.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// The states of `agenticspec.session.Agent`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Agent<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// `Ended`.
    Ended,
    /// `Idle`.
    Idle,
    /// `Working`.
    Working,
}

/// AgentId — `agenticspec.session.AgentId`: a distinct wrapper around `String`.
///
/// One agent loop in a session; the main loop or a sub-agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentId(pub String);

/// AgentRole — `agenticspec.session.AgentRole`: one of a closed set of names.
///
/// SideCall is a model call of the harness outside every agent's conversation (a title, a summary, a quota probe): the wire tells it by a call with no tools and no history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRole {
    /// `Main`.
    Main,
    /// `SubAgent`.
    SubAgent,
    /// `SideCall`.
    SideCall,
}

/// Attribution — `agenticspec.session.Attribution`: one of a closed set of names.
///
/// How the author was established. Known from a structured field of the transcript, Inferred from a text prefix, Unknown otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribution {
    /// `Known`.
    Known,
    /// `Inferred`.
    Inferred,
    /// `Unknown`.
    Unknown,
}

/// Content — `agenticspec.session.Content`.
///
/// Content without the content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    /// `digest` — `String`.
    pub digest: String,
    /// `bytes` — `Integer`.
    pub bytes: i64,
}

/// Decider — `agenticspec.session.Decider`: one of a closed set of names.
///
/// What decided a tool call. NotRecorded where the transcript carries no decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decider {
    /// `PermissionMode`.
    PermissionMode,
    /// `Classifier`.
    Classifier,
    /// `Hook`.
    Hook,
    /// `Human`.
    Human,
    /// `SafetyCheck`.
    SafetyCheck,
    /// `Rule`.
    Rule,
    /// `NotRecorded`.
    NotRecorded,
}

/// Decision — `agenticspec.session.Decision`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// `Accept`.
    Accept,
    /// `Refuse`.
    Refuse,
}

/// The states of `agenticspec.session.Exposure`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Exposure<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureState {
    /// `Recorded`.
    Recorded,
}

/// ExposureChannel — `agenticspec.session.ExposureChannel`: one of a closed set of names.
///
/// Where a secret or a personal datum appeared. ToolInput is the severe one when the tool's category is Network, Message or Execute: the value left the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureChannel {
    /// `Prompt`.
    Prompt,
    /// `ToolInput`.
    ToolInput,
    /// `ToolOutput`.
    ToolOutput,
    /// `AgentText`.
    AgentText,
}

/// ExposureClass — `agenticspec.session.ExposureClass`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExposureClass {
    /// `Secret`.
    Secret,
    /// `PersonalData`.
    PersonalData,
}

/// ExposureId — `agenticspec.session.ExposureId`: a distinct wrapper around `String`.
///
/// One finding of the redaction pass; the adapter mints it from the record and the match position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposureId(pub String);

/// Harness — `agenticspec.session.Harness`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Harness {
    /// `ClaudeCode`.
    ClaudeCode,
    /// `Codex`.
    Codex,
    /// `Other`.
    Other,
}

/// IncompleteReason — `agenticspec.session.IncompleteReason`: one of a closed set of names.
///
/// Why a model turn on the wire ended before its answer was whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncompleteReason {
    /// `client_disconnected`.
    ClientDisconnected,
    /// `upstream_cut`.
    UpstreamCut,
    /// `wire_error`.
    WireError,
    /// `refused`.
    Refused,
}

/// PromptAuthor — `agenticspec.session.PromptAuthor`: one of a closed set of names.
///
/// Who wrote a prompt. Delegate is a session acting for the human; Harness covers task notifications, compaction summaries and auto-continuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptAuthor {
    /// `Human`.
    Human,
    /// `Delegate`.
    Delegate,
    /// `PeerSession`.
    PeerSession,
    /// `ParentAgent`.
    ParentAgent,
    /// `Harness`.
    Harness,
    /// `Hook`.
    Hook,
}

/// PromptClass — `agenticspec.session.PromptClass`: one of a closed set of names.
///
/// What a prompt asks, from rules over its text run on the host before redaction; Unknown when no rule matched or the author is the harness or a hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptClass {
    /// `Task`.
    Task,
    /// `Correction`.
    Correction,
    /// `Approval`.
    Approval,
    /// `Question`.
    Question,
    /// `Continue`.
    Continue,
    /// `Unknown`.
    Unknown,
}

/// The states of `agenticspec.session.Session`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `Session<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// `Ended`.
    Ended,
    /// `Open`.
    Open,
}

/// SessionId — `agenticspec.session.SessionId`: a distinct wrapper around `String`.
///
/// The harness's own session identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionId(pub String);

/// Source — `agenticspec.session.Source`: one of a closed set of names.
///
/// Where a record was observed: a harness transcript on disk, or a model call on a gateway's wire (`design/interaction-records.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// `Disk`.
    Disk,
    /// `Wire`.
    Wire,
}

/// The states of `agenticspec.session.ToolCall`, as runtime values.
///
/// Synthesised from the lifecycle, so the two cannot disagree. Which *moves* are legal is not
/// carried here — it is carried by `ToolCall<S>`, where an undeclared move does not compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCallState {
    /// `Accepted`.
    Accepted,
    /// `Failed`.
    Failed,
    /// `Refused`.
    Refused,
    /// `Requested`.
    Requested,
    /// `Succeeded`.
    Succeeded,
}

/// ToolCallId — `agenticspec.session.ToolCallId`: a distinct wrapper around `String`.
///
/// The harness's identifier of one tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallId(pub String);

/// ToolCategory — `agenticspec.session.ToolCategory`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCategory {
    /// `Read`.
    Read,
    /// `Search`.
    Search,
    /// `Edit`.
    Edit,
    /// `Execute`.
    Execute,
    /// `Network`.
    Network,
    /// `Spawn`.
    Spawn,
    /// `Message`.
    Message,
    /// `Plan`.
    Plan,
    /// `Other`.
    Other,
}

/// Usage — `agenticspec.session.Usage`.
///
/// Token usage of one model message, as the harness reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Usage {
    /// `uncached_input` — `Integer`.
    pub uncached_input: i64,
    /// `cache_read` — `Integer`.
    pub cache_read: i64,
    /// `cache_write_5m` — `Integer`.
    pub cache_write_5m: i64,
    /// `cache_write_1h` — `Integer`.
    pub cache_write_1h: i64,
    /// `output` — `Integer`.
    pub output: i64,
}

/// Verdict — `agenticspec.session.Verdict`: one of a closed set of names.
///
/// Ok is a known fixture or allowed value, Gap a confirmed exposure, Unk a match nobody confirmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// `Ok`.
    Ok,
    /// `Gap`.
    Gap,
    /// `Unk`.
    Unk,
}

/// What Agent — `agenticspec.session.Agent` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Agent<S>`], and at a boundary by [`AgentSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentData {
    /// The identity: `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `session_id` — `agenticspec.session.SessionId`.
    ///
    /// Carries `agents`: `agenticspec.session.Session` owns many `agenticspec.session.Agent`.
    pub session_id: SessionId,
    /// `parent_agent_id` — `Optional<agenticspec.session.AgentId>`.
    ///
    /// Carries `parent`: `agenticspec.session.Agent` references one `agenticspec.session.Agent`.
    pub parent_agent_id: Option<AgentId>,
    /// `role` — `agenticspec.session.AgentRole`.
    pub role: AgentRole,
    /// `model` — `Optional<String>`.
    pub model: Option<String>,
    /// `context_tokens` — `Integer`.
    pub context_tokens: i64,
}

/// The states of `agenticspec.session.Agent`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](agent_state::Marker), so [`Agent<S>`](Agent) can only ever rest in a real state.
pub mod agent_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Ended {}
        impl Sealed for super::Idle {}
        impl Sealed for super::Working {}
    }

    /// A declared state of `Agent`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::AgentState;
    }

    /// `Ended`. Terminal: an instance may rest here forever.
    pub struct Ended;

    impl Marker for Ended {
        const STATE: super::AgentState = super::AgentState::Ended;
    }

    /// `Idle`. Where a new instance starts.
    pub struct Idle;

    impl Marker for Idle {
        const STATE: super::AgentState = super::AgentState::Idle;
    }

    /// `Working`.
    pub struct Working;

    impl Marker for Working {
        const STATE: super::AgentState = super::AgentState::Working;
    }
}

/// Agent — `agenticspec.session.Agent` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Idle`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`AgentSnapshot`]
/// and [`AgentSnapshot::refine`].
pub struct Agent<S: agent_state::Marker> {
    data: AgentData,
    state: core::marker::PhantomData<S>,
}

impl<S: agent_state::Marker> Agent<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> AgentState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &AgentData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> AgentData {
        self.data
    }
}

impl Agent<agent_state::Idle> {
    /// A new instance, resting in `Idle` — the only state the lifecycle starts one in.
    pub fn new(data: AgentData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Agent<agent_state::Idle> {
    /// `resume` — `Idle` → `Working`. Taken by the `submitted` outcome of `agenticspec.session.SubmitPrompt`.
    pub fn resume(self) -> Agent<agent_state::Working> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `end` — `Idle` → `Ended`. Taken by the `ended` outcome of `agenticspec.session.EndAgent`.
    pub fn end(self) -> Agent<agent_state::Ended> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl Agent<agent_state::Working> {
    /// `resume` — `Working` → `Working`. Taken by the `submitted` outcome of `agenticspec.session.SubmitPrompt`.
    pub fn resume(self) -> Agent<agent_state::Working> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `end_turn` — `Working` → `Idle`. Taken by the `ended` outcome of `agenticspec.session.EndTurn`.
    pub fn end_turn(self) -> Agent<agent_state::Idle> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `end` — `Working` → `Ended`. Taken by the `ended` outcome of `agenticspec.session.EndAgent`.
    pub fn end(self) -> Agent<agent_state::Ended> {
        Agent {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agenticspec.session.Agent` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`AgentSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: AgentState,
    /// What it holds.
    pub data: AgentData,
}

/// An `Agent` in whichever declared state it was found.
pub enum AnyAgent {
    /// Resting in `Ended`.
    Ended(Agent<agent_state::Ended>),
    /// Resting in `Idle`.
    Idle(Agent<agent_state::Idle>),
    /// Resting in `Working`.
    Working(Agent<agent_state::Working>),
}

impl AgentSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `AgentState` cannot spell one.
    pub fn refine(self) -> AnyAgent {
        match self.state {
            AgentState::Ended => AnyAgent::Ended(Agent {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentState::Idle => AnyAgent::Idle(Agent {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            AgentState::Working => AnyAgent::Working(Agent {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyAgent {
    /// The state, as the runtime value.
    pub fn state(&self) -> AgentState {
        match self {
            Self::Ended(_) => AgentState::Ended,
            Self::Idle(_) => AgentState::Idle,
            Self::Working(_) => AgentState::Working,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> AgentSnapshot {
        match self {
            Self::Ended(instance) => AgentSnapshot {
                state: AgentState::Ended,
                data: instance.into_data(),
            },
            Self::Idle(instance) => AgentSnapshot {
                state: AgentState::Idle,
                data: instance.into_data(),
            },
            Self::Working(instance) => AgentSnapshot {
                state: AgentState::Working,
                data: instance.into_data(),
            },
        }
    }
}

/// What Exposure — `agenticspec.session.Exposure` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Exposure<S>`], and at a boundary by [`ExposureSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposureData {
    /// The identity: `exposure_id` — `agenticspec.session.ExposureId`.
    pub exposure_id: ExposureId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    ///
    /// Carries `exposures`: `agenticspec.session.Agent` owns many `agenticspec.session.Exposure`.
    pub agent_id: AgentId,
    /// `channel` — `agenticspec.session.ExposureChannel`.
    pub channel: ExposureChannel,
    /// `class` — `agenticspec.session.ExposureClass`.
    pub class: ExposureClass,
    /// `kind` — `String`.
    pub kind: String,
    /// `alias` — `String`.
    pub alias: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: Verdict,
    /// `tool_call_id` — `Optional<agenticspec.session.ToolCallId>`.
    ///
    /// Carries `tool_call`: `agenticspec.session.Exposure` references one `agenticspec.session.ToolCall`.
    pub tool_call_id: Option<ToolCallId>,
}

/// The states of `agenticspec.session.Exposure`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](exposure_state::Marker), so [`Exposure<S>`](Exposure) can only ever rest in a real state.
pub mod exposure_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Recorded {}
    }

    /// A declared state of `Exposure`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::ExposureState;
    }

    /// `Recorded`. Where a new instance starts.
    pub struct Recorded;

    impl Marker for Recorded {
        const STATE: super::ExposureState = super::ExposureState::Recorded;
    }
}

/// Exposure — `agenticspec.session.Exposure` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Recorded`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`ExposureSnapshot`]
/// and [`ExposureSnapshot::refine`].
pub struct Exposure<S: exposure_state::Marker> {
    data: ExposureData,
    state: core::marker::PhantomData<S>,
}

impl<S: exposure_state::Marker> Exposure<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> ExposureState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &ExposureData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> ExposureData {
        self.data
    }
}

impl Exposure<exposure_state::Recorded> {
    /// A new instance, resting in `Recorded` — the only state the lifecycle starts one in.
    pub fn new(data: ExposureData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agenticspec.session.Exposure` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`ExposureSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposureSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: ExposureState,
    /// What it holds.
    pub data: ExposureData,
}

/// An `Exposure` in whichever declared state it was found.
pub enum AnyExposure {
    /// Resting in `Recorded`.
    Recorded(Exposure<exposure_state::Recorded>),
}

impl ExposureSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `ExposureState` cannot spell one.
    pub fn refine(self) -> AnyExposure {
        match self.state {
            ExposureState::Recorded => AnyExposure::Recorded(Exposure {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyExposure {
    /// The state, as the runtime value.
    pub fn state(&self) -> ExposureState {
        match self {
            Self::Recorded(_) => ExposureState::Recorded,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> ExposureSnapshot {
        match self {
            Self::Recorded(instance) => ExposureSnapshot {
                state: ExposureState::Recorded,
                data: instance.into_data(),
            },
        }
    }
}

/// What Session — `agenticspec.session.Session` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`Session<S>`], and at a boundary by [`SessionSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionData {
    /// The identity: `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `harness` — `agenticspec.session.Harness`.
    pub harness: Harness,
    /// `opened_at` — `Timestamp`.
    pub opened_at: crate::primitives::Timestamp,
}

/// The states of `agenticspec.session.Session`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](session_state::Marker), so [`Session<S>`](Session) can only ever rest in a real state.
pub mod session_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Ended {}
        impl Sealed for super::Open {}
    }

    /// A declared state of `Session`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::SessionState;
    }

    /// `Ended`. Terminal: an instance may rest here forever.
    pub struct Ended;

    impl Marker for Ended {
        const STATE: super::SessionState = super::SessionState::Ended;
    }

    /// `Open`. Where a new instance starts.
    pub struct Open;

    impl Marker for Open {
        const STATE: super::SessionState = super::SessionState::Open;
    }
}

/// Session — `agenticspec.session.Session` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Open`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`SessionSnapshot`]
/// and [`SessionSnapshot::refine`].
pub struct Session<S: session_state::Marker> {
    data: SessionData,
    state: core::marker::PhantomData<S>,
}

impl<S: session_state::Marker> Session<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> SessionState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &SessionData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> SessionData {
        self.data
    }
}

impl Session<session_state::Open> {
    /// A new instance, resting in `Open` — the only state the lifecycle starts one in.
    pub fn new(data: SessionData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl Session<session_state::Open> {
    /// `end` — `Open` → `Ended`. Taken by the `ended` outcome of `agenticspec.session.EndSession`.
    pub fn end(self) -> Session<session_state::Ended> {
        Session {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agenticspec.session.Session` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`SessionSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: SessionState,
    /// What it holds.
    pub data: SessionData,
}

/// An `Session` in whichever declared state it was found.
pub enum AnySession {
    /// Resting in `Ended`.
    Ended(Session<session_state::Ended>),
    /// Resting in `Open`.
    Open(Session<session_state::Open>),
}

impl SessionSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `SessionState` cannot spell one.
    pub fn refine(self) -> AnySession {
        match self.state {
            SessionState::Ended => AnySession::Ended(Session {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            SessionState::Open => AnySession::Open(Session {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnySession {
    /// The state, as the runtime value.
    pub fn state(&self) -> SessionState {
        match self {
            Self::Ended(_) => SessionState::Ended,
            Self::Open(_) => SessionState::Open,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> SessionSnapshot {
        match self {
            Self::Ended(instance) => SessionSnapshot {
                state: SessionState::Ended,
                data: instance.into_data(),
            },
            Self::Open(instance) => SessionSnapshot {
                state: SessionState::Open,
                data: instance.into_data(),
            },
        }
    }
}

/// What ToolCall — `agenticspec.session.ToolCall` — holds, apart from where it is in its lifecycle.
///
/// The identity and every declared field. The state is deliberately not one: inside the domain it
/// is carried by the type parameter of [`ToolCall<S>`], and at a boundary by [`ToolCallSnapshot::state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallData {
    /// The identity: `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    ///
    /// Carries `tool_calls`: `agenticspec.session.Agent` owns many `agenticspec.session.ToolCall`.
    pub agent_id: AgentId,
    /// `tool` — `String`.
    pub tool: String,
    /// `category` — `agenticspec.session.ToolCategory`.
    pub category: ToolCategory,
}

/// The states of `agenticspec.session.ToolCall`, at the type level.
///
/// One marker type per declared state, sealed: a state the lifecycle does not declare cannot
/// implement [`Marker`](tool_call_state::Marker), so [`ToolCall<S>`](ToolCall) can only ever rest in a real state.
pub mod tool_call_state {
    /// Closes [`Marker`] over the declared states.
    mod sealed {
        /// Implemented only by the marker types beside this module.
        pub trait Sealed {}
        impl Sealed for super::Accepted {}
        impl Sealed for super::Failed {}
        impl Sealed for super::Refused {}
        impl Sealed for super::Requested {}
        impl Sealed for super::Succeeded {}
    }

    /// A declared state of `ToolCall`, as a type.
    pub trait Marker: sealed::Sealed {
        /// The same state, as the runtime value.
        const STATE: super::ToolCallState;
    }

    /// `Accepted`.
    pub struct Accepted;

    impl Marker for Accepted {
        const STATE: super::ToolCallState = super::ToolCallState::Accepted;
    }

    /// `Failed`. Terminal: an instance may rest here forever.
    pub struct Failed;

    impl Marker for Failed {
        const STATE: super::ToolCallState = super::ToolCallState::Failed;
    }

    /// `Refused`. Terminal: an instance may rest here forever.
    pub struct Refused;

    impl Marker for Refused {
        const STATE: super::ToolCallState = super::ToolCallState::Refused;
    }

    /// `Requested`. Where a new instance starts.
    pub struct Requested;

    impl Marker for Requested {
        const STATE: super::ToolCallState = super::ToolCallState::Requested;
    }

    /// `Succeeded`. Terminal: an instance may rest here forever.
    pub struct Succeeded;

    impl Marker for Succeeded {
        const STATE: super::ToolCallState = super::ToolCallState::Succeeded;
    }
}

/// ToolCall — `agenticspec.session.ToolCall` — with its lifecycle state carried by the type.
///
/// The one constructor rests in `Requested`, and the only way to change `S` is a method generated from
/// a declared transition. A move the specification does not declare is therefore not an error
/// case: it does not compile. Where the state is data — wire, storage — use [`ToolCallSnapshot`]
/// and [`ToolCallSnapshot::refine`].
pub struct ToolCall<S: tool_call_state::Marker> {
    data: ToolCallData,
    state: core::marker::PhantomData<S>,
}

impl<S: tool_call_state::Marker> ToolCall<S> {
    /// The state this instance rests in, as the runtime value.
    pub fn state(&self) -> ToolCallState {
        S::STATE
    }

    /// What it holds.
    pub fn data(&self) -> &ToolCallData {
        &self.data
    }

    /// Hands the data back, giving up the typed state.
    pub fn into_data(self) -> ToolCallData {
        self.data
    }
}

impl ToolCall<tool_call_state::Requested> {
    /// A new instance, resting in `Requested` — the only state the lifecycle starts one in.
    pub fn new(data: ToolCallData) -> Self {
        Self {
            data,
            state: core::marker::PhantomData,
        }
    }
}

impl ToolCall<tool_call_state::Accepted> {
    /// `succeed` — `Accepted` → `Succeeded`. Taken by the `succeeded` outcome of `agenticspec.session.CompleteToolCall`.
    pub fn succeed(self) -> ToolCall<tool_call_state::Succeeded> {
        ToolCall {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `fail` — `Accepted` → `Failed`. Taken by the `failed` outcome of `agenticspec.session.CompleteToolCall`.
    pub fn fail(self) -> ToolCall<tool_call_state::Failed> {
        ToolCall {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

impl ToolCall<tool_call_state::Requested> {
    /// `accept` — `Requested` → `Accepted`. Taken by the `accepted` outcome of `agenticspec.session.DecideToolCall`.
    pub fn accept(self) -> ToolCall<tool_call_state::Accepted> {
        ToolCall {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }

    /// `refuse` — `Requested` → `Refused`. Taken by the `refused` outcome of `agenticspec.session.DecideToolCall`.
    pub fn refuse(self) -> ToolCall<tool_call_state::Refused> {
        ToolCall {
            data: self.data,
            state: core::marker::PhantomData,
        }
    }
}

/// `agenticspec.session.ToolCall` as it crosses a boundary: the state as a value beside the data.
///
/// Wire and storage know states only at runtime; [`ToolCallSnapshot::refine`] is the one door back
/// into the typed lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallSnapshot {
    /// Where the instance is in its lifecycle.
    pub state: ToolCallState,
    /// What it holds.
    pub data: ToolCallData,
}

/// An `ToolCall` in whichever declared state it was found.
pub enum AnyToolCall {
    /// Resting in `Accepted`.
    Accepted(ToolCall<tool_call_state::Accepted>),
    /// Resting in `Failed`.
    Failed(ToolCall<tool_call_state::Failed>),
    /// Resting in `Refused`.
    Refused(ToolCall<tool_call_state::Refused>),
    /// Resting in `Requested`.
    Requested(ToolCall<tool_call_state::Requested>),
    /// Resting in `Succeeded`.
    Succeeded(ToolCall<tool_call_state::Succeeded>),
}

impl ToolCallSnapshot {
    /// Refines the runtime state into the typed one.
    ///
    /// Total: every declared state has an arm, and an undeclared state cannot reach here because
    /// `ToolCallState` cannot spell one.
    pub fn refine(self) -> AnyToolCall {
        match self.state {
            ToolCallState::Accepted => AnyToolCall::Accepted(ToolCall {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolCallState::Failed => AnyToolCall::Failed(ToolCall {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolCallState::Refused => AnyToolCall::Refused(ToolCall {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolCallState::Requested => AnyToolCall::Requested(ToolCall {
                data: self.data,
                state: core::marker::PhantomData,
            }),
            ToolCallState::Succeeded => AnyToolCall::Succeeded(ToolCall {
                data: self.data,
                state: core::marker::PhantomData,
            }),
        }
    }
}

impl AnyToolCall {
    /// The state, as the runtime value.
    pub fn state(&self) -> ToolCallState {
        match self {
            Self::Accepted(_) => ToolCallState::Accepted,
            Self::Failed(_) => ToolCallState::Failed,
            Self::Refused(_) => ToolCallState::Refused,
            Self::Requested(_) => ToolCallState::Requested,
            Self::Succeeded(_) => ToolCallState::Succeeded,
        }
    }

    /// Back to the boundary shape.
    pub fn snapshot(self) -> ToolCallSnapshot {
        match self {
            Self::Accepted(instance) => ToolCallSnapshot {
                state: ToolCallState::Accepted,
                data: instance.into_data(),
            },
            Self::Failed(instance) => ToolCallSnapshot {
                state: ToolCallState::Failed,
                data: instance.into_data(),
            },
            Self::Refused(instance) => ToolCallSnapshot {
                state: ToolCallState::Refused,
                data: instance.into_data(),
            },
            Self::Requested(instance) => ToolCallSnapshot {
                state: ToolCallState::Requested,
                data: instance.into_data(),
            },
            Self::Succeeded(instance) => ToolCallSnapshot {
                state: ToolCallState::Succeeded,
                data: instance.into_data(),
            },
        }
    }
}

/// Compact the context — the input of `agenticspec.session.CompactContext`.
///
/// Everything it can result in is [`CompactContextOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactContext {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `context_tokens_before` — `Integer`.
    pub context_tokens_before: i64,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.CompactContext` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactContextOutcome {
    /// `compacted` — otherwise.
    ///
    /// The context was replaced by a summary; the next model turn writes the cache again.
    Compacted {
        /// The `agenticspec.session.ContextCompacted` this outcome publishes.
        context_compacted: ContextCompacted,
    },
    /// `no-such-agent` — for an identity no record carries.
    NoSuchAgent {
        /// Why it was refused: `agenticspec.session.UnknownAgent`.
        error: UnknownAgent,
    },
}

/// Complete a tool call — the input of `agenticspec.session.CompleteToolCall`.
///
/// Everything it can result in is [`CompleteToolCallOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteToolCall {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `ok` — `Boolean`.
    pub ok: bool,
    /// `output` — `agenticspec.session.Content`.
    pub output: Content,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.CompleteToolCall` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompleteToolCallOutcome {
    /// `succeeded` — when `ok == true`.
    ///
    /// The tool returned a result.
    Succeeded {
        /// The `agenticspec.session.ToolCallCompleted` this outcome publishes.
        tool_call_completed: ToolCallCompleted,
    },
    /// `failed` — otherwise.
    ///
    /// The tool returned an error.
    Failed {
        /// The `agenticspec.session.ToolCallCompleted` this outcome publishes.
        tool_call_completed: ToolCallCompleted,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The tool call was not running.
    WrongState {
        /// Why it was refused: `agenticspec.session.ToolCallStateConflict`.
        error: ToolCallStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// Decide a tool call — the input of `agenticspec.session.DecideToolCall`.
///
/// Everything it can result in is [`DecideToolCallOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecideToolCall {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `decision` — `agenticspec.session.Decision`.
    pub decision: Decision,
    /// `decider` — `agenticspec.session.Decider`.
    pub decider: Decider,
    /// `reason` — `Optional<String>`.
    pub reason: Option<String>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.DecideToolCall` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecideToolCallOutcome {
    /// `accepted` — when `decision == Accept`.
    ///
    /// The tool runs.
    Accepted {
        /// The `agenticspec.session.ToolCallDecided` this outcome publishes.
        tool_call_decided: ToolCallDecided,
    },
    /// `refused` — otherwise.
    ///
    /// The tool does not run; the agent gets the refusal as its result.
    Refused {
        /// The `agenticspec.session.ToolCallDecided` this outcome publishes.
        tool_call_decided: ToolCallDecided,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The tool call was already decided.
    WrongState {
        /// Why it was refused: `agenticspec.session.ToolCallStateConflict`.
        error: ToolCallStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// End an agent — the input of `agenticspec.session.EndAgent`.
///
/// Everything it can result in is [`EndAgentOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndAgent {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.EndAgent` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndAgentOutcome {
    /// `ended` — otherwise.
    ///
    /// The agent will take no further prompt.
    Ended {
        /// The `agenticspec.session.AgentEnded` this outcome publishes.
        agent_ended: AgentEnded,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The agent had already ended.
    WrongState {
        /// Why it was refused: `agenticspec.session.AgentStateConflict`.
        error: AgentStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// End a session — the input of `agenticspec.session.EndSession`.
///
/// Everything it can result in is [`EndSessionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndSession {
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.EndSession` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndSessionOutcome {
    /// `ended` — otherwise.
    ///
    /// The session has ended; the transcript holds no later record.
    Ended {
        /// The `agenticspec.session.SessionEnded` this outcome publishes.
        session_ended: SessionEnded,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The session had already ended.
    WrongState {
        /// Why it was refused: `agenticspec.session.SessionStateConflict`.
        error: SessionStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// End a turn — the input of `agenticspec.session.EndTurn`.
///
/// Everything it can result in is [`EndTurnOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndTurn {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.EndTurn` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndTurnOutcome {
    /// `ended` — otherwise.
    ///
    /// The agent is Idle and waits for its next prompt.
    Ended {
        /// The `agenticspec.session.TurnEnded` this outcome publishes.
        turn_ended: TurnEnded,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The agent was not Working.
    WrongState {
        /// Why it was refused: `agenticspec.session.AgentStateConflict`.
        error: AgentStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// Open a session — the input of `agenticspec.session.OpenSession`.
///
/// Everything it can result in is [`OpenSessionOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSession {
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `harness` — `agenticspec.session.Harness`.
    pub harness: Harness,
    /// `source` — `Optional<agenticspec.session.Source>`.
    pub source: Option<Source>,
    /// `session_hash` — `Optional<String>`.
    pub session_hash: Option<String>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.OpenSession` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenSessionOutcome {
    /// `opened` — otherwise.
    ///
    /// The session exists and holds no agent yet.
    Opened {
        /// The `agenticspec.session.SessionOpened` this outcome publishes.
        session_opened: SessionOpened,
    },
}

/// Record an exposure — the input of `agenticspec.session.RecordExposure`.
///
/// The redaction pass found a secret or a personal datum and replaced it.
///
/// Everything it can result in is [`RecordExposureOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordExposure {
    /// `exposure_id` — `agenticspec.session.ExposureId`.
    pub exposure_id: ExposureId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `channel` — `agenticspec.session.ExposureChannel`.
    pub channel: ExposureChannel,
    /// `class` — `agenticspec.session.ExposureClass`.
    pub class: ExposureClass,
    /// `kind` — `String`.
    pub kind: String,
    /// `alias` — `String`.
    pub alias: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: Verdict,
    /// `tool_call_id` — `Optional<agenticspec.session.ToolCallId>`.
    pub tool_call_id: Option<ToolCallId>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.RecordExposure` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordExposureOutcome {
    /// `recorded` — otherwise.
    ///
    /// The exposure is on record under its alias; the value is not.
    Recorded {
        /// The `agenticspec.session.ExposureDetected` this outcome publishes.
        exposure_detected: ExposureDetected,
    },
}

/// Record a model turn — the input of `agenticspec.session.RecordModelTurn`.
///
/// One model message of the agent, with the model that wrote it and its token usage.
///
/// Everything it can result in is [`RecordModelTurnOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordModelTurn {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `model` — `String`.
    pub model: String,
    /// `usage` — `agenticspec.session.Usage`.
    pub usage: Usage,
    /// `context_tokens` — `Integer`.
    pub context_tokens: i64,
    /// `message_id` — `Optional<String>`.
    pub message_id: Option<String>,
    /// `call_id` — `Optional<String>`.
    pub call_id: Option<String>,
    /// `route` — `Optional<String>`.
    pub route: Option<String>,
    /// `upstream` — `Optional<String>`.
    pub upstream: Option<String>,
    /// `attempts` — `Optional<Integer>`.
    pub attempts: Option<i64>,
    /// `queue_wait_ms` — `Optional<Integer>`.
    pub queue_wait_ms: Option<i64>,
    /// `ttft_ms` — `Optional<Integer>`.
    pub ttft_ms: Option<i64>,
    /// `declared_cost_microdollars` — `Optional<Integer>`.
    pub declared_cost_microdollars: Option<i64>,
    /// `incomplete` — `Optional<agenticspec.session.IncompleteReason>`.
    pub incomplete: Option<IncompleteReason>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.RecordModelTurn` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordModelTurnOutcome {
    /// `recorded` — otherwise.
    ///
    /// The agent's model and context size are those of this message.
    Recorded {
        /// The `agenticspec.session.ModelTurnRecorded` this outcome publishes.
        model_turn_recorded: ModelTurnRecorded,
    },
    /// `no-such-agent` — for an identity no record carries.
    NoSuchAgent {
        /// Why it was refused: `agenticspec.session.UnknownAgent`.
        error: UnknownAgent,
    },
}

/// Request a tool call — the input of `agenticspec.session.RequestToolCall`.
///
/// Everything it can result in is [`RequestToolCallOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestToolCall {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `tool` — `String`.
    pub tool: String,
    /// `category` — `agenticspec.session.ToolCategory`.
    pub category: ToolCategory,
    /// `input` — `agenticspec.session.Content`.
    pub input: Content,
    /// `cwd` — `Optional<String>`.
    pub cwd: Option<String>,
    /// `background` — `Boolean`.
    pub background: bool,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.RequestToolCall` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestToolCallOutcome {
    /// `requested` — otherwise.
    ///
    /// The tool call waits for a decision.
    Requested {
        /// The `agenticspec.session.ToolCallRequested` this outcome publishes.
        tool_call_requested: ToolCallRequested,
    },
}

/// Start an agent — the input of `agenticspec.session.StartAgent`.
///
/// Everything it can result in is [`StartAgentOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartAgent {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `parent_agent_id` — `Optional<agenticspec.session.AgentId>`.
    pub parent_agent_id: Option<AgentId>,
    /// `role` — `agenticspec.session.AgentRole`.
    pub role: AgentRole,
    /// `spawned_by` — `Optional<agenticspec.session.ToolCallId>`.
    pub spawned_by: Option<ToolCallId>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.StartAgent` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartAgentOutcome {
    /// `started` — otherwise.
    ///
    /// The agent exists and is Idle until its first prompt.
    Started {
        /// The `agenticspec.session.AgentStarted` this outcome publishes.
        agent_started: AgentStarted,
    },
}

/// Submit a prompt — the input of `agenticspec.session.SubmitPrompt`.
///
/// A prompt reaches an agent. From Idle it starts a turn; while Working it is queued into the running turn, as Claude Code does with a message typed mid-turn.
///
/// Everything it can result in is [`SubmitPromptOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitPrompt {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `author` — `agenticspec.session.PromptAuthor`.
    pub author: PromptAuthor,
    /// `attribution` — `agenticspec.session.Attribution`.
    pub attribution: Attribution,
    /// `class` — `agenticspec.session.PromptClass`.
    pub class: PromptClass,
    /// `content` — `agenticspec.session.Content`.
    pub content: Content,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// Everything `agenticspec.session.SubmitPrompt` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitPromptOutcome {
    /// `submitted` — otherwise.
    ///
    /// The agent is Working on the prompt.
    Submitted {
        /// The `agenticspec.session.PromptSubmitted` this outcome publishes.
        prompt_submitted: PromptSubmitted,
    },
    /// `wrong-state` — from a state no declared move starts in.
    ///
    /// The agent had ended, so the prompt reached nobody.
    WrongState {
        /// Why it was refused: `agenticspec.session.AgentStateConflict`.
        error: AgentStateConflict,
    },
    /// `wrong-state` — for an instance no record carries.
    ///
    /// The same declared branch and error as [`Self::WrongState`], without the error's fields: an instance
    /// that does not exist has nothing for them to describe (`docs/design/unknown-instance-seams.md`).
    WrongStateUnknownInstance,
}

/// AgentEnded — the event `agenticspec.session.AgentEnded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEnded {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// AgentStarted — the event `agenticspec.session.AgentStarted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStarted {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `parent_agent_id` — `Optional<agenticspec.session.AgentId>`.
    pub parent_agent_id: Option<AgentId>,
    /// `role` — `agenticspec.session.AgentRole`.
    pub role: AgentRole,
    /// `spawned_by` — `Optional<agenticspec.session.ToolCallId>`.
    pub spawned_by: Option<ToolCallId>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ContextCompacted — the event `agenticspec.session.ContextCompacted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCompacted {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `context_tokens_before` — `Integer`.
    pub context_tokens_before: i64,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ExposureDetected — the event `agenticspec.session.ExposureDetected`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposureDetected {
    /// `exposure_id` — `agenticspec.session.ExposureId`.
    pub exposure_id: ExposureId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `channel` — `agenticspec.session.ExposureChannel`.
    pub channel: ExposureChannel,
    /// `class` — `agenticspec.session.ExposureClass`.
    pub class: ExposureClass,
    /// `kind` — `String`.
    pub kind: String,
    /// `alias` — `String`.
    pub alias: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: Verdict,
    /// `tool_call_id` — `Optional<agenticspec.session.ToolCallId>`.
    pub tool_call_id: Option<ToolCallId>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ModelTurnRecorded — the event `agenticspec.session.ModelTurnRecorded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelTurnRecorded {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `model` — `String`.
    pub model: String,
    /// `usage` — `agenticspec.session.Usage`.
    pub usage: Usage,
    /// `context_tokens` — `Integer`.
    pub context_tokens: i64,
    /// `message_id` — `Optional<String>`.
    pub message_id: Option<String>,
    /// `call_id` — `Optional<String>`.
    pub call_id: Option<String>,
    /// `route` — `Optional<String>`.
    pub route: Option<String>,
    /// `upstream` — `Optional<String>`.
    pub upstream: Option<String>,
    /// `attempts` — `Optional<Integer>`.
    pub attempts: Option<i64>,
    /// `queue_wait_ms` — `Optional<Integer>`.
    pub queue_wait_ms: Option<i64>,
    /// `ttft_ms` — `Optional<Integer>`.
    pub ttft_ms: Option<i64>,
    /// `declared_cost_microdollars` — `Optional<Integer>`.
    pub declared_cost_microdollars: Option<i64>,
    /// `incomplete` — `Optional<agenticspec.session.IncompleteReason>`.
    pub incomplete: Option<IncompleteReason>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// PromptSubmitted — the event `agenticspec.session.PromptSubmitted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptSubmitted {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `author` — `agenticspec.session.PromptAuthor`.
    pub author: PromptAuthor,
    /// `attribution` — `agenticspec.session.Attribution`.
    pub attribution: Attribution,
    /// `class` — `agenticspec.session.PromptClass`.
    pub class: PromptClass,
    /// `content` — `agenticspec.session.Content`.
    pub content: Content,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// SessionEnded — the event `agenticspec.session.SessionEnded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionEnded {
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// SessionOpened — the event `agenticspec.session.SessionOpened`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionOpened {
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `harness` — `agenticspec.session.Harness`.
    pub harness: Harness,
    /// `source` — `Optional<agenticspec.session.Source>`.
    pub source: Option<Source>,
    /// `session_hash` — `Optional<String>`.
    pub session_hash: Option<String>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ToolCallCompleted — the event `agenticspec.session.ToolCallCompleted`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallCompleted {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `ok` — `Boolean`.
    pub ok: bool,
    /// `output` — `agenticspec.session.Content`.
    pub output: Content,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ToolCallDecided — the event `agenticspec.session.ToolCallDecided`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallDecided {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `decision` — `agenticspec.session.Decision`.
    pub decision: Decision,
    /// `decider` — `agenticspec.session.Decider`.
    pub decider: Decider,
    /// `reason` — `Optional<String>`.
    pub reason: Option<String>,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// ToolCallRequested — the event `agenticspec.session.ToolCallRequested`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallRequested {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `tool` — `String`.
    pub tool: String,
    /// `category` — `agenticspec.session.ToolCategory`.
    pub category: ToolCategory,
    /// `input` — `agenticspec.session.Content`.
    pub input: Content,
    /// `cwd` — `Optional<String>`.
    pub cwd: Option<String>,
    /// `background` — `Boolean`.
    pub background: bool,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// TurnEnded — the event `agenticspec.session.TurnEnded`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnEnded {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `at` — `Timestamp`.
    pub at: crate::primitives::Timestamp,
}

/// The declared error `agenticspec.session.AgentStateConflict`.
///
/// The agent is not in a state this command acts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStateConflict {
    /// `state` — `agenticspec.session.Agent.State`.
    pub state: AgentState,
}

/// The declared error `agenticspec.session.SessionStateConflict`.
///
/// The session is not in a state this command acts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionStateConflict {
    /// `state` — `agenticspec.session.Session.State`.
    pub state: SessionState,
}

/// The declared error `agenticspec.session.ToolCallStateConflict`.
///
/// The tool call is not in a state this command acts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallStateConflict {
    /// `state` — `agenticspec.session.ToolCall.State`.
    pub state: ToolCallState,
}

/// The declared error `agenticspec.session.UnknownAgent`.
///
/// No agent of this identity was started; the adapter lost a StartAgent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownAgent {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
}

/// Agents — one row of the view `agenticspec.session.Agents`.
///
/// Projects `agenticspec.session.Agent` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agents {
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `parent_agent_id` — `Optional<agenticspec.session.AgentId>`.
    pub parent_agent_id: Option<AgentId>,
    /// `role` — `agenticspec.session.AgentRole`.
    pub role: AgentRole,
    /// `model` — `Optional<String>`.
    pub model: Option<String>,
    /// `context_tokens` — `Integer`.
    pub context_tokens: i64,
    /// `state` — `agenticspec.session.Agent.State`.
    pub state: AgentState,
}

/// Exposures — one row of the view `agenticspec.session.Exposures`.
///
/// Projects `agenticspec.session.Exposure` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exposures {
    /// `exposure_id` — `agenticspec.session.ExposureId`.
    pub exposure_id: ExposureId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `channel` — `agenticspec.session.ExposureChannel`.
    pub channel: ExposureChannel,
    /// `class` — `agenticspec.session.ExposureClass`.
    pub class: ExposureClass,
    /// `kind` — `String`.
    pub kind: String,
    /// `alias` — `String`.
    pub alias: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: Verdict,
    /// `state` — `agenticspec.session.Exposure.State`.
    pub state: ExposureState,
}

/// Sessions — one row of the view `agenticspec.session.Sessions`.
///
/// Projects `agenticspec.session.Session` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sessions {
    /// `session_id` — `agenticspec.session.SessionId`.
    pub session_id: SessionId,
    /// `harness` — `agenticspec.session.Harness`.
    pub harness: Harness,
    /// `opened_at` — `Timestamp`.
    pub opened_at: crate::primitives::Timestamp,
    /// `state` — `agenticspec.session.Session.State`.
    pub state: SessionState,
}

/// Tool calls — one row of the view `agenticspec.session.ToolCalls`.
///
/// Projects `agenticspec.session.ToolCall` at `read_your_writes` consistency.
/// The specification fully determines every row, so its query is generated over the storage port —
/// see the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCalls {
    /// `tool_call_id` — `agenticspec.session.ToolCallId`.
    pub tool_call_id: ToolCallId,
    /// `agent_id` — `agenticspec.session.AgentId`.
    pub agent_id: AgentId,
    /// `tool` — `String`.
    pub tool: String,
    /// `category` — `agenticspec.session.ToolCategory`.
    pub category: ToolCategory,
    /// `state` — `agenticspec.session.ToolCall.State`.
    pub state: ToolCallState,
}

/// What this bounded context owes its implementor, and the seams of what is generated.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract, and one
/// per generated behaviour, which [`Generated`](crate::behaviour::Generated) implements.
pub mod obligations {
    /// The behaviour `agenticspec.session.CompactContext` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CompactContextBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.CompactContext`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn compact_context(&mut self, input: super::CompactContext) -> Result<super::CompactContextOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.CompleteToolCall` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait CompleteToolCallBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.CompleteToolCall`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn complete_tool_call(&mut self, input: super::CompleteToolCall) -> Result<super::CompleteToolCallOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.DecideToolCall` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait DecideToolCallBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.DecideToolCall`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn decide_tool_call(&mut self, input: super::DecideToolCall) -> Result<super::DecideToolCallOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.EndAgent` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait EndAgentBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.EndAgent`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn end_agent(&mut self, input: super::EndAgent) -> Result<super::EndAgentOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.EndSession` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait EndSessionBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.EndSession`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn end_session(&mut self, input: super::EndSession) -> Result<super::EndSessionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.EndTurn` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait EndTurnBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.EndTurn`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn end_turn(&mut self, input: super::EndTurn) -> Result<super::EndTurnOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.OpenSession` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait OpenSessionBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.OpenSession`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn open_session(&mut self, input: super::OpenSession) -> Result<super::OpenSessionOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.RecordExposure` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordExposureBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.RecordExposure`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_exposure(&mut self, input: super::RecordExposure) -> Result<super::RecordExposureOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.RecordModelTurn` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RecordModelTurnBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.RecordModelTurn`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn record_model_turn(&mut self, input: super::RecordModelTurn) -> Result<super::RecordModelTurnOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.RequestToolCall` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait RequestToolCallBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.RequestToolCall`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn request_tool_call(&mut self, input: super::RequestToolCall) -> Result<super::RequestToolCallOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.StartAgent` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait StartAgentBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.StartAgent`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn start_agent(&mut self, input: super::StartAgent) -> Result<super::StartAgentOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `agenticspec.session.SubmitPrompt` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage and context ports. Implement it yourself to replace that behaviour.
    pub trait SubmitPromptBehavior {
        /// Decides and enacts exactly one declared outcome of `agenticspec.session.SubmitPrompt`.
        ///
        /// `Err` is the typed refusal of a request the model declares no outcome for.
        fn submit_prompt(&mut self, input: super::SubmitPrompt) -> Result<super::SubmitPromptOutcome, crate::obligation::UnmetObligation>;
    }

    /// The query `agenticspec.session.Agents` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait AgentsQuery {
        /// Serves `agenticspec.session.Agents` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn agents(&self) -> Result<Vec<super::Agents>, crate::obligation::UnmetObligation>;
    }

    /// The query `agenticspec.session.Exposures` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait ExposuresQuery {
        /// Serves `agenticspec.session.Exposures` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn exposures(&self) -> Result<Vec<super::Exposures>, crate::obligation::UnmetObligation>;
    }

    /// The query `agenticspec.session.Sessions` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait SessionsQuery {
        /// Serves `agenticspec.session.Sessions` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn sessions(&self) -> Result<Vec<super::Sessions>, crate::obligation::UnmetObligation>;
    }

    /// The query `agenticspec.session.ToolCalls` — generated.
    ///
    /// The specification fully determines it: [`crate::behaviour::Generated`] implements it
    /// over the storage port. Implement it yourself to replace that query.
    pub trait ToolCallsQuery {
        /// Serves `agenticspec.session.ToolCalls` rows at the view's declared consistency.
        ///
        /// `Err` is the typed refusal of a row whose declared type cannot hold its value.
        fn tool_calls(&self) -> Result<Vec<super::ToolCalls>, crate::obligation::UnmetObligation>;
    }

}
