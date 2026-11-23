//! `absent_engine_reason`: NOT RUN's reason for a case whose engine, or a tool it needs, is missing for `temperare`.

use crate::on_tm_path::on_tm_path;
use crate::tm::Answer;

/// None when every one of `programs` is on the PATH `temperare` gets, an absolute one being the file at that path
/// (`on_tm_path`, which runs nothing). Otherwise
/// `tm: <why>`, `temperare`'s own reason in `a` for reporting `rule` not available, which it must then give (panics
/// when it does not).
pub fn absent_engine_reason(a: &Answer, programs: &[&str], rule: &str) -> Option<String> {
    if programs.iter().all(|program| on_tm_path(program)) {
        return None;
    }
    let why = a.json["not_available"]
        .as_array()
        .and_then(|list| list.iter().find(|n| n["rule"] == rule))
        .and_then(|n| n["why"].as_str())
        .unwrap_or_else(|| {
            panic!("{} not all on the PATH tm gets, yet tm did not report {rule}: {:#}", programs.join(", "), a.json)
        });
    Some(format!("tm: {why}"))
}
