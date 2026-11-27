//! house@2's Python citation rule reads a test's strings: a Python file committed with a test_* function holding no
//! citation, then changed to add a citing string inside that test and another in a plain function, is checked against
//! the base with the real ast-grep. The string in the test alone is reported; the one outside test code is left. Only
//! when ast-grep is not on the PATH `cctao` gets is the case NOT RUN, with `cctao`'s own reason.

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::cctao;
use crate::citation_rules::citation_rules;

const RULE: &str = "tm-house-no-register-citation-python";

// The base: a function and its test, no citation anywhere.
const BASE: &str = "def read(units: int) -> int:
    return units // 2


def test_rounds_down() -> None:
    assert read(5) == 2
";

// The change: line 6 is a new string citing a register number in a plain function, which the rule leaves; line 10
// is a new string citing one inside test_rounds_down, which it reports.
const CHANGED: &str = "def read(units: int) -> int:
    return units // 2


def unit_name() -> str:
    return \"S-7: whole litres\"


def test_rounds_down() -> None:
    note = \"R-12: the part litre is dropped\"
    assert read(5) == 2, note
";

#[test]
fn a_citing_string_in_a_python_test_is_reported() {
    let mut check = Check::new(
        "a_citing_string_in_a_python_test_is_reported",
        "house@2 with only its citation rules on; tests/test_meter.py committed with a test_* function and no \
         citation, then changed to add a citing string in a plain function at line 6 and one inside the test at \
         line 10",
    );
    let repo = Repo::new();
    repo.write("tao.toml", &citation_rules());
    repo.write("tests/test_meter.py", BASE);
    let base = repo.commit("base: a test with no citation");
    repo.write("tests/test_meter.py", CHANGED);
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
    assert_eq!(f[0]["path"], "tests/test_meter.py");
    assert_eq!(f[0]["line"], 10);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["severity"], "warning");
}
