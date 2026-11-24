//! `unknown_kind`: the run's own list of kinds, when the rule's check is not among them.

use procio::Output;
use serde::Deserialize;

/// The part of the template's JSON this reads: the kinds the run defined (`allKinds`).
#[derive(Deserialize)]
struct Report {
    kinds: Vec<Kind>,
}

/// One kind; its `Description` is not read.
#[derive(Deserialize)]
struct Kind {
    #[serde(rename = "Name")]
    name: String,
}

/// The kinds the run defined, named, when stdout of `out` is the template's JSON and `check` is not among its `kinds`;
/// None when it is, or when stdout is not that JSON (the reader then says so). actionlint defines `shellcheck` and
/// `pyflakes` only when their program was given and found, so this is how a run says it did not check a kind.
pub(crate) fn unknown_kind(out: &Output, check: &str) -> Option<String> {
    let report: Report = serde_json::from_slice(&out.stdout).ok()?;
    if report.kinds.iter().any(|kind| kind.name == check) {
        return None;
    }
    let names: Vec<&str> = report.kinds.iter().map(|kind| kind.name.as_str()).collect();
    Some(format!("the kinds this run defined are {}", names.join(", ")))
}
