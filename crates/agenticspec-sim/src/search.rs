//! Experiments (`agenticspec-experiment/1`), the search over marked places and the result
//! (`agenticspec-sim-result/1`).

use crate::check::{GATE_KPIS, gate_counts};
use crate::doc::{load_yaml, num};
use crate::marked::{Marked, collect, resolve};
use crate::profile::Profile;
use crate::requirements::Requirement;
use crate::rng::Rng;
use crate::sim::{Fault, Model, RUN_KPIS, RunOut, parse_bursts, parse_faults, simulate};
use crate::whole::{measure, measure_of};
use rayon::prelude::*;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

/// The most events of the timeline in a result.
pub const TIMELINE_LIMIT: usize = 2000;

/// The kind of a KPI: a check, a share of what was promised, or a number with a direction.
pub fn kind(name: &str) -> &'static str {
    if GATE_KPIS.contains(&name)
        || name == "false_completions_shipped"
        || name == "firing_failure_share"
    {
        "Correctness"
    } else if name == "task_completion"
        || measure_of(name).is_some_and(|m| m.kind == "completeness")
    {
        "Completeness"
    } else {
        "Quality"
    }
}

#[derive(Debug, Clone)]
pub struct Search {
    pub mode: String,
    pub budget: usize,
    pub restarts: usize,
    pub limit: usize,
    pub weights: BTreeMap<String, f64>,
    pub gates: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Experiment {
    /// The experiment document as written.
    pub doc: Value,
    /// The architecture document as written, markers included, each setting of a block's enum as
    /// its word (`crate::settings::canonical`); a nested architecture resolved beside its part
    /// (`crate::nest::load`).
    pub architecture: Value,
    pub profile: Value,
    pub horizon: f64,
    pub seeds: u64,
    pub seed: u64,
    pub faults: Vec<Fault>,
    /// Tasks that arrive at once, `(at, tasks)`.
    pub bursts: Vec<(f64, usize)>,
    pub marked: Vec<Marked>,
    pub search: Search,
    /// The architecture's `requirements:`: constraints of the search, before its weights.
    pub requirements: Vec<Requirement>,
    profile_parsed: Profile,
}

fn known_kpi(name: &str) -> bool {
    RUN_KPIS.contains(&name) || GATE_KPIS.contains(&name) || measure_of(name).is_some()
}

impl Experiment {
    /// Loads an experiment file and the architecture and profile it names, relative to it.
    pub fn load(path: &Path) -> Result<Self, String> {
        let doc = load_yaml(path)?;
        let dir = path.parent().unwrap_or(Path::new("."));
        let file = |key: &str| {
            doc[key]
                .as_str()
                .map(|p| dir.join(p))
                .ok_or_else(|| format!("{}: needs `{key}`", path.display()))
        };
        let architecture = crate::nest::load(&file("architecture")?).map_err(|e| e.to_string())?;
        let profile_path = file("profile")?;
        let profile = load_yaml(&profile_path)?;
        Self::from_docs_in(
            doc,
            architecture,
            profile,
            Some(profile_path.parent().unwrap_or(Path::new("."))),
        )
    }

    pub fn from_docs(doc: Value, architecture: Value, profile: Value) -> Result<Self, String> {
        Self::from_docs_in(doc, architecture, profile, None)
    }

    /// As [`Experiment::from_docs`]; a corpus the profile names is read relative to `profile_dir`.
    pub fn from_docs_in(
        doc: Value,
        architecture: Value,
        profile: Value,
        profile_dir: Option<&Path>,
    ) -> Result<Self, String> {
        let architecture = crate::settings::canonical(&architecture);
        for (d, want) in [
            (&doc, "agenticspec-experiment/1"),
            (&architecture, "agenticspec-architecture/1"),
        ] {
            if let Some(f) = d["format"].as_str()
                && f != want
            {
                return Err(format!("format `{f}`, expected {want}"));
            }
        }
        if let Some(part) = crate::nest::unresolved(&architecture) {
            return Err(format!(
                "part `{part}`: a nested architecture is resolved from its file; load the experiment from its path"
            ));
        }
        let mut profile_parsed = Profile::parse(&profile)?;
        if let Some(dir) = profile_dir {
            profile_parsed.read_corpus(dir)?;
        }
        let horizon = match &doc["horizon"] {
            Value::Null => 86400.0,
            v => num(v)
                .filter(|h| *h > 0.0)
                .ok_or("experiment: horizon must be a positive duration")?,
        };
        let seeds = doc["seeds"].as_u64().unwrap_or(1).max(1);
        let seed = doc["seed"].as_u64().unwrap_or(0);
        let faults = parse_faults(&doc["faults"])?;
        let bursts = parse_bursts(&doc["bursts"])?;
        let all = match doc["optimize"].as_str().unwrap_or("marked") {
            "marked" => false,
            "all" => true,
            other => {
                return Err(format!(
                    "experiment: optimize `{other}`, expected marked or all"
                ));
            }
        };
        let marked = collect(&architecture, all)?;
        let (requirements, refused) =
            crate::requirements::parse(&crate::nest::flatten(&architecture));
        if !refused.is_empty() {
            let lines: Vec<String> = refused.iter().map(ToString::to_string).collect();
            return Err(lines.join("; "));
        }
        let s = &doc["search"];
        let usize_or = |k: &str, d: usize| s[k].as_u64().map_or(d, |v| v as usize);
        let mut weights = BTreeMap::new();
        for (k, v) in s["weights"].as_object().into_iter().flatten() {
            if !known_kpi(k) {
                return Err(format!("search.weights: unknown KPI `{k}`"));
            }
            weights.insert(
                k.clone(),
                v.as_f64()
                    .ok_or(format!("search.weights.{k}: not a number"))?,
            );
        }
        let gates: Vec<String> = match s["gates"].as_array() {
            Some(list) => list
                .iter()
                .map(|g| g.as_str().unwrap_or("").to_string())
                .collect(),
            None => GATE_KPIS.iter().map(|g| g.to_string()).collect(),
        };
        for g in &gates {
            if !known_kpi(g) {
                return Err(format!("search.gates: unknown KPI `{g}`"));
            }
        }
        let search = Search {
            mode: s["mode"].as_str().unwrap_or("fit").to_string(),
            budget: usize_or("budget", 24).max(1),
            restarts: usize_or("restarts", 4),
            limit: usize_or("limit", 256),
            weights,
            gates,
        };
        if search.mode != "fit" && search.mode != "exhaustive" {
            return Err(format!(
                "search.mode `{}`: expected fit or exhaustive",
                search.mode
            ));
        }
        let e = Experiment {
            doc,
            architecture,
            profile,
            horizon,
            seeds,
            seed,
            faults,
            bursts,
            marked,
            search,
            requirements,
            profile_parsed,
        };
        e.model(&e.params(&e.baseline_point()))?;
        Ok(e)
    }

    fn baseline_point(&self) -> Vec<usize> {
        self.marked.iter().map(Marked::baseline_index).collect()
    }

    fn params(&self, point: &[usize]) -> Map<String, Value> {
        self.marked
            .iter()
            .zip(point)
            .map(|(m, i)| (m.path.clone(), m.range[*i].clone()))
            .collect()
    }

    /// The variant at `params`, flattened (its nested architectures as parts of its own), and its
    /// model.
    fn model(&self, params: &Map<String, Value>) -> Result<(Value, Model), String> {
        let arch = crate::nest::flatten(&resolve(&self.architecture, params)?);
        let model = Model::build(
            &arch,
            &self.profile_parsed,
            self.horizon,
            &self.faults,
            &self.bursts,
        )?;
        Ok((arch, model))
    }

    fn run_seed(&self, s: u64) -> u64 {
        Rng::stream(self.seed, s).next_u64()
    }

    /// The baseline at the experiment's first seed, recorded in full, without a search.
    pub fn baseline_shown(&self) -> Result<Shown, String> {
        let (architecture, model) = self.model(&self.params(&self.baseline_point()))?;
        let seed = self.run_seed(0);
        Ok(Shown {
            architecture,
            seed,
            run: simulate(&model, seed, true),
        })
    }
}

/// One evaluated variant, averaged over the seeds.
#[derive(Debug, Clone)]
struct Eval {
    point: Vec<usize>,
    params: Map<String, Value>,
    kpis: BTreeMap<String, f64>,
    per_part: BTreeMap<String, BTreeMap<String, f64>>,
    fitness: f64,
    feasible: bool,
    /// Each requirement with its state in this variant, as written.
    requirements: Vec<Value>,
    /// Requirements decided in this variant, and of those the ones missed.
    decided: usize,
    missed: usize,
}

impl Eval {
    /// How the search ranks a variant: a feasible one above every infeasible one, then fewer
    /// requirements missed, then a higher fitness. Requirements are constraints before weights;
    /// without requirements this is the fitness of a feasible variant.
    fn score(&self) -> (f64, f64) {
        if self.feasible {
            self.rank()
        } else {
            (f64::NEG_INFINITY, f64::NEG_INFINITY)
        }
    }

    /// Fewer requirements missed first, then the fitness.
    fn rank(&self) -> (f64, f64) {
        (-(self.missed as f64), self.fitness)
    }

    /// Feasible, and every requirement decided and met, at least one of them decided.
    fn meets_all(&self) -> bool {
        self.feasible && self.decided > 0 && self.missed == 0
    }
}

type PerPart = BTreeMap<String, BTreeMap<String, f64>>;

fn mean(runs: &[RunOut]) -> (BTreeMap<String, f64>, PerPart) {
    let n = runs.len().max(1) as f64;
    let mut kpis: BTreeMap<String, f64> = BTreeMap::new();
    let mut per_part: PerPart = BTreeMap::new();
    for r in runs {
        for (k, v) in &r.kpis {
            *kpis.entry(k.clone()).or_default() += v / n;
        }
        for (p, m) in &r.per_part {
            let row = per_part.entry(p.clone()).or_default();
            for (k, v) in m {
                *row.entry(k.clone()).or_default() += v / n;
            }
        }
    }
    (kpis, per_part)
}

/// Evaluates `points` in order; the variants and seeds run concurrently, and the result does not
/// depend on how many threads run them.
fn evaluate(e: &Experiment, points: &[Vec<usize>]) -> Result<Vec<Eval>, String> {
    let built: Vec<(Map<String, Value>, Value, Model)> = points
        .iter()
        .map(|p| {
            let params = e.params(p);
            e.model(&params).map(|(a, m)| (params, a, m))
        })
        .collect::<Result<_, _>>()?;
    let jobs: Vec<(usize, u64)> = (0..points.len())
        .flat_map(|v| (0..e.seeds).map(move |s| (v, s)))
        .collect();
    let runs: Vec<RunOut> = jobs
        .par_iter()
        .map(|(v, s)| simulate(&built[*v].2, e.run_seed(*s), false))
        .collect();
    let mut out = Vec::with_capacity(points.len());
    for (v, (params, arch, _)) in built.into_iter().enumerate() {
        let chunk = &runs[v * e.seeds as usize..(v + 1) * e.seeds as usize];
        let (mut kpis, per_part) = mean(chunk);
        for (g, n) in gate_counts(&arch) {
            kpis.insert(g.to_string(), n);
        }
        for (k, n) in measure(&arch) {
            kpis.insert(k.to_string(), n);
        }
        let feasible = e
            .search
            .gates
            .iter()
            .all(|g| kpis.get(g).copied().unwrap_or(0.0) == 0.0);
        let fitness = e
            .search
            .weights
            .iter()
            .map(|(k, w)| w * kpis.get(k).copied().unwrap_or(0.0))
            .sum();
        let (requirements, decided, missed) =
            crate::requirements::at_evaluation(&e.requirements, &kpis, &per_part);
        out.push(Eval {
            point: points[v].clone(),
            params,
            kpis,
            per_part,
            fitness,
            feasible,
            requirements,
            decided,
            missed,
        });
    }
    Ok(out)
}

/// Every evaluation so far, in order, and where each point was evaluated.
struct Book<'a> {
    e: &'a Experiment,
    evals: Vec<Eval>,
    seen: HashMap<Vec<usize>, usize>,
}

impl Book<'_> {
    fn add(&mut self, points: Vec<Vec<usize>>) -> Result<Vec<usize>, String> {
        let mut fresh: Vec<Vec<usize>> = Vec::new();
        for p in &points {
            if !self.seen.contains_key(p) && !fresh.contains(p) {
                fresh.push(p.clone());
            }
        }
        for ev in evaluate(self.e, &fresh)? {
            self.seen.insert(ev.point.clone(), self.evals.len());
            self.evals.push(ev);
        }
        Ok(points.iter().map(|p| self.seen[p]).collect())
    }
}

fn neighbours(marked: &[Marked], at: &[usize]) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for (i, m) in marked.iter().enumerate() {
        for step in [-1i64, 1] {
            let v = at[i] as i64 + step;
            if v >= 0 && (v as usize) < m.range.len() {
                let mut n = at.to_vec();
                n[i] = v as usize;
                out.push(n);
            }
        }
    }
    out
}

/// Hill climbing: from the baseline, evaluate every neighbour not yet seen (one marked place one
/// step), move to the best that improves, and restart from a random unseen point at a local
/// optimum, within the budget of evaluations. A variant improves on another when it is feasible
/// and the other is not, misses fewer requirements, or has a higher fitness, in that order; the
/// climb stops after the round in which a variant first meets every requirement.
fn fit(book: &mut Book<'_>) -> Result<(), String> {
    let e = book.e;
    let budget = e.search.budget;
    let space: f64 = e.marked.iter().map(|m| m.range.len() as f64).product();
    let mut rng = Rng::stream(e.seed, u64::MAX);
    let mut current = e.baseline_point();
    let first = book.add(vec![current.clone()])?[0];
    let mut score = book.evals[first].score();
    let mut restarts = e.search.restarts;
    while book.evals.len() < budget {
        // With requirements, the search stops at the first round in which a variant meets
        // every one of them.
        if book.evals.iter().any(Eval::meets_all) {
            break;
        }
        let mut fresh: Vec<Vec<usize>> = neighbours(&e.marked, &current)
            .into_iter()
            .filter(|n| !book.seen.contains_key(n))
            .collect();
        if fresh.is_empty() {
            if restarts == 0 || book.seen.len() as f64 >= space {
                break;
            }
            restarts -= 1;
            let mut start = None;
            for _ in 0..1000 {
                let p: Vec<usize> = e.marked.iter().map(|m| rng.below(m.range.len())).collect();
                if !book.seen.contains_key(&p) {
                    start = Some(p);
                    break;
                }
            }
            let Some(p) = start else { break };
            let id = book.add(vec![p.clone()])?[0];
            current = p;
            score = book.evals[id].score();
            continue;
        }
        fresh.truncate(budget - book.evals.len());
        let ids = book.add(fresh.clone())?;
        let mut improved: Option<((f64, f64), usize)> = None;
        for (n, id) in ids.iter().enumerate() {
            let s = book.evals[*id].score();
            if s > score && improved.is_none_or(|(b, _)| s > b) {
                improved = Some((s, n));
            }
        }
        if let Some((s, n)) = improved {
            current = fresh[n].clone();
            score = s;
        }
    }
    Ok(())
}

fn exhaustive(book: &mut Book<'_>) -> Result<(), String> {
    let e = book.e;
    let size = e
        .marked
        .iter()
        .try_fold(1usize, |acc, m| acc.checked_mul(m.range.len()))
        .unwrap_or(usize::MAX);
    if size > e.search.limit {
        return Err(format!(
            "exhaustive search refused: {size} combinations exceed the limit {}",
            e.search.limit
        ));
    }
    let points: Vec<Vec<usize>> = (0..size)
        .map(|mut index| {
            e.marked
                .iter()
                .map(|m| {
                    let i = index % m.range.len();
                    index /= m.range.len();
                    i
                })
                .collect()
        })
        .collect();
    book.add(points)?;
    Ok(())
}

fn kpi_json(kpis: &BTreeMap<String, f64>) -> Value {
    Value::Object(
        kpis.iter()
            .map(|(k, v)| (k.clone(), json!({"value": v, "kind": kind(k)})))
            .collect(),
    )
}

fn per_part_json(per_part: &PerPart) -> Value {
    Value::Object(
        per_part
            .iter()
            .map(|(p, m)| (p.clone(), kpi_json(m)))
            .collect(),
    )
}

fn summary(ev: &Eval) -> Value {
    let mut v = json!({
        "params": ev.params,
        "kpis": kpi_json(&ev.kpis),
        "per_part": per_part_json(&ev.per_part),
        "fitness": ev.fitness,
        "feasible": ev.feasible,
    });
    if !ev.requirements.is_empty() {
        v["requirements"] = json!(ev.requirements);
    }
    v
}

/// The run a result's timeline samples: the best variant, or the baseline when none is
/// feasible, at the experiment's first seed, recorded in full.
#[derive(Debug, Clone)]
pub struct Shown {
    /// The variant's resolved architecture.
    pub architecture: Value,
    /// The seed the run was simulated with.
    pub seed: u64,
    pub run: RunOut,
}

/// Runs the experiment's search on `threads` threads and returns the result document.
pub fn run(e: &Experiment, threads: usize) -> Result<Value, String> {
    run_shown(e, threads).map(|(result, _)| result)
}

/// [`run`], and the run its timeline samples.
pub fn run_shown(e: &Experiment, threads: usize) -> Result<(Value, Shown), String> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .map_err(|err| err.to_string())?;
    pool.install(|| run_inside(e))
}

fn run_inside(e: &Experiment) -> Result<(Value, Shown), String> {
    let mut book = Book {
        e,
        evals: Vec::new(),
        seen: HashMap::new(),
    };
    match e.search.mode.as_str() {
        "exhaustive" => exhaustive(&mut book)?,
        _ => fit(&mut book)?,
    }
    let baseline_id = book.add(vec![e.baseline_point()])?[0];
    let mut best: Option<usize> = None;
    let mut trace = Vec::new();
    for (i, ev) in book.evals.iter().enumerate() {
        if ev.feasible && best.is_none_or(|b| ev.rank() > book.evals[b].rank()) {
            best = Some(i);
        }
        trace.push(json!({
            "eval": i,
            "variant": i,
            "fitness": ev.fitness,
            "best_so_far": best.map(|b| book.evals[b].fitness),
        }));
    }
    let variants: Vec<Value> = book
        .evals
        .iter()
        .enumerate()
        .map(|(i, ev)| {
            let mut v = json!({"id": i, "params": ev.params, "kpis": kpi_json(&ev.kpis), "fitness": ev.fitness, "feasible": ev.feasible});
            if !e.requirements.is_empty() {
                v["requirements_missed"] = json!(ev.missed);
            }
            v
        })
        .collect();
    let shown = &book.evals[best.unwrap_or(baseline_id)];
    let (shown_arch, model) = e.model(&shown.params)?;
    let seed = e.run_seed(0);
    let run = simulate(&model, seed, true);
    let timeline: Vec<Value> = sample(&run.timeline, TIMELINE_LIMIT)
        .into_iter()
        .map(|s| json!({"t_seconds": s.t, "part": s.part, "kind": s.kind}))
        .collect();
    let marked: Vec<Value> = e
        .marked
        .iter()
        .map(|m| json!({"path": m.path, "value": m.value, "range": m.range}))
        .collect();
    let mut result = json!({
        "format": "agenticspec-sim-result/1",
        "experiment": e.doc,
        "architecture": e.architecture,
        "profile": e.profile,
        "marked": marked,
        "baseline": summary(&book.evals[baseline_id]),
        "best": best.map(|b| summary(&book.evals[b])),
        "variants": variants,
        "trace": trace,
        "timeline": timeline,
    });
    if !e.requirements.is_empty() {
        // The requirements of the variant shown: the best, else the baseline.
        result["requirements"] = json!(shown.requirements);
    }
    let shown = Shown {
        architecture: shown_arch,
        seed,
        run,
    };
    Ok((result, shown))
}

/// At most `limit` items, evenly spaced, in order.
fn sample<T: Clone>(items: &[T], limit: usize) -> Vec<T> {
    if items.len() <= limit {
        return items.to_vec();
    }
    (0..limit)
        .map(|i| items[i * items.len() / limit].clone())
        .collect()
}
