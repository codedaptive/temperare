//! ZUT for no_script_listed: a listed ShellCheck rule with no script in its paths is `NoFiles`; a script in its
//! paths, a failed listing, no folder and another engine give None, the last two without listing.

use super::no_script_listed::no_script_listed;
use crate::types::Stop;
use rules::RuleFile;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[test]
fn no_script_listed_settles_only_a_listed_shellcheck_rule() {
    let failed = || Err(Stop::NotAvailable { why: "git failed".into() });
    let no_files = Some(Stop::NoFiles { why: "no file its engine checks".into() });
    let cases = [
        ("no script in paths", "shellcheck", "SC2086", Some("."), Ok(vec!["elsewhere/a.sh"]), no_files, vec!["."]),
        ("script in paths", "shellcheck", "SC2086", Some("."), Ok(vec!["docs/a.sh"]), None, vec!["."]),
        ("listing fails", "shellcheck", "SC2086", Some("."), failed(), None, vec!["."]),
        ("no folder", "shellcheck", "SC2086", None, Ok(vec![]), None, vec![]),
        ("other engine", "ruff", "E711", Some("."), Ok(vec![]), None, vec![]),
    ];
    for (name, engine, check, folder, listing, want, calls) in cases {
        let text = format!(
            "version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\npaths = [\"docs/**\"]\n"
        );
        let rule = RuleFile::parse(&text, Path::new(".")).unwrap().rules.remove(0);
        let seen = RefCell::new(Vec::new());
        let lister = |folder: &Path| {
            seen.borrow_mut().push(folder.to_path_buf());
            listing.clone().map(|scripts| scripts.into_iter().map(PathBuf::from).collect())
        };
        assert_eq!(no_script_listed(&rule, folder.map(Path::new), &lister), want, "{name}");
        let calls: Vec<PathBuf> = calls.into_iter().map(PathBuf::from).collect();
        assert_eq!(seen.into_inner(), calls, "{name}: lister calls");
    }
}
