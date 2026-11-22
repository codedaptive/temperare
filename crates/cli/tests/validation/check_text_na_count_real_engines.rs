//! Not-applicable rules are a count in tm check's text, and tm prints less than the engines' raw output, on Bob's
//! repository shape (Bob #2235; SPEC-report WO-report-4: "on that repository, tm check prints less than the engines'
//! raw output"). A throwaway repository takes `standard@1`; its base commit holds only the rule file, and a later
//! commit adds a Python file with five ruff findings, so `tm check --base <base>` reports them as new, with the real
//! ruff and ast-grep. In text, a not-applicable rule (here every clippy, SwiftLint and other-language rule) gets no
//! line of its own, only its count in the counts line.

use crate::env::tm_env;
use crate::on_tm_path::on_tm_path;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

const RULES: &str = "version = 1\nbaseline = [\"standard@1\"]\n";

/// Two unused imports (F401), an unused local (F841), a print (T201) and a bare except (E722): ruff's raw JSON for
/// these is several times the size of tm's text, where one finding alone might not be.
const PYTHON: &str = "import os
import sys


def run():
    unused = 1
    print(\"hi\")
    try:
        pass
    except:
        pass
";

const SAVED: &str = "[temperare] tokens saved ≈ ";

#[test]
fn not_applicable_rules_are_a_count_and_tm_prints_less_than_the_engines() {
    let repo = Repo::new();
    let home = TempHome::new();
    repo.write("temperare.toml", RULES);
    let base = repo.commit("rule file only");
    repo.write("app.py", PYTHON);
    repo.commit("python with ruff findings");
    let out = run_clean(
        env!("CARGO_BIN_EXE_temperare"),
        &["check", "--base", &base],
        repo.path(),
        tm_env(None, home.path()),
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    let shown = format!("stdout:\n{stdout}\nstderr:\n{}", String::from_utf8_lossy(&out.stderr));
    let absent: Vec<&str> = ["ruff", "ast-grep"].into_iter().filter(|engine| !on_tm_path(engine)).collect();
    if !absent.is_empty() {
        assert_eq!(out.code, Some(3), "{shown}");
        for engine in absent {
            let named = format!("({engine}):");
            assert!(
                stdout.lines().any(|line| line.starts_with("not available: ") && line.contains(&named)),
                "no not-available line for {engine}\n{shown}"
            );
        }
        return;
    }
    assert!(
        !stdout.lines().any(|line| line.starts_with("not applicable:")),
        "assertion 1: a not-applicable rule has a line of its own\n{shown}"
    );
    let counts = stdout
        .lines()
        .find(|line| line.contains(" not applicable, "))
        .unwrap_or_else(|| panic!("assertion 2: no counts line\n{shown}"));
    assert!(count(counts, "not applicable") > 0, "assertion 2: no rule counted not applicable\n{shown}");
    assert_eq!(count(counts, "not available"), 0, "assertion 2: an engine was not available\n{shown}");
    assert_eq!(count(counts, "failed"), 0, "assertion 2: a rule failed\n{shown}");
    assert!(count(counts, "findings") >= 1, "assertion 2: the new findings were not reported\n{shown}");
    let saved = stdout
        .lines()
        .find(|line| line.starts_with(SAVED))
        .unwrap_or_else(|| panic!("assertion 3: no tokens-saved line\n{shown}"));
    let tokens_saved = number_after(saved, SAVED);
    let this_output = number_after(saved, "this output ≈ ");
    let raw = number_after(saved, "engine run(s) ≈ ");
    assert!(tokens_saved > 0, "assertion 3: tokens saved is 0\n{shown}");
    assert!(this_output < raw, "assertion 3: tm's output ({this_output}) is not below the raw ({raw})\n{shown}");
}

/// The number the counts line prints before ` <label>`: `3` from `3 findings` or from `, 3 failed,`.
fn count(line: &str, label: &str) -> u64 {
    let (before, _) = line
        .split_once(&format!(" {label}"))
        .unwrap_or_else(|| panic!("no `{label}` in the counts line: {line}"));
    let word = before.rsplit(' ').next().unwrap_or_default();
    word.parse().unwrap_or_else(|e| panic!("`{word}` before `{label}` is not a number ({e}): {line}"))
}

/// The number, thousands commas dropped, that follows `prefix` in the tokens-saved line.
fn number_after(line: &str, prefix: &str) -> u64 {
    let (_, after) = line.split_once(prefix).unwrap_or_else(|| panic!("no `{prefix}` in: {line}"));
    let digits: String = after.chars().take_while(|c| c.is_ascii_digit() || *c == ',').filter(|c| *c != ',').collect();
    digits.parse().unwrap_or_else(|e| panic!("no number after `{prefix}` ({e}): {line}"))
}
