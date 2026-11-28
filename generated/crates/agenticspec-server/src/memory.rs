// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! Ephemeral stores for the generated network entry points; no durability.

/// Ephemeral storage of `agenticspec.session.Agent`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryAgentStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::session::AgentSnapshot>>>);
impl agenticspec_types::behaviour::AgentStorage for MemoryAgentStorage {
fn get(&self, identity: &agenticspec_types::session::AgentId) -> Option<agenticspec_types::session::AgentSnapshot> { let key = memory_key_103(identity); self.0.borrow().iter().find(|row| memory_key_103(&row.data.agent_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::session::AgentSnapshot) { self.delete(&snapshot.data.agent_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_103(&row.data.agent_id).cmp(&memory_key_103(&other.data.agent_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::AgentId) { let key = memory_key_103(identity); self.0.borrow_mut().retain(|row| memory_key_103(&row.data.agent_id) != key); }
fn list(&self) -> Vec<agenticspec_types::session::AgentSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::AgentStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::AgentId) -> Option<agenticspec_types::session::AgentSnapshot> { agenticspec_types::behaviour::AgentStorage::get(&self.agent_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::session::AgentSnapshot) { agenticspec_types::behaviour::AgentStorage::put(&mut self.agent_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::AgentId) { agenticspec_types::behaviour::AgentStorage::delete(&mut self.agent_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::session::AgentSnapshot> { agenticspec_types::behaviour::AgentStorage::list(&self.agent_storage) }
}

/// Ephemeral storage of `agenticspec.session.Exposure`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryExposureStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::session::ExposureSnapshot>>>);
impl agenticspec_types::behaviour::ExposureStorage for MemoryExposureStorage {
fn get(&self, identity: &agenticspec_types::session::ExposureId) -> Option<agenticspec_types::session::ExposureSnapshot> { let key = memory_key_112(identity); self.0.borrow().iter().find(|row| memory_key_112(&row.data.exposure_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::session::ExposureSnapshot) { self.delete(&snapshot.data.exposure_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_112(&row.data.exposure_id).cmp(&memory_key_112(&other.data.exposure_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::ExposureId) { let key = memory_key_112(identity); self.0.borrow_mut().retain(|row| memory_key_112(&row.data.exposure_id) != key); }
fn list(&self) -> Vec<agenticspec_types::session::ExposureSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::ExposureStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::ExposureId) -> Option<agenticspec_types::session::ExposureSnapshot> { agenticspec_types::behaviour::ExposureStorage::get(&self.exposure_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::session::ExposureSnapshot) { agenticspec_types::behaviour::ExposureStorage::put(&mut self.exposure_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::ExposureId) { agenticspec_types::behaviour::ExposureStorage::delete(&mut self.exposure_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::session::ExposureSnapshot> { agenticspec_types::behaviour::ExposureStorage::list(&self.exposure_storage) }
}

/// Ephemeral storage of `agenticspec.session.Session`, shared by clones.
#[derive(Clone, Default)]
pub struct MemorySessionStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::session::SessionSnapshot>>>);
impl agenticspec_types::behaviour::SessionStorage for MemorySessionStorage {
fn get(&self, identity: &agenticspec_types::session::SessionId) -> Option<agenticspec_types::session::SessionSnapshot> { let key = memory_key_118(identity); self.0.borrow().iter().find(|row| memory_key_118(&row.data.session_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::session::SessionSnapshot) { self.delete(&snapshot.data.session_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_118(&row.data.session_id).cmp(&memory_key_118(&other.data.session_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::SessionId) { let key = memory_key_118(identity); self.0.borrow_mut().retain(|row| memory_key_118(&row.data.session_id) != key); }
fn list(&self) -> Vec<agenticspec_types::session::SessionSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::SessionStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::SessionId) -> Option<agenticspec_types::session::SessionSnapshot> { agenticspec_types::behaviour::SessionStorage::get(&self.session_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::session::SessionSnapshot) { agenticspec_types::behaviour::SessionStorage::put(&mut self.session_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::SessionId) { agenticspec_types::behaviour::SessionStorage::delete(&mut self.session_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::session::SessionSnapshot> { agenticspec_types::behaviour::SessionStorage::list(&self.session_storage) }
}

/// Ephemeral storage of `agenticspec.session.ToolCall`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryToolCallStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::session::ToolCallSnapshot>>>);
impl agenticspec_types::behaviour::ToolCallStorage for MemoryToolCallStorage {
fn get(&self, identity: &agenticspec_types::session::ToolCallId) -> Option<agenticspec_types::session::ToolCallSnapshot> { let key = memory_key_121(identity); self.0.borrow().iter().find(|row| memory_key_121(&row.data.tool_call_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::session::ToolCallSnapshot) { self.delete(&snapshot.data.tool_call_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_121(&row.data.tool_call_id).cmp(&memory_key_121(&other.data.tool_call_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::ToolCallId) { let key = memory_key_121(identity); self.0.borrow_mut().retain(|row| memory_key_121(&row.data.tool_call_id) != key); }
fn list(&self) -> Vec<agenticspec_types::session::ToolCallSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::ToolCallStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::ToolCallId) -> Option<agenticspec_types::session::ToolCallSnapshot> { agenticspec_types::behaviour::ToolCallStorage::get(&self.tool_call_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::session::ToolCallSnapshot) { agenticspec_types::behaviour::ToolCallStorage::put(&mut self.tool_call_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::ToolCallId) { agenticspec_types::behaviour::ToolCallStorage::delete(&mut self.tool_call_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::session::ToolCallSnapshot> { agenticspec_types::behaviour::ToolCallStorage::list(&self.tool_call_storage) }
}

/// Ephemeral storage of `agenticspec.view.AgentListing`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryAgentListingStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::AgentListingSnapshot>>>);
impl agenticspec_types::behaviour::AgentListingStorage for MemoryAgentListingStorage {
fn get(&self, identity: &agenticspec_types::session::AgentId) -> Option<agenticspec_types::view::AgentListingSnapshot> { let key = memory_key_103(identity); self.0.borrow().iter().find(|row| memory_key_103(&row.data.agent_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::AgentListingSnapshot) { self.delete(&snapshot.data.agent_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_103(&row.data.agent_id).cmp(&memory_key_103(&other.data.agent_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::AgentId) { let key = memory_key_103(identity); self.0.borrow_mut().retain(|row| memory_key_103(&row.data.agent_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::AgentListingSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::AgentListingStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::AgentId) -> Option<agenticspec_types::view::AgentListingSnapshot> { agenticspec_types::behaviour::AgentListingStorage::get(&self.agent_listing_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::AgentListingSnapshot) { agenticspec_types::behaviour::AgentListingStorage::put(&mut self.agent_listing_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::AgentId) { agenticspec_types::behaviour::AgentListingStorage::delete(&mut self.agent_listing_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::AgentListingSnapshot> { agenticspec_types::behaviour::AgentListingStorage::list(&self.agent_listing_storage) }
}

/// Ephemeral storage of `agenticspec.view.EventRow`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryEventRowStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::EventRowSnapshot>>>);
impl agenticspec_types::behaviour::EventRowStorage for MemoryEventRowStorage {
fn get(&self, identity: &agenticspec_types::view::EventId) -> Option<agenticspec_types::view::EventRowSnapshot> { let key = memory_key_133(identity); self.0.borrow().iter().find(|row| memory_key_133(&row.data.event_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::EventRowSnapshot) { self.delete(&snapshot.data.event_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_133(&row.data.event_id).cmp(&memory_key_133(&other.data.event_id))); }
fn delete(&mut self, identity: &agenticspec_types::view::EventId) { let key = memory_key_133(identity); self.0.borrow_mut().retain(|row| memory_key_133(&row.data.event_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::EventRowSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::EventRowStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::view::EventId) -> Option<agenticspec_types::view::EventRowSnapshot> { agenticspec_types::behaviour::EventRowStorage::get(&self.event_row_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::EventRowSnapshot) { agenticspec_types::behaviour::EventRowStorage::put(&mut self.event_row_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::view::EventId) { agenticspec_types::behaviour::EventRowStorage::delete(&mut self.event_row_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::EventRowSnapshot> { agenticspec_types::behaviour::EventRowStorage::list(&self.event_row_storage) }
}

/// Ephemeral storage of `agenticspec.view.FigureBucket`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryFigureBucketStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::FigureBucketSnapshot>>>);
impl agenticspec_types::behaviour::FigureBucketStorage for MemoryFigureBucketStorage {
fn get(&self, identity: &agenticspec_types::view::BucketId) -> Option<agenticspec_types::view::FigureBucketSnapshot> { let key = memory_key_127(identity); self.0.borrow().iter().find(|row| memory_key_127(&row.data.bucket_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::FigureBucketSnapshot) { self.delete(&snapshot.data.bucket_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_127(&row.data.bucket_id).cmp(&memory_key_127(&other.data.bucket_id))); }
fn delete(&mut self, identity: &agenticspec_types::view::BucketId) { let key = memory_key_127(identity); self.0.borrow_mut().retain(|row| memory_key_127(&row.data.bucket_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::FigureBucketSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::FigureBucketStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::view::BucketId) -> Option<agenticspec_types::view::FigureBucketSnapshot> { agenticspec_types::behaviour::FigureBucketStorage::get(&self.figure_bucket_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::FigureBucketSnapshot) { agenticspec_types::behaviour::FigureBucketStorage::put(&mut self.figure_bucket_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::view::BucketId) { agenticspec_types::behaviour::FigureBucketStorage::delete(&mut self.figure_bucket_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::FigureBucketSnapshot> { agenticspec_types::behaviour::FigureBucketStorage::list(&self.figure_bucket_storage) }
}

/// Ephemeral storage of `agenticspec.view.SessionCheck`, shared by clones.
#[derive(Clone, Default)]
pub struct MemorySessionCheckStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::SessionCheckSnapshot>>>);
impl agenticspec_types::behaviour::SessionCheckStorage for MemorySessionCheckStorage {
fn get(&self, identity: &agenticspec_types::view::CheckId) -> Option<agenticspec_types::view::SessionCheckSnapshot> { let key = memory_key_128(identity); self.0.borrow().iter().find(|row| memory_key_128(&row.data.check_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::SessionCheckSnapshot) { self.delete(&snapshot.data.check_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_128(&row.data.check_id).cmp(&memory_key_128(&other.data.check_id))); }
fn delete(&mut self, identity: &agenticspec_types::view::CheckId) { let key = memory_key_128(identity); self.0.borrow_mut().retain(|row| memory_key_128(&row.data.check_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::SessionCheckSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::SessionCheckStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::view::CheckId) -> Option<agenticspec_types::view::SessionCheckSnapshot> { agenticspec_types::behaviour::SessionCheckStorage::get(&self.session_check_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::SessionCheckSnapshot) { agenticspec_types::behaviour::SessionCheckStorage::put(&mut self.session_check_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::view::CheckId) { agenticspec_types::behaviour::SessionCheckStorage::delete(&mut self.session_check_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::SessionCheckSnapshot> { agenticspec_types::behaviour::SessionCheckStorage::list(&self.session_check_storage) }
}

/// Ephemeral storage of `agenticspec.view.SessionListing`, shared by clones.
#[derive(Clone, Default)]
pub struct MemorySessionListingStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::SessionListingSnapshot>>>);
impl agenticspec_types::behaviour::SessionListingStorage for MemorySessionListingStorage {
fn get(&self, identity: &agenticspec_types::session::SessionId) -> Option<agenticspec_types::view::SessionListingSnapshot> { let key = memory_key_118(identity); self.0.borrow().iter().find(|row| memory_key_118(&row.data.session_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::SessionListingSnapshot) { self.delete(&snapshot.data.session_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_118(&row.data.session_id).cmp(&memory_key_118(&other.data.session_id))); }
fn delete(&mut self, identity: &agenticspec_types::session::SessionId) { let key = memory_key_118(identity); self.0.borrow_mut().retain(|row| memory_key_118(&row.data.session_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::SessionListingSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::SessionListingStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::session::SessionId) -> Option<agenticspec_types::view::SessionListingSnapshot> { agenticspec_types::behaviour::SessionListingStorage::get(&self.session_listing_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::SessionListingSnapshot) { agenticspec_types::behaviour::SessionListingStorage::put(&mut self.session_listing_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::session::SessionId) { agenticspec_types::behaviour::SessionListingStorage::delete(&mut self.session_listing_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::SessionListingSnapshot> { agenticspec_types::behaviour::SessionListingStorage::list(&self.session_listing_storage) }
}

/// Ephemeral storage of `agenticspec.view.TimelineSlot`, shared by clones.
#[derive(Clone, Default)]
pub struct MemoryTimelineSlotStorage(std::rc::Rc<std::cell::RefCell<Vec<agenticspec_types::view::TimelineSlotSnapshot>>>);
impl agenticspec_types::behaviour::TimelineSlotStorage for MemoryTimelineSlotStorage {
fn get(&self, identity: &agenticspec_types::view::SlotId) -> Option<agenticspec_types::view::TimelineSlotSnapshot> { let key = memory_key_139(identity); self.0.borrow().iter().find(|row| memory_key_139(&row.data.slot_id) == key).cloned() }
fn put(&mut self, snapshot: agenticspec_types::view::TimelineSlotSnapshot) { self.delete(&snapshot.data.slot_id); let mut rows = self.0.borrow_mut(); rows.push(snapshot); rows.sort_by(|row, other| memory_key_139(&row.data.slot_id).cmp(&memory_key_139(&other.data.slot_id))); }
fn delete(&mut self, identity: &agenticspec_types::view::SlotId) { let key = memory_key_139(identity); self.0.borrow_mut().retain(|row| memory_key_139(&row.data.slot_id) != key); }
fn list(&self) -> Vec<agenticspec_types::view::TimelineSlotSnapshot> { self.0.borrow().clone() }
}
impl agenticspec_types::behaviour::TimelineSlotStorage for MemoryPorts {
fn get(&self, identity: &agenticspec_types::view::SlotId) -> Option<agenticspec_types::view::TimelineSlotSnapshot> { agenticspec_types::behaviour::TimelineSlotStorage::get(&self.timeline_slot_storage, identity) }
fn put(&mut self, snapshot: agenticspec_types::view::TimelineSlotSnapshot) { agenticspec_types::behaviour::TimelineSlotStorage::put(&mut self.timeline_slot_storage, snapshot); }
fn delete(&mut self, identity: &agenticspec_types::view::SlotId) { agenticspec_types::behaviour::TimelineSlotStorage::delete(&mut self.timeline_slot_storage, identity); }
fn list(&self) -> Vec<agenticspec_types::view::TimelineSlotSnapshot> { agenticspec_types::behaviour::TimelineSlotStorage::list(&self.timeline_slot_storage) }
}

/// Ports shared by generated components. Values disappear when the process exits.
#[derive(Clone, Default)]
pub struct MemoryPorts {
/// Storage of `agenticspec.session.Agent`.
pub agent_storage: MemoryAgentStorage,
/// Storage of `agenticspec.session.Exposure`.
pub exposure_storage: MemoryExposureStorage,
/// Storage of `agenticspec.session.Session`.
pub session_storage: MemorySessionStorage,
/// Storage of `agenticspec.session.ToolCall`.
pub tool_call_storage: MemoryToolCallStorage,
/// Storage of `agenticspec.view.AgentListing`.
pub agent_listing_storage: MemoryAgentListingStorage,
/// Storage of `agenticspec.view.EventRow`.
pub event_row_storage: MemoryEventRowStorage,
/// Storage of `agenticspec.view.FigureBucket`.
pub figure_bucket_storage: MemoryFigureBucketStorage,
/// Storage of `agenticspec.view.SessionCheck`.
pub session_check_storage: MemorySessionCheckStorage,
/// Storage of `agenticspec.view.SessionListing`.
pub session_listing_storage: MemorySessionListingStorage,
/// Storage of `agenticspec.view.TimelineSlot`.
pub timeline_slot_storage: MemoryTimelineSlotStorage,
}

fn memory_key_103(value: &agenticspec_types::session::AgentId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_112(value: &agenticspec_types::session::ExposureId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_118(value: &agenticspec_types::session::SessionId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_121(value: &agenticspec_types::session::ToolCallId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_127(value: &agenticspec_types::view::BucketId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_128(value: &agenticspec_types::view::CheckId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_133(value: &agenticspec_types::view::EventId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

fn memory_key_139(value: &agenticspec_types::view::SlotId) -> MemoryKey { let _ = value; MemoryKey::Text((&value.0).clone()) }

// A common structural order; individual specifications use only a subset of the variants.
#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MemoryKey {
    Null,
    Boolean(bool),
    Integer(i64),
    Number(String),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<MemoryKey>),
    Map(Vec<(MemoryKey, MemoryKey)>),
    Optional(Option<Box<MemoryKey>>),
}

impl agenticspec_types::view::obligations::SessionEventCountsQuery for MemoryPorts {
fn session_event_counts(&self, session_id: agenticspec_types::session::SessionId, agent: Option<agenticspec_types::session::AgentId>) -> Result<Vec<agenticspec_types::view::SessionEventCounts>, agenticspec_types::obligation::UnmetObligation> { agenticspec_types::view::obligations::SessionEventCountsQuery::session_event_counts(&agenticspec_types::view::obligations::Unimplemented, session_id, agent) }
}
impl agenticspec_types::view::obligations::SessionEventsQuery for MemoryPorts {
fn session_events(&self, session_id: agenticspec_types::session::SessionId, agent: Option<agenticspec_types::session::AgentId>, kind: Option<agenticspec_types::view::EventKind>, notable: Option<bool>, before: Option<i64>) -> Result<Vec<agenticspec_types::view::SessionEvents>, agenticspec_types::obligation::UnmetObligation> { agenticspec_types::view::obligations::SessionEventsQuery::session_events(&agenticspec_types::view::obligations::Unimplemented, session_id, agent, kind, notable, before) }
}
impl agenticspec_types::view::obligations::SessionListQuery for MemoryPorts {
fn session_list(&self, since: Option<agenticspec_types::primitives::Timestamp>, search: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<agenticspec_types::view::SessionList>, agenticspec_types::obligation::UnmetObligation> { agenticspec_types::view::obligations::SessionListQuery::session_list(&agenticspec_types::view::obligations::Unimplemented, since, search, limit, offset) }
}
