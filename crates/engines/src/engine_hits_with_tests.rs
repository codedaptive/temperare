//! ZUT for engine_hits_with: ShellCheck's script scope settles before availability.

use crate::engine_hits_with::engine_hits_with;
use crate::host::Host;
use crate::types::Stop;
use procio::Cmd;
use rules::RuleFile;
use std::cell::Cell;
use std::path::{Path, PathBuf};

#[test]
fn scripts_settle_before_shellcheck_availability_and_other_engines_skip_the_lister() {
    let cases = [
        ("empty", "shellcheck", "SC2086", "", Some(vec![]), false, "no file its engine checks"),
        ("outside", "shellcheck", "SC2086", "paths = [\"src/**\"]", Some(vec!["docs/x.sh"]), false, "no file its engine checks"),
        ("absent shellcheck", "shellcheck", "SC2086", "", Some(vec!["src/a.sh"]), true, "shellcheck"),
        ("absent ruff", "ruff", "E711", "", None, true, "ruff"),
    ];
    // The last column is the `NoFiles` why when nothing is looked up, else the program `NotOnPath` names.
    for (name, engine, check, scope, scripts, lookup, why) in cases {
        let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{scope}\n");
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let on_path = |program: &str| {
            assert!(lookup, "{name}: unexpected lookup of {program}");
            assert_eq!(program, engine, "{name}");
            false
        };
        let run = |cmd: &Cmd| panic!("{name}: nothing may run: {cmd:?}");
        let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
        let list = |_: &Path| Ok(scripts.as_ref().expect("lister must not run").iter().map(|path| PathBuf::from(*path)).collect());
        let want = if lookup {
            Stop::NotOnPath { program: why.into() }
        } else {
            Stop::NoFiles { why: why.into() }
        };
        assert_eq!(engine_hits_with(&rule, Ok(Path::new(".")), &host, &list), Err(want), "{name}");
    }
}
