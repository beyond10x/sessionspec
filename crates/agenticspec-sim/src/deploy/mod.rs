//! The deploy side of an architecture: the binding (`agenticspec-binding/1`) with its `packing:`
//! section (`binding`, `packing`), the admission of a packing into a plan of units or a list of
//! named refusals (`admit`), the secrets and egress a unit is granted, by name (`grants`), and
//! the deployment bundle (`agenticspec-deployment/1`) a translator reads (`bundle`), checked
//! again when read back (`bundle::check`); the environment and arguments a binding gives a unit
//! (`config`); the scratch directories a binding gives a unit (`scratch`); the UIs a binding asks
//! for as units of their own (`ui`);
//! the fixture bundles of `bundles/` (`fixtures`). The
//! formats are on `docs/deployment.md`.
//!
//! Nothing here changes a simulation result: the catalog digest leaves this directory out.

pub mod admit;
pub mod binding;
pub mod bundle;
pub mod config;
pub mod fixtures;
pub mod grants;
pub mod packing;
pub mod scratch;
pub mod ui;

use serde_json::Value;
use sha2::{Digest, Sha256};

/// `sha256:<hex>` of some bytes.
pub fn sha256(bytes: &[u8]) -> String {
    let hex: String = Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!("sha256:{hex}")
}

/// The canonical JSON of a value: keys sorted, no space. `serde_json` keeps the keys of an
/// object sorted in this workspace (no `preserve_order`), so its compact form is canonical.
pub fn canonical(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_default()
}

/// `sha256:<hex>` of the canonical JSON of a value.
pub fn digest(v: &Value) -> String {
    sha256(canonical(v).as_bytes())
}
