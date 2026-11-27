//! ZIT for ast-grep rule YAML hard links on real temporary trees, with a fake host.

use super::super::{hits_ast_grep, AST_GREP};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::fs;

const YAML: &str = "id: names\nlanguage: JavaScript\nrule:\n  regex: 'foo'\n";
const RULE: &str = "version = 1\n[[rule]]\nid = \"names\"\nengine = \"ast-grep\"\ncheck = \"link.yml\"\nfix = \"f\"\n";

fn run_case(name: &str, source_inside: bool, hard_link: bool) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>) {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-ast-grep-link-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    let folder = root.join("rules");
    fs::create_dir(&folder).unwrap();
    if hard_link {
        let source = if source_inside { folder.join("source.yml") } else { root.join("source.yml") };
        fs::write(&source, YAML).unwrap();
        fs::hard_link(source, folder.join("link.yml")).unwrap();
    } else {
        fs::write(folder.join("link.yml"), YAML).unwrap();
    }
    let file = RuleFile::parse(RULE, &folder).unwrap();
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        seen.borrow_mut().push(cmd.clone());
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let on_path = |_: &str| -> bool { panic!("hits_ast_grep looks nothing up on PATH") };
    let got = hits_ast_grep(&AST_GREP, &file.rules[0], &folder, &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None });
    fs::remove_dir_all(&root).unwrap();
    (got, seen.into_inner())
}

// `rule_yaml_text` reads the hard-link count on unix only.
#[cfg(unix)]
fn assert_refused(got: Result<Vec<Hit>, Stop>, seen: Vec<Cmd>) {
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ast-grep rule link.yml has more than one hard link: cctao reads a rule YAML only when its folder holds the only link".to_string() }));
    assert!(seen.is_empty());
}

// `rule_yaml_text` reads the hard-link count on unix only.
#[cfg(unix)]
#[test]
fn hard_link_to_outside_yaml_is_refused_without_running() {
    let (got, seen) = run_case("outside", false, true);
    assert_refused(got, seen);
}

// `rule_yaml_text` reads the hard-link count on unix only.
#[cfg(unix)]
#[test]
fn hard_link_with_both_names_inside_is_refused_without_running() {
    // cctao cannot tell where the other links are, so any second link is refused.
    let (got, seen) = run_case("inside", true, true);
    assert_refused(got, seen);
}

#[test]
fn plain_yaml_inside_folder_still_runs() {
    let (got, seen) = run_case("plain", true, false);
    assert_eq!(got, Ok(vec![]));
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].program, "ast-grep");
}
