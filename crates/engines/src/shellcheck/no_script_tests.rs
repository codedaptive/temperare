//! ZUT for no_script: only scripts in the rule's scope count.

use super::no_script::no_script;
use crate::types::Stop;
use rules::RuleFile;
use std::path::{Path, PathBuf};

#[test]
fn scripts_in_scope_are_not_stopped() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"shellcheck\"\ncheck = \"SC2086\"\nfix = \"f\"\npaths = [\"src/**\"]\n";
    let rule = RuleFile::parse(text, Path::new(".")).unwrap().rules.remove(0);
    let cases = [
        ("in scope", vec!["src/a.sh"], None),
        ("outside scope", vec!["docs/x.sh"], Some(Stop::NoFiles { why: "no file its engine checks".into() })),
        ("empty", vec![], Some(Stop::NoFiles { why: "no file its engine checks".into() })),
    ];
    for (name, scripts, want) in cases {
        let scripts: Vec<PathBuf> = scripts.into_iter().map(PathBuf::from).collect();
        assert_eq!(no_script(&rule, &scripts), want, "{name}");
    }
}
