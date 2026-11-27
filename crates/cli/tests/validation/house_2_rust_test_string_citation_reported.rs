//! house@2's Rust citation rule reads a test's strings: a Rust file committed with a #[test] fn holding no citation,
//! then changed to add a citing string inside that test and another in a plain function, is checked against the base
//! with the real ast-grep. The string in the test alone is reported; the one outside test code is left. Only when
//! ast-grep is not on the PATH `cctao` gets is the case NOT RUN, with `cctao`'s own reason.

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::cctao;
use crate::citation_rules::citation_rules;

const RULE: &str = "tm-house-no-register-citation-rust";

// The base: a function and its #[cfg(test)] module, no citation anywhere.
const BASE: &str = r#"//! The meter.

pub fn read(units: u32) -> u32 {
    units / 2
}

#[cfg(test)]
mod tests {
    use super::read;

    #[test]
    fn rounds_down() {
        assert_eq!(read(5), 2);
    }
}
"#;

// The change: line 8 is a new string citing a register number in a plain function, which the rule leaves; line 17
// is a new string citing one inside the #[test] fn, which it reports.
const CHANGED: &str = r#"//! The meter.

pub fn read(units: u32) -> u32 {
    units / 2
}

pub fn unit_name() -> &'static str {
    "S-7: whole litres"
}

#[cfg(test)]
mod tests {
    use super::read;

    #[test]
    fn rounds_down() {
        let note = "R-12: the part litre is dropped";
        assert_eq!(read(5), 2, "{note}");
    }
}
"#;

#[test]
fn a_citing_string_in_a_rust_test_is_reported() {
    let mut check = Check::new(
        "a_citing_string_in_a_rust_test_is_reported",
        "house@2 with only its citation rules on; src/meter.rs committed with a #[test] fn and no citation, then \
         changed to add a citing string in a plain function at line 8 and one inside the test at line 17",
    );
    let repo = Repo::new();
    repo.write("tao.toml", &citation_rules());
    repo.write("src/meter.rs", BASE);
    let base = repo.commit("base: a test with no citation");
    repo.write("src/meter.rs", CHANGED);
    repo.commit("adds a citing string in code and in the test");

    let args = ["check", "--json", "--base", base.as_str()];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["ast-grep"], RULE) {
        check.not_run(why);
        return;
    }
    let f = a.findings();
    assert_eq!(f.len(), 1, "the string in the test alone: {:#}\n{}", a.json, a.stderr);
    assert_eq!(f[0]["path"], "src/meter.rs");
    assert_eq!(f[0]["line"], 17);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["severity"], "warning");
}
