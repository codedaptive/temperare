//! Unit test for no_file_listed: a Ruff rule does not request a script listing.

use crate::no_file_listed::no_file_listed;
use crate::types::Stop;
use rules::RuleFile;
use std::cell::Cell;
use std::path::Path;

#[test]
fn no_file_listed_cases() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"SC2086\"\nfix = \"f\"\npaths = [\"docs/**\"]\n";
    let rule = RuleFile::parse(text, Path::new(".")).unwrap().rules.remove(0);
    let called = Cell::new(false);
    let scripts = |_: &Path| {
        called.set(true);
        Err(Stop::Failed { code: None, stderr: "listing failed".into() })
    };
    assert_eq!(no_file_listed(&rule, Some(Path::new("/repo")), &scripts), None);
    assert!(!called.get(), "Ruff must not list scripts");
}
