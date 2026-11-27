//! house@2's citation rule reports only what a change adds: a Rust file committed with a comment citing a register
//! number, then changed to add unrelated code and a second citing comment, is checked against the base with the real
//! ast-grep. The new comment's line alone is reported; the old one, in the same touched file, is not. Without a base
//! the rule is not applicable. Only when ast-grep is not on the PATH `temperare` gets is the case NOT RUN, with
//! `temperare`'s own reason.

use crate::check::Check;
use crate::on_tm_path::on_tm_path;
use crate::repo::Repo;
use crate::tm::tm;

const RULE: &str = "tm-house-no-register-citation-rust";

// house@2's rules other than its three citation rules, each turned off so only those three are in play.
const OFF: [&str; 36] = [
    "tm-house-no-unwrap-rust",
    "tm-house-no-expect-rust",
    "tm-house-no-unsafe-rust",
    "tm-house-no-print-stdout-rust",
    "tm-house-no-todo-python",
    "tm-house-no-todo-swift",
    "tm-house-no-todo-rust",
    "tm-house-no-force-cast-swift",
    "tm-house-no-force-try-swift",
    "tm-house-no-force-unwrapping-swift",
    "tm-house-no-as-conversions-rust",
    "tm-house-no-assign-then-return-python",
    "tm-house-no-let-and-return-rust",
    "tm-house-arg-type-hints-python",
    "tm-house-return-type-hints-python",
    "tm-house-pathlib-python",
    "tm-house-one-public-symbol-rust",
    "tm-house-one-declaration-swift",
    "tm-house-file-name-swift",
    "tm-house-file-length-swift",
    "tm-house-no-stored-bool-swift",
    "tm-house-no-stored-bool-rust",
    "tm-house-bitmap-int64-swift",
    "tm-house-bitmap-int64-rust",
    "tm-house-leading-trailing-swift",
    "tm-house-display-text-literal-swift",
    "tm-house-swift-testing-swift",
    "tm-house-test-ratio",
    "tm-house-test-asserts-sentence-rust",
    "tm-house-test-asserts-sentence-python",
    "tm-house-test-asserts-sentence-swift",
    "tm-house-unit-test-fakes-own-kit-rust",
    "tm-house-unit-test-fakes-own-kit-python",
    "tm-house-test-setup-heavier-rust",
    "tm-house-test-setup-heavier-python",
    "tm-house-test-setup-heavier-swift",
];

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

/// The rule file: house@2 with every rule but the citation ones turned off.
fn rules() -> String {
    OFF.iter().fold(String::from("version = 1\nbaseline = [\"house@2\"]\n"), |text, id| {
        text + &format!("\n[[override]]\nrule = \"{id}\"\noff = true\n")
    })
}

#[test]
fn an_old_citation_in_a_touched_file_is_not_reported() {
    let mut check = Check::new(
        "an_old_citation_in_a_touched_file_is_not_reported",
        "house@2 with only its citation rules on; src/meter.rs committed with a citing comment at line 3, then \
         changed to add a function holding a new citing comment at line 9",
    );
    let repo = Repo::new();
    repo.write("temperare.toml", &rules());
    repo.write("src/meter.rs", BASE);
    let base = repo.commit("base: one old citation");
    repo.write("src/meter.rs", CHANGED);
    repo.commit("adds a function and a new citation");

    let args = ["check", "--json", "--base", base.as_str()];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    if !on_tm_path("ast-grep") {
        let why = a.json["not_available"]
            .as_array()
            .and_then(|list| list.iter().find(|n| n["rule"] == RULE))
            .and_then(|n| n["why"].as_str())
            .unwrap_or_else(|| {
                panic!("ast-grep is not on the PATH tm gets, yet tm did not report {RULE}: {:#}", a.json)
            });
        check.not_run(format!("tm: {why}"));
        return;
    }
    let f = a.findings();
    assert_eq!(f.len(), 1, "the new comment alone: {:#}\n{}", a.json, a.stderr);
    assert_eq!(f[0]["path"], "src/meter.rs");
    assert_eq!(f[0]["line"], 9);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["severity"], "warning");

    let all_args = ["check", "--json"];
    let all = tm(&repo, &all_args);
    check.saw_tm(&all_args, all.code, &all.json);
    let not_applicable = all.json["not_applicable"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_applicable list: {:#}", all.json));
    assert!(not_applicable.iter().any(|n| n["rule"] == RULE), "the rule needs a base: {:#}", all.json);
    assert!(all.findings().iter().all(|f| f["rule"] != RULE), "no citation reported without a base: {:#}", all.json);
}
