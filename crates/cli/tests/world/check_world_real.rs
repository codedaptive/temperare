//! Tests of crates/cli/src/app/check_world_real.rs: six of the nine seams of `CheckWorld::real()` (toplevel, exists,
//! read, rev_parse, run_rules, build_report) are wired to their real functions. `availability`, `home` and `now` have
//! no case here: `availability` would look at the real PATH, `home` would read the real HOME and `now` the real
//! clock. The file mixes kinds, each case marked: RITs (cli→procio/rules/engines/report, kept beside their caller)
//! and a std disk seam. `exists` and `read` are told from stubs on
//! a file the tests write in a throwaway folder under the system temp folder, removed on drop; `run_rules` (wired to
//! `engines::run_rules_versioned`) and `build_report` on a rule file with no rules in such a folder; `toplevel` and
//! `rev_parse` are told by a folder that does not exist, where git fails. Nothing here needs a real repository or an
//! engine: `run_rules` and `build_report` get rule files with no rules, so no engine is looked up, and the report has
//! no base, so no git diff is run.

use cli::CheckWorld;
use rules::{Exception, RuleFile};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

const RULE_FILE: &str = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."
"#;

/// The next number for a folder name, unique within this process.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// A folder under the system temp folder that this test made, removed on drop (also when an assertion fails).
struct TempFolder(PathBuf);

impl TempFolder {
    /// A new folder named by the process id and the next free number; a name already taken moves on to the next.
    fn new(tag: &str) -> TempFolder {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-cli-cw-{}-{n}-{tag}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return TempFolder(path),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A rule file with no rules in `folder`.
fn empty_rules(folder: PathBuf, exceptions: Vec<Exception>) -> RuleFile {
    let mut rf = RuleFile::new(folder);
    rf.exceptions = exceptions;
    rf
}

/// Std disk seam, not a zone test: the seam is the standard library's `Path::is_file`, reached through the disk
/// (crates/cli/src/app/check_world_real.rs:13).
#[test]
fn exists_is_true_for_a_file_only() {
    let world = CheckWorld::real();
    let dir = TempFolder::new("exists");
    let file = dir.0.join("temperare.toml");
    fs::write(&file, RULE_FILE).unwrap();
    assert!((world.exists)(&file));
    assert!(!(world.exists)(&dir.0), "a folder is not a rule file");
    assert!(!(world.exists)(&dir.0.join("missing.toml")));
}

/// RIT: `RuleFile::read` reads the disk (crates/rules/src/rule_file/rule_file_read.rs:11).
#[test]
fn read_reads_a_rule_file_from_disk() {
    let world = CheckWorld::real();
    let dir = TempFolder::new("read");
    let file = dir.0.join("temperare.toml");
    fs::write(&file, RULE_FILE).unwrap();
    let rule_file = (world.read)(&file).unwrap();
    let ids: Vec<&str> = rule_file.rules.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(ids, vec!["lint-all"]);
}

/// RIT: git runs (crates/procio/src/git/ops/toplevel.rs:13-16, rev_parse.rs:11-13).
#[test]
fn toplevel_and_rev_parse_of_a_missing_folder_are_errors() {
    let world = CheckWorld::real();
    let dir = TempFolder::new("git-missing");
    let missing = dir.0.join("missing");
    assert!((world.toplevel)(&missing).is_err());
    assert!((world.rev_parse)(&missing, "HEAD").is_err());
}

/// RIT: `engines::run_rules_versioned` (which wires procio in) and `rule_folder` (`fs::canonicalize` of the rule
/// file's folder): a rule file with no rules runs nothing and names no engine.
#[test]
fn run_rules_of_a_rule_file_with_no_rules_is_empty() {
    let world = CheckWorld::real();
    let dir = TempFolder::new("run");
    let checked = (world.run_rules)(&empty_rules(dir.0.clone(), Vec::new()), &dir.0, &[]);
    assert!(checked.runs.is_empty());
    assert!(checked.engines.is_empty());
}

/// RIT: with no base, report's `build_report` returns pure `assemble_report`; the setup touches disk.
#[test]
fn build_report_with_no_base_counts_the_exception_in_force() {
    let world = CheckWorld::real();
    let dir = TempFolder::new("report");
    let exception = Exception::new("lint-all", "old.rs", "legacy", "Bob, 2026-11-17");
    let report = (world.build_report)(&empty_rules(dir.0.clone(), vec![exception]), Vec::new(), None).unwrap();
    assert!(report.findings.is_empty());
    assert_eq!(report.exceptions_in_force, 1);
}
