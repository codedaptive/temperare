//! ZIT for swiftlint_entry.rs's thresholds, through `run_plain(&SWIFTLINT, ..)` on a fake host over a temp folder the
//! test makes and removes: the threshold's temporary `.yml` is the last `--config`, holds the check's two levels
//! during the run and is gone after it; it follows the rule's config; and with no rule config the folder's own
//! `.swiftlint.yml` is passed first, when it is there.

use crate::entry::run_plain;
use crate::host::Host;
use crate::swiftlint::SWIFTLINT;
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::{Path, PathBuf};

/// A fresh, canonical temp folder for one test.
fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint-threshold-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::canonicalize(root).unwrap()
}

/// Runs the one rule (`check`, `extra` lines) over `folder` once; gives the run's arguments, with the last one's
/// `--config=` path and that file's text as read during the run.
fn run_threshold(folder: &Path, check: &str, extra: &str) -> (Vec<String>, PathBuf, Option<String>) {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"swiftlint\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
    let file = RuleFile::parse(&text, folder).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let args: Vec<String> = cmd.args.iter().map(|arg| arg.to_str().unwrap().to_string()).collect();
        let path = PathBuf::from(args.last().unwrap().strip_prefix("--config=").unwrap());
        let during = fs::read_to_string(&path).ok();
        *seen.borrow_mut() = Some((args, path, during));
        Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() })
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None), base: None };
    assert_eq!(run_plain(&SWIFTLINT, &file.rules[0], folder, &host), Ok(Vec::new()));
    seen.into_inner().unwrap()
}

#[test]
fn max_complexity_is_the_last_config_holding_both_levels_and_is_gone_after() {
    let folder = tree("complexity");
    let (args, path, during) = run_threshold(&folder, "cyclomatic_complexity", "params = { max_complexity = 3 }\n");
    let gone = !path.exists();
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(args[..5], ["lint", "--reporter", "json", "--quiet", "--only-rule=cyclomatic_complexity"]);
    assert_eq!(args.len(), 6, "{args:?}");
    assert_eq!(path.extension().and_then(|ext| ext.to_str()), Some("yml"));
    assert_eq!(during.as_deref(), Some("cyclomatic_complexity:\n  warning: 3\n  error: 3\n"));
    assert!(gone, "threshold file remains at {}", path.display());
}

#[test]
fn max_lines_sets_both_levels_of_file_length() {
    let folder = tree("lines");
    let (args, path, during) = run_threshold(&folder, "file_length", "params = { max_lines = 150 }\n");
    let gone = !path.exists();
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(args.len(), 6, "{args:?}");
    assert_eq!(during.as_deref(), Some("file_length:\n  warning: 150\n  error: 150\n"));
    assert!(gone, "threshold file remains at {}", path.display());
}

#[test]
fn the_threshold_follows_the_rules_config() {
    let folder = tree("rule-config");
    fs::write(folder.join(".swiftlint.yml"), "excluded: [Pods]\n").unwrap();
    let extra = "config = \"two.yml\"\nparams = { max_complexity = 3 }\n";
    let (args, _, during) = run_threshold(&folder, "cyclomatic_complexity", extra);
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(args.len(), 7, "{args:?}");
    assert_eq!(args[5], "--config=two.yml");
    assert_eq!(during.as_deref(), Some("cyclomatic_complexity:\n  warning: 3\n  error: 3\n"));
}

#[test]
fn with_no_rule_config_the_folders_own_swiftlint_yml_comes_first() {
    let folder = tree("own");
    fs::write(folder.join(".swiftlint.yml"), "excluded: [Pods]\n").unwrap();
    let (args, _, during) = run_threshold(&folder, "file_length", "params = { max_lines = 150 }\n");
    let own = format!("--config={}", folder.join(".swiftlint.yml").display());
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(args.len(), 7, "{args:?}");
    assert_eq!(args[5], own);
    assert_eq!(during.as_deref(), Some("file_length:\n  warning: 150\n  error: 150\n"));
}
