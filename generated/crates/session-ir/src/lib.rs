// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! session-ir — the `session-ir` component of `agenticspec` v1.
//!
//! Replays the commands an adapter lifted from a transcript and holds the resulting session state.
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
}

/// session-ir — the port over the component's obligations.
///
/// `B` bundles every behaviour and query this component owes; constructing it over the domain's
/// `obligations::Unimplemented` yields a component that compiles and refuses, in the type system,
/// everything not yet implemented.
pub struct SessionIr<B> {
    behaviors: B,
    outbox: Vec<PublishedEvent>,
}

impl<B> SessionIr<B> {
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

impl<B> SessionIr<B>
where
    B: agenticspec_types::session::obligations::CompactContextBehavior + agenticspec_types::session::obligations::CompleteToolCallBehavior + agenticspec_types::session::obligations::DecideToolCallBehavior + agenticspec_types::session::obligations::EndAgentBehavior + agenticspec_types::session::obligations::EndSessionBehavior + agenticspec_types::session::obligations::EndTurnBehavior + agenticspec_types::session::obligations::OpenSessionBehavior + agenticspec_types::session::obligations::RecordExposureBehavior + agenticspec_types::session::obligations::RecordModelTurnBehavior + agenticspec_types::session::obligations::RequestToolCallBehavior + agenticspec_types::session::obligations::StartAgentBehavior + agenticspec_types::session::obligations::SubmitPromptBehavior + agenticspec_types::session::obligations::AgentsQuery + agenticspec_types::session::obligations::ExposuresQuery + agenticspec_types::session::obligations::SessionsQuery + agenticspec_types::session::obligations::ToolCallsQuery,
{
    /// Accepts `agenticspec.session.CompactContext`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn compact_context(&mut self, input: agenticspec_types::session::CompactContext) -> Result<agenticspec_types::session::CompactContextOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.compact_context(input)?;
        match &outcome {
            agenticspec_types::session::CompactContextOutcome::Compacted { context_compacted, .. } => {
                self.outbox.push(PublishedEvent::ContextCompacted(context_compacted.clone()));
            }
            agenticspec_types::session::CompactContextOutcome::NoSuchAgent { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.CompleteToolCall`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn complete_tool_call(&mut self, input: agenticspec_types::session::CompleteToolCall) -> Result<agenticspec_types::session::CompleteToolCallOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.complete_tool_call(input)?;
        match &outcome {
            agenticspec_types::session::CompleteToolCallOutcome::Succeeded { tool_call_completed, .. } => {
                self.outbox.push(PublishedEvent::ToolCallCompleted(tool_call_completed.clone()));
            }
            agenticspec_types::session::CompleteToolCallOutcome::Failed { tool_call_completed, .. } => {
                self.outbox.push(PublishedEvent::ToolCallCompleted(tool_call_completed.clone()));
            }
            agenticspec_types::session::CompleteToolCallOutcome::WrongState { .. } => {}
            agenticspec_types::session::CompleteToolCallOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.DecideToolCall`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn decide_tool_call(&mut self, input: agenticspec_types::session::DecideToolCall) -> Result<agenticspec_types::session::DecideToolCallOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.decide_tool_call(input)?;
        match &outcome {
            agenticspec_types::session::DecideToolCallOutcome::Accepted { tool_call_decided, .. } => {
                self.outbox.push(PublishedEvent::ToolCallDecided(tool_call_decided.clone()));
            }
            agenticspec_types::session::DecideToolCallOutcome::Refused { tool_call_decided, .. } => {
                self.outbox.push(PublishedEvent::ToolCallDecided(tool_call_decided.clone()));
            }
            agenticspec_types::session::DecideToolCallOutcome::WrongState { .. } => {}
            agenticspec_types::session::DecideToolCallOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.EndAgent`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn end_agent(&mut self, input: agenticspec_types::session::EndAgent) -> Result<agenticspec_types::session::EndAgentOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.end_agent(input)?;
        match &outcome {
            agenticspec_types::session::EndAgentOutcome::Ended { agent_ended, .. } => {
                self.outbox.push(PublishedEvent::AgentEnded(agent_ended.clone()));
            }
            agenticspec_types::session::EndAgentOutcome::WrongState { .. } => {}
            agenticspec_types::session::EndAgentOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.EndSession`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn end_session(&mut self, input: agenticspec_types::session::EndSession) -> Result<agenticspec_types::session::EndSessionOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.end_session(input)?;
        match &outcome {
            agenticspec_types::session::EndSessionOutcome::Ended { session_ended, .. } => {
                self.outbox.push(PublishedEvent::SessionEnded(session_ended.clone()));
            }
            agenticspec_types::session::EndSessionOutcome::WrongState { .. } => {}
            agenticspec_types::session::EndSessionOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.EndTurn`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn end_turn(&mut self, input: agenticspec_types::session::EndTurn) -> Result<agenticspec_types::session::EndTurnOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.end_turn(input)?;
        match &outcome {
            agenticspec_types::session::EndTurnOutcome::Ended { turn_ended, .. } => {
                self.outbox.push(PublishedEvent::TurnEnded(turn_ended.clone()));
            }
            agenticspec_types::session::EndTurnOutcome::WrongState { .. } => {}
            agenticspec_types::session::EndTurnOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.OpenSession`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn open_session(&mut self, input: agenticspec_types::session::OpenSession) -> Result<agenticspec_types::session::OpenSessionOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.open_session(input)?;
        match &outcome {
            agenticspec_types::session::OpenSessionOutcome::Opened { session_opened, .. } => {
                self.outbox.push(PublishedEvent::SessionOpened(session_opened.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.RecordExposure`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_exposure(&mut self, input: agenticspec_types::session::RecordExposure) -> Result<agenticspec_types::session::RecordExposureOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_exposure(input)?;
        match &outcome {
            agenticspec_types::session::RecordExposureOutcome::Recorded { exposure_detected, .. } => {
                self.outbox.push(PublishedEvent::ExposureDetected(exposure_detected.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.RecordModelTurn`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn record_model_turn(&mut self, input: agenticspec_types::session::RecordModelTurn) -> Result<agenticspec_types::session::RecordModelTurnOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.record_model_turn(input)?;
        match &outcome {
            agenticspec_types::session::RecordModelTurnOutcome::Recorded { model_turn_recorded, .. } => {
                self.outbox.push(PublishedEvent::ModelTurnRecorded(model_turn_recorded.clone()));
            }
            agenticspec_types::session::RecordModelTurnOutcome::NoSuchAgent { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.RequestToolCall`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn request_tool_call(&mut self, input: agenticspec_types::session::RequestToolCall) -> Result<agenticspec_types::session::RequestToolCallOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.request_tool_call(input)?;
        match &outcome {
            agenticspec_types::session::RequestToolCallOutcome::Requested { tool_call_requested, .. } => {
                self.outbox.push(PublishedEvent::ToolCallRequested(tool_call_requested.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.StartAgent`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn start_agent(&mut self, input: agenticspec_types::session::StartAgent) -> Result<agenticspec_types::session::StartAgentOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.start_agent(input)?;
        match &outcome {
            agenticspec_types::session::StartAgentOutcome::Started { agent_started, .. } => {
                self.outbox.push(PublishedEvent::AgentStarted(agent_started.clone()));
            }
        }
        Ok(outcome)
    }

    /// Accepts `agenticspec.session.SubmitPrompt`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn submit_prompt(&mut self, input: agenticspec_types::session::SubmitPrompt) -> Result<agenticspec_types::session::SubmitPromptOutcome, agenticspec_types::obligation::UnmetObligation> {
        let outcome = self.behaviors.submit_prompt(input)?;
        match &outcome {
            agenticspec_types::session::SubmitPromptOutcome::Submitted { prompt_submitted, .. } => {
                self.outbox.push(PublishedEvent::PromptSubmitted(prompt_submitted.clone()));
            }
            agenticspec_types::session::SubmitPromptOutcome::WrongState { .. } => {}
            agenticspec_types::session::SubmitPromptOutcome::WrongStateUnknownInstance => {}
        }
        Ok(outcome)
    }

    /// Serves `agenticspec.session.Agents` at `read_your_writes` consistency, from the owed projection.
    pub fn agents(&self) -> Result<Vec<agenticspec_types::session::Agents>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.agents()
    }

    /// Serves `agenticspec.session.Exposures` at `read_your_writes` consistency, from the owed projection.
    pub fn exposures(&self) -> Result<Vec<agenticspec_types::session::Exposures>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.exposures()
    }

    /// Serves `agenticspec.session.Sessions` at `read_your_writes` consistency, from the owed projection.
    pub fn sessions(&self) -> Result<Vec<agenticspec_types::session::Sessions>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.sessions()
    }

    /// Serves `agenticspec.session.ToolCalls` at `read_your_writes` consistency, from the owed projection.
    pub fn tool_calls(&self) -> Result<Vec<agenticspec_types::session::ToolCalls>, agenticspec_types::obligation::UnmetObligation> {
        self.behaviors.tool_calls()
    }
}
