//! ZUT for no_file_match.rs: a rule's paths, excludes and kits, as the rule file compiles them, against a listing
//! of the folder's files. The rules come from `RuleFile::parse` on in-memory text; nothing is read from disk.

use crate::folder_files::folder_files;
use crate::no_file_match::no_file_match;
use rules::{RelPath, Rule, RuleFile};
use std::path::{Path, PathBuf};

/// The one rule of a rule file whose rule carries `scope` (TOML lines), with a kit `py` over `py/**`.
fn rule(scope: &str) -> Rule {
    let text = format!(
        "version = 1\n[kits.py]\npaths = [\"py/**\"]\nowner = \"py-zone\"\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\n\
         check = \"E711\"\n{scope}\nfix = \"f\"\n"
    );
    RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0)
}

fn listing(paths: &[&str]) -> Vec<RelPath> {
    paths.iter().map(|path| RelPath::new(path)).collect()
}

/// The folder's files as `files_in_folder` makes them, from procio-shaped listings (top-relative) given by hand.
fn listed(tracked: &[&str], untracked: &[&str]) -> Vec<RelPath> {
    let paths = |list: &[&str]| list.iter().map(PathBuf::from).collect::<Vec<_>>();
    folder_files(Path::new("/repo"), Path::new("/repo"), &paths(tracked), &paths(untracked))
}

#[test]
fn a_rule_matching_only_an_untracked_file_is_not_flagged() {
    let files = listed(&["src/a.py", "docs/readme.md"], &["lib/b.py"]);
    assert!(!no_file_match(&rule("paths = [\"lib/**\"]"), &files));
}

#[test]
fn a_rule_matching_only_an_ignored_file_is_flagged() {
    // An ignored build/c.py is in neither of git's listings (the ZIT tests/no_file_flag.rs proves that with a
    // .gitignore), so the listings given here leave it out.
    let files = listed(&["src/a.py", "docs/readme.md"], &["lib/b.py"]);
    assert!(no_file_match(&rule("paths = [\"build/**\"]"), &files));
}

#[test]
fn paths_matching_one_file_match() {
    let files = listing(&["docs/readme.md", "src/a.py"]);
    assert!(!no_file_match(&rule("paths = [\"src/**\"]"), &files));
}

#[test]
fn paths_matching_no_file_match_nothing() {
    let files = listing(&["docs/readme.md", "src/a.py"]);
    assert!(no_file_match(&rule("paths = [\"lib/**\"]"), &files));
}

#[test]
fn a_file_its_exclude_removes_is_not_a_match() {
    let files = listing(&["src/gen/a.py", "docs/readme.md"]);
    assert!(no_file_match(&rule("paths = [\"src/**\"]\nexclude = [\"src/gen/**\"]"), &files));
    let more = listing(&["src/gen/a.py", "src/b.py"]);
    assert!(!no_file_match(&rule("paths = [\"src/**\"]\nexclude = [\"src/gen/**\"]"), &more));
}

#[test]
fn a_file_only_in_an_outside_kit_is_not_a_match() {
    let files = listing(&["py/a.py"]);
    assert!(no_file_match(&rule("outside_kits = [\"py\"]"), &files));
    assert!(!no_file_match(&rule("outside_kits = [\"py\"]"), &listing(&["py/a.py", "tools/t.py"])));
}

#[test]
fn a_kit_limits_the_files_that_match() {
    assert!(no_file_match(&rule("kits = [\"py\"]"), &listing(&["tools/t.py"])));
    assert!(!no_file_match(&rule("kits = [\"py\"]"), &listing(&["tools/t.py", "py/a.py"])));
}

#[test]
fn a_rule_with_no_paths_matches_any_file() {
    assert!(!no_file_match(&rule(""), &listing(&["anything/at/all.txt"])));
}

#[test]
fn no_files_match_nothing() {
    assert!(no_file_match(&rule(""), &[]));
}
