//! `absent_engine_reason`: NOT RUN's reason for a case whose engine, or a tool it needs, is missing for `cctao`.

use crate::on_cctao_path::on_cctao_path;
use crate::cctao::Answer;

/// None when every one of `programs` is on the PATH `cctao` gets, an absolute one being the file at that path
/// (`on_cctao_path`, which runs nothing). Otherwise
/// `cctao: <why>`, `cctao`'s own reason in `a` for reporting `rule` not available, which it must then give (panics
/// when it does not).
pub fn absent_engine_reason(a: &Answer, programs: &[&str], rule: &str) -> Option<String> {
    if programs.iter().all(|program| on_cctao_path(program)) {
        return None;
    }
    let why = a.json["not_available"]
        .as_array()
        .and_then(|list| list.iter().find(|n| n["rule"] == rule))
        .and_then(|n| n["why"].as_str())
        .unwrap_or_else(|| {
            panic!("{} not all on the PATH cctao gets, yet cctao did not report {rule}: {:#}", programs.join(", "), a.json)
        });
    Some(format!("cctao: {why}"))
}
