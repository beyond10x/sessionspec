<!--
  generated from agenticspec v1
  contract digest ccb8808b577e000f62b25eea26b56257870a4af7bcbda0f276d5b2bb26a8a862
  do not edit: regenerate with `ess synthesize`
-->
# Synthesis plan — agenticspec v1

Scope: `component-skeletons`, planned by `ess-synth`. Regenerate with `ess synthesize`.

237 capabilities: **234 generated**, **3 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `agenticspec.cli.AgentScope` |
| domain type | `agenticspec.cli.AnalyzeInput` |
| domain type | `agenticspec.cli.AnalyzeResult` |
| domain type | `agenticspec.cli.AutonomyFigures` |
| domain type | `agenticspec.cli.BlocksLiftInput` |
| domain type | `agenticspec.cli.BlocksLiftResult` |
| domain type | `agenticspec.cli.BlocksScenarioInput` |
| domain type | `agenticspec.cli.BlocksScenarioResult` |
| domain type | `agenticspec.cli.BustCause` |
| domain type | `agenticspec.cli.BustFigures` |
| domain type | `agenticspec.cli.CacheFigures` |
| domain type | `agenticspec.cli.Certainty` |
| domain type | `agenticspec.cli.CheckFigures` |
| domain type | `agenticspec.cli.CheckVerdict` |
| domain type | `agenticspec.cli.ConcurrencyFigures` |
| domain type | `agenticspec.cli.CorpusAction` |
| domain type | `agenticspec.cli.Dashboard` |
| domain type | `agenticspec.cli.DecisionFigures` |
| domain type | `agenticspec.cli.DeployBundleInput` |
| domain type | `agenticspec.cli.DeployBundleResult` |
| domain type | `agenticspec.cli.DeployCheckInput` |
| domain type | `agenticspec.cli.DeployCheckResult` |
| domain type | `agenticspec.cli.EntityRows` |
| domain type | `agenticspec.cli.ExportInput` |
| domain type | `agenticspec.cli.ExportResult` |
| domain type | `agenticspec.cli.ExposureFigures` |
| domain type | `agenticspec.cli.IngressDefaultInput` |
| domain type | `agenticspec.cli.IngressDefaultResult` |
| domain type | `agenticspec.cli.IngressProjectInput` |
| domain type | `agenticspec.cli.IngressProjectResult` |
| domain type | `agenticspec.cli.InvariantStatus` |
| domain type | `agenticspec.cli.KpiInput` |
| domain type | `agenticspec.cli.KpiReport` |
| domain type | `agenticspec.cli.LiftInput` |
| domain type | `agenticspec.cli.LiftResult` |
| domain type | `agenticspec.cli.LiftedCommand` |
| domain type | `agenticspec.cli.MarkKind` |
| domain type | `agenticspec.cli.MineInput` |
| domain type | `agenticspec.cli.MineReport` |
| domain type | `agenticspec.cli.ModelFigures` |
| domain type | `agenticspec.cli.PackingDefaultInput` |
| domain type | `agenticspec.cli.PackingDefaultResult` |
| domain type | `agenticspec.cli.Pattern` |
| domain type | `agenticspec.cli.Procedure` |
| domain type | `agenticspec.cli.ProcedureInput` |
| domain type | `agenticspec.cli.ProcedureParameter` |
| domain type | `agenticspec.cli.ProcedureStep` |
| domain type | `agenticspec.cli.PromptFigures` |
| domain type | `agenticspec.cli.QueryInput` |
| domain type | `agenticspec.cli.QueryReport` |
| domain type | `agenticspec.cli.QueryRow` |
| domain type | `agenticspec.cli.Refusal` |
| domain type | `agenticspec.cli.ScenarioAction` |
| domain type | `agenticspec.cli.ServeInput` |
| domain type | `agenticspec.cli.ServeResult` |
| domain type | `agenticspec.cli.SessionSummary` |
| domain type | `agenticspec.cli.SimCheckInput` |
| domain type | `agenticspec.cli.SimCheckResult` |
| domain type | `agenticspec.cli.SimCheckViolation` |
| domain type | `agenticspec.cli.SimContractInput` |
| domain type | `agenticspec.cli.SimContractResult` |
| domain type | `agenticspec.cli.SimCorpusInput` |
| domain type | `agenticspec.cli.SimCorpusResult` |
| domain type | `agenticspec.cli.SimCorpusRow` |
| domain type | `agenticspec.cli.SimDigestInput` |
| domain type | `agenticspec.cli.SimDigestResult` |
| domain type | `agenticspec.cli.SimInvariantCheck` |
| domain type | `agenticspec.cli.SimLadderInput` |
| domain type | `agenticspec.cli.SimLadderResult` |
| domain type | `agenticspec.cli.SimNested` |
| domain type | `agenticspec.cli.SimPortsInput` |
| domain type | `agenticspec.cli.SimPortsResult` |
| domain type | `agenticspec.cli.SimRenderInput` |
| domain type | `agenticspec.cli.SimRenderResult` |
| domain type | `agenticspec.cli.SimRunInput` |
| domain type | `agenticspec.cli.SimRunResult` |
| domain type | `agenticspec.cli.SimScenarioInput` |
| domain type | `agenticspec.cli.SimScenarioResult` |
| domain type | `agenticspec.cli.SimScoreInput` |
| domain type | `agenticspec.cli.SimScoreResult` |
| domain type | `agenticspec.cli.SimTraceInput` |
| domain type | `agenticspec.cli.SimTraceResult` |
| domain type | `agenticspec.cli.StepKind` |
| domain type | `agenticspec.cli.SyncInput` |
| domain type | `agenticspec.cli.SyncResult` |
| domain type | `agenticspec.cli.TimelineMark` |
| domain type | `agenticspec.cli.ToolFigures` |
| domain type | `agenticspec.cli.UiBuildInput` |
| domain type | `agenticspec.cli.UiBuildResult` |
| domain type | `agenticspec.cli.UiConformCheck` |
| domain type | `agenticspec.cli.UiConformInput` |
| domain type | `agenticspec.cli.UiConformResult` |
| domain type | `agenticspec.cli.UiDeriveInput` |
| domain type | `agenticspec.cli.UiDeriveResult` |
| domain type | `agenticspec.cli.UiGenerateInput` |
| domain type | `agenticspec.cli.UiGenerateResult` |
| domain type | `agenticspec.cli.UiIngressDerived` |
| domain type | `agenticspec.cli.UiIngressServed` |
| domain type | `agenticspec.cli.UiPanelsInput` |
| domain type | `agenticspec.cli.UiPanelsResult` |
| domain type | `agenticspec.cli.UiServeInput` |
| domain type | `agenticspec.cli.UiServeResult` |
| domain type | `agenticspec.session.Agent.State` |
| domain type | `agenticspec.session.AgentId` |
| domain type | `agenticspec.session.AgentRole` |
| domain type | `agenticspec.session.Attribution` |
| domain type | `agenticspec.session.Content` |
| domain type | `agenticspec.session.Decider` |
| domain type | `agenticspec.session.Decision` |
| domain type | `agenticspec.session.Exposure.State` |
| domain type | `agenticspec.session.ExposureChannel` |
| domain type | `agenticspec.session.ExposureClass` |
| domain type | `agenticspec.session.ExposureId` |
| domain type | `agenticspec.session.Harness` |
| domain type | `agenticspec.session.IncompleteReason` |
| domain type | `agenticspec.session.PromptAuthor` |
| domain type | `agenticspec.session.PromptClass` |
| domain type | `agenticspec.session.Session.State` |
| domain type | `agenticspec.session.SessionId` |
| domain type | `agenticspec.session.Source` |
| domain type | `agenticspec.session.ToolCall.State` |
| domain type | `agenticspec.session.ToolCallId` |
| domain type | `agenticspec.session.ToolCategory` |
| domain type | `agenticspec.session.Usage` |
| domain type | `agenticspec.session.Verdict` |
| domain type | `agenticspec.view.Agent` |
| domain type | `agenticspec.view.AgentListing.State` |
| domain type | `agenticspec.view.BucketId` |
| domain type | `agenticspec.view.CheckId` |
| domain type | `agenticspec.view.CountId` |
| domain type | `agenticspec.view.Document` |
| domain type | `agenticspec.view.Event` |
| domain type | `agenticspec.view.EventCount.State` |
| domain type | `agenticspec.view.EventId` |
| domain type | `agenticspec.view.EventKind` |
| domain type | `agenticspec.view.EventRow.State` |
| domain type | `agenticspec.view.FigureBucket.State` |
| domain type | `agenticspec.view.SessionCheck.State` |
| domain type | `agenticspec.view.SessionListing.State` |
| domain type | `agenticspec.view.SlotId` |
| domain type | `agenticspec.view.TimelineSlot.State` |
| domain type | `agenticspec.view.Update` |
| entity lifecycle | `agenticspec.session.Agent` |
| entity lifecycle | `agenticspec.session.Exposure` |
| entity lifecycle | `agenticspec.session.Session` |
| entity lifecycle | `agenticspec.session.ToolCall` |
| entity lifecycle | `agenticspec.view.AgentListing` |
| entity lifecycle | `agenticspec.view.EventCount` |
| entity lifecycle | `agenticspec.view.EventRow` |
| entity lifecycle | `agenticspec.view.FigureBucket` |
| entity lifecycle | `agenticspec.view.SessionCheck` |
| entity lifecycle | `agenticspec.view.SessionListing` |
| entity lifecycle | `agenticspec.view.TimelineSlot` |
| command contract | `agenticspec.session.CompactContext` |
| command behaviour | `agenticspec.session.CompactContext` |
| command contract | `agenticspec.session.CompleteToolCall` |
| command behaviour | `agenticspec.session.CompleteToolCall` |
| command contract | `agenticspec.session.DecideToolCall` |
| command behaviour | `agenticspec.session.DecideToolCall` |
| command contract | `agenticspec.session.EndAgent` |
| command behaviour | `agenticspec.session.EndAgent` |
| command contract | `agenticspec.session.EndSession` |
| command behaviour | `agenticspec.session.EndSession` |
| command contract | `agenticspec.session.EndTurn` |
| command behaviour | `agenticspec.session.EndTurn` |
| command contract | `agenticspec.session.OpenSession` |
| command behaviour | `agenticspec.session.OpenSession` |
| command contract | `agenticspec.session.RecordExposure` |
| command behaviour | `agenticspec.session.RecordExposure` |
| command contract | `agenticspec.session.RecordModelTurn` |
| command behaviour | `agenticspec.session.RecordModelTurn` |
| command contract | `agenticspec.session.RequestToolCall` |
| command behaviour | `agenticspec.session.RequestToolCall` |
| command contract | `agenticspec.session.StartAgent` |
| command behaviour | `agenticspec.session.StartAgent` |
| command contract | `agenticspec.session.SubmitPrompt` |
| command behaviour | `agenticspec.session.SubmitPrompt` |
| event type | `agenticspec.session.AgentEnded` |
| event type | `agenticspec.session.AgentStarted` |
| event type | `agenticspec.session.ContextCompacted` |
| event type | `agenticspec.session.ExposureDetected` |
| event type | `agenticspec.session.ModelTurnRecorded` |
| event type | `agenticspec.session.PromptSubmitted` |
| event type | `agenticspec.session.SessionEnded` |
| event type | `agenticspec.session.SessionOpened` |
| event type | `agenticspec.session.ToolCallCompleted` |
| event type | `agenticspec.session.ToolCallDecided` |
| event type | `agenticspec.session.ToolCallRequested` |
| event type | `agenticspec.session.TurnEnded` |
| event type | `agenticspec.view.AgentUpdated` |
| event type | `agenticspec.view.EventAppended` |
| event type | `agenticspec.view.SessionUpdated` |
| error type | `agenticspec.session.AgentStateConflict` |
| error type | `agenticspec.session.SessionStateConflict` |
| error type | `agenticspec.session.ToolCallStateConflict` |
| error type | `agenticspec.session.UnknownAgent` |
| view type | `agenticspec.session.Agents` |
| view query | `agenticspec.session.Agents` |
| view type | `agenticspec.session.Exposures` |
| view query | `agenticspec.session.Exposures` |
| view type | `agenticspec.session.Sessions` |
| view query | `agenticspec.session.Sessions` |
| view type | `agenticspec.session.ToolCalls` |
| view query | `agenticspec.session.ToolCalls` |
| view type | `agenticspec.view.AgentTurns` |
| view query | `agenticspec.view.AgentTurns` |
| view type | `agenticspec.view.OverviewFigures` |
| view query | `agenticspec.view.OverviewFigures` |
| view type | `agenticspec.view.SessionAgents` |
| view query | `agenticspec.view.SessionAgents` |
| view type | `agenticspec.view.SessionById` |
| view query | `agenticspec.view.SessionById` |
| view type | `agenticspec.view.SessionChecks` |
| view query | `agenticspec.view.SessionChecks` |
| view type | `agenticspec.view.SessionEventCounts` |
| view type | `agenticspec.view.SessionEvents` |
| view type | `agenticspec.view.SessionList` |
| view type | `agenticspec.view.SessionTimeline` |
| view query | `agenticspec.view.SessionTimeline` |
| view type | `agenticspec.view.SessionTurns` |
| view query | `agenticspec.view.SessionTurns` |
| actor grants | `agenticspec.session.AgentLoop` |
| actor grants | `agenticspec.session.Delegate` |
| actor grants | `agenticspec.session.HarnessRuntime` |
| actor grants | `agenticspec.session.Human` |
| actor grants | `agenticspec.session.PeerSession` |
| actor grants | `agenticspec.session.PermissionAuthority` |
| actor grants | `agenticspec.session.Redactor` |
| actor grants | `agenticspec.session.ToolProvider` |
| actor grants | `agenticspec.view.Viewer` |
| component port | `session-ir` |
| component transport | `session-ir` |
| component port | `session-view` |
| component transport | `session-view` |

## Ports — yours to provide

What the specification fully determines is generated; what it cannot determine is an obligation. A generated command behaviour or view query reads and writes through the ports below, and they are yours to provide: synthesis generates each port's contract and never an implementation of one, so where instances live stays your decision.

| port | what it answers |
| --- | --- |
| storage | one per entity a generated behaviour or query reads or writes: the instance stored under an identity; storing, replacing and removing one; and every stored instance, in the order the store keeps them |
| context | where a generated behaviour asks it: the caller's attributes, every identity and value the specification says the implementation assigns, and whether each `external:` branch is taken |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |
| view query | `agenticspec.view.SessionEventCounts` | kept an obligation by a comparison with the optional view parameter `agent`, which the interpreted target refuses a read that leaves out, where a generated query would hide rows, in `filter:` | a query answering `agenticspec.view.SessionEventCounts` with rows projected from `agenticspec.view.EventCount` at `eventual` consistency, containing instances where `(session_id == param.session_id and (not (defined(param.agent)) or agent == param.agent))` |
| view query | `agenticspec.view.SessionEvents` | kept an obligation by a comparison with the optional view parameter `agent`, which the interpreted target refuses a read that leaves out, where a generated query would hide rows, in `filter:` | a query answering `agenticspec.view.SessionEvents` with rows projected from `agenticspec.view.EventRow` at `eventual` consistency, containing instances where `(session_id == param.session_id and (not (defined(param.agent)) or agent == param.agent) and (not (defined(param.kind)) or kind == param.kind) and (not (defined(param.notable)) or notable == param.notable) and (not (defined(param.before)) or ord < param.before))` |
| view query | `agenticspec.view.SessionList` | kept an obligation by a comparison with the optional view parameter `since`, which the interpreted target refuses a read that leaves out, where a generated query would hide rows, in `filter:` | a query answering `agenticspec.view.SessionList` with rows projected from `agenticspec.view.SessionListing` at `eventual` consistency, containing instances where `((not (defined(param.since)) or last_at >= param.since as timestamp) and (not (defined(param.search)) or names contains {param: search}) and (not (defined(param.limit)) or rank <= param.limit) and (not (defined(param.offset)) or rank > param.offset))` |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
