// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! Every actor the specification declares, and the commands each may invoke — as data.
//!
//! A grant is checked against a caller identity, which these types do not read from anywhere:
//! whatever authenticates a request builds a [`Caller`], and a served surface checks it against
//! [`may`] before the command runs. The `PLAN.md` beside this workspace says, per actor,
//! whether a generated surface enforces the grant or the caller does.

/// An actor the specification declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Actor {
    /// `agenticspec.session.AgentLoop`.
    AgentLoop,
    /// `agenticspec.session.Delegate`.
    Delegate,
    /// `agenticspec.session.HarnessRuntime`.
    HarnessRuntime,
    /// `agenticspec.session.Human`.
    Human,
    /// `agenticspec.session.PeerSession`.
    PeerSession,
    /// `agenticspec.session.PermissionAuthority`.
    PermissionAuthority,
    /// `agenticspec.session.Redactor`.
    Redactor,
    /// `agenticspec.session.ToolProvider`.
    ToolProvider,
    /// `agenticspec.view.Viewer`.
    Viewer,
}

impl Actor {
    /// Every declared actor, ordered by qualified name.
    pub const ALL: &'static [Actor] = &[
        Actor::AgentLoop,
        Actor::Delegate,
        Actor::HarnessRuntime,
        Actor::Human,
        Actor::PeerSession,
        Actor::PermissionAuthority,
        Actor::Redactor,
        Actor::ToolProvider,
        Actor::Viewer,
    ];

    /// The actor's qualified name, as the specification spells it.
    pub const fn name(self) -> &'static str {
        match self {
            Actor::AgentLoop => "agenticspec.session.AgentLoop",
            Actor::Delegate => "agenticspec.session.Delegate",
            Actor::HarnessRuntime => "agenticspec.session.HarnessRuntime",
            Actor::Human => "agenticspec.session.Human",
            Actor::PeerSession => "agenticspec.session.PeerSession",
            Actor::PermissionAuthority => "agenticspec.session.PermissionAuthority",
            Actor::Redactor => "agenticspec.session.Redactor",
            Actor::ToolProvider => "agenticspec.session.ToolProvider",
            Actor::Viewer => "agenticspec.view.Viewer",
        }
    }
}

/// The qualified names of the commands `actor` may invoke, ordered by name; empty for an
/// actor that only observes.
pub fn may(actor: Actor) -> &'static [&'static str] {
    match actor {
        Actor::AgentLoop => &[
            "agenticspec.session.EndTurn",
            "agenticspec.session.RecordModelTurn",
            "agenticspec.session.RequestToolCall",
            "agenticspec.session.SubmitPrompt",
        ],
        Actor::Delegate => &[
            "agenticspec.session.SubmitPrompt",
        ],
        Actor::HarnessRuntime => &[
            "agenticspec.session.CompactContext",
            "agenticspec.session.EndAgent",
            "agenticspec.session.EndSession",
            "agenticspec.session.OpenSession",
            "agenticspec.session.StartAgent",
            "agenticspec.session.SubmitPrompt",
        ],
        Actor::Human => &[
            "agenticspec.session.DecideToolCall",
            "agenticspec.session.SubmitPrompt",
        ],
        Actor::PeerSession => &[
            "agenticspec.session.SubmitPrompt",
        ],
        Actor::PermissionAuthority => &[
            "agenticspec.session.DecideToolCall",
        ],
        Actor::Redactor => &[
            "agenticspec.session.RecordExposure",
        ],
        Actor::ToolProvider => &[
            "agenticspec.session.CompleteToolCall",
        ],
        Actor::Viewer => &[],
    }
}

/// Who a request was authenticated as.
///
/// Built by whatever authenticates the request — a session, a token, a certificate — and
/// handed to the served surface's `dispatch` and `handle`, which check its grant
/// before the command runs. Never derived from the request itself: a client can write
/// anything into a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Caller {
    /// The declared actor.
    pub actor: Actor,
}

impl Caller {
    /// `true` when this caller may invoke `command`, named by its qualified name.
    pub fn may(&self, command: &str) -> bool {
        may(self.actor).contains(&command)
    }
}

/// The qualified names of the views `actor` may read, ordered by name: the views its grant
/// names. A view no actor's grant names is open to every caller and listed for none.
pub fn may_read(actor: Actor) -> &'static [&'static str] {
    match actor {
        Actor::AgentLoop => &[],
        Actor::Delegate => &[],
        Actor::HarnessRuntime => &[],
        Actor::Human => &[],
        Actor::PeerSession => &[],
        Actor::PermissionAuthority => &[],
        Actor::Redactor => &[],
        Actor::ToolProvider => &[],
        Actor::Viewer => &[
            "agenticspec.view.AgentTurns",
            "agenticspec.view.OverviewFigures",
            "agenticspec.view.SessionAgents",
            "agenticspec.view.SessionById",
            "agenticspec.view.SessionChecks",
            "agenticspec.view.SessionEventCounts",
            "agenticspec.view.SessionEvents",
            "agenticspec.view.SessionList",
            "agenticspec.view.SessionTimeline",
            "agenticspec.view.SessionTurns",
        ],
    }
}

impl Caller {
    /// `true` when this caller may read `view`, a view some actor's grant names, named by
    /// its qualified name.
    pub fn may_read(&self, view: &str) -> bool {
        may_read(self.actor).contains(&view)
    }
}
