//! house@2's citation rule reports only what a change adds: a Rust file committed with a comment citing a register
//! number, then changed to add unrelated code and a second citing comment, is checked against the base with the real
//! ast-grep. The new comment's line alone is reported; the old one, in the same touched file, is not. Without a base
//! the rule is not applicable. Only when ast-grep is not on the PATH `cctao` gets is the case NOT RUN, with
//! `cctao`'s own reason.

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::cctao;
use crate::citation_rules::citation_rules;

const RULE: &str = "tm-house-no-register-citation-rust";

// The base: line 3 is a comment citing an R-number.
const BASE: &str = "//! The meter.

// R-12: keep the meter in whole units.
pub fn read(units: u32) -> u32 {
    units
}
";

// The change: BASE with an unrelated function added; its line 9 is a new comment citing an S-number.
const CHANGED: &str = "//! The meter.

// R-12: keep the meter in whole units.
pub fn read(units: u32) -> u32 {
    units
}

pub fn half(units: u32) -> u32 {
    // S-7: the reading is rounded down.
    units / 2
}
";

#[test]
fn an_old_citation_in_a_touched_file_is_not_reported() {
    let mut check = Check::new(
        "an_old_citation_in_a_touched_file_is_not_reported",
        "house@2 with only its citation rules on; src/meter.rs committed with a citing comment at line 3, then \
         changed to add a function holding a new citing comment at line 9",
    );
    let repo = Repo::new();
    repo.write("tao.toml", &citation_rules());
    repo.write("src/meter.rs", BASE);
    let base = repo.commit("base: one old citation");
    repo.write("src/meter.rs", CHANGED);
    repo.commit("adds a function and a new citation");

    let args = ["check", "--json", "--base", base.as_str()];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["ast-grep"], RULE) {
        check.not_run(why);
        return;
    }
    let f = a.findings();
    assert_eq!(f.len(), 1, "the new comment alone: {:#}\n{}", a.json, a.stderr);
    assert_eq!(f[0]["path"], "src/meter.rs");
    assert_eq!(f[0]["line"], 9);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["severity"], "warning");

    let all_args = ["check", "--json"];
    let all = cctao(&repo, &all_args);
    check.saw_cctao(&all_args, all.code, &all.json);
    let not_applicable = all.json["not_applicable"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_applicable list: {:#}", all.json));
    assert!(not_applicable.iter().any(|n| n["rule"] == RULE), "the rule needs a base: {:#}", all.json);
    assert!(all.findings().iter().all(|f| f["rule"] != RULE), "no citation reported without a base: {:#}", all.json);
}
