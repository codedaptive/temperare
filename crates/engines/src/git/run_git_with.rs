//! `run_git_with`: the git entry's flow for one rule, with the change read by a given reader.

use super::{count_change, ratio_hit, read_test_ratio};
use crate::entry::{param_why, Entry};
use crate::host::Host;
use crate::only_with_base_why::ONLY_WITH_BASE_WHY;
use crate::types::{Hit, Stop};
use crate::ParamError;
use rules::{RelPath, Rule};
use serde_json::{Map, Value};
use std::ops::Range;
use std::path::Path;

/// The one hit, or none, of `rule` under `folder` (canonical), `change` giving the files the change adds lines to
/// (relative to `folder`, sorted) for a folder and a base. The run loop settles refusals (`refused`) before this flow.
/// Each step can stop the rule, in this order:
/// 1. the params, the rule's table turned into one JSON object, are read with `read_test_ratio`: a param it cannot
///    read, or a value with no JSON form, is `Failed` with no exit code and `param_why`'s text;
/// 2. no `host.base` is `NotAvailable` with `ONLY_WITH_BASE_WHY`: the rule judges only what a change adds, and
///    nothing is read;
/// 3. `change(folder, base, keep)`, or its `Stop`, with `keep` the rule's scope (`rule.applies`), so the reader drops
///    a file outside the rule's paths before reading it; its files are counted by `count_change` and judged by
///    `ratio_hit`.
///
/// Nothing runs through the host and `check_run` is never called, so `host.raw_chars` stays `None`.
pub(crate) fn run_git_with(
    change: &dyn Fn(&Path, &str, &dyn Fn(&RelPath) -> bool) -> Result<Vec<(RelPath, Vec<Range<u32>>)>, Stop>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let param_failed = |err| Stop::Failed { code: None, stderr: param_why(entry.name, err) };
    let params = rule
        .params
        .iter()
        .map(|(name, value)| Ok((name.clone(), serde_json::to_value(value)?)))
        .collect::<Result<Map<String, Value>, serde_json::Error>>()
        .map_err(|_| {
            param_failed(ParamError::Bad { name: "params".to_string(), why: "not a table of named params".to_string() })
        })?;
    let ratio = read_test_ratio(&params).map_err(param_failed)?;
    let Some(base) = host.base else {
        return Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() });
    };
    let counts = count_change(rule, &ratio, &change(folder, base, &|path| rule.applies(path))?);
    Ok(ratio_hit(&counts, &ratio).into_iter().collect())
}
