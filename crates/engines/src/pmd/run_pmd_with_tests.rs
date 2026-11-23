//! ZUT for run_pmd_with.rs: `run_pmd_with(.., &PMD, ..)` with a fake lister and a host that records the command and
//! the file list it names, answering with PMD 7.29.0's real violation log (fixtures/violation.sarif). Nothing runs;
//! the only disk touched is the file list, written under the temporary folder and removed after the run.

use super::{run_pmd_with, PMD};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const DIR: &str = "/private/tmp/tm-pmd-capture/data/viol";
const FIXED: [&str; 6] = [
    "check",
    "--no-cache",
    "--no-progress",
    "--format=sarif",
    "--suppress-marker=TM-NOPMD-OFF",
    "--rulesets=category/java/bestpractices.xml/UnusedPrivateField",
];

#[test]
fn run_pmd_with_hands_pmd_the_files_the_rule_applies_to() {
    let hit = Hit { path: RelPath::new("Foo.java"), line: Some(2), col: Some(17), what: "Avoid unused private fields such as 'unused'.".to_string() };
    let failed = Stop::Failed { code: Some(128), stderr: "fatal: not a git repository".into() };
    let no_files = Stop::NoFiles { why: "no file its engine checks".into() };
    let comma = Stop::Failed {
        code: None,
        stderr: "a,b/Foo.java: pmd reads ',' in --file-list as a separator, so tm cannot hand it this file".into(),
    };
    let rows: [(&str, Result<Vec<&str>, Stop>, Result<Vec<Hit>, Stop>, Option<&str>); 5] = [
        ("one file applies", Ok(vec!["lib/Bar.java", "Foo.java"]), Ok(vec![hit.clone()]), Some("Foo.java\n")),
        ("two files apply", Ok(vec!["Foo.java", "src/Foo.java"]), Ok(vec![hit]), Some("Foo.java\nsrc/Foo.java\n")),
        ("none applies", Ok(vec!["lib/Bar.java"]), Err(no_files), None),
        ("a kept path holds a comma", Ok(vec!["Foo.java", "a,b/Foo.java", "c,d/Foo.java"]), Err(comma), None),
        ("the listing fails", Err(failed.clone()), Err(failed), None),
    ];
    for (name, listed, want, list) in rows {
        let text = "version = 1\n[[rule]]\nid = \"f\"\nengine = \"pmd\"\ncheck = \"category/java/bestpractices.xml/UnusedPrivateField\"\nfix = \"f\"\npaths = [\"**/Foo.java\"]\n";
        let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
        let seen = RefCell::new(None);
        let run = |cmd: &Cmd| {
            let (fixed, last) = cmd.args.split_at(cmd.args.len() - 1);
            assert_eq!(fixed, FIXED.map(OsString::from), "{name}");
            let path = PathBuf::from(last[0].to_str().unwrap().strip_prefix("--file-list=").unwrap());
            *seen.borrow_mut() = Some((std::fs::read_to_string(&path).unwrap(), path, cmd.env_remove.clone()));
            Ok(Output { code: Some(4), stdout: include_bytes!("fixtures/violation.sarif").to_vec(), stderr: Vec::new() })
        };
        let on_path = |_: &str| -> bool { panic!("run_pmd_with looks nothing up on PATH") };
        let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
        let lister = |folder: &Path| {
            assert_eq!(folder, Path::new(DIR), "{name}");
            listed.clone().map(|files| files.into_iter().map(PathBuf::from).collect())
        };
        assert_eq!(run_pmd_with(&lister, &PMD, &file.rules[0], Path::new(DIR), &host), want, "{name}");
        match (seen.into_inner(), list) {
            (Some((lines, path, removed)), Some(list)) => {
                assert_eq!(lines, list, "{name}");
                assert_eq!(removed, [OsString::from("CLASSPATH")], "{name}");
                assert!(!path.exists(), "{name}: the file list outlived the run");
            }
            (None, None) => {}
            (ran, _) => panic!("{name}: ran {ran:?}"),
        }
    }
}
