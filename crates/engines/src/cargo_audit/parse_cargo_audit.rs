//! `parse_cargo_audit`: cargo-audit's JSON report, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The parts of `cargo-audit audit --json`'s one report object cctao reads; `database` and `lockfile` are not read.
#[derive(Deserialize)]
struct Report {
    settings: Map<String, Value>,
    vulnerabilities: Vulnerabilities,
    /// Keyed by warning kind (`unmaintained`, `unsound`, `notice`, `yanked`).
    warnings: BTreeMap<String, Vec<Found>>,
}

#[derive(Deserialize)]
struct Vulnerabilities {
    list: Vec<Found>,
}

/// One vulnerability or warning. A yanked warning has a null advisory and null versions.
#[derive(Deserialize)]
struct Found {
    advisory: Option<Advisory>,
    versions: Option<Versions>,
    package: Package,
}

#[derive(Deserialize)]
struct Advisory {
    id: String,
    title: String,
}

#[derive(Deserialize)]
struct Versions {
    patched: Vec<String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

/// The hits of rule `check` in cargo-audit's report `out`, any exit code (it exits 1 when it finds a vulnerability):
/// each vulnerability (kind `vulnerability`), then each warning with an advisory (kind = its key in `warnings`), kept
/// when `check` is `all`, its kind or its advisory id. Every hit is on `Cargo.lock` with no line, its `what` reading
/// `<id>: <crate> <version>: <title>; upgrade to <patched, joined with " or ">` or `...; no patched version`. stdout
/// that is not the report is a `ParseError` quoting `printed_start`. A report whose `settings` differ from
/// cargo-audit's defaults is a `ParseError` naming the differing fields: an audit.toml cctao was not given applied, and
/// it can silence findings. `cwd` and `folder` are not read: the one file cargo-audit checks is the folder's
/// Cargo.lock.
pub fn parse_cargo_audit(out: &Output, check: &str, _cwd: &Path, _folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let report: Report = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("cargo-audit printed no JSON report: {}", printed_start(out)) })?;
    let changed = changed_settings(&report.settings);
    if !changed.is_empty() {
        return Err(ParseError {
            why: format!(
                "cargo-audit applied an audit.toml cctao was not given ({changed}); the rule file is the only home for an exception, so cctao reads no findings under it"
            ),
        });
    }
    let vulnerabilities = report.vulnerabilities.list.into_iter().map(|found| ("vulnerability".to_string(), found));
    let warnings = report.warnings.into_iter().flat_map(|(kind, list)| list.into_iter().map(move |found| (kind.clone(), found)));
    Ok(vulnerabilities.chain(warnings).filter_map(|(kind, found)| hit(&kind, found, check)).collect())
}

/// The settings that differ from cargo-audit 0.23.0's defaults, as `name = <json value>` joined with ", " (a field
/// one side lacks reads as null); empty when none differs.
fn changed_settings(settings: &Map<String, Value>) -> String {
    let defaults = json!({
        "target_arch": [], "target_os": [], "severity": null, "ignore": [],
        "informational_warnings": ["unmaintained", "unsound", "notice"]
    });
    let names: BTreeSet<&String> = settings.keys().chain(defaults.as_object().into_iter().flat_map(Map::keys)).collect();
    names
        .into_iter()
        .filter_map(|name| {
            let value = settings.get(name).unwrap_or(&Value::Null);
            (value != defaults.get(name).unwrap_or(&Value::Null)).then(|| format!("{name} = {value}"))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// `found` as a hit when it has an advisory and `check` is `all`, `kind` or the advisory's id.
fn hit(kind: &str, found: Found, check: &str) -> Option<Hit> {
    let advisory = found.advisory?;
    if check != "all" && check != kind && check != advisory.id {
        return None;
    }
    let patched = found.versions.map(|versions| versions.patched).unwrap_or_default();
    let fix = if patched.is_empty() { "no patched version".to_string() } else { format!("upgrade to {}", patched.join(" or ")) };
    Some(Hit {
        path: RelPath::new("Cargo.lock"),
        line: None,
        col: None,
        what: format!("{}: {} {}: {}; {fix}", advisory.id, found.package.name, found.package.version, advisory.title),
    })
}
