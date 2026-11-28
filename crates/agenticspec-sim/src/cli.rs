//! The commands of `agenticspec sim` (check, digest, run, render, ladder, ports, contract and
//! trace), of `agenticspec ingress` (default, project) and of the deploy side (`agenticspec
//! packing default`, `agenticspec deploy bundle`, `agenticspec deploy check`). Each returns the
//! result the command line prints, a JSON value of the shape `spec/domains/cli.yaml` declares
//! (`agenticspec.cli.Sim*Result`), or a [`Refusal`] with one of the error codes `spec/cli.yaml`
//! binds to the command. The parser and the process adapter are the generated
//! `agenticspec-cli-contract` of the `agenticspec` crate.

use crate::check::{GATE_KPIS, Refusal as ArchRefusal, check as check_architecture};
use crate::doc::load_yaml;
use crate::event_log::{FILE as EVENTS_FILE, jsonl, log as event_log};
use crate::{Experiment, ladder as ladder_doc, nest, run_shown};
use serde_json::{Value, json};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// A refused command: the error code of the binding and its detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub code: &'static str,
    pub detail: String,
}

pub(crate) fn refused(code: &'static str, detail: impl Into<String>) -> Refusal {
    Refusal {
        code,
        detail: detail.into(),
    }
}

fn write(path: &Path, text: &str) -> Result<(), Refusal> {
    let unwritable = |e: String| refused("out_unwritable", e);
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| unwritable(format!("{}: {e}", dir.display())))?;
    }
    std::fs::write(path, text).map_err(|e| unwritable(format!("{}: {e}", path.display())))
}

/// The document at `path` with its nested architectures resolved, or the refusal of `sim check`.
fn load_nested(path: &Path) -> Result<Value, Refusal> {
    nest::load(path).map_err(|e| match e {
        nest::Error::Unreadable(e) => refused("document_unreadable", e),
        refusals @ nest::Error::Refused(_) => refused("architecture_refused", refusals.to_string()),
    })
}

/// The options of `sim check`.
#[derive(Debug, Clone)]
pub struct CheckOptions<'a> {
    pub architecture: &'a Path,
    /// The ports file; `blocks/ports.json` when absent. Read only for a document with `ingress:`.
    pub ports: Option<&'a Path>,
    /// The KPI catalogue; `kpis/catalogue.yaml` when absent. Read only for a document with
    /// `ingress:` or `requirements:`.
    pub kpis: Option<&'a Path>,
}

/// `sim check <architecture>` with the default ports file and KPI catalogue. With `--json`,
/// [`check_json`].
pub fn check(path: &Path) -> Result<Value, Refusal> {
    check_with(&CheckOptions {
        architecture: path,
        ports: None,
        kpis: None,
    })
}

/// The contract of a document the structural check accepts, its ingresses (the declared ones
/// and the defaults in effect) resolved: refused with every refusal of the check of `ingress:`
/// (`architecture_refused`).
fn derive_contract(
    arch: &Value,
    ports: &crate::contract::ports::PortsFile,
    kpis: Option<&Path>,
) -> Result<crate::contract::derive::Contract, Refusal> {
    let kpis =
        crate::contract::ingress::Kpis::load(kpis.unwrap_or(Path::new("kpis/catalogue.yaml")))
            .map_err(|e| refused("kpis_unreadable", e))?;
    derive_contract_with(arch, ports, &kpis)
}

/// [`derive_contract`] with the KPI catalogue in memory.
fn derive_contract_with(
    arch: &Value,
    ports: &crate::contract::ports::PortsFile,
    kpis: &crate::contract::ingress::Kpis,
) -> Result<crate::contract::derive::Contract, Refusal> {
    use crate::contract::{derive as d, ingress};
    let mut contract =
        d::derive(arch, ports).map_err(|r| refused("contract_refused", r.join("; ")))?;
    let (resolved, refusals) = ingress::resolve(arch, &contract, ports, kpis);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    contract.ingress = resolved;
    Ok(contract)
}

/// The refusals of the reserved names of the default control plane, as the error of a command.
fn refuse_reserved(arch: &Value) -> Result<(), Refusal> {
    let refusals = crate::contract::ingress::reserved(arch);
    if refusals.is_empty() {
        return Ok(());
    }
    let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
    Err(refused("architecture_refused", names.join("; ")))
}

/// `sim check <architecture> [--ports <file>] [--kpis <file>]`: the architecture with its
/// counts, the KPIs of the architecture as a whole (`whole.rs`) and its nested architectures,
/// or every refusal of the check. The references are resolved first; the check and the KPIs
/// then read the document flattened. Every document is checked for the reserved names of the
/// default control plane; a document with `ingress:` is checked against the ports file and the
/// KPI catalogue too. Without `ingress:` it has the two default ingresses, which the check never
/// refuses.
pub fn check_with(o: &CheckOptions) -> Result<Value, Refusal> {
    let arch = load_nested(o.architecture)?;
    let flat = nest::flatten(&arch);
    let refusals = check_architecture(&flat);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    accepted(&arch, &flat, o)
}

/// The result of `sim check` for a document (`arch`, `flat` flattened) the structural check
/// accepted: its counts, the KPIs of the architecture as a whole, its single points, its nested
/// architectures and its ingresses (the declared ones and the defaults in effect), or the
/// refusal of a reserved name or of its `ingress:`.
fn accepted(arch: &Value, flat: &Value, o: &CheckOptions) -> Result<Value, Refusal> {
    refuse_reserved(flat)?;
    let kpis: serde_json::Map<String, Value> = crate::whole::measure(flat)
        .into_iter()
        .map(|(k, v)| (k.to_string(), crate::doc::number(v)))
        .collect();
    let mut ingresses = crate::contract::ingress::effective(flat).0.len();
    if crate::contract::ingress::declares(flat) {
        let ports_path = o.ports.unwrap_or(Path::new("blocks/ports.json"));
        let ports = crate::contract::ports::PortsFile::load(ports_path)
            .map_err(|e| refused("ports_unreadable", e))?;
        let contract = derive_contract(flat, &ports, o.kpis).map_err(|r| match r.code {
            "contract_refused" => refused("architecture_refused", r.detail),
            _ => r,
        })?;
        ingresses = contract.ingress.len();
    }
    let requirements = check_requirements(flat, o.kpis)?;
    let mut out = summary(arch);
    out["kpis"] = Value::Object(kpis);
    out["single_points"] = json!(crate::whole::single_point_parts(flat));
    out["nested"] = json!(nest::listed(arch));
    out["ingresses"] = json!(ingresses);
    out["requirements"] = json!(requirements);
    Ok(out)
}

/// The requirements of a document (`flat`, flattened) the structural check accepted, each with
/// its stage and state ([`crate::requirements::at_check`]), or every refusal of its
/// `requirements:` (`architecture_refused`). The KPI catalogue (`kpis/catalogue.yaml` when
/// `kpis` is absent) is read only for a document with `requirements:`. A missed requirement is
/// reported in the list, never refused.
fn check_requirements(flat: &Value, kpis: Option<&Path>) -> Result<Vec<Value>, Refusal> {
    use crate::requirements as req;
    if !req::declares(flat) {
        return Ok(Vec::new());
    }
    let kpis =
        crate::contract::ingress::Kpis::load(kpis.unwrap_or(Path::new("kpis/catalogue.yaml")))
            .map_err(|e| refused("kpis_unreadable", e))?;
    let refusals = req::refusals(flat, &kpis);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    Ok(req::at_check(flat, &kpis))
}

fn summary(arch: &Value) -> Value {
    json!({
        "architecture": arch["architecture"].as_str().unwrap_or("architecture"),
        "parts": arch["parts"].as_object().map_or(0, |p| p.len()),
        "links": arch["links"].as_array().map_or(0, |l| l.len()),
    })
}

/// The format of the document `sim check --json` prints.
pub const CHECK_FORMAT: &str = "agenticspec-check/1";

/// What `sim check --json` says about a readable architecture: the `agenticspec-check/1`
/// document, and the refusal the command ends with when a gate rule is violated.
#[derive(Debug, Clone, PartialEq)]
pub struct Checked {
    pub document: Value,
    pub refusal: Option<Refusal>,
}

/// Whether a rule of the check refuses the architecture. Every rule of the check is a gate.
pub fn is_gate(rule: &str) -> bool {
    GATE_KPIS.contains(&rule)
}

/// The index of the link a refusal concerns, when its subject is a link.
fn link_of(r: &ArchRefusal) -> Option<usize> {
    match r {
        ArchRefusal::UnknownLinkPart { link, .. }
        | ArchRefusal::UnknownCarries { link, .. }
        | ArchRefusal::QuestionWithoutAnswerer { link, .. }
        | ArchRefusal::ReviewWithoutReviewer { link, .. }
        | ArchRefusal::TriggerSourceNotTrigger { link, .. }
        | ArchRefusal::TriggerTargetNotAgent { link, .. } => Some(*link),
        _ => None,
    }
}

/// One violation as the document carries it.
fn violation(arch: &Value, r: &ArchRefusal) -> Value {
    let line = r.to_string();
    let prefix = format!("{}: part `{}`: ", r.rule(), r.part());
    let message = line.strip_prefix(&prefix).unwrap_or(&line);
    let link = link_of(r);
    let subject = link.map_or_else(
        || r.part().to_string(),
        |i| {
            let l = &arch["links"][i];
            let end = |k: &str| l[k].as_str().unwrap_or("").to_string();
            format!("{}->{}:{}", end("from"), end("to"), end("carries"))
        },
    );
    let mut v = json!({
        "rule": r.rule(),
        "gate": is_gate(r.rule()),
        "subject": subject,
        "part": r.part(),
        "message": message,
    });
    if let Some(i) = link {
        v["link"] = json!(i);
    }
    v
}

/// The `agenticspec-check/1` document of an architecture, its nested architectures resolved:
/// its summary, `valid`, every violation of the flattened document sorted by rule, subject,
/// part, link and message, and the count of every rule.
pub fn check_document(arch: &Value) -> Value {
    let flat = nest::flatten(arch);
    let refusals = check_architecture(&flat);
    let mut violations: Vec<Value> = refusals.iter().map(|r| violation(&flat, r)).collect();
    let key = |v: &Value| {
        (
            v["rule"].as_str().unwrap_or("").to_string(),
            v["subject"].as_str().unwrap_or("").to_string(),
            v["part"].as_str().unwrap_or("").to_string(),
            v["link"].as_u64(),
            v["message"].as_str().unwrap_or("").to_string(),
        )
    };
    violations.sort_by_key(key);
    let counts: serde_json::Map<String, Value> = GATE_KPIS
        .iter()
        .map(|g| {
            let n = refusals.iter().filter(|r| r.rule() == *g).count();
            ((*g).to_string(), json!(n))
        })
        .collect();
    let mut doc = summary(arch);
    doc["format"] = json!(CHECK_FORMAT);
    doc["valid"] = json!(!violations.iter().any(|v| v["gate"] == true));
    doc["violations"] = Value::Array(violations);
    doc["counts"] = Value::Object(counts);
    doc
}

/// `sim check --json <architecture> [--ports <file>] [--kpis <file>]`: the document, also for a
/// refused architecture; only an unreadable file or a nested reference that does not resolve is
/// refused without one. An accepted architecture's document also carries what `sim check`
/// prints (KPIs, single points, nested architectures, ingresses); a refused `ingress:` is the
/// command's refusal after the document.
pub fn check_json(o: &CheckOptions) -> Result<Checked, Refusal> {
    let arch = load_nested(o.architecture)?;
    let flat = nest::flatten(&arch);
    let mut document = check_document(&arch);
    let gates: Vec<String> = check_architecture(&flat)
        .iter()
        .filter(|r| is_gate(r.rule()))
        .map(ToString::to_string)
        .collect();
    if !gates.is_empty() {
        let refusal = Some(refused("architecture_refused", gates.join("; ")));
        return Ok(Checked { document, refusal });
    }
    match accepted(&arch, &flat, o) {
        Ok(fields) => {
            for (k, v) in fields.as_object().into_iter().flatten() {
                document[k] = v.clone();
            }
            Ok(Checked {
                document,
                refusal: None,
            })
        }
        Err(r) => Ok(Checked {
            document,
            refusal: Some(r),
        }),
    }
}

/// `sim digest <architecture>`: the digest a referring document pins it with.
pub fn digest(path: &Path) -> Result<Value, Refusal> {
    let unreadable = |e: String| refused("document_unreadable", format!("{}: {e}", path.display()));
    let text = std::fs::read_to_string(path).map_err(|e| unreadable(e.to_string()))?;
    let digest = nest::digest_text(&text).map_err(unreadable)?;
    Ok(json!({ "architecture": path.display().to_string(), "digest": digest }))
}

/// KPIs of triggers and swarms, printed only when a run has them.
const TRIGGER_KPIS: &[&str] = &[
    "firings",
    "firings_skipped",
    "firing_start_wait_p90_seconds",
    "firing_failure_share",
    "claim_conflicts",
    "coordination_calls",
    "merge_conflicts",
];

/// The KPIs `sim run` prints for the baseline and the best variant.
const RUN_KPIS: &[&str] = &[
    "tasks_arrived",
    "tasks_done",
    "tasks_correct",
    "false_completions_shipped",
    "rework_turns",
    "review_calls",
    "backlog_wait_p90_seconds",
    "worker_busy_share",
    "owner_wait_share",
    "questions_defaulted",
    "queue_wait_p90_seconds",
    "cost_usd",
    "firings",
    "firings_skipped",
    "firing_start_wait_p90_seconds",
    "firing_failure_share",
    "claim_conflicts",
    "coordination_calls",
    "merge_conflicts",
];

fn kpi(r: &Value, which: &str, name: &str) -> String {
    r[which]["kpis"][name]["value"]
        .as_f64()
        .map_or("-".into(), |v| format!("{v:.4}"))
}

/// `sim run <experiment> --out <dir> [--threads N] [--events]`: writes `result.json`,
/// `index.html` and, when the result has a best variant, `best.yaml` into `out`; with `events`,
/// also `events.jsonl`, the run the result's timeline samples as block events
/// ([`crate::event_log`]). Refusals of the architecture and the KPIs of the baseline and the best
/// variant go to `log`.
pub fn run(
    path: &Path,
    out: &Path,
    threads: Option<usize>,
    events: bool,
    log: &mut dyn Write,
) -> Result<Value, Refusal> {
    let invalid = |e: String| refused("experiment_invalid", e);
    let threads = threads
        .filter(|&t| t > 0)
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get().min(8)));
    let started = Instant::now();
    let e = Experiment::load(path).map_err(invalid)?;
    for r in check_architecture(&nest::flatten(&e.architecture)) {
        let _ = writeln!(log, "agenticspec sim: refused: {r}");
    }
    let (result, shown) = run_shown(&e, threads).map_err(invalid)?;
    let mut text = serde_json::to_string_pretty(&result).map_err(|e| invalid(e.to_string()))?;
    text.push('\n');
    write(&out.join("result.json"), &text)?;
    let written = if events {
        let log = event_log(&shown.architecture, &shown.run.timeline).map_err(invalid)?;
        write(&out.join(EVENTS_FILE), &jsonl(&log))?;
        Some(log.len())
    } else {
        None
    };
    write(
        &out.join("index.html"),
        &agenticspec_render::architecture::page(&result["architecture"], Some(&result)),
    )?;
    let source = e.doc["architecture"].as_str().and_then(|p| {
        std::fs::read_to_string(path.parent().unwrap_or(Path::new(".")).join(p)).ok()
    });
    let best = crate::best::yaml_of_result(&result, source.as_deref()).map_err(invalid)?;
    if let Some(best) = &best {
        write(&out.join("best.yaml"), best)?;
    }
    let variants = result["variants"].as_array().map_or(0, |v| v.len());
    let _ = writeln!(
        log,
        "agenticspec sim: {variants} variants x {} seeds in {:.1} s on {threads} threads; fitness baseline {} best {}",
        e.seeds,
        started.elapsed().as_secs_f64(),
        result["baseline"]["fitness"],
        result["best"]["fitness"],
    );
    for name in RUN_KPIS {
        let zero = |which| kpi(&result, which, name) == "0.0000";
        if TRIGGER_KPIS.contains(name) && zero("baseline") && zero("best") {
            continue;
        }
        let _ = writeln!(
            log,
            "  {name:<26} baseline {:>12} best {:>12}",
            kpi(&result, "baseline", name),
            kpi(&result, "best", name)
        );
    }
    let _ = writeln!(log, "  best params {}", result["best"]["params"]);
    let mut answer = json!({
        "out": out.display().to_string(),
        "variants": variants,
        "seeds": e.seeds,
        "threads": threads,
        "best_written": best.is_some(),
    });
    if let Some(n) = written {
        answer["events"] = json!(n);
    }
    Ok(answer)
}

/// `sim render <architecture> [--result result.json] --out <page.html>`. A nested architecture is
/// drawn as one box; a reference that does not resolve is drawn as written. A document of format
/// `agenticspec-render-model/1` is drawn as that page (`--result` is not read); one the page's
/// check refuses is `document_unreadable` with the check's refusal.
pub fn render(path: &Path, result: Option<&Path>, out: &Path) -> Result<Value, Refusal> {
    let unreadable = |e: String| refused("document_unreadable", e);
    if let Ok(doc) = crate::doc::load_yaml(path)
        && doc.get("format").and_then(Value::as_str) == Some(agenticspec_render::page::FORMAT)
    {
        let page: agenticspec_render::page::Page = serde_json::from_value(doc)
            .map_err(|e| unreadable(format!("{}: {e}", path.display())))?;
        write(out, &agenticspec_render::page::render(&page))?;
        return Ok(json!({ "out": out.display().to_string() }));
    }
    let arch = match nest::load(path) {
        Ok(a) => a,
        Err(nest::Error::Unreadable(e)) => return Err(unreadable(e)),
        Err(nest::Error::Refused(_)) => load_yaml(path).map_err(unreadable)?,
    };
    let result: Option<Value> = match result {
        Some(p) => {
            let text = std::fs::read_to_string(p)
                .map_err(|e| unreadable(format!("{}: {e}", p.display())))?;
            Some(
                serde_json::from_str(&text)
                    .map_err(|e| unreadable(format!("{}: {e}", p.display())))?,
            )
        }
        None => None,
    };
    write(
        out,
        &agenticspec_render::architecture::page(&arch, result.as_ref()),
    )?;
    Ok(json!({ "out": out.display().to_string() }))
}

/// `sim ladder <ladder.yaml> --out <ladder.json> [--check]`: writes the data file of the ladder
/// page, or with `check` refuses a file that differs from a fresh generation.
pub fn ladder(path: &Path, out: &Path, check: bool) -> Result<Value, Refusal> {
    let (doc, dir) = ladder_doc::load(path).map_err(|e| refused("document_unreadable", e))?;
    let refusals = ladder_doc::check(&doc, &dir);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("ladder_refused", names.join("; ")));
    }
    let text = ladder_doc::render(path).map_err(|e| refused("ladder_refused", e))?;
    let rungs = doc["rungs"].as_array().map_or(0, |r| r.len());
    let answer = |written: bool| json!({ "out": out.display().to_string(), "rungs": rungs, "written": written });
    if check {
        if std::fs::read_to_string(out).ok().as_deref() == Some(text.as_str()) {
            return Ok(answer(false));
        }
        return Err(refused(
            "ladder_stale",
            format!(
                "{} differs from a fresh generation of {}: run task ladder:generate",
                out.display(),
                path.display()
            ),
        ));
    }
    write(out, &text)?;
    Ok(answer(true))
}

/// `sim scenario check <scenario>`: the scenario checked against its architecture and profile,
/// or every problem; `sim scenario run <scenario> --out <dir>`: runs it, writes
/// `scenario-result.json` into `out`, and refuses with `expectation_failed` when an expectation
/// or a KPI bound failed.
pub fn scenario(action: &str, path: &Path, out: Option<&Path>) -> Result<Value, Refusal> {
    let l = crate::scenario::load_checked(path)?;
    let s = &l.scenario;
    let mut answer = json!({
        "scenario": s.name,
        "architecture": l.architecture["architecture"].as_str().unwrap_or("architecture"),
        "inputs": s.when.len(),
        "expectations": s.then.events.len() + s.then.kpis.len(),
    });
    match action {
        "check" => return Ok(answer),
        "run" => {}
        other => {
            return Err(refused(
                "scenario_invalid",
                format!("action `{other}`, expected check or run"),
            ));
        }
    }
    let Some(out) = out else {
        return Err(refused(
            "out_missing",
            "sim scenario run writes scenario-result.json into the directory --out names",
        ));
    };
    let ran = crate::scenario::run(&l)?;
    let doc = crate::scenario::result_json(&l, &ran);
    let mut text =
        serde_json::to_string_pretty(&doc).map_err(|e| refused("out_unwritable", e.to_string()))?;
    text.push('\n');
    let file = out.join("scenario-result.json");
    write(&file, &text)?;
    use crate::scenario::Status;
    let (held, failed, not_checked) = (
        ran.count(Status::Held),
        ran.count(Status::Failed),
        ran.count(Status::NotChecked),
    );
    if failed > 0 {
        return Err(refused(
            "expectation_failed",
            format!(
                "{}: {failed} failed, {held} held, {not_checked} not checked; see {}",
                s.name,
                file.display()
            ),
        ));
    }
    answer["out"] = json!(out.display().to_string());
    answer["verdict"] = json!("held");
    answer["predicted"] = json!(ran.predicted.name());
    answer["held"] = json!(held);
    answer["failed"] = json!(failed);
    answer["not_checked"] = json!(not_checked);
    Ok(answer)
}

/// `sim corpus run <corpus>`: runs every entry, prints the table to `log`, and refuses with
/// `corpus_failed` when an entry's expectations failed or it predicts another class than it
/// expects.
pub fn corpus(action: &str, path: &Path, log: &mut dyn Write) -> Result<Value, Refusal> {
    if action != "run" {
        return Err(refused(
            "corpus_invalid",
            format!("action `{action}`, expected run"),
        ));
    }
    let (c, rows) = crate::corpus::run(path)?;
    let _ = write!(log, "{}", crate::corpus::table(&rows));
    let failed: Vec<&str> = rows
        .iter()
        .filter(|r| !r.holds())
        .map(|r| r.entry.as_str())
        .collect();
    if !failed.is_empty() {
        return Err(refused(
            "corpus_failed",
            format!(
                "{}: {} of {} entries failed: {}",
                c.name,
                failed.len(),
                rows.len(),
                failed.join(", ")
            ),
        ));
    }
    Ok(json!({
        "corpus": c.name,
        "entries": rows.len(),
        "held": rows.len(),
        "failed": 0,
        "rows": rows.iter().map(crate::corpus::Row::json).collect::<Vec<_>>(),
    }))
}

/// The options of `sim ports`.
#[derive(Debug, Clone, Default)]
pub struct PortsOptions<'a> {
    /// The blocks directory; `blocks` when absent.
    pub blocks: Option<&'a Path>,
    pub out: Option<&'a Path>,
    pub check: Option<&'a Path>,
    pub expect: Option<&'a Path>,
}

/// `sim ports [--blocks <dir>] [--out <file>] [--check <file>] [--expect <file>]`: derive the
/// ports of the building blocks and write them (or print the file), compare a ports file's
/// digest with its sources (`--check`, no derivation and no `ess` call), or list every difference
/// from an expected list (`--expect`: one line per difference to `log`, refused as
/// `ports_differ` when there is one).
pub fn ports(o: &PortsOptions, log: &mut dyn Write) -> Result<Value, Refusal> {
    use crate::contract::ports as p;
    let blocks = o.blocks.unwrap_or(Path::new("blocks"));
    if let Some(file) = o.check {
        let have = p::PortsFile::load(file).map_err(|e| refused("document_unreadable", e))?;
        let digest =
            p::sources_digest(blocks, &have.ess).map_err(|e| refused("blocks_unreadable", e))?;
        if digest != have.sources_sha256 {
            return Err(refused(
                "ports_stale",
                format!(
                    "{} was derived from other sources than {}: run task ports:generate",
                    file.display(),
                    blocks.display()
                ),
            ));
        }
        return Ok(json!({
            "ports": have.ports.len(),
            "sources_sha256": digest,
            "written": false,
            "differences": [],
        }));
    }
    let unreadable = |e: String| refused("blocks_unreadable", e);
    let ess = p::ess_version().map_err(unreadable)?;
    let digest = p::sources_digest(blocks, &ess).map_err(unreadable)?;
    let ir = p::compile(blocks).map_err(unreadable)?;
    let derived = p::derive(&ir).map_err(|e| refused("ports_refused", e.join("; ")))?;
    if let Some(expect) = o.expect {
        let rows = p::load_expected(expect).map_err(|e| refused("document_unreadable", e))?;
        let lines = p::compare(&derived.ports, &rows);
        for line in &lines {
            let _ = writeln!(log, "{line}");
        }
        if !lines.is_empty() {
            return Err(refused(
                "ports_differ",
                format!("{} difference(s) from {}", lines.len(), expect.display()),
            ));
        }
        return Ok(json!({
            "ports": derived.ports.len(),
            "sources_sha256": digest,
            "written": false,
            "differences": [],
        }));
    }
    let text = p::file_text(
        &derived.ports,
        &derived.bindings,
        &derived.surface,
        &ess,
        &digest,
    );
    let mut answer = json!({
        "ports": derived.ports.len(),
        "sources_sha256": digest,
        "written": o.out.is_some(),
        "differences": [],
    });
    match o.out {
        Some(out) => write(out, &text)?,
        None => answer["file"] = json!(text),
    }
    Ok(answer)
}

/// The options of `sim contract`.
#[derive(Debug, Clone)]
pub struct ContractOptions<'a> {
    pub architecture: &'a Path,
    /// The ports file; `blocks/ports.json` when absent.
    pub ports: Option<&'a Path>,
    /// The contract file to write, or with `suite` the cut suite.
    pub out: Option<&'a Path>,
    /// A written contract to check against the document and the ports, without deriving.
    pub check: Option<&'a Path>,
    /// A conformance suite of the building blocks to cut to the contract's ports.
    pub suite: Option<&'a Path>,
    /// A trace (`agenticspec-trace/1`) to check the contract's routing invariants on.
    pub trace: Option<&'a Path>,
    /// The KPI catalogue; `kpis/catalogue.yaml` when absent. The metrics of the default
    /// `metrics` ingress are resolved against it, so every derivation reads it.
    pub kpis: Option<&'a Path>,
}

/// `sim contract <architecture> [--ports <file>] [--kpis <file>] [--out <file>] [--check <file>]
/// [--suite <suite.json>] [--trace <trace.json>]`: derive the contract of an architecture
/// document, its ingresses resolved, and check it, then write it (or print it); refused, writing
/// nothing, when the document fails `sim check`. With `--check`, read a written contract and check
/// it against the document and the ports instead. With `--suite`, cut the suite to the contract's
/// ports and write the cut suite to `--out`. With `--trace`, check every routing invariant on the
/// trace, one line each to `log`, refused as `invariant_violated` when one is violated.
pub fn contract(o: &ContractOptions, log: &mut dyn Write) -> Result<Value, Refusal> {
    use crate::contract::derive as d;
    use crate::contract::ports::PortsFile;
    let arch = nest::flatten(&load_nested(o.architecture)?);
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let ports_path = o.ports.unwrap_or(Path::new("blocks/ports.json"));
    let ports = PortsFile::load(ports_path).map_err(|e| refused("ports_unreadable", e))?;
    let contract = match o.check {
        Some(file) => {
            let text = std::fs::read_to_string(file)
                .map_err(|e| refused("document_unreadable", format!("{}: {e}", file.display())))?;
            d::Contract::parse(&text)
                .map_err(|e| refused("contract_refused", format!("{}: {e}", file.display())))?
        }
        None => derive_contract(&arch, &ports, o.kpis)?,
    };
    let problems = d::check(&contract, &arch, &ports);
    if !problems.is_empty() {
        return Err(refused("contract_refused", problems.join("; ")));
    }
    let mut answer = json!({
        "architecture": contract.architecture,
        "ports": contract.ports.len(),
        "invariants": contract.invariants.len(),
        "ingresses": contract.ingress.len(),
        "written": false,
        "checks": [],
    });
    if let Some(trace) = o.trace {
        answer["checks"] = check_trace(&contract, trace, log)?;
    }
    if let Some(suite) = o.suite {
        let (kept, total) = cut_suite(&contract, &ports, suite, o.out)?;
        answer["suite_scenarios"] = json!(total);
        answer["kept_scenarios"] = json!(kept);
        answer["written"] = json!(o.out.is_some());
        return Ok(answer);
    }
    if o.check.is_some() || o.trace.is_some() {
        return Ok(answer);
    }
    let text = contract.text();
    match o.out {
        Some(out) => {
            write(out, &text)?;
            answer["written"] = json!(true);
        }
        None => answer["contract"] = json!(text),
    }
    Ok(answer)
}

/// The port catalogue of the blocks this binary was built with.
const BUILT_IN_PORTS: &str = include_str!("../../../blocks/ports.json");
/// The KPI catalogue this binary was built with.
const BUILT_IN_KPIS: &str = include_str!("../../../kpis/catalogue.yaml");

/// The contract text of the architecture at `architecture` (what `sim contract` prints), derived
/// from the port and KPI catalogues built into this binary, so it reads no file but the
/// architecture and its nested documents (`agenticspec ui serve` and `ui conform` outside a
/// checkout).
///
/// # Errors
///
/// The refusals of [`contract`] for the same document.
pub fn contract_built_in(architecture: &Path) -> Result<String, Refusal> {
    use crate::contract::{derive as d, ingress::Kpis, ports::PortsFile};
    let arch = nest::flatten(&load_nested(architecture)?);
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let ports = PortsFile::parse(BUILT_IN_PORTS)
        .map_err(|e| refused("ports_unreadable", format!("the built-in ports: {e}")))?;
    let kpis = serde_yaml_ng::from_str::<Value>(BUILT_IN_KPIS)
        .map_err(|e| e.to_string())
        .and_then(|doc| Kpis::parse(&doc))
        .map_err(|e| {
            refused(
                "kpis_unreadable",
                format!("the built-in KPI catalogue: {e}"),
            )
        })?;
    let contract = derive_contract_with(&arch, &ports, &kpis)?;
    let problems = d::check(&contract, &arch, &ports);
    if !problems.is_empty() {
        return Err(refused("contract_refused", problems.join("; ")));
    }
    Ok(contract.text())
}

/// Cut the suite at `path` to the contract's ports and write it to `out`; the counts kept and
/// whole.
fn cut_suite(
    contract: &crate::contract::derive::Contract,
    ports: &crate::contract::ports::PortsFile,
    path: &Path,
    out: Option<&Path>,
) -> Result<(usize, usize), Refusal> {
    use crate::contract::suite;
    let unreadable = |e: String| refused("suite_unreadable", format!("{}: {e}", path.display()));
    let text = std::fs::read_to_string(path).map_err(|e| unreadable(e.to_string()))?;
    let whole: Value = serde_json::from_str(&text).map_err(|e| unreadable(e.to_string()))?;
    let commands = suite::commands(contract, ports);
    let cut = suite::cut(&whole, &commands, &ports.bindings).map_err(unreadable)?;
    if let Some(out) = out {
        let mut text = serde_json::to_string(&cut.suite)
            .map_err(|e| refused("out_unwritable", e.to_string()))?;
        text.push('\n');
        write(out, &text)?;
    }
    Ok((cut.kept.len(), cut.total))
}

/// Check the contract's routing invariants on the trace at `path`: one line per invariant to
/// `log`, the checks as JSON, refused when one is violated.
fn check_trace(
    contract: &crate::contract::derive::Contract,
    path: &Path,
    log: &mut dyn Write,
) -> Result<Value, Refusal> {
    use crate::contract::trace::{self, Status, Trace};
    let unreadable = |e: String| refused("trace_unreadable", format!("{}: {e}", path.display()));
    let text = std::fs::read_to_string(path).map_err(|e| unreadable(e.to_string()))?;
    let t = Trace::parse(&text).map_err(unreadable)?;
    let checks = trace::check(contract, &t).map_err(unreadable)?;
    let mut violated = Vec::new();
    for c in &checks {
        let note = match (&c.detail, c.status) {
            (Some(d), _) => format!(": {d}"),
            (None, _) => String::new(),
        };
        let _ = writeln!(
            log,
            "{:<11} {} ({} uses){note}",
            c.status.as_str(),
            c.link,
            c.uses
        );
        if c.status == Status::Violated {
            violated.push(format!(
                "link `{}`: {}",
                c.link,
                c.detail.as_deref().unwrap_or("violated")
            ));
        }
    }
    if !violated.is_empty() {
        return Err(refused("invariant_violated", violated.join("; ")));
    }
    Ok(Value::Array(checks.iter().map(|c| c.to_json()).collect()))
}

/// The architecture at `architecture` as read (nested architectures resolved beside their
/// parts), the flattened document at the values it is built with, and its simulation model under
/// the profile at `profile` for `horizon` seconds: what `sim trace` and `ui serve --sim` run.
///
/// # Errors
///
/// A document cannot be read, the check refuses the architecture, or the simulator refuses the
/// pair.
pub fn built_model(
    architecture: &Path,
    profile: &Path,
    horizon: f64,
) -> Result<(Value, Value, crate::sim::Model), Refusal> {
    use crate::profile::Profile;
    let document = load_nested(architecture)?;
    let arch = nest::flatten(&document);
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let profile_doc = load_yaml(profile).map_err(|e| refused("document_unreadable", e))?;
    let profile = Profile::parse(&profile_doc).map_err(|e| refused("simulation_refused", e))?;
    let built = crate::marked::resolve(&arch, &serde_json::Map::new())
        .map_err(|e| refused("simulation_refused", e))?;
    let model = crate::sim::Model::build(&built, &profile, horizon, &[], &[])
        .map_err(|e| refused("simulation_refused", e))?;
    Ok((document, built, model))
}

/// The options of `sim trace`.
#[derive(Debug, Clone)]
pub struct TraceOptions<'a> {
    pub architecture: &'a Path,
    pub profile: &'a Path,
    /// The seed of the run; 1 when absent.
    pub seed: Option<u64>,
    /// The simulated time, as a duration; one day when absent.
    pub horizon: Option<&'a str>,
    pub out: Option<&'a Path>,
}

/// `sim trace <architecture> --profile <profile> [--seed <n>] [--horizon <duration>]
/// [--out <trace.json>]`: simulate the architecture once at the values it is built with and
/// write the port uses of the run as a trace (`agenticspec-trace/1`), or print it.
pub fn trace(o: &TraceOptions) -> Result<Value, Refusal> {
    use crate::contract::trace::Trace;
    use crate::sim::{RECORDED, simulate};
    let horizon = match o.horizon {
        None => 86400.0,
        Some(h) => crate::doc::parse_duration(h)
            .filter(|s| *s > 0.0)
            .ok_or_else(|| {
                refused(
                    "simulation_refused",
                    format!("horizon `{h}` is no positive duration"),
                )
            })?,
    };
    let seed = o.seed.unwrap_or(1);
    let (_, built, model) = built_model(o.architecture, o.profile, horizon)?;
    let profile_doc = load_yaml(o.profile).map_err(|e| refused("document_unreadable", e))?;
    let run = simulate(&model, seed, true);
    let name = built["architecture"].as_str().unwrap_or("architecture");
    let source = json!({
        "simulator": "agenticspec-sim",
        "profile": profile_doc["profile"],
        "seed": seed,
        "horizon_seconds": horizon,
    });
    let t = Trace::from_uses(name, source, RECORDED, &run.uses);
    let text = t.text();
    let mut answer = json!({
        "architecture": name,
        "seed": seed,
        "uses": run.uses.len(),
        "events": t.events.len(),
        "written": o.out.is_some(),
    });
    match o.out {
        Some(out) => write(out, &text)?,
        None => answer["trace"] = json!(text),
    }
    Ok(answer)
}

/// The fixed arguments of `sim score`.
#[derive(Debug, Clone)]
pub struct ScoreOptions<'a> {
    pub profile: &'a Path,
    /// The seed, as an experiment's `seed:`; 0 when absent.
    pub seed: Option<u64>,
    /// The simulated time, as a duration; one day when absent.
    pub horizon: Option<&'a str>,
}

/// The exit code of `sim score` for a refused document; a refusal of its fixed arguments exits 1.
pub const SCORE_REFUSED_EXIT: i32 = 3;

/// Whether a refusal of `sim score` is about the document (exit [`SCORE_REFUSED_EXIT`]) rather
/// than about its fixed arguments (`profile_refused`, `horizon_invalid`, exit 1).
pub fn score_refuses_document(code: &str) -> bool {
    !matches!(
        code,
        "profile_refused" | "horizon_invalid" | "documents_missing"
    )
}

/// The answer of `sim score` for a refusal: `{"error": {"code", "message"}}`.
pub fn score_error(r: &Refusal) -> Value {
    json!({"error": {"code": r.code, "message": r.detail}})
}

/// `sim score`: the profile read once, then one simulation per document.
#[derive(Debug, Clone)]
pub struct Scorer {
    profile: crate::profile::Profile,
    horizon: f64,
    seed: u64,
}

impl Scorer {
    /// Reads the profile (with the corpus it names, relative to it) and the horizon.
    ///
    /// # Errors
    ///
    /// `profile_refused` for a profile that cannot be read or parsed, `horizon_invalid` for a
    /// horizon that is no positive duration.
    pub fn new(o: &ScoreOptions) -> Result<Self, Refusal> {
        let horizon = match o.horizon {
            None => 86400.0,
            Some(h) => crate::doc::parse_duration(h)
                .filter(|s| *s > 0.0)
                .ok_or_else(|| {
                    refused("horizon_invalid", format!("`{h}` is no positive duration"))
                })?,
        };
        let profile_refused = |e: String| refused("profile_refused", e);
        let doc = load_yaml(o.profile).map_err(profile_refused)?;
        let mut profile = crate::profile::Profile::parse(&doc)
            .map_err(|e| profile_refused(format!("{}: {e}", o.profile.display())))?;
        profile
            .read_corpus(o.profile.parent().unwrap_or(Path::new(".")))
            .map_err(profile_refused)?;
        Ok(Scorer {
            profile,
            horizon,
            seed: o.seed.unwrap_or(0),
        })
    }

    /// The KPIs of the architecture at `path`, simulated once as written: `{kpis, per_part}`,
    /// each KPI a number by its id. The run is the baseline `sim run` evaluates for an experiment
    /// with this seed and `seeds: 1` (no faults, no bursts): every marked place at its value, the
    /// run seeded with the experiment's first run seed, and the gate counts and the KPIs of the
    /// architecture as a whole beside the simulator's.
    ///
    /// # Errors
    ///
    /// Every refusal of `sim check` with its code, and `simulation_refused` when the simulator
    /// refuses the document under the profile.
    pub fn score(&self, path: &Path) -> Result<Value, Refusal> {
        use crate::marked::{Marked, collect, resolve};
        let document = load_nested(path)?;
        let flat = nest::flatten(&document);
        let refusals = check_architecture(&flat);
        if !refusals.is_empty() {
            let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
            return Err(refused("architecture_refused", names.join("; ")));
        }
        accepted(
            &document,
            &flat,
            &CheckOptions {
                architecture: path,
                ports: None,
                kpis: None,
            },
        )?;
        let not_simulated = |e: String| refused("simulation_refused", e);
        let params: serde_json::Map<String, Value> = collect(&document, false)
            .map_err(not_simulated)?
            .iter()
            .map(|m: &Marked| (m.path.clone(), m.range[m.baseline_index()].clone()))
            .collect();
        let arch = nest::flatten(&resolve(&document, &params).map_err(not_simulated)?);
        let model = crate::sim::Model::build(&arch, &self.profile, self.horizon, &[], &[])
            .map_err(not_simulated)?;
        let seed = crate::rng::Rng::stream(self.seed, 0).next_u64();
        let run = crate::sim::simulate(&model, seed, false);
        // As the search's mean over one seed: 0 + v / 1, so a -0.0 reads 0.0 there and here.
        let mean = |v: &f64| 0.0 + v / 1.0;
        let mut kpis: std::collections::BTreeMap<String, f64> =
            run.kpis.iter().map(|(k, v)| (k.clone(), mean(v))).collect();
        for (g, n) in crate::check::gate_counts(&arch) {
            kpis.insert(g.to_string(), n);
        }
        for (k, n) in crate::whole::measure(&arch) {
            kpis.insert(k.to_string(), n);
        }
        let per_part: serde_json::Map<String, Value> = run
            .per_part
            .iter()
            .map(|(p, row)| {
                let row: serde_json::Map<String, Value> = row
                    .iter()
                    .map(|(k, v)| (k.clone(), json!(mean(v))))
                    .collect();
                (p.clone(), Value::Object(row))
            })
            .collect();
        let kpis: serde_json::Map<String, Value> =
            kpis.into_iter().map(|(k, v)| (k, json!(v))).collect();
        Ok(json!({"kpis": kpis, "per_part": per_part}))
    }

    /// [`Scorer::score`], a refusal as its error answer ([`score_error`]).
    pub fn answer(&self, path: &Path) -> Value {
        self.score(path).unwrap_or_else(|r| score_error(&r))
    }
}

/// What `sim score` answers.
#[derive(Debug, Clone)]
pub enum Scored {
    /// One document without `--batch`: its KPIs, or its refusal.
    One(Result<Value, Refusal>),
    /// `{"results": [...]}`, one answer per document in order.
    Batch(Value),
}

/// `sim score --profile <profile> [--seed <n>] [--horizon <duration>] [--batch] [--]
/// <architecture>...`: one document without `--batch` answers its KPIs or its refusal; with
/// `--batch` or more than one document, `{"results": [...]}`, each entry the answer of its
/// document alone.
///
/// # Errors
///
/// A refusal of the fixed arguments ([`Scorer::new`]), or `documents_missing`.
pub fn score(o: &ScoreOptions, documents: &[&Path], batch: bool) -> Result<Scored, Refusal> {
    if documents.is_empty() {
        return Err(refused(
            "documents_missing",
            "name an architecture document, or several after `--`",
        ));
    }
    let scorer = Scorer::new(o)?;
    Ok(match documents {
        [one] if !batch => Scored::One(scorer.score(one)),
        _ => Scored::Batch(json!({
            "results": documents.iter().map(|d| scorer.answer(d)).collect::<Vec<_>>()
        })),
    })
}

/// `packing default <architecture> [--strategy <strategy>]`: the resolved `packing:` section of
/// a binding under a strategy (`single` when absent), to paste into a binding and change.
pub fn packing_default(path: &Path, strategy: Option<&str>) -> Result<Value, Refusal> {
    use crate::deploy::packing;
    let arch = load_yaml(path).map_err(|e| refused("document_unreadable", e))?;
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let section = match strategy {
        Some(s) => json!({ "strategy": s }),
        None => Value::Null,
    };
    let p =
        packing::resolve(&arch, &section).map_err(|e| refused("packing_refused", e.join("; ")))?;
    Ok(json!({
        "architecture": arch["architecture"].as_str().unwrap_or("architecture"),
        "strategy": p.strategy,
        "units": p.units.len(),
        "packing": p.text(),
    }))
}

/// The options of `deploy bundle`.
#[derive(Debug, Clone)]
pub struct BundleOptions<'a> {
    pub architecture: &'a Path,
    pub binding: &'a Path,
    pub out: Option<&'a Path>,
    /// The ports file; `blocks/ports.json` when absent.
    pub ports: Option<&'a Path>,
    /// The copy of cloudspec's accepted list; `bundles/cloudspec-accepts.yaml` when absent.
    pub accepts: Option<&'a Path>,
    /// The KPI catalogue the default `metrics` ingress is resolved against;
    /// `kpis/catalogue.yaml` when absent.
    pub kpis: Option<&'a Path>,
}

/// `deploy bundle <architecture> --binding <file> [--out <file>] [--ports <file>]
/// [--accepts <file>] [--kpis <file>]`: admit the architecture (its nesting chain resolved and
/// flattened, each nested document carried), its requirements, its contract (its ingresses
/// resolved), the binding and its packing, and only then write the bundle
/// (`agenticspec-deployment/1`), or print it when no `--out` is given. A refusal writes nothing
/// and names every finding.
pub fn deploy_bundle(o: &BundleOptions) -> Result<Value, Refusal> {
    use crate::contract::derive as d;
    use crate::contract::ports::PortsFile;
    use crate::deploy::{admit::Accepts, binding, bundle, grants, packing};
    let unreadable = |e: String| refused("document_unreadable", e);
    let bytes = std::fs::read(o.architecture)
        .map_err(|e| unreadable(format!("{}: {e}", o.architecture.display())))?;
    let written = std::str::from_utf8(&bytes)
        .map_err(|e| e.to_string())
        .and_then(crate::doc::parse_yaml)
        .map_err(|e| unreadable(format!("{}: {e}", o.architecture.display())))?;
    // The nesting chain resolved and pinned; what follows reads the document flattened.
    let resolved = load_nested(o.architecture)?;
    let nested = bundle::nested_documents(o.architecture, &written).map_err(unreadable)?;
    let arch = nest::flatten(&resolved);
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let ports_path = o.ports.unwrap_or(Path::new("blocks/ports.json"));
    let ports = PortsFile::load(ports_path).map_err(|e| refused("ports_unreadable", e))?;
    let kpis =
        crate::contract::ingress::Kpis::load(o.kpis.unwrap_or(Path::new("kpis/catalogue.yaml")))
            .map_err(|e| refused("kpis_unreadable", e))?;
    let contract = derive_contract_with(&arch, &ports, &kpis)?;
    let problems = d::check(&contract, &arch, &ports);
    if !problems.is_empty() {
        return Err(refused("contract_refused", problems.join("; ")));
    }
    let b = binding::load(o.binding).map_err(|e| refused("binding_invalid", e))?;
    // A secret value is refused by admission, by name; the form is checked without it.
    let form = grants::without_values(&b.doc);
    let mut problems = binding::problems(&form, &arch);
    let p = match packing::resolve(&arch, &form["packing"]) {
        Ok(p) => Some(p),
        Err(e) => {
            problems.extend(e);
            None
        }
    };
    if let Some(p) = &p {
        let contract_ingress = &contract.to_value()["ingress"];
        problems.extend(binding::served_problems(&form, contract_ingress, p));
    }
    let Some(p) = p.filter(|_| problems.is_empty()) else {
        return Err(refused(
            "binding_invalid",
            format!("{}: {}", o.binding.display(), problems.join("; ")),
        ));
    };
    let accepts_path = o
        .accepts
        .unwrap_or(Path::new("bundles/cloudspec-accepts.yaml"));
    let accepts = Accepts::load(accepts_path).map_err(|e| refused("accepts_unreadable", e))?;
    let built = bundle::build(&bundle::Inputs {
        architecture: o.architecture,
        arch_bytes: &bytes,
        written: &written,
        arch: &arch,
        nested: &nested,
        kpis: &kpis,
        contract: &contract,
        binding: &b,
        packing: &p,
        accepts: &accepts,
    })
    .map_err(|f| {
        let lines: Vec<String> = f.iter().map(ToString::to_string).collect();
        refused("not_admitted", lines.join("; "))
    })?;
    let mut answer = json!({
        "architecture": arch["architecture"].as_str().unwrap_or("architecture"),
        "binding": b.doc["binding"].as_str().unwrap_or(""),
        "digest": built.digest,
        "units": built.units,
        "written": false,
    });
    match o.out {
        Some(out) => {
            write(out, &built.text)?;
            answer["written"] = json!(true);
            answer["out"] = json!(out.display().to_string());
        }
        None => answer["bundle"] = json!(built.text),
    }
    Ok(answer)
}

/// `ingress default <architecture>`: the `ingress:` section of the default ingresses the
/// document has (those it neither declares nor sets to `none`), to paste into it and change;
/// pasted unchanged, it gives the same contract. Refused when the document takes a reserved
/// name.
pub fn ingress_default(path: &Path) -> Result<Value, Refusal> {
    use crate::contract::ingress;
    let arch = load_yaml(path).map_err(|e| refused("document_unreadable", e))?;
    refuse_reserved(&arch)?;
    Ok(json!({
        "architecture": arch["architecture"].as_str().unwrap_or("architecture"),
        "defaults": ingress::defaults_in_effect(&arch),
        "ingress": ingress::default_text(&arch),
    }))
}

/// The options of `ingress project`.
#[derive(Debug, Clone)]
pub struct ProjectOptions<'a> {
    pub architecture: &'a Path,
    /// The directory the projections are written to.
    pub out: Option<&'a Path>,
    /// A directory of written projections compared with a fresh generation, writing nothing.
    pub check: Option<&'a Path>,
    /// The blocks directory, an ESS system; `blocks` when absent.
    pub blocks: Option<&'a Path>,
    /// The ports file; `blocks/ports.json` when absent.
    pub ports: Option<&'a Path>,
    /// The KPI catalogue; `kpis/catalogue.yaml` when absent.
    pub kpis: Option<&'a Path>,
}

/// The projections of an architecture's exports, by file name ([`crate::contract::projection`]):
/// when an export reads the blocks, ESS compiles them and writes their OpenAPI document into a
/// scratch directory once, and validates each `ess-http/1` binding. A metric list reads the KPI
/// catalogue only; a `ui` export has no projection here.
fn projections(
    contract: &crate::contract::derive::Contract,
    blocks: &Path,
    kpis: &crate::contract::ingress::Kpis,
) -> Result<std::collections::BTreeMap<String, String>, Refusal> {
    use crate::contract::ports::ess;
    use crate::contract::projection::{self as pr, agent, cli, metrics, stream, tool};
    let mut files = std::collections::BTreeMap::new();
    let exports = pr::projected(contract);
    if exports.is_empty() {
        return Ok(files);
    }
    let unreadable = |e: String| refused("blocks_unreadable", e);
    let b = blocks.display().to_string();
    let reads_blocks = exports.iter().any(|(_, p)| *p != "metrics_scrape");
    let work = std::env::temp_dir().join(format!("agenticspec-ingress-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let result = (|| {
        let blocks_model = if reads_blocks {
            let ir_text = ess(&["specify", "compile", "--path", &b, "--format", "json"])
                .map_err(unreadable)?;
            let pin = pr::Pin::of_ir(&ir_text).map_err(unreadable)?;
            let ir: Value = serde_json::from_str(&ir_text)
                .map_err(|e| unreadable(format!("ess specify compile: {e}")))?;
            let oa = work.join("openapi");
            let oa_text = oa.display().to_string();
            ess(&[
                "generate", "--path", &b, "--kind", "openapi", "--out", &oa_text,
            ])
            .map_err(unreadable)?;
            let docs: Vec<std::path::PathBuf> = std::fs::read_dir(oa.join("openapi"))
                .map_err(|e| unreadable(format!("{}: {e}", oa.display())))?
                .filter_map(Result::ok)
                .map(|e| e.path())
                .collect();
            let [doc] = docs.as_slice() else {
                return Err(unreadable(format!(
                    "ess wrote {} OpenAPI documents for {b}; one component reached by network serves the blocks",
                    docs.len()
                )));
            };
            let component = doc
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("blocks")
                .to_owned();
            let full = load_yaml(doc).map_err(unreadable)?;
            Some((pin, ir, full, component))
        } else {
            None
        };
        for (i, protocol) in exports {
            let refuse = |e: String| refused("projection_refused", e);
            let names = pr::files_of(&i.name, protocol);
            let texts: Vec<String> = match (protocol, &blocks_model) {
                ("metrics_scrape", _) => vec![metrics::metrics(contract, i, kpis).map_err(refuse)?],
                (_, None) => continue,
                ("http", Some((pin, _, full, _))) => {
                    let binding = pr::http_binding(contract, i, pin);
                    let at = work.join(&names[0]);
                    std::fs::write(&at, &binding)
                        .map_err(|e| unreadable(format!("{}: {e}", at.display())))?;
                    let at_text = at.display().to_string();
                    ess(&[
                        "specify",
                        "http",
                        "validate",
                        "--path",
                        &b,
                        "--binding",
                        &at_text,
                    ])
                    .map_err(|e| refuse(format!("ingress {}: {e}", i.name)))?;
                    let narrowed = pr::openapi(full, contract, i).map_err(refuse)?;
                    let text = pr::openapi_text(contract, i, &narrowed).map_err(refuse)?;
                    vec![binding, text]
                }
                ("event_stream", Some((pin, ir, _, _))) => {
                    vec![stream::streams(contract, i, pin, ir).map_err(refuse)?]
                }
                ("cli", Some((_, ir, _, component))) => {
                    vec![cli::cli_binding(contract, i, ir, component).map_err(refuse)?]
                }
                ("tool", Some((pin, ir, full, _))) => {
                    vec![tool::tools(contract, i, pin, ir, full).map_err(refuse)?]
                }
                ("agent", Some((pin, ir, full, _))) => {
                    vec![agent::agent_card(contract, i, pin, ir, full).map_err(refuse)?]
                }
                _ => continue,
            };
            for (name, text) in names.into_iter().zip(texts) {
                files.insert(name, text);
            }
        }
        Ok(())
    })();
    let _ = std::fs::remove_dir_all(&work);
    result.map(|()| files)
}

/// `ingress project <architecture> (--out <dir> | --check <dir>) [--blocks <dir>] [--ports
/// <file>] [--kpis <file>]`: the projections of each export of each ingress (for `http` the
/// `ess-http/1` protocol binding and the OpenAPI document, for `event_stream` the stream list, for
/// `cli` the `ess-cli/2` binding, for `tool` the tool list, for `agent` the agent card, for
/// `metrics_scrape` the metric list), the same bytes on every run; written into `--out`, removing
/// projections the document no longer has, or compared with the files in `--check`.
pub fn ingress_project(o: &ProjectOptions) -> Result<Value, Refusal> {
    let (dir, check) = match (o.out, o.check) {
        (Some(d), None) => (d, false),
        (None, Some(d)) => (d, true),
        _ => {
            return Err(refused(
                "projection_refused",
                "name one of --out <dir> and --check <dir>",
            ));
        }
    };
    let arch = load_yaml(o.architecture).map_err(|e| refused("document_unreadable", e))?;
    let refusals = check_architecture(&arch);
    if !refusals.is_empty() {
        let names: Vec<String> = refusals.iter().map(ToString::to_string).collect();
        return Err(refused("architecture_refused", names.join("; ")));
    }
    let ports_path = o.ports.unwrap_or(Path::new("blocks/ports.json"));
    let ports = crate::contract::ports::PortsFile::load(ports_path)
        .map_err(|e| refused("ports_unreadable", e))?;
    let kpis =
        crate::contract::ingress::Kpis::load(o.kpis.unwrap_or(Path::new("kpis/catalogue.yaml")))
            .map_err(|e| refused("kpis_unreadable", e))?;
    let contract = derive_contract_with(&arch, &ports, &kpis)?;
    let exports = crate::contract::projection::projected(&contract).len();
    let files = projections(&contract, o.blocks.unwrap_or(Path::new("blocks")), &kpis)?;
    let on_disk: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(|e| e.file_name().to_str().map(str::to_owned))
        .filter(|n| crate::contract::projection::is_projection(n))
        .collect();
    let answer = |written: bool| {
        json!({
            "architecture": contract.architecture,
            "exports": exports,
            "files": files.keys().collect::<Vec<_>>(),
            "written": written,
        })
    };
    if check {
        let mut stale: Vec<String> = Vec::new();
        for (name, text) in &files {
            match std::fs::read(dir.join(name)) {
                Ok(bytes) if bytes == text.as_bytes() => {}
                Ok(_) => stale.push(format!("changed: {name}")),
                Err(_) => stale.push(format!("missing: {name}")),
            }
        }
        for name in &on_disk {
            if !files.contains_key(name) {
                stale.push(format!("not generated: {name}"));
            }
        }
        if !stale.is_empty() {
            return Err(refused(
                "projections_stale",
                format!(
                    "{} differs from a fresh projection of {} ({}): run agenticspec ingress project {} --out {}",
                    dir.display(),
                    o.architecture.display(),
                    stale.join(", "),
                    o.architecture.display(),
                    dir.display()
                ),
            ));
        }
        return Ok(answer(false));
    }
    for (name, text) in &files {
        write(&dir.join(name), text)?;
    }
    for name in &on_disk {
        if !files.contains_key(name) {
            std::fs::remove_file(dir.join(name))
                .map_err(|e| refused("out_unwritable", format!("{}: {e}", dir.display())))?;
        }
    }
    Ok(answer(true))
}

/// `deploy check <bundle> [--accepts <file>]`: admit a written bundle again from what it carries,
/// for a reader that did not write it. A bundle of the wrong form or whose digests do not hold is
/// `bundle_invalid`; one that admission refuses, an undeclared secret or egress or a secret value
/// among them, is `not_admitted` with every finding.
pub fn deploy_check(path: &Path, accepts: Option<&Path>) -> Result<Value, Refusal> {
    use crate::deploy::admit::Accepts;
    use crate::deploy::bundle::{self, Refused};
    let v = load_yaml(path).map_err(|e| refused("document_unreadable", e))?;
    let accepts_path = accepts.unwrap_or(Path::new("bundles/cloudspec-accepts.yaml"));
    let accepts = Accepts::load(accepts_path).map_err(|e| refused("accepts_unreadable", e))?;
    let lines = |f: &[crate::deploy::admit::Finding]| -> String {
        f.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("; ")
    };
    bundle::check(&v, &accepts).map_err(|r| match r {
        Refused::Invalid(p) => refused(
            "bundle_invalid",
            format!("{}: {}", path.display(), p.join("; ")),
        ),
        Refused::NotAdmitted(f) => refused("not_admitted", lines(&f)),
    })?;
    Ok(json!({
        "architecture": v["architecture"]["name"].as_str().unwrap_or(""),
        "binding": v["binding"]["name"].as_str().unwrap_or(""),
        "digest": v["digest"].as_str().unwrap_or(""),
        "units": v["units"].as_array().map_or(0, Vec::len),
    }))
}
