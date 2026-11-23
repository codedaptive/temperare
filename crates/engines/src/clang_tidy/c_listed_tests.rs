//! Unit tests for c_listed.rs: a clang-tidy rule is `NoFiles` with no listed C source in its paths; a source in its paths,
//! a listing error, no folder and another engine give None, the last two without probing.

use super::c_listed;
use crate::types::Stop;
use rules::RuleFile;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[test]
fn c_listed_cases() {
    let no_files = || Some(Stop::NoFiles { why: "no file its engine checks".into() });
    let failed = Stop::Failed { code: Some(128), stderr: "fatal: not a git repository".into() };
    let rows: [(&str, &str, &str, Option<&str>, Result<Vec<&str>, Stop>, Option<Stop>, Vec<&str>); 6] = [
        ("other engine", "ruff", "**", Some("."), Ok(vec![]), None, vec![]),
        ("no folder", "clang-tidy", "**", None, Ok(vec![]), None, vec![]),
        ("source in paths", "clang-tidy", "src/**", Some("."), Ok(vec!["src/a.c"]), None, vec!["."]),
        ("no source", "clang-tidy", "**", Some("."), Ok(vec![]), no_files(), vec!["."]),
        ("source outside paths", "clang-tidy", "lib/**", Some("."), Ok(vec!["src/a.c"]), no_files(), vec!["."]),
        ("listing error", "clang-tidy", "**", Some("."), Err(failed), None, vec!["."]),
    ];
    for (label, engine, paths, folder, listed, expected, probes) in rows {
        let text = format!(
            "version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"all\"\nfix = \"f\"\npaths = [\"{paths}\"]\n"
        );
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let seen = RefCell::new(Vec::new());
        let probe = |folder: &Path| {
            seen.borrow_mut().push(folder.to_path_buf());
            listed.clone().map(|files| files.into_iter().map(PathBuf::from).collect())
        };
        assert_eq!(c_listed(&rule, folder.map(Path::new), &probe), expected, "{label}");
        let probes: Vec<PathBuf> = probes.into_iter().map(PathBuf::from).collect();
        assert_eq!(seen.into_inner(), probes, "{label}: probe calls");
    }
}
