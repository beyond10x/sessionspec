// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! session-view — the `session-view` component of `agenticspec` v1.
//!
//! Serves the read models of the UI (`ui/agenticspec.ui.yaml`) over HTTP: the session list, one session, its agents and its events, and publishes their changes for a live page.
//!
//! The component's outer surface exactly as the specification declares it: accepted commands as
//! handlers, declared views as queries, published events as a typed outbox. The behaviour behind
//! every handler is an implementation obligation — see the `PLAN.md` beside this workspace — and
//! until one is satisfied, its stub answers with a typed refusal naming what is owed.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event this component declares it publishes, on its way to the system's transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedEvent {
    /// `agenticspec.view.AgentUpdated`.
    AgentUpdated(agenticspec_types::view::AgentUpdated),
    /// `agenticspec.view.EventAppended`.
    EventAppended(agenticspec_types::view::EventAppended),
    /// `agenticspec.view.SessionUpdated`.
    SessionUpdated(agenticspec_types::view::SessionUpdated),
}

/// session-view — the port over the component's obligations.
///
/// `B` bundles every behaviour and query this component owes; constructing it over the domain's
/// `obligations::Unimplemented` yields a component that compiles and refuses, in the type system,
/// everything not yet implemented.
pub struct SessionView<B> {
    behaviors: B,
    outbox: Vec<PublishedEvent>,
}

impl<B> SessionView<B> {
    /// A new port over the given obligation implementations.
    pub fn new(behaviors: B) -> Self {
        Self {
            behaviors,
            outbox: Vec::new(),
        }
    }

    /// Hands over everything published since the last drain, in publication order.
    ///
    /// The system's transport calls this; anything else reading it is taking events the transport
    /// will then never deliver.
    pub fn drain_outbox(&mut self) -> Vec<PublishedEvent> {
        core::mem::take(&mut self.outbox)
    }
}

impl<B> SessionView<B>
where
    B: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    /// Serves `agenticspec.view.AgentTurns` at `eventual` consistency, from the owed projection.
    pub fn agent_turns(&self, session_id: agenticspec_types::session::SessionId, agent: agenticspec_types::session::AgentId) -> Result<Vec<agenticspec_types::view::AgentTurns>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.agent_turns(session_id, agent)
    }

    /// Serves `agenticspec.view.OverviewFigures` at `eventual` consistency, from the owed projection.
    pub fn overview_figures(&self, since: agenticspec_types::primitives::Timestamp, bucket_seconds: i64) -> Result<Vec<agenticspec_types::view::OverviewFigures>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.overview_figures(since, bucket_seconds)
    }

    /// Serves `agenticspec.view.SessionAgents` at `eventual` consistency, from the owed projection.
    pub fn session_agents(&self, session_id: agenticspec_types::session::SessionId) -> Result<Vec<agenticspec_types::view::SessionAgents>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_agents(session_id)
    }

    /// Serves `agenticspec.view.SessionById` at `eventual` consistency, from the owed projection.
    pub fn session_by_id(&self, session_id: agenticspec_types::session::SessionId) -> Result<Vec<agenticspec_types::view::SessionById>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_by_id(session_id)
    }

    /// Serves `agenticspec.view.SessionChecks` at `eventual` consistency, from the owed projection.
    pub fn session_checks(&self, session_id: agenticspec_types::session::SessionId) -> Result<Vec<agenticspec_types::view::SessionChecks>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_checks(session_id)
    }

    /// Serves `agenticspec.view.SessionEventCounts` at `eventual` consistency, from the owed projection.
    pub fn session_event_counts(&self, session_id: agenticspec_types::session::SessionId, agent: Option<agenticspec_types::session::AgentId>) -> Result<Vec<agenticspec_types::view::SessionEventCounts>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_event_counts(session_id, agent)
    }

    /// Serves `agenticspec.view.SessionEvents` at `eventual` consistency, from the owed projection.
    pub fn session_events(&self, session_id: agenticspec_types::session::SessionId, agent: Option<agenticspec_types::session::AgentId>, kind: Option<agenticspec_types::view::EventKind>, notable: Option<bool>, before: Option<i64>) -> Result<Vec<agenticspec_types::view::SessionEvents>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_events(session_id, agent, kind, notable, before)
    }

    /// Serves `agenticspec.view.SessionList` at `eventual` consistency, from the owed projection.
    pub fn session_list(&self, since: Option<agenticspec_types::primitives::Timestamp>, search: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<agenticspec_types::view::SessionList>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_list(since, search, limit, offset)
    }

    /// Serves `agenticspec.view.SessionTimeline` at `eventual` consistency, from the owed projection.
    pub fn session_timeline(&self, session_id: agenticspec_types::session::SessionId, slot_seconds: i64) -> Result<Vec<agenticspec_types::view::SessionTimeline>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_timeline(session_id, slot_seconds)
    }

    /// Serves `agenticspec.view.SessionTurns` at `eventual` consistency, from the owed projection.
    pub fn session_turns(&self, session_id: agenticspec_types::session::SessionId) -> Result<Vec<agenticspec_types::view::SessionTurns>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.session_turns(session_id)
    }
}
