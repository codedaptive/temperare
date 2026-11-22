//! ZUT for lock_listed.rs: a cargo-audit rule is `NoFiles` without a Cargo.lock in its folder or in its paths; a
//! present lock in its paths, no folder and another engine give None, the last two without probing.

use super::lock_listed;
use crate::types::Stop;
use rules::RuleFile;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[test]
fn lock_listed_cases() {
    let no_files = || Some(Stop::NoFiles { why: "no file its engine checks".into() });
    let rows: [(&str, &str, &str, Option<&str>, bool, Option<Stop>, Vec<&str>); 5] = [
        ("other engine", "ruff", "**", Some("."), true, None, vec![]),
        ("no folder", "cargo-audit", "**", None, true, None, vec![]),
        ("lock present, in paths", "cargo-audit", "Cargo.lock", Some("."), true, None, vec!["."]),
        ("lock absent", "cargo-audit", "**", Some("."), false, no_files(), vec!["."]),
        ("lock present, outside paths", "cargo-audit", "src/**", Some("."), true, no_files(), vec!["."]),
    ];
    for (label, engine, paths, folder, present, expected, probes) in rows {
        let text = format!(
            "version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"all\"\nfix = \"f\"\npaths = [\"{paths}\"]\n"
        );
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let seen = RefCell::new(Vec::new());
        let probe = |folder: &Path| {
            seen.borrow_mut().push(folder.to_path_buf());
            present
        };
        assert_eq!(lock_listed(&rule, folder.map(Path::new), &probe), expected, "{label}");
        let probes: Vec<PathBuf> = probes.into_iter().map(PathBuf::from).collect();
        assert_eq!(seen.into_inner(), probes, "{label}: probe calls");
    }
}
