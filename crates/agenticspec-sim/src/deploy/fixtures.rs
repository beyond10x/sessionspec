//! The fixture bundles of `bundles/`: one bundle per catalog architecture and test binding,
//! `bundles/<architecture>.<binding>.yaml`, and `bundles/sources.sha256`, one digest of the
//! sources of each bundle. A binding of `bundles/bindings/` that names an `architecture:` is
//! that architecture's alone; one that names none is bundled with every catalog architecture
//! (the architectures the experiments of `experiments/` name) that no binding of the same name
//! names. So `local.yaml` is the base of the per-architecture `local-<architecture>.yaml`, which
//! declare the egress of their parts.
//!
//! The digest of a bundle's sources covers the generator line, the architecture file and every
//! document it nests, the binding file and every base it refines, each agent document it names,
//! the ports file, the accepted list, the generator's own sources (this directory and the contract
//! derivation), for a binding that asks for a built-in UI (`ui:`), what its archive is built from,
//! and the archive of each UI of the consumer's own.
//! [`check`] compares those digests without building a bundle; [`stale`] builds every bundle
//! afresh and compares bytes.

use super::binding::{self, FORMAT};
use crate::cli::{BundleOptions, deploy_bundle};
use crate::doc::load_yaml;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The digest file of the fixture bundles.
pub const SOURCES: &str = "bundles/sources.sha256";

/// The command that writes the fixture bundles.
pub const REGENERATE: &str = "task bundles:generate";

/// The generator's own sources, relative to the repository root.
pub const GENERATOR: &[&str] = &[
    "crates/agenticspec-sim/src/contract/derive.rs",
    "crates/agenticspec-sim/src/contract/ingress.rs",
    "crates/agenticspec-sim/src/deploy/admit.rs",
    "crates/agenticspec-sim/src/deploy/binding.rs",
    "crates/agenticspec-sim/src/deploy/bundle.rs",
    "crates/agenticspec-sim/src/deploy/config.rs",
    "crates/agenticspec-sim/src/deploy/grants.rs",
    "crates/agenticspec-sim/src/deploy/mod.rs",
    "crates/agenticspec-sim/src/deploy/packing.rs",
    "crates/agenticspec-sim/src/deploy/scratch.rs",
    "crates/agenticspec-sim/src/deploy/ui.rs",
    "crates/agenticspec-sim/src/nest.rs",
    "crates/agenticspec-sim/src/requirements.rs",
];

/// What the archive of a built-in UI is built from, a source of each bundle whose binding asks for
/// one (`ui:`): the embedded operator app, panels and model, and the UI's derivation.
const UI_SOURCES: &[&str] = &[
    "crates/agenticspec-ui/src/derive/control.rs",
    "crates/agenticspec-ui/src/derive/mod.rs",
    "crates/agenticspec-ui/src/derive/model.rs",
    "crates/agenticspec-ui/src/derive/view_key.rs",
    "crates/agenticspec-ui/src/derive/write.rs",
    "crates/agenticspec-ui/src/operator.rs",
    "ui/operator/built/app.css",
    "ui/operator/built/app.js",
    "ui/operator/built/index.html",
    "ui/views/blocks.model.json",
    "ui/views/blocks.ui.yaml",
];

/// One fixture bundle: its file name and what it is built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    /// `<architecture>.<binding>.yaml`, in `bundles/`.
    pub file: String,
    pub architecture: PathBuf,
    pub binding: PathBuf,
}

fn yaml_files(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "yaml"))
        .collect();
    out.sort();
    out
}

fn stem(p: &Path) -> String {
    p.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// The architectures of the catalog, by file, in name order.
pub fn catalog_architectures(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = BTreeSet::new();
    for e in yaml_files(&root.join("experiments")) {
        let doc = load_yaml(&e)?;
        let a = doc["architecture"]
            .as_str()
            .ok_or(format!("{}: needs `architecture`", e.display()))?;
        let path = e.parent().unwrap_or(root).join(a);
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        out.insert(root.join("architectures").join(name));
    }
    Ok(out.into_iter().collect())
}

/// Every fixture bundle, in file order. A binding that names an architecture is bundled with it;
/// one that names none, with every catalog architecture that no binding of the same name names.
/// Two bindings of one name for one architecture are refused.
pub fn fixtures(root: &Path) -> Result<Vec<Fixture>, String> {
    let archs = catalog_architectures(root)?;
    let mut named = Vec::new();
    let mut generic = Vec::new();
    for b in yaml_files(&root.join("bundles/bindings")) {
        let doc = binding::load(&b)?.doc;
        let name = doc["binding"]
            .as_str()
            .ok_or(format!("{}: needs `binding:`", b.display()))?
            .to_owned();
        match doc["architecture"].as_str() {
            Some(a) => {
                let p = root.join("architectures").join(format!("{a}.yaml"));
                if !p.is_file() {
                    return Err(format!("{}: no architecture {}", b.display(), p.display()));
                }
                named.push((name, p, b));
            }
            None => generic.push((name, b)),
        }
    }
    let mut out: Vec<Fixture> = Vec::new();
    for (name, a, b) in &named {
        out.push(Fixture {
            file: format!("{}.{name}.yaml", stem(a)),
            architecture: a.clone(),
            binding: b.clone(),
        });
    }
    for (name, b) in &generic {
        for a in &archs {
            if named.iter().any(|(n, x, _)| n == name && x == a) {
                continue;
            }
            out.push(Fixture {
                file: format!("{}.{name}.yaml", stem(a)),
                architecture: a.clone(),
                binding: b.clone(),
            });
        }
    }
    out.sort_by(|a, b| a.file.cmp(&b.file));
    if let Some(w) = out.windows(2).find(|w| w[0].file == w[1].file) {
        return Err(format!(
            "bundles/{}: two bindings ({} and {}) of one name for one architecture",
            w[0].file,
            w[0].binding.display(),
            w[1].binding.display()
        ));
    }
    Ok(out)
}

/// The files a fixture bundle is built from, relative to the root, without the generator's own.
pub fn sources(root: &Path, f: &Fixture) -> Result<Vec<PathBuf>, String> {
    let mut out = vec![f.architecture.clone()];
    let mut nested: Vec<PathBuf> =
        super::bundle::nested_documents(&f.architecture, &load_yaml(&f.architecture)?)?
            .into_values()
            .map(|c| c.file)
            .collect();
    nested.sort_by_key(|p| relative(root, p));
    out.extend(nested);
    let mut at = f.binding.clone();
    for _ in 0..32 {
        out.push(at.clone());
        let doc = load_yaml(&at)?;
        if doc["format"] != FORMAT {
            return Err(format!("{}: not {FORMAT}", at.display()));
        }
        match doc["refines"].as_str() {
            Some(base) => at = at.parent().unwrap_or(root).join(base),
            None => break,
        }
    }
    let b = binding::load(&f.binding)?;
    let builds_ui = super::ui::asked(&b.doc)
        .iter()
        .any(|u| matches!(u.source, super::ui::Source::Built(_)));
    out.extend(b.agent_files.into_values());
    out.extend(b.ui_files.into_values());
    out.push(root.join("blocks/ports.json"));
    out.push(root.join("bundles/cloudspec-accepts.yaml"));
    out.push(root.join("kpis/catalogue.yaml"));
    if builds_ui {
        out.extend(UI_SOURCES.iter().map(|s| root.join(s)));
    }
    Ok(out)
}

/// `<path>` relative to the root, with `/`, without `.` and `..` steps.
fn relative(root: &Path, path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    let rel = path.strip_prefix(root).unwrap_or(path);
    for c in rel.components() {
        match c {
            std::path::Component::ParentDir => {
                parts.pop();
            }
            std::path::Component::Normal(s) => parts.push(s.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    parts.join("/")
}

/// The digest of a fixture's sources: SHA-256 over the generator line, then `<sha256> <path>`
/// for each source and each of the generator's own files.
pub fn digest(root: &Path, f: &Fixture) -> Result<String, String> {
    let mut text = format!("generator agenticspec {}\n", env!("CARGO_PKG_VERSION"));
    let mut files = sources(root, f)?;
    files.extend(GENERATOR.iter().map(|g| root.join(g)));
    for p in files {
        let bytes = std::fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        text.push_str(&format!("{} {}\n", hex(&bytes), relative(root, &p)));
    }
    Ok(hex(text.as_bytes()))
}

/// The text of `bundles/sources.sha256`: `<digest>  <bundle file>` per fixture.
pub fn sources_text(root: &Path) -> Result<String, String> {
    let mut s = String::new();
    for f in fixtures(root)? {
        s.push_str(&format!("{}  {}\n", digest(root, &f)?, f.file));
    }
    Ok(s)
}

/// Build one fixture bundle: its bytes, or the refusal of `deploy bundle`.
pub fn build(root: &Path, f: &Fixture) -> Result<String, String> {
    let ports = root.join("blocks/ports.json");
    let accepts = root.join("bundles/cloudspec-accepts.yaml");
    let answer = deploy_bundle(&BundleOptions {
        architecture: &f.architecture,
        binding: &f.binding,
        out: None,
        ports: Some(&ports),
        accepts: Some(&accepts),
        kpis: Some(&root.join("kpis/catalogue.yaml")),
    })
    .map_err(|r| format!("{}: {}: {}", f.file, r.code, r.detail))?;
    Ok(answer["bundle"].as_str().unwrap_or("").to_owned())
}

/// Write every fixture bundle and the digest file; remove a bundle no fixture names. Returns the
/// files written.
pub fn write(root: &Path) -> Result<Vec<String>, String> {
    let dir = root.join("bundles");
    let all = fixtures(root)?;
    let mut written = Vec::new();
    for f in &all {
        let text = build(root, f)?;
        let path = dir.join(&f.file);
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        written.push(format!("bundles/{}", f.file));
    }
    for p in yaml_files(&dir) {
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if stem(&p).contains('.') && !all.iter().any(|f| f.file == name) {
            std::fs::remove_file(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        }
    }
    let path = root.join(SOURCES);
    std::fs::write(&path, sources_text(root)?).map_err(|e| format!("{}: {e}", path.display()))?;
    written.push(SOURCES.to_owned());
    Ok(written)
}

/// The digest check: one line per fixture whose sources changed since its bundle was written, a
/// bundle file that is missing, or a line of the digest file no fixture has. Builds nothing.
pub fn check(root: &Path) -> Result<Vec<String>, String> {
    let path = root.join(SOURCES);
    let have = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let want = sources_text(root)?;
    let mut out = Vec::new();
    let have_lines: BTreeSet<&str> = have.lines().collect();
    let want_lines: BTreeSet<&str> = want.lines().collect();
    for w in &want_lines {
        let file = w.split("  ").nth(1).unwrap_or("");
        if !root.join("bundles").join(file).is_file() {
            out.push(format!("bundles/{file}: missing"));
        } else if !have_lines.contains(w) {
            out.push(format!("bundles/{file}: its sources changed"));
        }
    }
    for h in have_lines.difference(&want_lines) {
        let file = h.split("  ").nth(1).unwrap_or("");
        if !want_lines.iter().any(|w| w.ends_with(&format!("  {file}"))) {
            out.push(format!("{SOURCES}: `{file}` is no fixture"));
        }
    }
    out.sort();
    Ok(out)
}

/// The byte check: one line per fixture bundle that differs from a fresh build.
pub fn stale(root: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for f in fixtures(root)? {
        let fresh = build(root, &f)?;
        let path = root.join("bundles").join(&f.file);
        if std::fs::read_to_string(&path).ok().as_deref() != Some(fresh.as_str()) {
            out.push(format!("bundles/{}: differs from a fresh build", f.file));
        }
    }
    Ok(out)
}
