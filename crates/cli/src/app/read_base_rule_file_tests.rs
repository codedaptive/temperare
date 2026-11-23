//! Unit tests for read_base_rule_file.rs: git is asked in the rule file's folder, for the commit `--base` names and then for
//! `./<file name>` at that commit, and the bytes come back parsed. Each error arm, as printed, is in
//! run_loosened_tests.rs.

use super::read_base_rule_file::read_base_rule_file;
use super::RulesWorld;
use engines::Availability;
use procio::{GitError, Oid};
use rules::{Engine, Problem, RuleFile};
use std::path::{Path, PathBuf};

const OID: &str = "0123456789abcdef0123456789abcdef01234567";

const BASE: &str = r#"version = 1
[[rule]]
id = "at-base"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."
"#;

#[test]
fn loosened_base_is_asked_of_git_in_the_folder_and_parsed() {
    let rev_parse = |folder: &Path, rev: &str| -> Result<Option<Oid>, GitError> {
        assert_eq!((folder, rev), (Path::new("/repo/sub"), "release-1"));
        Ok(Oid::parse(OID))
    };
    let blob_at = |folder: &Path, oid: &str, path: &Path| -> Result<Option<Vec<u8>>, GitError> {
        assert_eq!((folder, oid, path), (Path::new("/repo/sub"), OID, Path::new("./rules.toml")));
        Ok(Some(BASE.as_bytes().to_vec()))
    };
    let unused = |_: &Path| -> Result<PathBuf, GitError> { panic!("toplevel must not be called") };
    let read = |_: &Path| -> Result<RuleFile, Vec<Problem>> { panic!("read must not be called") };
    let world = RulesWorld {
        toplevel: &unused,
        exists: &|_| panic!("exists must not be called"),
        read: &read,
        availability: &|_: Engine| Availability::Available,
        now: &|| panic!("now must not be called"),
        rev_parse: &rev_parse,
        blob_at: &blob_at,
        // --loosened never judges no-file rules.
        no_file_rules: &|_: &RuleFile, _: &Path| -> Vec<engines::NoFileRule> {
            panic!("no_file_rules must not be called")
        },
    };
    let Ok(base) = read_base_rule_file(Path::new("/repo/sub/rules.toml"), "release-1", &world) else {
        panic!("the base rule file did not parse");
    };
    assert_eq!(base.rules.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["at-base"]);
}
