// generated from agenticspec v1
// contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
// do not edit: regenerate with `ess synthesize`

//! cli — `agenticspec.cli`.
//!
//! The typed inputs, results and refusals of the `agenticspec` command line, presented by the ess-cli/2 binding in `spec/cli.yaml`. It declares no entity. The KPI report is typed here, so every figure the report carries is a declared field.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// AgentScope — `agenticspec.cli.AgentScope`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentScope {
    /// `main`.
    Main,
    /// `sub`.
    Sub,
}

/// AnalyzeInput — `agenticspec.cli.AnalyzeInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
}

/// AnalyzeResult — `agenticspec.cli.AnalyzeResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeResult {
    /// `path` — `String`.
    pub path: String,
    /// `sessions` — `Integer`.
    pub sessions: i64,
}

/// AutonomyFigures — `agenticspec.cli.AutonomyFigures`.
///
/// Prompts and decisions of main agents. Working is time in which at least one agent of the session was Working (gaps up to 10 minutes). Waiting on the human is time in which no agent was Working, from a main agent's turn end to its next Human or Delegate prompt; over one hour it counts as away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutonomyFigures {
    /// `working_seconds` — `Integer`.
    pub working_seconds: i64,
    /// `waiting_on_human_seconds` — `Integer`.
    pub waiting_on_human_seconds: i64,
    /// `away_seconds` — `Integer`.
    pub away_seconds: i64,
    /// `prompts` — `List<agenticspec.cli.PromptFigures>`.
    pub prompts: Vec<PromptFigures>,
    /// `decisions` — `List<agenticspec.cli.DecisionFigures>`.
    pub decisions: Vec<DecisionFigures>,
}

/// BlocksLiftInput — `agenticspec.cli.BlocksLiftInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocksLiftInput {
    /// `transcript` — `String`.
    pub transcript: String,
}

/// BlocksLiftResult — `agenticspec.cli.BlocksLiftResult`.
///
/// The rows of one session, in the form of a scenario's `given`, each checked against the building blocks, with the count per entity and the entities the session cannot show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocksLiftResult {
    /// `session_id` — `String`.
    pub session_id: String,
    /// `given` — `Json`.
    pub given: crate::json::Value,
    /// `entities` — `List<agenticspec.cli.EntityRows>`.
    pub entities: Vec<EntityRows>,
    /// `not_observable` — `List<String>`.
    pub not_observable: Vec<String>,
}

/// BlocksScenarioInput — `agenticspec.cli.BlocksScenarioInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocksScenarioInput {
    /// `transcript` — `String`.
    pub transcript: String,
    /// `architecture` — `String`.
    pub architecture: String,
    /// `profile` — `String`.
    pub profile: String,
    /// `map` — `String`.
    pub map: String,
    /// `out` — `String`.
    pub out: String,
    /// `name` — `Optional<String>`.
    pub name: Option<String>,
}

/// BlocksScenarioResult — `agenticspec.cli.BlocksScenarioResult`.
///
/// The draft scenario written and checked against its architecture and profile, with the inputs no turn followed and the inputs whose author plays no part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocksScenarioResult {
    /// `out` — `String`.
    pub out: String,
    /// `scenario` — `String`.
    pub scenario: String,
    /// `inputs` — `Integer`.
    pub inputs: i64,
    /// `expectations` — `Integer`.
    pub expectations: i64,
    /// `unjoined` — `List<String>`.
    pub unjoined: Vec<String>,
    /// `unmapped` — `List<String>`.
    pub unmapped: Vec<String>,
}

/// BustCause — `agenticspec.cli.BustCause`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BustCause {
    /// `ttl_expired`.
    TtlExpired,
    /// `model_changed`.
    ModelChanged,
    /// `compacted`.
    Compacted,
    /// `unexplained`.
    Unexplained,
}

/// BustFigures — `agenticspec.cli.BustFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BustFigures {
    /// `scope` — `agenticspec.cli.AgentScope`.
    pub scope: AgentScope,
    /// `cause` — `agenticspec.cli.BustCause`.
    pub cause: BustCause,
    /// `count` — `Integer`.
    pub count: i64,
    /// `tokens_written_again` — `Integer`.
    pub tokens_written_again: i64,
}

/// CacheFigures — `agenticspec.cli.CacheFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheFigures {
    /// `scope` — `agenticspec.cli.AgentScope`.
    pub scope: AgentScope,
    /// `read_tokens` — `Integer`.
    pub read_tokens: i64,
    /// `write_tokens` — `Integer`.
    pub write_tokens: i64,
    /// `uncached_tokens` — `Integer`.
    pub uncached_tokens: i64,
    /// `hit_permille` — `Integer`.
    pub hit_permille: i64,
}

/// Certainty — `agenticspec.cli.Certainty`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Certainty {
    /// `definite`.
    Definite,
    /// `possible`.
    Possible,
}

/// CheckFigures — `agenticspec.cli.CheckFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckFigures {
    /// `check` — `String`.
    pub check: String,
    /// `about` — `String`.
    pub about: String,
    /// `severity` — `String`.
    pub severity: String,
    /// `ok` — `Integer`.
    pub ok: i64,
    /// `gap` — `Integer`.
    pub gap: i64,
    /// `unk` — `Integer`.
    pub unk: i64,
}

/// CheckVerdict — `agenticspec.cli.CheckVerdict`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckVerdict {
    /// `check` — `String`.
    pub check: String,
    /// `session_id` — `String`.
    pub session_id: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: crate::session::Verdict,
    /// `rows` — `Integer`.
    pub rows: i64,
}

/// ConcurrencyFigures — `agenticspec.cli.ConcurrencyFigures`.
///
/// Agents active in the same minute; active means two of its records at most 5 minutes apart around the minute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcurrencyFigures {
    /// `scope` — `agenticspec.cli.AgentScope`.
    pub scope: AgentScope,
    /// `p50` — `Integer`.
    pub p50: i64,
    /// `p90` — `Integer`.
    pub p90: i64,
    /// `max` — `Integer`.
    pub max: i64,
}

/// CorpusAction — `agenticspec.cli.CorpusAction`: one of a closed set of names.
///
/// run: every entry of the corpus on the simulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusAction {
    /// `run`.
    Run,
}

/// Dashboard — `agenticspec.cli.Dashboard`.
///
/// What the dashboard file embeds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dashboard {
    /// `generated_at` — `String`.
    pub generated_at: String,
    /// `overall` — `agenticspec.cli.KpiReport`.
    pub overall: KpiReport,
    /// `sessions` — `List<agenticspec.cli.SessionSummary>`.
    pub sessions: Vec<SessionSummary>,
}

/// DecisionFigures — `agenticspec.cli.DecisionFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFigures {
    /// `decider` — `agenticspec.session.Decider`.
    pub decider: crate::session::Decider,
    /// `decision` — `agenticspec.session.Decision`.
    pub decision: crate::session::Decision,
    /// `count` — `Integer`.
    pub count: i64,
}

/// DeployBundleInput — `agenticspec.cli.DeployBundleInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployBundleInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `binding` — `String`.
    pub binding: String,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `ports` — `Optional<String>`.
    pub ports: Option<String>,
    /// `accepts` — `Optional<String>`.
    pub accepts: Option<String>,
    /// `kpis` — `Optional<String>`.
    pub kpis: Option<String>,
}

/// DeployBundleResult — `agenticspec.cli.DeployBundleResult`.
///
/// The bundle of an admitted architecture, its digest and its units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployBundleResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `binding` — `String`.
    pub binding: String,
    /// `digest` — `String`.
    pub digest: String,
    /// `units` — `Integer`.
    pub units: i64,
    /// `written` — `Boolean`.
    pub written: bool,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `bundle` — `Optional<String>`.
    pub bundle: Option<String>,
}

/// DeployCheckInput — `agenticspec.cli.DeployCheckInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployCheckInput {
    /// `bundle` — `String`.
    pub bundle: String,
    /// `accepts` — `Optional<String>`.
    pub accepts: Option<String>,
}

/// DeployCheckResult — `agenticspec.cli.DeployCheckResult`.
///
/// A written bundle admitted again from what it carries, its digest and its units.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeployCheckResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `binding` — `String`.
    pub binding: String,
    /// `digest` — `String`.
    pub digest: String,
    /// `units` — `Integer`.
    pub units: i64,
}

/// EntityRows — `agenticspec.cli.EntityRows`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRows {
    /// `entity` — `String`.
    pub entity: String,
    /// `rows` — `Integer`.
    pub rows: i64,
}

/// ExportInput — `agenticspec.cli.ExportInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportInput {
    /// `procedure` — `String`.
    pub procedure: String,
    /// `out` — `String`.
    pub out: String,
}

/// ExportResult — `agenticspec.cli.ExportResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportResult {
    /// `protocol` — `String`.
    pub protocol: String,
    /// `bindings` — `String`.
    pub bindings: String,
    /// `validated` — `Boolean`.
    pub validated: bool,
    /// `detail` — `String`.
    pub detail: String,
}

/// ExposureFigures — `agenticspec.cli.ExposureFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposureFigures {
    /// `channel` — `agenticspec.session.ExposureChannel`.
    pub channel: crate::session::ExposureChannel,
    /// `class` — `agenticspec.session.ExposureClass`.
    pub class: crate::session::ExposureClass,
    /// `kind` — `String`.
    pub kind: String,
    /// `verdict` — `agenticspec.session.Verdict`.
    pub verdict: crate::session::Verdict,
    /// `count` — `Integer`.
    pub count: i64,
    /// `sessions` — `Integer`.
    pub sessions: i64,
}

/// IngressDefaultInput — `agenticspec.cli.IngressDefaultInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressDefaultInput {
    /// `architecture` — `String`.
    pub architecture: String,
}

/// IngressDefaultResult — `agenticspec.cli.IngressDefaultResult`.
///
/// The default ingresses an architecture has, as the `ingress:` section to paste into it and change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressDefaultResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `defaults` — `List<String>`.
    pub defaults: Vec<String>,
    /// `ingress` — `String`.
    pub ingress: String,
}

/// IngressProjectInput — `agenticspec.cli.IngressProjectInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressProjectInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `check` — `Optional<String>`.
    pub check: Option<String>,
    /// `blocks` — `Optional<String>`.
    pub blocks: Option<String>,
    /// `ports` — `Optional<String>`.
    pub ports: Option<String>,
    /// `kpis` — `Optional<String>`.
    pub kpis: Option<String>,
}

/// IngressProjectResult — `agenticspec.cli.IngressProjectResult`.
///
/// The projections of the exports of an architecture, and whether they were written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngressProjectResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `exports` — `Integer`.
    pub exports: i64,
    /// `files` — `List<String>`.
    pub files: Vec<String>,
    /// `written` — `Boolean`.
    pub written: bool,
}

/// InvariantStatus — `agenticspec.cli.InvariantStatus`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantStatus {
    /// `held`.
    Held,
    /// `violated`.
    Violated,
    /// `not_checked`.
    NotChecked,
}

/// KpiInput — `agenticspec.cli.KpiInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KpiInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
}

/// KpiReport — `agenticspec.cli.KpiReport`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KpiReport {
    /// `window_from` — `String`.
    pub window_from: String,
    /// `window_to` — `String`.
    pub window_to: String,
    /// `transcripts` — `Integer`.
    pub transcripts: i64,
    /// `sessions` — `Integer`.
    pub sessions: i64,
    /// `sub_agents` — `Integer`.
    pub sub_agents: i64,
    /// `refused_commands` — `Integer`.
    pub refused_commands: i64,
    /// `cost_microdollars` — `Integer`.
    pub cost_microdollars: i64,
    /// `unpriced_messages` — `Integer`.
    pub unpriced_messages: i64,
    /// `models` — `List<agenticspec.cli.ModelFigures>`.
    pub models: Vec<ModelFigures>,
    /// `cache` — `List<agenticspec.cli.CacheFigures>`.
    pub cache: Vec<CacheFigures>,
    /// `busts` — `List<agenticspec.cli.BustFigures>`.
    pub busts: Vec<BustFigures>,
    /// `concurrency` — `List<agenticspec.cli.ConcurrencyFigures>`.
    pub concurrency: Vec<ConcurrencyFigures>,
    /// `autonomy` — `agenticspec.cli.AutonomyFigures`.
    pub autonomy: AutonomyFigures,
    /// `exposures` — `List<agenticspec.cli.ExposureFigures>`.
    pub exposures: Vec<ExposureFigures>,
}

/// LiftInput — `agenticspec.cli.LiftInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftInput {
    /// `transcript` — `String`.
    pub transcript: String,
}

/// LiftResult — `agenticspec.cli.LiftResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftResult {
    /// `session_id` — `String`.
    pub session_id: String,
    /// `agents` — `Integer`.
    pub agents: i64,
    /// `commands` — `List<agenticspec.cli.LiftedCommand>`.
    pub commands: Vec<LiftedCommand>,
    /// `refused` — `Integer`.
    pub refused: i64,
}

/// LiftedCommand — `agenticspec.cli.LiftedCommand`.
///
/// One command of the lifted session, as replayed, with the outcome the session IR answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftedCommand {
    /// `seq` — `Integer`.
    pub seq: i64,
    /// `at` — `String`.
    pub at: String,
    /// `actor` — `String`.
    pub actor: String,
    /// `command` — `String`.
    pub command: String,
    /// `outcome` — `String`.
    pub outcome: String,
    /// `input` — `Json`.
    pub input: crate::json::Value,
}

/// MarkKind — `agenticspec.cli.MarkKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkKind {
    /// `turn`.
    Turn,
    /// `human_prompt`.
    HumanPrompt,
    /// `delegate_prompt`.
    DelegatePrompt,
    /// `peer_prompt`.
    PeerPrompt,
    /// `harness_prompt`.
    HarnessPrompt,
    /// `compaction`.
    Compaction,
    /// `cache_bust`.
    CacheBust,
    /// `refusal`.
    Refusal,
}

/// MineInput — `agenticspec.cli.MineInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MineInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
    /// `level` — `Optional<String>`.
    pub level: Option<String>,
    /// `min_sessions` — `Optional<Integer>`.
    pub min_sessions: Option<i64>,
    /// `min_projects` — `Optional<Integer>`.
    pub min_projects: Option<i64>,
    /// `top` — `Optional<Integer>`.
    pub top: Option<i64>,
}

/// MineReport — `agenticspec.cli.MineReport`.
///
/// The report of `agenticspec mine`, format `agenticspec-patterns/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MineReport {
    /// `format` — `String`.
    pub format: String,
    /// `level` — `String`.
    pub level: String,
    /// `sessions` — `Integer`.
    pub sessions: i64,
    /// `turns` — `Integer`.
    pub turns: i64,
    /// `patterns` — `List<agenticspec.cli.Pattern>`.
    pub patterns: Vec<Pattern>,
}

/// ModelFigures — `agenticspec.cli.ModelFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFigures {
    /// `scope` — `agenticspec.cli.AgentScope`.
    pub scope: AgentScope,
    /// `model` — `String`.
    pub model: String,
    /// `messages` — `Integer`.
    pub messages: i64,
    /// `output_tokens` — `Integer`.
    pub output_tokens: i64,
    /// `context_p50` — `Integer`.
    pub context_p50: i64,
    /// `context_p90` — `Integer`.
    pub context_p90: i64,
    /// `context_max` — `Integer`.
    pub context_max: i64,
    /// `cost_microdollars` — `Integer`.
    pub cost_microdollars: i64,
}

/// PackingDefaultInput — `agenticspec.cli.PackingDefaultInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackingDefaultInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `strategy` — `Optional<String>`.
    pub strategy: Option<String>,
}

/// PackingDefaultResult — `agenticspec.cli.PackingDefaultResult`.
///
/// The resolved packing of an architecture under a strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackingDefaultResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `strategy` — `String`.
    pub strategy: String,
    /// `units` — `Integer`.
    pub units: i64,
    /// `packing` — `String`.
    pub packing: String,
}

/// Pattern — `agenticspec.cli.Pattern`.
///
/// A run of tokens that recurs in the turns of agents across sessions. Support counts distinct sessions, not occurrences. The query is the sq text that finds the pattern again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    /// `id` — `String`.
    pub id: String,
    /// `tokens` — `List<String>`.
    pub tokens: Vec<String>,
    /// `sessions` — `Integer`.
    pub sessions: i64,
    /// `projects` — `Integer`.
    pub projects: i64,
    /// `occurrences` — `Integer`.
    pub occurrences: i64,
    /// `ok_permille` — `Integer`.
    pub ok_permille: i64,
    /// `query` — `String`.
    pub query: String,
}

/// Procedure — `agenticspec.cli.Procedure`.
///
/// A mined pattern lifted into a procedure, format `agenticspec-procedure/1`: steps, the parameters that vary across matches, and the sessions it came from. A draft until a person promotes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Procedure {
    /// `format` — `String`.
    pub format: String,
    /// `id` — `String`.
    pub id: String,
    /// `description` — `String`.
    pub description: String,
    /// `pattern` — `List<String>`.
    pub pattern: Vec<String>,
    /// `matches` — `Integer`.
    pub matches: i64,
    /// `sessions` — `List<String>`.
    pub sessions: Vec<String>,
    /// `parameters` — `List<agenticspec.cli.ProcedureParameter>`.
    pub parameters: Vec<ProcedureParameter>,
    /// `steps` — `List<agenticspec.cli.ProcedureStep>`.
    pub steps: Vec<ProcedureStep>,
    /// `query` — `String`.
    pub query: String,
}

/// ProcedureInput — `agenticspec.cli.ProcedureInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureInput {
    /// `pattern` — `String`.
    pub pattern: String,
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
    /// `level` — `Optional<String>`.
    pub level: Option<String>,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
}

/// ProcedureParameter — `agenticspec.cli.ProcedureParameter`.
///
/// A value that varies across the matches of a pattern. Values are kept only when they are not content (a tool name, a category, a count), never a path or a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureParameter {
    /// `name` — `String`.
    pub name: String,
    /// `step` — `String`.
    pub step: String,
    /// `kind` — `String`.
    pub kind: String,
    /// `values` — `List<String>`.
    pub values: Vec<String>,
}

/// ProcedureStep — `agenticspec.cli.ProcedureStep`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureStep {
    /// `id` — `String`.
    pub id: String,
    /// `kind` — `agenticspec.cli.StepKind`.
    pub kind: StepKind,
    /// `token` — `String`.
    pub token: String,
    /// `tool` — `String`.
    pub tool: String,
    /// `category` — `String`.
    pub category: String,
    /// `shape` — `Optional<String>`.
    pub shape: Option<String>,
    /// `effect` — `String`.
    pub effect: String,
    /// `after` — `List<String>`.
    pub after: Vec<String>,
    /// `retry_max` — `Integer`.
    pub retry_max: i64,
    /// `timeout_seconds` — `Integer`.
    pub timeout_seconds: i64,
    /// `ok_permille` — `Integer`.
    pub ok_permille: i64,
}

/// PromptFigures — `agenticspec.cli.PromptFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptFigures {
    /// `author` — `agenticspec.session.PromptAuthor`.
    pub author: crate::session::PromptAuthor,
    /// `attribution` — `agenticspec.session.Attribution`.
    pub attribution: crate::session::Attribution,
    /// `count` — `Integer`.
    pub count: i64,
}

/// QueryInput — `agenticspec.cli.QueryInput`.
///
/// An sq module, from a file or inline text, run over one session or over the sessions of a window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryInput {
    /// `module` — `Optional<String>`.
    pub module: Option<String>,
    /// `text` — `Optional<String>`.
    pub text: Option<String>,
    /// `transcript` — `Optional<String>`.
    pub transcript: Option<String>,
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
}

/// QueryReport — `agenticspec.cli.QueryReport`.
///
/// The report of `agenticspec query`, format `agenticspec-query-report/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryReport {
    /// `format` — `String`.
    pub format: String,
    /// `sessions` — `Integer`.
    pub sessions: i64,
    /// `queries` — `List<String>`.
    pub queries: Vec<String>,
    /// `rows` — `List<agenticspec.cli.QueryRow>`.
    pub rows: Vec<QueryRow>,
    /// `checks` — `List<agenticspec.cli.CheckFigures>`.
    pub checks: Vec<CheckFigures>,
    /// `verdicts` — `List<agenticspec.cli.CheckVerdict>`.
    pub verdicts: Vec<CheckVerdict>,
}

/// QueryRow — `agenticspec.cli.QueryRow`.
///
/// One answer of a query. Possible when it rests on an inferred or missing fact; unknown names each such fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryRow {
    /// `query` — `Integer`.
    pub query: i64,
    /// `session_id` — `String`.
    pub session_id: String,
    /// `values` — `List<String>`.
    pub values: Vec<String>,
    /// `certainty` — `agenticspec.cli.Certainty`.
    pub certainty: Certainty,
    /// `unknown` — `List<String>`.
    pub unknown: Vec<String>,
}

/// Refusal — `agenticspec.cli.Refusal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// `detail` — `String`.
    pub detail: String,
}

/// ScenarioAction — `agenticspec.cli.ScenarioAction`: one of a closed set of names.
///
/// check: the scenario against its architecture and profile. run: the scenario on the simulator, with its expectations evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioAction {
    /// `check`.
    Check,
    /// `run`.
    Run,
}

/// ServeInput — `agenticspec.cli.ServeInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServeInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
    /// `port` — `Optional<Integer>`.
    pub port: Option<i64>,
}

/// ServeResult — `agenticspec.cli.ServeResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServeResult {
    /// `url` — `String`.
    pub url: String,
}

/// SessionSummary — `agenticspec.cli.SessionSummary`.
///
/// One session with its own KPI report. The title is the harness's title of the session and is shown only on the host that holds the transcript.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSummary {
    /// `session_id` — `String`.
    pub session_id: String,
    /// `project` — `String`.
    pub project: String,
    /// `title` — `Optional<String>`.
    pub title: Option<String>,
    /// `first_at` — `String`.
    pub first_at: String,
    /// `last_at` — `String`.
    pub last_at: String,
    /// `kpis` — `agenticspec.cli.KpiReport`.
    pub kpis: KpiReport,
    /// `tools` — `List<agenticspec.cli.ToolFigures>`.
    pub tools: Vec<ToolFigures>,
    /// `timeline` — `List<agenticspec.cli.TimelineMark>`.
    pub timeline: Vec<TimelineMark>,
}

/// SimCheckInput — `agenticspec.cli.SimCheckInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCheckInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `ports` — `Optional<String>`.
    pub ports: Option<String>,
    /// `kpis` — `Optional<String>`.
    pub kpis: Option<String>,
    /// `json` — `Boolean`.
    pub json: bool,
}

/// SimCheckResult — `agenticspec.cli.SimCheckResult`.
///
/// The architecture the check accepted, with its counts of parts, links and ingresses, the KPIs of the architecture as a whole, the architectures it nests and its requirements, each met, missed or undecided. With `--json`, the `agenticspec-check/1` document, which carries `format`, `valid`, `violations` and `counts` too; a refused architecture prints the document of the check alone before its refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCheckResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `parts` — `Integer`.
    pub parts: i64,
    /// `links` — `Integer`.
    pub links: i64,
    /// `kpis` — `Json`.
    pub kpis: crate::json::Value,
    /// `single_points` — `List<String>`.
    pub single_points: Vec<String>,
    /// `nested` — `List<agenticspec.cli.SimNested>`.
    pub nested: Vec<SimNested>,
    /// `ingresses` — `Integer`.
    pub ingresses: i64,
    /// `requirements` — `Json`.
    pub requirements: crate::json::Value,
    /// `format` — `Optional<String>`.
    pub format: Option<String>,
    /// `valid` — `Optional<Boolean>`.
    pub valid: Option<bool>,
    /// `violations` — `Optional<List<agenticspec.cli.SimCheckViolation>>`.
    pub violations: Option<Vec<SimCheckViolation>>,
    /// `counts` — `Optional<Map<String, Integer>>`.
    pub counts: Option<std::collections::BTreeMap<String, i64>>,
}

/// SimCheckViolation — `agenticspec.cli.SimCheckViolation`.
///
/// One violation of a rule of the architecture check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCheckViolation {
    /// `rule` — `String`.
    pub rule: String,
    /// `gate` — `Boolean`.
    pub gate: bool,
    /// `subject` — `String`.
    pub subject: String,
    /// `part` — `String`.
    pub part: String,
    /// `link` — `Optional<Integer>`.
    pub link: Option<i64>,
    /// `message` — `String`.
    pub message: String,
}

/// SimContractInput — `agenticspec.cli.SimContractInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimContractInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `ports` — `Optional<String>`.
    pub ports: Option<String>,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `check` — `Optional<String>`.
    pub check: Option<String>,
    /// `suite` — `Optional<String>`.
    pub suite: Option<String>,
    /// `trace` — `Optional<String>`.
    pub trace: Option<String>,
    /// `kpis` — `Optional<String>`.
    pub kpis: Option<String>,
}

/// SimContractResult — `agenticspec.cli.SimContractResult`.
///
/// The contract of an architecture, its counts, whether it was written, the suite cut to it and the checks of its routing invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimContractResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `ports` — `Integer`.
    pub ports: i64,
    /// `invariants` — `Integer`.
    pub invariants: i64,
    /// `ingresses` — `Integer`.
    pub ingresses: i64,
    /// `written` — `Boolean`.
    pub written: bool,
    /// `contract` — `Optional<String>`.
    pub contract: Option<String>,
    /// `suite_scenarios` — `Optional<Integer>`.
    pub suite_scenarios: Option<i64>,
    /// `kept_scenarios` — `Optional<Integer>`.
    pub kept_scenarios: Option<i64>,
    /// `checks` — `List<agenticspec.cli.SimInvariantCheck>`.
    pub checks: Vec<SimInvariantCheck>,
}

/// SimCorpusInput — `agenticspec.cli.SimCorpusInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCorpusInput {
    /// `action` — `agenticspec.cli.CorpusAction`.
    pub action: CorpusAction,
    /// `corpus` — `String`.
    pub corpus: String,
}

/// SimCorpusResult — `agenticspec.cli.SimCorpusResult`.
///
/// Every entry of a corpus held. The table of the rows is printed to standard error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCorpusResult {
    /// `corpus` — `String`.
    pub corpus: String,
    /// `entries` — `Integer`.
    pub entries: i64,
    /// `held` — `Integer`.
    pub held: i64,
    /// `failed` — `Integer`.
    pub failed: i64,
    /// `rows` — `List<agenticspec.cli.SimCorpusRow>`.
    pub rows: Vec<SimCorpusRow>,
}

/// SimCorpusRow — `agenticspec.cli.SimCorpusRow`.
///
/// One entry of a corpus run, the class it expects beside the class its run predicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCorpusRow {
    /// `entry` — `String`.
    pub entry: String,
    /// `kind` — `String`.
    pub kind: String,
    /// `expected` — `String`.
    pub expected: String,
    /// `predicted` — `String`.
    pub predicted: String,
    /// `held` — `Integer`.
    pub held: i64,
    /// `failed` — `Integer`.
    pub failed: i64,
    /// `not_checked` — `Integer`.
    pub not_checked: i64,
    /// `verdict` — `String`.
    pub verdict: String,
}

/// SimDigestInput — `agenticspec.cli.SimDigestInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimDigestInput {
    /// `architecture` — `String`.
    pub architecture: String,
}

/// SimDigestResult — `agenticspec.cli.SimDigestResult`.
///
/// The digest a referring document pins the architecture with: `sha256:` and the SHA-256 of the canonical JSON (keys sorted, no whitespace) of its parts, links and border.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimDigestResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `digest` — `String`.
    pub digest: String,
}

/// SimInvariantCheck — `agenticspec.cli.SimInvariantCheck`.
///
/// The check of one routing invariant on a trace. An invariant whose predicate names a port the trace does not record is not checked, never held.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimInvariantCheck {
    /// `link` — `String`.
    pub link: String,
    /// `predicate` — `String`.
    pub predicate: String,
    /// `status` — `agenticspec.cli.InvariantStatus`.
    pub status: InvariantStatus,
    /// `uses` — `Integer`.
    pub uses: i64,
    /// `event` — `Optional<Integer>`.
    pub event: Option<i64>,
    /// `detail` — `Optional<String>`.
    pub detail: Option<String>,
}

/// SimLadderInput — `agenticspec.cli.SimLadderInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimLadderInput {
    /// `ladder` — `String`.
    pub ladder: String,
    /// `out` — `String`.
    pub out: String,
    /// `check` — `Boolean`.
    pub check: bool,
}

/// SimLadderResult — `agenticspec.cli.SimLadderResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimLadderResult {
    /// `out` — `String`.
    pub out: String,
    /// `rungs` — `Integer`.
    pub rungs: i64,
    /// `written` — `Boolean`.
    pub written: bool,
}

/// SimNested — `agenticspec.cli.SimNested`.
///
/// One nested architecture a check resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimNested {
    /// `part` — `String`.
    pub part: String,
    /// `architecture` — `String`.
    pub architecture: String,
    /// `reference` — `String`.
    pub reference: String,
    /// `digest` — `String`.
    pub digest: String,
    /// `parts` — `Integer`.
    pub parts: i64,
    /// `links` — `Integer`.
    pub links: i64,
}

/// SimPortsInput — `agenticspec.cli.SimPortsInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimPortsInput {
    /// `blocks` — `Optional<String>`.
    pub blocks: Option<String>,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `check` — `Optional<String>`.
    pub check: Option<String>,
    /// `expect` — `Optional<String>`.
    pub expect: Option<String>,
}

/// SimPortsResult — `agenticspec.cli.SimPortsResult`.
///
/// The derived ports, the digest of their sources, and the differences from an expected list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimPortsResult {
    /// `ports` — `Integer`.
    pub ports: i64,
    /// `sources_sha256` — `String`.
    pub sources_sha256: String,
    /// `written` — `Boolean`.
    pub written: bool,
    /// `file` — `Optional<String>`.
    pub file: Option<String>,
    /// `differences` — `List<String>`.
    pub differences: Vec<String>,
}

/// SimRenderInput — `agenticspec.cli.SimRenderInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimRenderInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `result` — `Optional<String>`.
    pub result: Option<String>,
    /// `out` — `String`.
    pub out: String,
}

/// SimRenderResult — `agenticspec.cli.SimRenderResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimRenderResult {
    /// `out` — `String`.
    pub out: String,
}

/// SimRunInput — `agenticspec.cli.SimRunInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimRunInput {
    /// `experiment` — `String`.
    pub experiment: String,
    /// `out` — `String`.
    pub out: String,
    /// `threads` — `Optional<Integer>`.
    pub threads: Option<i64>,
    /// `events` — `Boolean`.
    pub events: bool,
}

/// SimRunResult — `agenticspec.cli.SimRunResult`.
///
/// One run of an experiment. The KPIs of the baseline and the best variant are printed to standard error and written to `result.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimRunResult {
    /// `out` — `String`.
    pub out: String,
    /// `variants` — `Integer`.
    pub variants: i64,
    /// `seeds` — `Integer`.
    pub seeds: i64,
    /// `threads` — `Integer`.
    pub threads: i64,
    /// `best_written` — `Boolean`.
    pub best_written: bool,
    /// `events` — `Optional<Integer>`.
    pub events: Option<i64>,
}

/// SimScenarioInput — `agenticspec.cli.SimScenarioInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimScenarioInput {
    /// `action` — `agenticspec.cli.ScenarioAction`.
    pub action: ScenarioAction,
    /// `scenario` — `String`.
    pub scenario: String,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
}

/// SimScenarioResult — `agenticspec.cli.SimScenarioResult`.
///
/// The scenario checked, with its counts; after `run` also the verdict, the outcome class the run predicts and the count of expectations held, failed and not checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimScenarioResult {
    /// `scenario` — `String`.
    pub scenario: String,
    /// `architecture` — `String`.
    pub architecture: String,
    /// `inputs` — `Integer`.
    pub inputs: i64,
    /// `expectations` — `Integer`.
    pub expectations: i64,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
    /// `verdict` — `Optional<String>`.
    pub verdict: Option<String>,
    /// `predicted` — `Optional<String>`.
    pub predicted: Option<String>,
    /// `held` — `Optional<Integer>`.
    pub held: Option<i64>,
    /// `failed` — `Optional<Integer>`.
    pub failed: Option<i64>,
    /// `not_checked` — `Optional<Integer>`.
    pub not_checked: Option<i64>,
}

/// SimScoreInput — `agenticspec.cli.SimScoreInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimScoreInput {
    /// `architecture` — `Optional<String>`.
    pub architecture: Option<String>,
    /// `profile` — `String`.
    pub profile: String,
    /// `seed` — `Optional<Integer>`.
    pub seed: Option<i64>,
    /// `horizon` — `Optional<String>`.
    pub horizon: Option<String>,
    /// `batch` — `Boolean`.
    pub batch: bool,
    /// `documents` — `List<String>`.
    pub documents: Vec<String>,
}

/// SimScoreResult — `agenticspec.cli.SimScoreResult`.
///
/// The KPIs of one architecture document simulated once, or with `--batch` or more than one document the answer of each document in their order. A refused document answers `{"error": {"code", "message"}}`: alone on standard output with exit 3, in a batch as its entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimScoreResult {
    /// `kpis` — `Optional<Json>`.
    pub kpis: Option<crate::json::Value>,
    /// `per_part` — `Optional<Json>`.
    pub per_part: Option<crate::json::Value>,
    /// `results` — `Optional<List<Json>>`.
    pub results: Option<Vec<crate::json::Value>>,
}

/// SimTraceInput — `agenticspec.cli.SimTraceInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimTraceInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `profile` — `String`.
    pub profile: String,
    /// `seed` — `Optional<Integer>`.
    pub seed: Option<i64>,
    /// `horizon` — `Optional<String>`.
    pub horizon: Option<String>,
    /// `out` — `Optional<String>`.
    pub out: Option<String>,
}

/// SimTraceResult — `agenticspec.cli.SimTraceResult`.
///
/// The trace of one simulated run, its counts, and whether it was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimTraceResult {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `seed` — `Integer`.
    pub seed: i64,
    /// `uses` — `Integer`.
    pub uses: i64,
    /// `events` — `Integer`.
    pub events: i64,
    /// `written` — `Boolean`.
    pub written: bool,
    /// `trace` — `Optional<String>`.
    pub trace: Option<String>,
}

/// StepKind — `agenticspec.cli.StepKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    /// `program`.
    Program,
    /// `judgement`.
    Judgement,
    /// `person`.
    Person,
    /// `child`.
    Child,
}

/// SyncInput — `agenticspec.cli.SyncInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `since_hours` — `Optional<Integer>`.
    pub since_hours: Option<i64>,
}

/// SyncResult — `agenticspec.cli.SyncResult`.
///
/// One pass of the store over the transcripts of a window. A main transcript whose own file and sub-agent files keep their size and modification time is `unchanged`; every other one is lifted, redacted and written again in one transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncResult {
    /// `db` — `String`.
    pub db: String,
    /// `scanned` — `Integer`.
    pub scanned: i64,
    /// `lifted` — `Integer`.
    pub lifted: i64,
    /// `unchanged` — `Integer`.
    pub unchanged: i64,
    /// `failed` — `Integer`.
    pub failed: i64,
    /// `sessions` — `Integer`.
    pub sessions: i64,
    /// `ms` — `Integer`.
    pub ms: i64,
}

/// TimelineMark — `agenticspec.cli.TimelineMark`.
///
/// One point of a session's timeline. For a turn the value is the context size, for a cache bust the tokens written again, otherwise 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineMark {
    /// `at` — `String`.
    pub at: String,
    /// `kind` — `agenticspec.cli.MarkKind`.
    pub kind: MarkKind,
    /// `scope` — `agenticspec.cli.AgentScope`.
    pub scope: AgentScope,
    /// `value` — `Integer`.
    pub value: i64,
}

/// ToolFigures — `agenticspec.cli.ToolFigures`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolFigures {
    /// `category` — `agenticspec.session.ToolCategory`.
    pub category: crate::session::ToolCategory,
    /// `calls` — `Integer`.
    pub calls: i64,
    /// `refused` — `Integer`.
    pub refused: i64,
    /// `failed` — `Integer`.
    pub failed: i64,
}

/// UiBuildInput — `agenticspec.cli.UiBuildInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiBuildInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `contract` — `String`.
    pub contract: String,
    /// `out` — `String`.
    pub out: String,
    /// `api_base` — `Optional<String>`.
    pub api_base: Option<String>,
    /// `archive` — `Boolean`.
    pub archive: bool,
    /// `control_plane` — `Boolean`.
    pub control_plane: bool,
    /// `prediction` — `Optional<String>`.
    pub prediction: Option<String>,
    /// `ingress` — `Optional<String>`.
    pub ingress: Option<String>,
}

/// UiBuildResult — `agenticspec.cli.UiBuildResult`.
///
/// The static files of an operator UI: `index.html`, `app.js` and `app.css` of the operator app built into the binary, `views.json` (the layout of the derived UI), `contract.json`, `ports.json` and `config.json`; the same inputs give the same bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiBuildResult {
    /// `out` — `String`.
    pub out: String,
    /// `files` — `Integer`.
    pub files: i64,
    /// `bytes` — `Integer`.
    pub bytes: i64,
    /// `archive` — `Optional<String>`.
    pub archive: Option<String>,
    /// `archive_sha256` — `Optional<String>`.
    pub archive_sha256: Option<String>,
}

/// UiConformCheck — `agenticspec.cli.UiConformCheck`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiConformCheck {
    /// `name` — `String`.
    pub name: String,
    /// `status` — `String`.
    pub status: String,
    /// `detail` — `String`.
    pub detail: String,
}

/// UiConformInput — `agenticspec.cli.UiConformInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiConformInput {
    /// `source` — `Optional<String>`.
    pub source: Option<String>,
    /// `plan` — `Optional<String>`.
    pub plan: Option<String>,
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `architecture` — `Optional<String>`.
    pub architecture: Option<String>,
    /// `contract` — `Optional<String>`.
    pub contract: Option<String>,
    /// `origin` — `Optional<String>`.
    pub origin: Option<String>,
    /// `header` — `List<String>`.
    pub header: Vec<String>,
    /// `json` — `Boolean`.
    pub json: bool,
}

/// UiConformResult — `agenticspec.cli.UiConformResult`.
///
/// The checks of the runtime interface (`ui/runtime.openapi.yaml`) against one source: scoped reads (R1), the placement views (R3), resuming a channel from a cursor (R2), command answers and refusals (R4), the action set (R5) and requests without credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiConformResult {
    /// `source` — `String`.
    pub source: String,
    /// `passed` — `Integer`.
    pub passed: i64,
    /// `failed` — `Integer`.
    pub failed: i64,
    /// `skipped` — `Integer`.
    pub skipped: i64,
    /// `checks` — `List<agenticspec.cli.UiConformCheck>`.
    pub checks: Vec<UiConformCheck>,
}

/// UiDeriveInput — `agenticspec.cli.UiDeriveInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiDeriveInput {
    /// `path` — `String`.
    pub path: String,
    /// `contract` — `Optional<String>`.
    pub contract: Option<String>,
    /// `ports` — `Optional<String>`.
    pub ports: Option<String>,
    /// `out` — `String`.
    pub out: String,
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `check` — `Boolean`.
    pub check: bool,
    /// `control_plane` — `Boolean`.
    pub control_plane: bool,
    /// `prediction` — `Optional<String>`.
    pub prediction: Option<String>,
}

/// UiDeriveResult — `agenticspec.cli.UiDeriveResult`.
///
/// The operator UI of an architecture: one `ess-ui/1` document (an overview per level, a page per part, a channel per level, the actions of the contract) with `<name>.sources.sha256`, `<name>.fixtures/` and `<name>.layout.json` beside it; when the contract gives a `ui` export to an ingress other than `operator`, one such document per ingress with a `ui` export, `<name>.<ingress>.ui.yaml`, each scoped to the ingress's interface, and the counts summed over them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiDeriveResult {
    /// `out` — `String`.
    pub out: String,
    /// `pages` — `Integer`.
    pub pages: i64,
    /// `actions` — `Integer`.
    pub actions: i64,
    /// `channels` — `Integer`.
    pub channels: i64,
    /// `files` — `Integer`.
    pub files: i64,
    /// `written` — `Boolean`.
    pub written: bool,
    /// `ingresses` — `Optional<List<agenticspec.cli.UiIngressDerived>>`.
    pub ingresses: Option<Vec<UiIngressDerived>>,
}

/// UiGenerateInput — `agenticspec.cli.UiGenerateInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiGenerateInput {
    /// `path` — `String`.
    pub path: String,
    /// `out` — `String`.
    pub out: String,
    /// `project` — `Boolean`.
    pub project: bool,
}

/// UiGenerateResult — `agenticspec.cli.UiGenerateResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiGenerateResult {
    /// `out` — `String`.
    pub out: String,
    /// `files` — `List<String>`.
    pub files: Vec<String>,
}

/// UiIngressDerived — `agenticspec.cli.UiIngressDerived`.
///
/// The composition of one ingress with a `ui` export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiIngressDerived {
    /// `ingress` — `String`.
    pub ingress: String,
    /// `out` — `String`.
    pub out: String,
    /// `pages` — `Integer`.
    pub pages: i64,
    /// `actions` — `Integer`.
    pub actions: i64,
    /// `channels` — `Integer`.
    pub channels: i64,
}

/// UiIngressServed — `agenticspec.cli.UiIngressServed`.
///
/// Where the UI of one ingress with a `ui` export is served.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiIngressServed {
    /// `ingress` — `String`.
    pub ingress: String,
    /// `url` — `String`.
    pub url: String,
    /// `interface` — `String`.
    pub interface: String,
}

/// UiPanelsInput — `agenticspec.cli.UiPanelsInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiPanelsInput {
    /// `root` — `Optional<String>`.
    pub root: Option<String>,
    /// `check` — `Boolean`.
    pub check: bool,
}

/// UiPanelsResult — `agenticspec.cli.UiPanelsResult`.
///
/// The block panels: one `ess-ui/1` widget per view of `blocks/` in `ui/views/blocks.ui.yaml`, a gallery page per block, a fixture per view and a script per channel under `ui/views/fixtures/`, and the digest of the inputs in `ui/views/blocks.sources.sha256`; beside them `ui/views/serve.fixtures.json`, the fixtures and scripted outcomes of `agenticspec ui serve` in one file with the digest of its sources, which the binary embeds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiPanelsResult {
    /// `out` — `String`.
    pub out: String,
    /// `panels` — `Integer`.
    pub panels: i64,
    /// `files` — `Integer`.
    pub files: i64,
    /// `written` — `Boolean`.
    pub written: bool,
}

/// UiServeInput — `agenticspec.cli.UiServeInput`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiServeInput {
    /// `architecture` — `String`.
    pub architecture: String,
    /// `contract` — `Optional<String>`.
    pub contract: Option<String>,
    /// `port` — `Optional<Integer>`.
    pub port: Option<i64>,
    /// `fixtures` — `Optional<String>`.
    pub fixtures: Option<String>,
    /// `sim` — `Optional<String>`.
    pub sim: Option<String>,
}

/// UiServeResult — `agenticspec.cli.UiServeResult`.
///
/// Where the operator UI was served, `http://127.0.0.1:<port>/`, the control-plane UI, `<url>control/`, and the root of their runtime interface, `<url>ingress/operator`, and the UI of every other ingress with a `ui` export at `<url>ingress/<ingress>/`. The server runs until it is stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiServeResult {
    /// `url` — `String`.
    pub url: String,
    /// `control` — `String`.
    pub control: String,
    /// `interface` — `String`.
    pub interface: String,
    /// `ingresses` — `Optional<List<agenticspec.cli.UiIngressServed>>`.
    pub ingresses: Option<Vec<UiIngressServed>>,
}
