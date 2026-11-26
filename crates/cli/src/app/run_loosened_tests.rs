//! ZUT for run_loosened.rs: each way the base or now cannot be read, as printed with its exit, and exit 0 and 1 with
//! their `next:` text, with git, the disk and the clock as closures and rule files parsed from literal TOML. The real
//! git, `--json` and every reachable kind are in tests/world/rules_loosened_real.rs.

use super::run_rules;
use super::RulesWorld;
use crate::RulesArgs;
use engines::Availability;
use ccvcslib::{VcsError, VcsFailure};
use report::Oid;
use rules::{Engine, Problem, RuleFile};
use std::path::{Path, PathBuf};

const OID: &str = "0123456789abcdef0123456789abcdef01234567";

const ONE_RULE: &str = r#"version = 1
[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."
"#;

/// What git answers for the base: each arm read_base_rule_file takes.
#[derive(Clone, Copy)]
enum Base {
    NoCommit,
    RevGitError,
    BlobGitError,
    NoFile,
    NotUtf8,
    Text(&'static str),
}

fn git_error() -> VcsError {
    VcsError::Failed(VcsFailure { message: "fatal: boom".into(), exit_status: Some(128) })
}

/// Runs `tm rules --loosened --base v1` in /repo/src, the repository top /repo; `now` is the working rule file's text,
/// or None for a file with one problem. (exit, out, err).
fn loosened(base: Base, now: Option<&'static str>) -> (u8, String, String) {
    let toplevel = |cwd: &Path| -> Result<PathBuf, VcsError> {
        assert_eq!(cwd, Path::new("/repo/src"));
        Ok(PathBuf::from("/repo"))
    };
    let read = |path: &Path| -> Result<RuleFile, Vec<Problem>> {
        assert_eq!(path, Path::new("/repo/temperare.toml"));
        match now {
            Some(text) => Ok(RuleFile::parse(text, Path::new("/repo")).unwrap()),
            None => Err(vec![Problem { line: 2, col: 1, what: "unknown key".into(), fix: "Remove the key.".into() }]),
        }
    };
    let rev_parse = |folder: &Path, rev: &str| -> Result<Option<Oid>, VcsError> {
        assert_eq!((folder, rev), (Path::new("/repo"), "v1"));
        match base {
            Base::NoCommit => Ok(None),
            Base::RevGitError => Err(git_error()),
            _ => Ok(Oid::parse(OID)),
        }
    };
    let blob_at = |folder: &Path, oid: &Oid, path: &str| -> Result<Option<Vec<u8>>, VcsError> {
        assert_eq!((folder, oid, path), (Path::new("/repo"), &Oid::parse(OID).unwrap(), "temperare.toml"));
        match base {
            Base::BlobGitError => Err(git_error()),
            Base::NoFile => Ok(None),
            Base::NotUtf8 => Ok(Some(vec![b'v', 0xff, 0xfe])),
            Base::Text(text) => Ok(Some(text.as_bytes().to_vec())),
            Base::NoCommit | Base::RevGitError => panic!("blob_at must not be called"),
        }
    };
    let world = RulesWorld {
        toplevel: &toplevel,
        exists: &|_| true,
        read: &read,
        availability: &|_: Engine| Availability::Available,
        now: &|| 0,
        rev_parse: &rev_parse,
        blob_at: &blob_at,
        // --loosened never judges no-file rules.
        no_file_rules: &|_: &RuleFile, _: &Path| -> Vec<engines::NoFileRule> {
            panic!("no_file_rules must not be called")
        },
    };
    let args = RulesArgs { loosened: true, base: Some("v1".to_string()), ..RulesArgs::default() };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run_rules(&args, Path::new("/repo/src"), &world, &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

#[test]
fn each_outcome_with_its_exit_and_text() {
    let rows: [(&str, Base, Option<&'static str>, u8, &str, &str); 9] = [
        ("rev names nothing", Base::NoCommit, Some(ONE_RULE), 2, "",
         "tm: base 'v1' does not name a commit\nnext: Pass a commit, branch or tag that exists, e.g. --base main.\n"),
        ("rev_parse git error", Base::RevGitError, Some(ONE_RULE), 2, "",
         "tm: git could not resolve base 'v1': fatal: boom\n\
          next: Run tm rules inside a git repository to use --base.\n"),
        ("blob_at git error", Base::BlobGitError, Some(ONE_RULE), 2, "",
         "tm: git could not read temperare.toml at base 'v1': fatal: boom\n\
          next: Run tm rules inside a git repository to use --base.\n"),
        ("blob missing", Base::NoFile, Some(ONE_RULE), 2, "",
         "tm: no rule file temperare.toml at base 'v1'\n\
          next: Pass a --base that has temperare.toml, or leave --loosened out.\n"),
        ("not UTF-8", Base::NotUtf8, Some(ONE_RULE), 2, "",
         "tm: temperare.toml at base 'v1' is not UTF-8 text\n\
          next: Pass a --base whose temperare.toml is UTF-8 text, or leave --loosened out.\n"),
        ("base problems", Base::Text("version = 2\n"), None, 2,
         "/repo/temperare.toml (at v1):1:11  `version` must be 1, not 2  Write `version = 1`.\n\
          next: fix each problem listed for /repo/temperare.toml (at v1)\n", ""),
        ("now problems", Base::Text(ONE_RULE), None, 2,
         "/repo/temperare.toml:2:1  unknown key  Remove the key.\n\
          next: fix each problem listed for /repo/temperare.toml\n", ""),
        ("something loosened", Base::Text(ONE_RULE), Some("version = 1\n"), 1,
         "rule file: /repo/temperare.toml\nbase: v1\nloosened (1):\n  rule_gone  lint-all\n\
          next: have each loosening above approved, or undo it in /repo/temperare.toml\n", ""),
        ("nothing loosened", Base::Text(ONE_RULE), Some(ONE_RULE), 0,
         "rule file: /repo/temperare.toml\nbase: v1\nloosened (0):\nnext: nothing to do\n", ""),
    ];
    for (name, base, now, exit, out, err) in rows {
        assert_eq!(loosened(base, now), (exit, out.to_string(), err.to_string()), "{name}");
    }
}
