// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! What the specification fully determines, generated: the behaviour of every command the plan
//! lists as generated, written against ports the implementor supplies.
//!
//! Storage is a port: one trait per entity, get, put and delete of a snapshot by identity. ess
//! preserves the trait; generated network entries supply an ephemeral store. `Context` carries the caller's attributes,
//! every identity and value the model says the implementation assigns, and the answer to each
//! `external:` branch. [`Generated`] implements every generated `…Behavior` trait over those ports
//! and forwards every behaviour and query the plan still owes to them, so it is a complete bundle
//! for every component port. To replace one generated behaviour, write a bundle of your own that
//! implements that trait and delegates the rest to a `Generated`.
//!
//! An `Err` from a generated behaviour is the typed refusal naming the command: the model declares
//! no outcome for the request (a guard is undecidable over it, or no declared branch answers it),
//! or — as `entity invariant` — the declared outcome would leave an entity breaking an invariant.

use crate::obligation::UnmetObligation;

/// Where `agenticspec.session.Agent` is stored — a port the implementor provides.
///
/// Keyed by the identity `agent_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait AgentStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::AgentId) -> Option<crate::session::AgentSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::session::AgentSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::AgentId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::session::AgentSnapshot>;
}

/// Where `agenticspec.session.Exposure` is stored — a port the implementor provides.
///
/// Keyed by the identity `exposure_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait ExposureStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::ExposureId) -> Option<crate::session::ExposureSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::session::ExposureSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::ExposureId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::session::ExposureSnapshot>;
}

/// Where `agenticspec.session.Session` is stored — a port the implementor provides.
///
/// Keyed by the identity `session_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait SessionStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::SessionId) -> Option<crate::session::SessionSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::session::SessionSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::SessionId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::session::SessionSnapshot>;
}

/// Where `agenticspec.session.ToolCall` is stored — a port the implementor provides.
///
/// Keyed by the identity `tool_call_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait ToolCallStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::ToolCallId) -> Option<crate::session::ToolCallSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::session::ToolCallSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::ToolCallId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::session::ToolCallSnapshot>;
}

/// Where `agenticspec.view.AgentListing` is stored — a port the implementor provides.
///
/// Keyed by the identity `agent_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait AgentListingStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::AgentId) -> Option<crate::view::AgentListingSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::AgentListingSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::AgentId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::AgentListingSnapshot>;
}

/// Where `agenticspec.view.EventRow` is stored — a port the implementor provides.
///
/// Keyed by the identity `event_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait EventRowStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::view::EventId) -> Option<crate::view::EventRowSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::EventRowSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::view::EventId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::EventRowSnapshot>;
}

/// Where `agenticspec.view.FigureBucket` is stored — a port the implementor provides.
///
/// Keyed by the identity `bucket_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait FigureBucketStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::view::BucketId) -> Option<crate::view::FigureBucketSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::FigureBucketSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::view::BucketId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::FigureBucketSnapshot>;
}

/// Where `agenticspec.view.SessionCheck` is stored — a port the implementor provides.
///
/// Keyed by the identity `check_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait SessionCheckStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::view::CheckId) -> Option<crate::view::SessionCheckSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::SessionCheckSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::view::CheckId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::SessionCheckSnapshot>;
}

/// Where `agenticspec.view.SessionListing` is stored — a port the implementor provides.
///
/// Keyed by the identity `session_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait SessionListingStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::session::SessionId) -> Option<crate::view::SessionListingSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::SessionListingSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::session::SessionId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::SessionListingSnapshot>;
}

/// Where `agenticspec.view.TimelineSlot` is stored — a port the implementor provides.
///
/// Keyed by the identity `slot_id`. Generated network entries supply an ephemeral implementation; durable storage remains a port.
pub trait TimelineSlotStorage {
    /// The instance with this identity, or `None` where none is stored.
    fn get(&self, identity: &crate::view::SlotId) -> Option<crate::view::TimelineSlotSnapshot>;

    /// Stores this instance under its identity, replacing what was held.
    fn put(&mut self, snapshot: crate::view::TimelineSlotSnapshot);

    /// Removes the instance with this identity.
    fn delete(&mut self, identity: &crate::view::SlotId);

    /// Every stored instance, in the order the store keeps them: the order a generated query
    /// answers an unordered view in.
    fn list(&self) -> Vec<crate::view::TimelineSlotSnapshot>;
}

/// Every generated behaviour of this workspace, over the ports `P` supplies.
///
/// `P` implements the storage trait of each entity a generated behaviour reads or writes, and
/// every `…Behavior` and `…Query` trait the plan still owes; `Generated<P>` forwards those to it.
pub struct Generated<P> {
    /// The storage ports, and every behaviour or query still owed.
    pub ports: P,
}

impl<P> Generated<P> {
    /// The generated behaviours, over `ports`.
    pub fn new(ports: P) -> Self {
        Self { ports }
    }
}

/// `agenticspec.session.CompactContext`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::CompactContextBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn compact_context(&mut self, input: crate::session::CompactContext) -> Result<crate::session::CompactContextOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentStorage::get(&self.ports, &input.agent_id).is_none() {
            return Ok(crate::session::CompactContextOutcome::NoSuchAgent { error: crate::session::UnknownAgent { agent_id: input.agent_id.clone() } });
        }
        // `compacted`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::session::CompactContextOutcome::NoSuchAgent { error: crate::session::UnknownAgent { agent_id: input.agent_id.clone() } });
        };
        let _ = &held;
        let mut next = held;
        next.data.context_tokens = 0;
        let answer = crate::session::CompactContextOutcome::Compacted { context_compacted: crate::session::ContextCompacted { agent_id: input.agent_id.clone(), context_tokens_before: input.context_tokens_before.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.CompleteToolCall`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::CompleteToolCallBehavior for Generated<P>
where
    P: ToolCallStorage,
{
    fn complete_tool_call(&mut self, input: crate::session::CompleteToolCall) -> Result<crate::session::CompleteToolCallOutcome, UnmetObligation> {
        let _ = &input;
        // `succeeded`: an accepting branch, in declaration order.
        if decided(equal(Some(&input.ok).map(|value| *value), Some(true)), "agenticspec.session.CompleteToolCall")? {
            let Some(held) = ToolCallStorage::get(&self.ports, &input.tool_call_id) else {
                return Ok(crate::session::CompleteToolCallOutcome::WrongStateUnknownInstance);
            };
            let _ = &held;
            let held_state = held.state;
            let moved = match held.refine() {
                crate::session::AnyToolCall::Accepted(instance) => crate::session::AnyToolCall::Succeeded(instance.succeed()),
                _ => return Ok(crate::session::CompleteToolCallOutcome::WrongState { error: crate::session::ToolCallStateConflict { state: held_state } }),
            };
            let next = moved.snapshot();
            let answer = crate::session::CompleteToolCallOutcome::Succeeded { tool_call_completed: crate::session::ToolCallCompleted { tool_call_id: input.tool_call_id.clone(), ok: input.ok.clone(), output: input.output.clone(), at: input.at.clone() } };
            ToolCallStorage::put(&mut self.ports, next);
            return Ok(answer);
        }
        // `failed`: the default.
        let Some(held) = ToolCallStorage::get(&self.ports, &input.tool_call_id) else {
            return Ok(crate::session::CompleteToolCallOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnyToolCall::Accepted(instance) => crate::session::AnyToolCall::Failed(instance.fail()),
            _ => return Ok(crate::session::CompleteToolCallOutcome::WrongState { error: crate::session::ToolCallStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::CompleteToolCallOutcome::Failed { tool_call_completed: crate::session::ToolCallCompleted { tool_call_id: input.tool_call_id.clone(), ok: input.ok.clone(), output: input.output.clone(), at: input.at.clone() } };
        ToolCallStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.DecideToolCall`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::DecideToolCallBehavior for Generated<P>
where
    P: ToolCallStorage,
{
    fn decide_tool_call(&mut self, input: crate::session::DecideToolCall) -> Result<crate::session::DecideToolCallOutcome, UnmetObligation> {
        let _ = &input;
        // `accepted`: an accepting branch, in declaration order.
        if decided(equal(Some(&input.decision).map(|value| match value { crate::session::Decision::Accept => "Accept", crate::session::Decision::Refuse => "Refuse" }.to_owned()), Some("Accept".to_owned())), "agenticspec.session.DecideToolCall")? {
            let Some(held) = ToolCallStorage::get(&self.ports, &input.tool_call_id) else {
                return Ok(crate::session::DecideToolCallOutcome::WrongStateUnknownInstance);
            };
            let _ = &held;
            let held_state = held.state;
            let moved = match held.refine() {
                crate::session::AnyToolCall::Requested(instance) => crate::session::AnyToolCall::Accepted(instance.accept()),
                _ => return Ok(crate::session::DecideToolCallOutcome::WrongState { error: crate::session::ToolCallStateConflict { state: held_state } }),
            };
            let next = moved.snapshot();
            let answer = crate::session::DecideToolCallOutcome::Accepted { tool_call_decided: crate::session::ToolCallDecided { tool_call_id: input.tool_call_id.clone(), decision: input.decision.clone(), decider: input.decider.clone(), reason: input.reason.clone(), at: input.at.clone() } };
            ToolCallStorage::put(&mut self.ports, next);
            return Ok(answer);
        }
        // `refused`: the default.
        let Some(held) = ToolCallStorage::get(&self.ports, &input.tool_call_id) else {
            return Ok(crate::session::DecideToolCallOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnyToolCall::Requested(instance) => crate::session::AnyToolCall::Refused(instance.refuse()),
            _ => return Ok(crate::session::DecideToolCallOutcome::WrongState { error: crate::session::ToolCallStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::DecideToolCallOutcome::Refused { tool_call_decided: crate::session::ToolCallDecided { tool_call_id: input.tool_call_id.clone(), decision: input.decision.clone(), decider: input.decider.clone(), reason: input.reason.clone(), at: input.at.clone() } };
        ToolCallStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.EndAgent`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::EndAgentBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn end_agent(&mut self, input: crate::session::EndAgent) -> Result<crate::session::EndAgentOutcome, UnmetObligation> {
        let _ = &input;
        // `ended`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::session::EndAgentOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnyAgent::Idle(instance) => crate::session::AnyAgent::Ended(instance.end()),
            crate::session::AnyAgent::Working(instance) => crate::session::AnyAgent::Ended(instance.end()),
            _ => return Ok(crate::session::EndAgentOutcome::WrongState { error: crate::session::AgentStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::EndAgentOutcome::Ended { agent_ended: crate::session::AgentEnded { agent_id: input.agent_id.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.EndSession`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::EndSessionBehavior for Generated<P>
where
    P: SessionStorage,
{
    fn end_session(&mut self, input: crate::session::EndSession) -> Result<crate::session::EndSessionOutcome, UnmetObligation> {
        let _ = &input;
        // `ended`: the default.
        let Some(held) = SessionStorage::get(&self.ports, &input.session_id) else {
            return Ok(crate::session::EndSessionOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnySession::Open(instance) => crate::session::AnySession::Ended(instance.end()),
            _ => return Ok(crate::session::EndSessionOutcome::WrongState { error: crate::session::SessionStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::EndSessionOutcome::Ended { session_ended: crate::session::SessionEnded { session_id: input.session_id.clone(), at: input.at.clone() } };
        SessionStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.EndTurn`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::EndTurnBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn end_turn(&mut self, input: crate::session::EndTurn) -> Result<crate::session::EndTurnOutcome, UnmetObligation> {
        let _ = &input;
        // `ended`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::session::EndTurnOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnyAgent::Working(instance) => crate::session::AnyAgent::Idle(instance.end_turn()),
            _ => return Ok(crate::session::EndTurnOutcome::WrongState { error: crate::session::AgentStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::EndTurnOutcome::Ended { turn_ended: crate::session::TurnEnded { agent_id: input.agent_id.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.OpenSession`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::OpenSessionBehavior for Generated<P>
where
    P: SessionStorage,
{
    fn open_session(&mut self, input: crate::session::OpenSession) -> Result<crate::session::OpenSessionOutcome, UnmetObligation> {
        let _ = &input;
        // `opened`: the default.
        let identity: crate::session::SessionId = input.session_id.clone();
        let data = crate::session::SessionData {
            session_id: identity.clone(),
            harness: input.harness.clone(),
            opened_at: input.at.clone(),
        };
        let answer = crate::session::OpenSessionOutcome::Opened { session_opened: crate::session::SessionOpened { session_id: identity.clone(), harness: input.harness.clone(), source: input.source.clone(), session_hash: input.session_hash.clone(), at: input.at.clone() } };
        SessionStorage::put(&mut self.ports, crate::session::AnySession::Open(crate::session::Session::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agenticspec.session.RecordExposure`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::RecordExposureBehavior for Generated<P>
where
    P: ExposureStorage,
{
    fn record_exposure(&mut self, input: crate::session::RecordExposure) -> Result<crate::session::RecordExposureOutcome, UnmetObligation> {
        let _ = &input;
        // `recorded`: the default.
        let identity: crate::session::ExposureId = input.exposure_id.clone();
        let data = crate::session::ExposureData {
            exposure_id: identity.clone(),
            agent_id: input.agent_id.clone(),
            channel: input.channel.clone(),
            class: input.class.clone(),
            kind: input.kind.clone(),
            alias: input.alias.clone(),
            verdict: input.verdict.clone(),
            tool_call_id: input.tool_call_id.clone(),
        };
        let answer = crate::session::RecordExposureOutcome::Recorded { exposure_detected: crate::session::ExposureDetected { exposure_id: identity.clone(), agent_id: input.agent_id.clone(), channel: input.channel.clone(), class: input.class.clone(), kind: input.kind.clone(), alias: input.alias.clone(), verdict: input.verdict.clone(), tool_call_id: input.tool_call_id.clone(), at: input.at.clone() } };
        ExposureStorage::put(&mut self.ports, crate::session::AnyExposure::Recorded(crate::session::Exposure::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agenticspec.session.RecordModelTurn`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::RecordModelTurnBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn record_model_turn(&mut self, input: crate::session::RecordModelTurn) -> Result<crate::session::RecordModelTurnOutcome, UnmetObligation> {
        let _ = &input;
        // The addressed row's existence, at step 2 of the one precedence rule.
        if AgentStorage::get(&self.ports, &input.agent_id).is_none() {
            return Ok(crate::session::RecordModelTurnOutcome::NoSuchAgent { error: crate::session::UnknownAgent { agent_id: input.agent_id.clone() } });
        }
        // `recorded`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::session::RecordModelTurnOutcome::NoSuchAgent { error: crate::session::UnknownAgent { agent_id: input.agent_id.clone() } });
        };
        let _ = &held;
        let mut next = held;
        next.data.model = Some(input.model.clone());
        next.data.context_tokens = input.context_tokens.clone();
        let answer = crate::session::RecordModelTurnOutcome::Recorded { model_turn_recorded: crate::session::ModelTurnRecorded { agent_id: input.agent_id.clone(), model: input.model.clone(), usage: input.usage.clone(), context_tokens: input.context_tokens.clone(), message_id: input.message_id.clone(), call_id: input.call_id.clone(), route: input.route.clone(), upstream: input.upstream.clone(), attempts: input.attempts.clone(), queue_wait_ms: input.queue_wait_ms.clone(), ttft_ms: input.ttft_ms.clone(), declared_cost_microdollars: input.declared_cost_microdollars.clone(), incomplete: input.incomplete.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.RequestToolCall`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::RequestToolCallBehavior for Generated<P>
where
    P: ToolCallStorage,
{
    fn request_tool_call(&mut self, input: crate::session::RequestToolCall) -> Result<crate::session::RequestToolCallOutcome, UnmetObligation> {
        let _ = &input;
        // `requested`: the default.
        let identity: crate::session::ToolCallId = input.tool_call_id.clone();
        let data = crate::session::ToolCallData {
            tool_call_id: identity.clone(),
            agent_id: input.agent_id.clone(),
            tool: input.tool.clone(),
            category: input.category.clone(),
        };
        let answer = crate::session::RequestToolCallOutcome::Requested { tool_call_requested: crate::session::ToolCallRequested { tool_call_id: identity.clone(), agent_id: input.agent_id.clone(), tool: input.tool.clone(), category: input.category.clone(), input: input.input.clone(), cwd: input.cwd.clone(), background: input.background.clone(), at: input.at.clone() } };
        ToolCallStorage::put(&mut self.ports, crate::session::AnyToolCall::Requested(crate::session::ToolCall::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agenticspec.session.StartAgent`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::StartAgentBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn start_agent(&mut self, input: crate::session::StartAgent) -> Result<crate::session::StartAgentOutcome, UnmetObligation> {
        let _ = &input;
        // `started`: the default.
        let identity: crate::session::AgentId = input.agent_id.clone();
        let data = crate::session::AgentData {
            agent_id: identity.clone(),
            session_id: input.session_id.clone(),
            parent_agent_id: input.parent_agent_id.clone(),
            role: input.role.clone(),
            model: None,
            context_tokens: 0,
        };
        let answer = crate::session::StartAgentOutcome::Started { agent_started: crate::session::AgentStarted { agent_id: identity.clone(), session_id: input.session_id.clone(), parent_agent_id: input.parent_agent_id.clone(), role: input.role.clone(), spawned_by: input.spawned_by.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, crate::session::AnyAgent::Idle(crate::session::Agent::new(data)).snapshot());
        return Ok(answer);
    }
}

/// `agenticspec.session.SubmitPrompt`, generated: every outcome is one the specification fully determines.
impl<P> crate::session::obligations::SubmitPromptBehavior for Generated<P>
where
    P: AgentStorage,
{
    fn submit_prompt(&mut self, input: crate::session::SubmitPrompt) -> Result<crate::session::SubmitPromptOutcome, UnmetObligation> {
        let _ = &input;
        // `submitted`: the default.
        let Some(held) = AgentStorage::get(&self.ports, &input.agent_id) else {
            return Ok(crate::session::SubmitPromptOutcome::WrongStateUnknownInstance);
        };
        let _ = &held;
        let held_state = held.state;
        let moved = match held.refine() {
            crate::session::AnyAgent::Idle(instance) => crate::session::AnyAgent::Working(instance.resume()),
            crate::session::AnyAgent::Working(instance) => crate::session::AnyAgent::Working(instance.resume()),
            _ => return Ok(crate::session::SubmitPromptOutcome::WrongState { error: crate::session::AgentStateConflict { state: held_state } }),
        };
        let next = moved.snapshot();
        let answer = crate::session::SubmitPromptOutcome::Submitted { prompt_submitted: crate::session::PromptSubmitted { agent_id: input.agent_id.clone(), author: input.author.clone(), attribution: input.attribution.clone(), class: input.class.clone(), content: input.content.clone(), at: input.at.clone() } };
        AgentStorage::put(&mut self.ports, next);
        return Ok(answer);
    }
}

/// `agenticspec.session.Agents`, generated: every row is one the specification fully determines from the stored `agenticspec.session.Agent`s.
impl<P> crate::session::obligations::AgentsQuery for Generated<P>
where
    P: AgentStorage,
{
    fn agents(&self) -> Result<Vec<crate::session::Agents>, UnmetObligation> {
        let admitted = AgentStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::session::Agents {
                agent_id: held.data.agent_id,
                session_id: held.data.session_id,
                parent_agent_id: held.data.parent_agent_id,
                role: held.data.role,
                model: held.data.model,
                context_tokens: held.data.context_tokens,
                state: held.state,
            })
            .collect())
    }
}

/// `agenticspec.session.Exposures`, generated: every row is one the specification fully determines from the stored `agenticspec.session.Exposure`s.
impl<P> crate::session::obligations::ExposuresQuery for Generated<P>
where
    P: ExposureStorage,
{
    fn exposures(&self) -> Result<Vec<crate::session::Exposures>, UnmetObligation> {
        let admitted = ExposureStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::session::Exposures {
                exposure_id: held.data.exposure_id,
                agent_id: held.data.agent_id,
                channel: held.data.channel,
                class: held.data.class,
                kind: held.data.kind,
                alias: held.data.alias,
                verdict: held.data.verdict,
                state: held.state,
            })
            .collect())
    }
}

/// `agenticspec.session.Sessions`, generated: every row is one the specification fully determines from the stored `agenticspec.session.Session`s.
impl<P> crate::session::obligations::SessionsQuery for Generated<P>
where
    P: SessionStorage,
{
    fn sessions(&self) -> Result<Vec<crate::session::Sessions>, UnmetObligation> {
        let admitted = SessionStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::session::Sessions {
                session_id: held.data.session_id,
                harness: held.data.harness,
                opened_at: held.data.opened_at,
                state: held.state,
            })
            .collect())
    }
}

/// `agenticspec.session.ToolCalls`, generated: every row is one the specification fully determines from the stored `agenticspec.session.ToolCall`s.
impl<P> crate::session::obligations::ToolCallsQuery for Generated<P>
where
    P: ToolCallStorage,
{
    fn tool_calls(&self) -> Result<Vec<crate::session::ToolCalls>, UnmetObligation> {
        let admitted = ToolCallStorage::list(&self.ports);
        Ok(admitted
            .into_iter()
            .map(|held| crate::session::ToolCalls {
                tool_call_id: held.data.tool_call_id,
                agent_id: held.data.agent_id,
                tool: held.data.tool,
                category: held.data.category,
                state: held.state,
            })
            .collect())
    }
}

/// `agenticspec.view.AgentTurns`, generated: every row is one the specification fully determines from the stored `agenticspec.view.EventRow`s.
impl<P> crate::view::obligations::AgentTurnsQuery for Generated<P>
where
    P: EventRowStorage,
{
    fn agent_turns(&self, param_0: crate::session::SessionId, param_1: crate::session::AgentId) -> Result<Vec<crate::view::AgentTurns>, UnmetObligation> {
        let mut admitted = EventRowStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| all(&[equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())), equal(Some(&held.data.agent).map(|value| &value.0).map(|value| value.clone()), Some(&param_1).map(|value| &value.0).map(|value| value.clone())), equal(Some(&held.data.kind).map(|value| match value { crate::view::EventKind::AgentStarted => "agent_started", crate::view::EventKind::Prompt => "prompt", crate::view::EventKind::ModelTurn => "model_turn", crate::view::EventKind::ToolCall => "tool_call", crate::view::EventKind::Decision => "decision", crate::view::EventKind::ToolResult => "tool_result", crate::view::EventKind::TurnEnd => "turn_end", crate::view::EventKind::Compaction => "compaction", crate::view::EventKind::Exposure => "exposure", crate::view::EventKind::CacheBust => "cache_bust" }.to_owned()), Some("model_turn".to_owned()))]) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::AgentTurns {
                event_id: held.data.event_id,
                session_id: held.data.session_id,
                seq: held.data.seq,
                ord: held.data.ord,
                at: held.data.at,
                agent: held.data.agent,
                kind: held.data.kind,
                notable: held.data.notable,
                label: held.data.label,
                detail: held.data.detail,
                call: held.data.call,
                ok: held.data.ok,
                context_tokens: held.data.context_tokens,
                output_tokens: held.data.output_tokens,
                cache_read_tokens: held.data.cache_read_tokens,
                cache_write_tokens: held.data.cache_write_tokens,
                cost_microdollars: held.data.cost_microdollars,
                bytes: held.data.bytes,
                agent_cost_usd: held.data.agent_cost_usd,
                session_cost_usd: held.data.session_cost_usd,
            })
            .collect())
    }
}

/// `agenticspec.view.OverviewFigures`, generated: every row is one the specification fully determines from the stored `agenticspec.view.FigureBucket`s.
impl<P> crate::view::obligations::OverviewFiguresQuery for Generated<P>
where
    P: FigureBucketStorage,
{
    fn overview_figures(&self, param_0: crate::primitives::Timestamp, param_1: i64) -> Result<Vec<crate::view::OverviewFigures>, UnmetObligation> {
        let mut admitted = FigureBucketStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| all(&[compare_instants(Some(&held.data.start).map(|value| value.0.clone()), Some(&param_0).map(|value| value.0.clone()), core::cmp::Ordering::is_ge), compare_numbers(Some(&held.data.bucket_seconds).map(|value| value.to_string()), Some(&param_1).map(|value| value.to_string()), core::cmp::Ordering::is_eq)]) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::OverviewFigures {
                bucket_id: held.data.bucket_id,
                start: held.data.start,
                bucket_seconds: held.data.bucket_seconds,
                sessions: held.data.sessions,
                cost_usd: held.data.cost_usd,
                working_seconds: held.data.working_seconds,
                waiting_share: held.data.waiting_share,
                cache_hit_share: held.data.cache_hit_share,
                tool_calls: held.data.tool_calls,
                refused_tool_calls: held.data.refused_tool_calls,
            })
            .collect())
    }
}

/// `agenticspec.view.SessionAgents`, generated: every row is one the specification fully determines from the stored `agenticspec.view.AgentListing`s.
impl<P> crate::view::obligations::SessionAgentsQuery for Generated<P>
where
    P: AgentListingStorage,
{
    fn session_agents(&self, param_0: crate::session::SessionId) -> Result<Vec<crate::view::SessionAgents>, UnmetObligation> {
        let mut admitted = AgentListingStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::SessionAgents {
                agent_id: held.data.agent_id,
                session_id: held.data.session_id,
                parent: held.data.parent,
                role: held.data.role,
                model: held.data.model,
                started_at: held.data.started_at,
                turns: held.data.turns,
                context_tokens: held.data.context_tokens,
                context_max: held.data.context_max,
                output_tokens: held.data.output_tokens,
                cost_usd: held.data.cost_usd,
                tool_calls: held.data.tool_calls,
                busts: held.data.busts,
            })
            .collect())
    }
}

/// `agenticspec.view.SessionById`, generated: every row is one the specification fully determines from the stored `agenticspec.view.SessionListing`s.
impl<P> crate::view::obligations::SessionByIdQuery for Generated<P>
where
    P: SessionListingStorage,
{
    fn session_by_id(&self, param_0: crate::session::SessionId) -> Result<Vec<crate::view::SessionById>, UnmetObligation> {
        let mut admitted = SessionListingStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::SessionById {
                session_id: held.data.session_id,
                project: held.data.project,
                title: held.data.title,
                harness: held.data.harness,
                first_at: held.data.first_at,
                last_at: held.data.last_at,
                agents: held.data.agents,
                cost_usd: held.data.cost_usd,
                output_tokens: held.data.output_tokens,
                context_max: held.data.context_max,
                cache_hit_share: held.data.cache_hit_share,
                busts: held.data.busts,
                human_prompts: held.data.human_prompts,
                tool_calls: held.data.tool_calls,
                refused_tool_calls: held.data.refused_tool_calls,
                exposures: held.data.exposures,
                working_seconds: held.data.working_seconds,
                waiting_on_human_seconds: held.data.waiting_on_human_seconds,
                waiting_share: held.data.waiting_share,
                kpis: held.data.kpis,
            })
            .collect())
    }
}

/// `agenticspec.view.SessionChecks`, generated: every row is one the specification fully determines from the stored `agenticspec.view.SessionCheck`s.
impl<P> crate::view::obligations::SessionChecksQuery for Generated<P>
where
    P: SessionCheckStorage,
{
    fn session_checks(&self, param_0: crate::session::SessionId) -> Result<Vec<crate::view::SessionChecks>, UnmetObligation> {
        let mut admitted = SessionCheckStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::SessionChecks {
                check_id: held.data.check_id,
                session_id: held.data.session_id,
                check: held.data.check,
                about: held.data.about,
                severity: held.data.severity,
                verdict: held.data.verdict,
                rows: held.data.rows,
                possible: held.data.possible,
            })
            .collect())
    }
}

impl<P: crate::view::obligations::SessionEventCountsQuery> crate::view::obligations::SessionEventCountsQuery for Generated<P> {
    fn session_event_counts(&self, session_id: crate::session::SessionId, agent: Option<crate::session::AgentId>) -> Result<Vec<crate::view::SessionEventCounts>, UnmetObligation> {
        crate::view::obligations::SessionEventCountsQuery::session_event_counts(&self.ports, session_id, agent)
    }
}

impl<P: crate::view::obligations::SessionEventsQuery> crate::view::obligations::SessionEventsQuery for Generated<P> {
    fn session_events(&self, session_id: crate::session::SessionId, agent: Option<crate::session::AgentId>, kind: Option<crate::view::EventKind>, notable: Option<bool>, before: Option<i64>) -> Result<Vec<crate::view::SessionEvents>, UnmetObligation> {
        crate::view::obligations::SessionEventsQuery::session_events(&self.ports, session_id, agent, kind, notable, before)
    }
}

impl<P: crate::view::obligations::SessionListQuery> crate::view::obligations::SessionListQuery for Generated<P> {
    fn session_list(&self, since: Option<crate::primitives::Timestamp>, search: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<crate::view::SessionList>, UnmetObligation> {
        crate::view::obligations::SessionListQuery::session_list(&self.ports, since, search, limit, offset)
    }
}

/// `agenticspec.view.SessionTimeline`, generated: every row is one the specification fully determines from the stored `agenticspec.view.TimelineSlot`s.
impl<P> crate::view::obligations::SessionTimelineQuery for Generated<P>
where
    P: TimelineSlotStorage,
{
    fn session_timeline(&self, param_0: crate::session::SessionId, param_1: i64) -> Result<Vec<crate::view::SessionTimeline>, UnmetObligation> {
        let mut admitted = TimelineSlotStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| all(&[equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())), compare_numbers(Some(&held.data.slot_seconds).map(|value| value.to_string()), Some(&param_1).map(|value| value.to_string()), core::cmp::Ordering::is_eq)]) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::SessionTimeline {
                slot_id: held.data.slot_id,
                session_id: held.data.session_id,
                agent: held.data.agent,
                slot_seconds: held.data.slot_seconds,
                start: held.data.start,
                end: held.data.end,
                events: held.data.events,
                read_calls: held.data.read_calls,
                search_calls: held.data.search_calls,
                edit_calls: held.data.edit_calls,
                execute_calls: held.data.execute_calls,
                network_calls: held.data.network_calls,
                spawn_calls: held.data.spawn_calls,
                message_calls: held.data.message_calls,
                plan_calls: held.data.plan_calls,
                other_calls: held.data.other_calls,
                human_prompts: held.data.human_prompts,
                relayed_prompts: held.data.relayed_prompts,
                refusals: held.data.refusals,
                busts: held.data.busts,
                compactions: held.data.compactions,
                context_max: held.data.context_max,
                agent_cost_usd: held.data.agent_cost_usd,
                session_cost_usd: held.data.session_cost_usd,
            })
            .collect())
    }
}

/// `agenticspec.view.SessionTurns`, generated: every row is one the specification fully determines from the stored `agenticspec.view.EventRow`s.
impl<P> crate::view::obligations::SessionTurnsQuery for Generated<P>
where
    P: EventRowStorage,
{
    fn session_turns(&self, param_0: crate::session::SessionId) -> Result<Vec<crate::view::SessionTurns>, UnmetObligation> {
        let mut admitted = EventRowStorage::list(&self.ports);
        // `filter:` shows a row where it holds; false or unknown hides it.
        admitted.retain(|held| all(&[equal(Some(&held.data.session_id).map(|value| &value.0).map(|value| value.clone()), Some(&param_0).map(|value| &value.0).map(|value| value.clone())), equal(Some(&held.data.kind).map(|value| match value { crate::view::EventKind::AgentStarted => "agent_started", crate::view::EventKind::Prompt => "prompt", crate::view::EventKind::ModelTurn => "model_turn", crate::view::EventKind::ToolCall => "tool_call", crate::view::EventKind::Decision => "decision", crate::view::EventKind::ToolResult => "tool_result", crate::view::EventKind::TurnEnd => "turn_end", crate::view::EventKind::Compaction => "compaction", crate::view::EventKind::Exposure => "exposure", crate::view::EventKind::CacheBust => "cache_bust" }.to_owned()), Some("model_turn".to_owned()))]) == Some(true));
        Ok(admitted
            .into_iter()
            .map(|held| crate::view::SessionTurns {
                event_id: held.data.event_id,
                session_id: held.data.session_id,
                seq: held.data.seq,
                ord: held.data.ord,
                at: held.data.at,
                agent: held.data.agent,
                kind: held.data.kind,
                notable: held.data.notable,
                label: held.data.label,
                detail: held.data.detail,
                call: held.data.call,
                ok: held.data.ok,
                context_tokens: held.data.context_tokens,
                output_tokens: held.data.output_tokens,
                cache_read_tokens: held.data.cache_read_tokens,
                cache_write_tokens: held.data.cache_write_tokens,
                cost_microdollars: held.data.cost_microdollars,
                bytes: held.data.bytes,
                agent_cost_usd: held.data.agent_cost_usd,
                session_cost_usd: held.data.session_cost_usd,
            })
            .collect())
    }
}

/// The typed refusal of a request the model declares no outcome for.
fn undeclared(source: &'static str) -> UnmetObligation {
    let capability = "command behaviour";
    UnmetObligation { capability, source }
}

/// A guard's truth, where it has one: Unknown selects no branch, so the model declares no outcome.
fn decided(truth: Option<bool>, command: &'static str) -> Result<bool, UnmetObligation> {
    truth.ok_or_else(|| undeclared(command))
}

/// Three-valued conjunction: false wins, then Unknown.
fn all(truths: &[Option<bool>]) -> Option<bool> {
    if truths.contains(&Some(false)) {
        Some(false)
    } else if truths.contains(&None) {
        None
    } else {
        Some(true)
    }
}

/// Equality of two read values; an unread one is Unknown.
fn equal<T: PartialEq>(left: Option<T>, right: Option<T>) -> Option<bool> {
    Some(left? == right?)
}

/// A decimal rendering as its sign, its whole digits and its fraction digits, without the zeros
/// that do not change its value; `None` where it is not a plain decimal.
fn number_parts(text: &str) -> Option<(bool, String, String)> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    if whole.is_empty() && fraction.is_empty() {
        return None;
    }
    if !whole.bytes().chain(fraction.bytes()).all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let whole = whole.trim_start_matches('0').to_owned();
    let fraction = fraction.trim_end_matches('0').to_owned();
    let zero = whole.is_empty() && fraction.is_empty();
    Some((negative && !zero, whole, fraction))
}

/// Compares two decimal renderings exactly; an unread or unparsable one is Unknown.
fn compare_numbers(
    left: Option<String>,
    right: Option<String>,
    accepts: fn(core::cmp::Ordering) -> bool,
) -> Option<bool> {
    let (left, right) = (number_parts(&left?)?, number_parts(&right?)?);
    let magnitude = left
        .1
        .len()
        .cmp(&right.1.len())
        .then_with(|| left.1.cmp(&right.1))
        .then_with(|| left.2.cmp(&right.2));
    let ordering = match (left.0, right.0) {
        (false, false) => magnitude,
        (true, true) => magnitude.reverse(),
        (true, false) => core::cmp::Ordering::Less,
        (false, true) => core::cmp::Ordering::Greater,
    };
    Some(accepts(ordering))
}

/// Compares two RFC 3339 renderings by the instant each names; an unread one, or one that names
/// no instant, is Unknown — never ordered by its spelling.
fn compare_instants(
    left: Option<String>,
    right: Option<String>,
    accepts: fn(core::cmp::Ordering) -> bool,
) -> Option<bool> {
    Some(accepts(instant_of(&left?)?.cmp(&instant_of(&right?)?)))
}

/// The instant an RFC 3339 `date-time` names, as seconds from the epoch and nanoseconds.
fn instant_of(text: &str) -> Option<(i64, u32)> {
    let bytes = text.as_bytes();
    let digits = |from: usize, to: usize| -> Option<u32> {
        let slice = bytes.get(from..to)?;
        if slice.is_empty() || !slice.iter().all(u8::is_ascii_digit) {
            return None;
        }
        slice
            .iter()
            .try_fold(0u32, |total, digit| Some(total * 10 + u32::from(digit - b'0')))
    };
    let at = |index: usize, expected: &[u8]| bytes.get(index).is_some_and(|b| expected.contains(b));
    if !(at(4, b"-") && at(7, b"-") && at(10, b"Tt") && at(13, b":") && at(16, b":")) {
        return None;
    }
    let (year, month, day) = (i64::from(digits(0, 4)?), digits(5, 7)?, digits(8, 10)?);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let length = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    let (hour, minute, second) = (digits(11, 13)?, digits(14, 16)?, digits(17, 19)?);
    if day < 1 || day > length || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let mut position = 19;
    let mut nanos = 0u32;
    if at(position, b".") {
        let mut end = position + 1;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        let width = end - position - 1;
        if width == 0 || width > 9 {
            return None;
        }
        nanos = digits(position + 1, end)? * 10u32.pow(u32::try_from(9 - width).ok()?);
        position = end;
    }
    let offset = match bytes.get(position..)? {
        b"Z" | b"z" => 0i64,
        [sign @ (b'+' | b'-'), _, _, b':', _, _] => {
            let (hours, minutes) = (digits(position + 1, position + 3)?, digits(position + 4, position + 6)?);
            if hours > 23 || minutes > 59 {
                return None;
            }
            let magnitude = i64::from(hours * 3600 + minutes * 60);
            if *sign == b'-' { -magnitude } else { magnitude }
        }
        _ => return None,
    };
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = shifted.div_euclid(400);
    let of_era = shifted - era * 400;
    let march = i64::from(if month > 2 { month - 3 } else { month + 9 });
    let of_year = (153 * march + 2) / 5 + i64::from(day) - 1;
    let of_cycle = of_era * 365 + of_era / 4 - of_era / 100 + of_year;
    let days = era * 146_097 + of_cycle - 719_468;
    Some((days * 86_400 + i64::from(hour * 3600 + minute * 60 + second) - offset, nanos))
}
