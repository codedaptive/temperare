//! ZUT for engine_listed.rs: a rule of the given engine is `NoFiles` with no listed file in its paths; a file in its
//! paths, a listing error, no folder and a rule of another engine give None, the last two without probing.

use crate::engine_listed::engine_listed;
use crate::types::Stop;
use rules::{Engine, RuleFile};
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[test]
fn engine_listed_cases() {
    let no_files = || Some(Stop::NoFiles { why: "no file its engine checks".into() });
    let failed = Stop::Failed { code: Some(128), stderr: "fatal: not a git repository".into() };
    let yamllint = Engine::Yamllint;
    let rows: [(&str, Engine, &str, &str, Option<&str>, Result<Vec<&str>, Stop>, Option<Stop>, Vec<&str>); 8] = [
        ("other engine", yamllint, "ruff", "**", Some("."), Ok(vec![]), None, vec![]),
        ("no folder", yamllint, "yamllint", "**", None, Ok(vec![]), None, vec![]),
        ("file in paths", yamllint, "yamllint", "conf/**", Some("."), Ok(vec!["conf/a.yaml"]), None, vec!["."]),
        ("no file", yamllint, "yamllint", "**", Some("."), Ok(vec![]), no_files(), vec!["."]),
        ("file outside paths", yamllint, "yamllint", "ci/**", Some("."), Ok(vec!["conf/a.yaml"]), no_files(), vec!["."]),
        ("listing error", yamllint, "yamllint", "**", Some("."), Err(failed), None, vec!["."]),
        ("shellcheck rule, yamllint asked", yamllint, "shellcheck", "**", Some("."), Ok(vec![]), None, vec![]),
        ("shellcheck no file", Engine::ShellCheck, "shellcheck", "**", Some("."), Ok(vec![]), no_files(), vec!["."]),
    ];
    for (label, asked, engine, paths, folder, listed, expected, probes) in rows {
        let text = format!(
            "version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"truthy\"\nfix = \"f\"\npaths = [\"{paths}\"]\n"
        );
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let seen = RefCell::new(Vec::new());
        let probe = |folder: &Path| {
            seen.borrow_mut().push(folder.to_path_buf());
            listed.clone().map(|files| files.into_iter().map(PathBuf::from).collect())
        };
        assert_eq!(engine_listed(asked, &rule, folder.map(Path::new), &probe), expected, "{label}");
        let probes: Vec<PathBuf> = probes.into_iter().map(PathBuf::from).collect();
        assert_eq!(seen.into_inner(), probes, "{label}: probe calls");
    }
}
