//! Unit test for no_file_rules_listed: refused rules left out; paths-match-no-file and no-script rules settled in cctao check's
//! order over given listings; `only_with_base` plays no part.

use crate::no_file_rules_listed::no_file_rules_listed;
use crate::types::Stop;
use crate::NoFileRule;
use rules::{Engine, RelPath, RuleFile};
use std::cell::Cell;
use std::path::{Path, PathBuf};

#[test]
fn no_file_rules_listed_cases() {
    const NO_MATCH: &str = "its paths match no file in the rule file's folder";
    const NO_SCRIPT: &str = "no file its engine checks";
    type RuleSpec = (&'static str, &'static str, &'static str, &'static str);
    type Expected = (&'static str, Engine, &'static str);
    struct Case {
        name: &'static str,
        rules: Vec<RuleSpec>,
        files: Option<Vec<&'static str>>,
        folder: bool,
        scripts: Option<Result<Vec<&'static str>, ()>>,
        expected: Vec<Expected>,
    }
    let ruff = ("ruff-rule", "ruff", "E711", "paths = [\"src/**\"]");
    let shell = ("shell-rule", "shellcheck", "SC2086", "paths = [\"docs/**\"]");
    let refused_ruff = ("refused-ruff", "ruff", "E711", "paths = [\"src/**\"]\nparams = { nope = [\"x\"] }");
    let refused_shell = ("refused-shell", "shellcheck", "SC2086", "paths = [\"docs/**\"]\nparams = { x = [\"y\"] }");
    let base_ruff = ("base-ruff", "ruff", "E711", "paths = [\"src/**\"]\nonly_with_base = true");
    let base_shell = ("base-shell", "shellcheck", "SC2086", "paths = [\"docs/**\"]\nonly_with_base = true");
    let cases = vec![
        Case { name: "kind 1", rules: vec![ruff], files: Some(vec!["docs/readme.md"]), folder: true, scripts: None, expected: vec![("ruff-rule", Engine::Ruff, NO_MATCH)] },
        Case { name: "kind 2", rules: vec![shell], files: Some(vec!["docs/readme.md"]), folder: true, scripts: Some(Ok(vec!["elsewhere/a.sh"])), expected: vec![("shell-rule", Engine::ShellCheck, NO_SCRIPT)] },
        Case { name: "refused kind 1", rules: vec![refused_ruff], files: Some(vec!["docs/readme.md"]), folder: true, scripts: None, expected: vec![] },
        Case { name: "refused kind 2", rules: vec![refused_shell], files: Some(vec!["docs/readme.md"]), folder: true, scripts: None, expected: vec![] },
        Case { name: "base kind 1", rules: vec![base_ruff], files: Some(vec!["docs/readme.md"]), folder: true, scripts: None, expected: vec![("base-ruff", Engine::Ruff, NO_MATCH)] },
        Case { name: "base kind 2", rules: vec![base_shell], files: Some(vec!["docs/readme.md"]), folder: true, scripts: Some(Ok(vec![])), expected: vec![("base-shell", Engine::ShellCheck, NO_SCRIPT)] },
        Case { name: "no files listing", rules: vec![ruff], files: None, folder: true, scripts: None, expected: vec![] },
        Case { name: "scripts listing failed", rules: vec![shell], files: Some(vec!["docs/readme.md"]), folder: true, scripts: Some(Err(())), expected: vec![] },
        Case { name: "no folder", rules: vec![shell], files: Some(vec!["docs/readme.md"]), folder: false, scripts: None, expected: vec![] },
        Case { name: "matching file", rules: vec![ruff], files: Some(vec!["src/a.py"]), folder: true, scripts: None, expected: vec![] },
        Case { name: "matching script", rules: vec![shell], files: Some(vec!["docs/a.sh"]), folder: true, scripts: Some(Ok(vec!["docs/a.sh"])), expected: vec![] },
        Case { name: "kind 1 before kind 2", rules: vec![shell], files: Some(vec!["elsewhere/a.txt"]), folder: true, scripts: Some(Ok(vec![])), expected: vec![("shell-rule", Engine::ShellCheck, NO_MATCH)] },
        Case { name: "rule-file order", rules: vec![shell, refused_ruff, base_ruff, ruff], files: Some(vec!["docs/readme.md"]), folder: true, scripts: Some(Ok(vec![])), expected: vec![("shell-rule", Engine::ShellCheck, NO_SCRIPT), ("base-ruff", Engine::Ruff, NO_MATCH), ("ruff-rule", Engine::Ruff, NO_MATCH)] },
    ];
    let mut wrong = Vec::new();
    for case in cases {
        let mut text = String::from("version = 1\n");
        for (id, engine, check, extra) in case.rules {
            text.push_str(&format!("[[rule]]\nid = \"{id}\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}\n"));
        }
        let file = RuleFile::parse(&text, Path::new(".")).unwrap();
        let files = case.files.map(|paths| paths.into_iter().map(RelPath::new).collect::<Vec<_>>());
        let unexpected_call = Cell::new(false);
        let scripts = |folder: &Path| {
            if folder != Path::new("/repo") || case.scripts.is_none() {
                unexpected_call.set(true);
            }
            match case.scripts.as_ref() {
                Some(Ok(paths)) => Ok(paths.iter().map(|path| PathBuf::from(*path)).collect()),
                _ => Err(Stop::Failed { code: None, stderr: "listing failed".into() }),
            }
        };
        let expected: Vec<_> = case.expected.into_iter().map(|(rule, engine, why)| NoFileRule {
            rule: rule.into(), engine, why: why.into(),
        }).collect();
        let got = no_file_rules_listed(&file, case.folder.then_some(Path::new("/repo")), files.as_deref(), &scripts);
        if got != expected || unexpected_call.get() {
            wrong.push(format!("{}: got {got:?}, expected {expected:?}, unexpected lister call: {}", case.name, unexpected_call.get()));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
