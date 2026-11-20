//! ZUT for refused.rs: rules from in-memory rule-file text, each source of refusal (ast-grep's count params among
//! them) and the rules refused by none, among them a good count rule and an ast-grep rule whose set carries its YAML
//! (`embedded_text`), whose absolute check names no file.
//! Nothing is read from disk and nothing runs.

use crate::refused::refused;
use crate::types::Stop;
use rules::{Rule, RuleFile};
use std::path::Path;

/// The one rule of a rule file: `engine`, `check`, then `extra` (TOML lines).
fn rule(engine: &str, check: &str, extra: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
    RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0)
}

#[test]
fn a_refused_check_or_param_is_failed_with_its_flows_stderr_and_any_other_rule_is_not_refused() {
    let cases: [(&str, &str, &str, Option<&str>); 20] = [
        (
            "ruff",
            "flake8-bugbear",
            "",
            Some("ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects"),
        ),
        (
            "shellcheck",
            "sc2086",
            "",
            Some("shellcheck check `sc2086` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects"),
        ),
        (
            "swiftlint",
            "custom_rules",
            "",
            Some("swiftlint check `custom_rules` names no rule swiftlint runs under --only-rule: name the custom rule's own identifier"),
        ),
        ("ruff", "E711", "params = { nope = [\"x\"] }\n", Some("ruff param nope: a ruff rule reads only the `max_complexity` param")),
        ("shellcheck", "SC2086", "params = { x = [\"y\"] }\n", Some("shellcheck param x: a shellcheck rule takes no params")),
        ("ruff", "E711", "params = { max_complexity = 5 }\n", Some("ruff param max_complexity: read only with check `C901`")),
        (
            "clippy",
            "clippy::excessive_nesting",
            "params = { max_nesting = 0 }\n",
            Some("clippy param max_nesting: must be a whole number from 1 to 1000"),
        ),
        (
            "git",
            "test_ratio",
            "params = { max_ratio = 0, floor = 40, test_paths = [\"t/**\"] }\n",
            Some("git param max_ratio: must be a number from 0.1 to 100"),
        ),
        (
            "git",
            "test_ratio",
            "params = { foo = 1 }\n",
            Some("git param foo: a git rule reads only the `max_ratio`, `floor`, `test_paths` param"),
        ),
        ("ast-grep", "/abs/names.yml", "", Some("ast-grep rule /abs/names.yml is an absolute path: a check names a file inside the rule file's folder")),
        ("clippy", "rustdoc::broken_intra_doc_links", "", Some("clippy check `rustdoc::broken_intra_doc_links` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it")),
        ("ruff", "E711", "", None),
        ("ruff", "C901", "params = { max_complexity = 5 }\n", None),
        ("ast-grep", "names.yml", "params = { nope = [\"x\"] }\n", None),
        ("ast-grep", "names.yml", "params = { count = \"wrong\" }\n", Some("param count in names.yml: takes only \"setup-vs-tests\"")),
        ("ast-grep", "names.yml", "params = { count = 3 }\n", Some("param count in names.yml: takes only \"setup-vs-tests\"")),
        (
            "ast-grep",
            "names.yml",
            "params = { shared_setup = [\"common/**\"] }\n",
            Some("param shared_setup in names.yml: needs count = \"setup-vs-tests\""),
        ),
        (
            "ast-grep",
            "names.yml",
            "params = { count = \"setup-vs-tests\", shared_setup = \"common/**\" }\n",
            Some("param shared_setup in names.yml: not a list of globs"),
        ),
        ("ast-grep", "names.yml", "params = { count = \"setup-vs-tests\", shared_setup = [\"common/**\"] }\n", None),
        ("pylint", "anything", "params = { x = [\"y\"] }\n", None),
    ];
    for (engine, check, extra, stderr) in cases {
        let want = stderr.map(|stderr| Stop::Failed { code: None, stderr: stderr.to_string() });
        assert_eq!(refused(&rule(engine, check, extra)), want, "{engine} {check} {extra}");
    }
}

#[test]
fn an_ast_grep_rule_carrying_its_yaml_is_not_refused_for_an_absolute_check() {
    let mut carried = rule("ast-grep", "/abs/names.yml", "");
    carried.embedded_text = Some("id: names\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n");
    assert_eq!(refused(&carried), None);
}
