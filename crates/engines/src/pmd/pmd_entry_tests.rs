//! ZUT for pmd_entry.rs: the command `entry_cmd` builds from `PMD` for a check, and that PMD takes no params.
//! Nothing runs.

use super::PMD;
use crate::entry::{entry_cmd, Params};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn the_command_checks_the_rulesets_as_sarif_with_nopmd_off() {
    let rows: [(&str, Option<&str>); 2] = [("no config", None), ("a config, never passed", Some("ruleset.xml"))];
    for (name, config) in rows {
        let cmd = entry_cmd(&PMD, OsStr::new("category/java/bestpractices.xml/UnusedPrivateField"), &[], config, Path::new("/w/java"));
        let want = [
            "check",
            "--no-cache",
            "--no-progress",
            "--format=sarif",
            "--suppress-marker=TM-NOPMD-OFF",
            "--rulesets=category/java/bestpractices.xml/UnusedPrivateField",
        ]
        .map(OsString::from);
        assert_eq!(cmd.program, OsString::from("pmd"), "{name}");
        assert_eq!(cmd.args, want, "{name}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w/java")), "{name}");
    }
    assert!(matches!(PMD.params, Params::Args(args) if args.is_empty()), "{:?}", PMD.params);
}
