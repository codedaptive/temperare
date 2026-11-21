//! `run_baseline_test` with the real world and real ruff on a const set `test@1`, in a child of this test binary:
//! `engines::run_rules_versioned_at` inherits the process environment, so the child is started through `run_clean`
//! with only `tm_env`'s variables (HOME a folder the test made) and a marker; the parent reads the exit, the printed
//! text and the case folders the child made from the child's stdout, between markers.

use crate::env::tm_env;
use crate::on_tm_path::on_tm_path;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;
use cli::{run_baseline_test, BaselineArgs, BaselineWorld, UsageError};
use rules::{CaseFile, Engine, ExpectedFinding, Rule, RuleCases};
use std::cell::RefCell;
use std::path::Path;

const CHILD: &str = "TEMPERARE_BASELINE_REAL_RUFF_CHILD";
const OUT_BEGIN: &str = "<<baseline-out>>\n";
const OUT_END: &str = "<<baseline-end>>\n";

/// A ruff F401 rule `id` whose failing case is `pkg/a.py` = `import os\n`, expected at line `line`, and whose
/// passing case is `pkg/a.py` = `print(1)\n`.
fn f401(id: &str, line: usize) -> Rule {
    let mut rule = Rule::new(id, Engine::Ruff, "F401");
    rule.embedded_cases = Some(RuleCases {
        failing: vec![CaseFile { path: "pkg/a.py".to_string(), text: "import os\n" }],
        passing: vec![CaseFile { path: "pkg/a.py".to_string(), text: "print(1)\n" }],
        expected: ExpectedFinding { rule_id: id.to_string(), path: "pkg/a.py".to_string(), line },
    });
    rule
}

/// The child: runs `tm baseline test --set test@1` on tm-zeta (holds: ruff reports line 1) and tm-alpha (expects
/// line 2, so its failing case is broken), out of sort order, and prints the exit, the text and each case folder.
fn child() {
    let rules_of = |set: &str| match set {
        "test@1" => Ok(vec![f401("tm-zeta", 1), f401("tm-alpha", 2)]),
        _ => Err(UsageError { what: format!("no test set {set}"), fix: String::new() }),
    };
    let made = RefCell::new(Vec::new());
    let real = BaselineWorld::real();
    let make_folder = || {
        let folder = (real.make_folder)()?;
        made.borrow_mut().push(folder.clone());
        Ok(folder)
    };
    let world = BaselineWorld { rules_of: &rules_of, make_folder: &make_folder, ..BaselineWorld::real() };
    let mut out = Vec::new();
    let args = BaselineArgs { set: Some("test@1".into()), json: false };
    let exit = run_baseline_test(&args, &world, &mut out, &mut std::io::stderr());
    let mut block = format!("\n<<baseline-exit {exit}>>\n{OUT_BEGIN}{}{OUT_END}", String::from_utf8_lossy(&out));
    for folder in made.borrow().iter() {
        block.push_str(&format!("<<baseline-folder {}>>\n", folder.display()));
    }
    print!("{block}");
}

/// The value of each `<<tag value>>` line in `stdout`, in order.
fn tagged<'s>(stdout: &'s str, tag: &str) -> Vec<&'s str> {
    let head = format!("<<{tag} ");
    stdout.lines().filter_map(|l| l.strip_prefix(head.as_str())?.strip_suffix(">>")).collect()
}

#[test]
fn baseline_test_real_ruff_on_a_const_set() {
    if std::env::var_os(CHILD).is_some() {
        return child();
    }
    let home = TempHome::new();
    let exe = std::env::current_exe().expect("the test binary's path");
    let mut vars = tm_env(None, home.path());
    vars.push((CHILD.to_string(), "1".to_string()));
    let args = ["--exact", "baseline_test_real_ruff::baseline_test_real_ruff_on_a_const_set", "--nocapture"];
    let child = run_clean(exe, &args, home.path(), vars);
    let stdout = String::from_utf8_lossy(&child.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&child.stderr);
    assert_eq!(child.code, Some(0), "child failed:\nstdout: {stdout}\nstderr: {stderr}");
    assert!(stdout.contains("1 passed"), "child ran no test:\nstdout: {stdout}\nstderr: {stderr}");

    let exit = tagged(&stdout, "baseline-exit");
    let start = stdout.find(OUT_BEGIN).expect("the printed-text marker") + OUT_BEGIN.len();
    let printed = &stdout[start..start + stdout[start..].find(OUT_END).expect("the end marker")];
    let (want_exit, want_text) = if on_tm_path("ruff") {
        (
            "1",
            "test@1 tm-zeta (ruff): holds\n\
             test@1 tm-alpha (ruff): broken\n  \
             failing: broken: gave tm-alpha at pkg/a.py:1; expected tm-alpha at pkg/a.py:2\n\
             2 rules: 1 hold, 1 broken, 0 not available, 0 not applicable\n\
             next: fix each broken rule or its cases above\n",
        )
    } else {
        (
            "3",
            "test@1 tm-zeta (ruff): not available\n  \
             failing: not available: ruff not found on PATH\n  \
             passing: not available: ruff not found on PATH\n\
             test@1 tm-alpha (ruff): not available\n  \
             failing: not available: ruff not found on PATH\n  \
             passing: not available: ruff not found on PATH\n\
             2 rules: 0 hold, 0 broken, 2 not available, 0 not applicable\n\
             next: install ruff, then run tm baseline test again\n",
        )
    };
    assert_eq!(exit, [want_exit], "stdout: {stdout}");
    assert_eq!(printed, want_text, "stdout: {stdout}");

    // Two rules, two cases each: four case folders, each named tm-baseline-*, none left behind.
    let folders = tagged(&stdout, "baseline-folder");
    assert_eq!(folders.len(), 4, "stdout: {stdout}");
    for folder in folders {
        let path = Path::new(folder);
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        assert!(name.starts_with("tm-baseline-"), "{folder}");
        assert!(!path.exists(), "case folder remains: {folder}");
    }
}
