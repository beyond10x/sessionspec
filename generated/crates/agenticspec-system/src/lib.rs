// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! The `agenticspec` system, v1: its components assembled, its bindings wired, and its one transport.
//!
//! The transport is derived from the specification, not chosen: `at_least_once` is the only
//! delivery guarantee the model declares, so published events land on an append-only log and a
//! pump delivers each to every binding that reacts to it. The log is the system's observable
//! record, and so is the record of what each binding invoked. What no specification determines
//! — how an escalation event is filled, behaviour behind the ports — stays an obligation; see
//! the `PLAN.md` beside this workspace.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event on the system's log: everything any component publishes, and everything a binding
/// escalates into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemEvent {
    /// `agenticspec.session.AgentEnded`.
    AgentEnded(agenticspec_types::session::AgentEnded),
    /// `agenticspec.session.AgentStarted`.
    AgentStarted(agenticspec_types::session::AgentStarted),
    /// `agenticspec.session.ContextCompacted`.
    ContextCompacted(agenticspec_types::session::ContextCompacted),
    /// `agenticspec.session.ExposureDetected`.
    ExposureDetected(agenticspec_types::session::ExposureDetected),
    /// `agenticspec.session.ModelTurnRecorded`.
    ModelTurnRecorded(agenticspec_types::session::ModelTurnRecorded),
    /// `agenticspec.session.PromptSubmitted`.
    PromptSubmitted(agenticspec_types::session::PromptSubmitted),
    /// `agenticspec.session.SessionEnded`.
    SessionEnded(agenticspec_types::session::SessionEnded),
    /// `agenticspec.session.SessionOpened`.
    SessionOpened(agenticspec_types::session::SessionOpened),
    /// `agenticspec.session.ToolCallCompleted`.
    ToolCallCompleted(agenticspec_types::session::ToolCallCompleted),
    /// `agenticspec.session.ToolCallDecided`.
    ToolCallDecided(agenticspec_types::session::ToolCallDecided),
    /// `agenticspec.session.ToolCallRequested`.
    ToolCallRequested(agenticspec_types::session::ToolCallRequested),
    /// `agenticspec.session.TurnEnded`.
    TurnEnded(agenticspec_types::session::TurnEnded),
    /// `agenticspec.view.AgentUpdated`.
    AgentUpdated(agenticspec_types::view::AgentUpdated),
    /// `agenticspec.view.EventAppended`.
    EventAppended(agenticspec_types::view::EventAppended),
    /// `agenticspec.view.SessionUpdated`.
    SessionUpdated(agenticspec_types::view::SessionUpdated),
}

impl SystemEvent {
    /// The qualified name the specification declares this event under.
    pub fn name(&self) -> &'static str {
        match self {
            Self::AgentEnded(_) => "agenticspec.session.AgentEnded",
            Self::AgentStarted(_) => "agenticspec.session.AgentStarted",
            Self::ContextCompacted(_) => "agenticspec.session.ContextCompacted",
            Self::ExposureDetected(_) => "agenticspec.session.ExposureDetected",
            Self::ModelTurnRecorded(_) => "agenticspec.session.ModelTurnRecorded",
            Self::PromptSubmitted(_) => "agenticspec.session.PromptSubmitted",
            Self::SessionEnded(_) => "agenticspec.session.SessionEnded",
            Self::SessionOpened(_) => "agenticspec.session.SessionOpened",
            Self::ToolCallCompleted(_) => "agenticspec.session.ToolCallCompleted",
            Self::ToolCallDecided(_) => "agenticspec.session.ToolCallDecided",
            Self::ToolCallRequested(_) => "agenticspec.session.ToolCallRequested",
            Self::TurnEnded(_) => "agenticspec.session.TurnEnded",
            Self::AgentUpdated(_) => "agenticspec.view.AgentUpdated",
            Self::EventAppended(_) => "agenticspec.view.EventAppended",
            Self::SessionUpdated(_) => "agenticspec.view.SessionUpdated",
        }
    }
}

impl From<session_ir::PublishedEvent> for SystemEvent {
    fn from(event: session_ir::PublishedEvent) -> Self {
        match event {
            session_ir::PublishedEvent::AgentEnded(event) => Self::AgentEnded(event),
            session_ir::PublishedEvent::AgentStarted(event) => Self::AgentStarted(event),
            session_ir::PublishedEvent::ContextCompacted(event) => Self::ContextCompacted(event),
            session_ir::PublishedEvent::ExposureDetected(event) => Self::ExposureDetected(event),
            session_ir::PublishedEvent::ModelTurnRecorded(event) => Self::ModelTurnRecorded(event),
            session_ir::PublishedEvent::PromptSubmitted(event) => Self::PromptSubmitted(event),
            session_ir::PublishedEvent::SessionEnded(event) => Self::SessionEnded(event),
            session_ir::PublishedEvent::SessionOpened(event) => Self::SessionOpened(event),
            session_ir::PublishedEvent::ToolCallCompleted(event) => Self::ToolCallCompleted(event),
            session_ir::PublishedEvent::ToolCallDecided(event) => Self::ToolCallDecided(event),
            session_ir::PublishedEvent::ToolCallRequested(event) => Self::ToolCallRequested(event),
            session_ir::PublishedEvent::TurnEnded(event) => Self::TurnEnded(event),
        }
    }
}

impl From<session_view::PublishedEvent> for SystemEvent {
    fn from(event: session_view::PublishedEvent) -> Self {
        match event {
            session_view::PublishedEvent::AgentUpdated(event) => Self::AgentUpdated(event),
            session_view::PublishedEvent::EventAppended(event) => Self::EventAppended(event),
            session_view::PublishedEvent::SessionUpdated(event) => Self::SessionUpdated(event),
        }
    }
}

/// The `agenticspec` system: every component behind its port, and the transport between them.
///
/// The component fields are public because commands enter the system through a component's own
/// port; the log and its delivery cursor are not, because publishing happens by pumping, not by
/// writing history directly.
pub struct System<SessionIrBehaviors, SessionViewBehaviors> {
    /// The `session-ir` component.
    pub session_ir: session_ir::SessionIr<SessionIrBehaviors>,
    /// The `session-view` component.
    pub session_view: session_view::SessionView<SessionViewBehaviors>,
    published: Vec<SystemEvent>,
    cursor: usize,
}

impl<SessionIrBehaviors, SessionViewBehaviors> System<SessionIrBehaviors, SessionViewBehaviors> {
    /// Assembles the system from its components.
    pub fn new(session_ir: session_ir::SessionIr<SessionIrBehaviors>, session_view: session_view::SessionView<SessionViewBehaviors>) -> Self {
        Self {
            session_ir,
            session_view,
            published: Vec::new(),
            cursor: 0,
        }
    }

    /// Everything published so far, in publication order — the system's observable record.
    pub fn published(&self) -> &[SystemEvent] {
        &self.published
    }

    /// Takes every event the pump has already delivered off the log, in publication order.
    ///
    /// A long-running shell calls this after each `pump`, or the log holds every event the
    /// process ever published. A `pump` returns with every logged event delivered: each
    /// reacting binding has had its attempt, and a binding whose attempt stopped holds the event in
    /// its own held-back list, not on the log. Events published since the last `pump` stay on the
    /// log, so the next `pump` still delivers them; taking never skips a binding.
    pub fn take_published(&mut self) -> Vec<SystemEvent> {
        let delivered: Vec<SystemEvent> = self.published.drain(..self.cursor).collect();
        self.cursor = 0;
        delivered
    }
}

impl<SessionIrBehaviors, SessionViewBehaviors> System<SessionIrBehaviors, SessionViewBehaviors>
where
    SessionIrBehaviors: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
    SessionViewBehaviors: agenticspec_types::view::obligations::AgentTurnsQuery + agenticspec_types::view::obligations::OverviewFiguresQuery + agenticspec_types::view::obligations::SessionAgentsQuery + agenticspec_types::view::obligations::SessionByIdQuery + agenticspec_types::view::obligations::SessionChecksQuery + agenticspec_types::view::obligations::SessionEventCountsQuery + agenticspec_types::view::obligations::SessionEventsQuery + agenticspec_types::view::obligations::SessionListQuery + agenticspec_types::view::obligations::SessionTimelineQuery + agenticspec_types::view::obligations::SessionTurnsQuery,
{
    /// Delivers until quiescent: collects every component's outbox onto the log. No binding
    /// reacts to anything this specification publishes, so collecting is the whole delivery.
    pub fn pump(&mut self) -> Result<(), agenticspec_types::obligation::UnmetObligation> {
        loop {
            self.collect();
            if self.cursor == self.published.len() {
                return Ok(());
            }
            self.cursor += 1;
        }
    }

    /// Moves every component's outbox onto the log, in component order.
    fn collect(&mut self) {
        for event in self.session_ir.drain_outbox() {
            self.published.push(SystemEvent::from(event));
        }
        for event in self.session_view.drain_outbox() {
            self.published.push(SystemEvent::from(event));
        }
    }
}
