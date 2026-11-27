//! Unit test for config_stop.rs: a config file outside the repository (a `..` climbing out included), then a style no
//! BasedOnStyles names, are `NotAvailable`; a named style in a config inside the repository is None. On Windows a
//! plain `C:\...` config is inside a verbatim `\\?\C:\...` top.

use super::{config_stop, ValeConfig};
use crate::types::Stop;
use std::path::{Path, PathBuf};

fn config(styles: &[&str], files: &[&str]) -> ValeConfig {
    ValeConfig {
        styles: styles.iter().map(|style| style.to_string()).collect(),
        vocab: vec![],
        paths: vec![PathBuf::from("/repo/styles")],
        files: files.iter().map(PathBuf::from).collect(),
    }
}

fn outside(file: &str) -> Option<Stop> {
    Some(Stop::NotAvailable {
        why: format!(
            "Vale would read the config {file}, outside the repository: cctao runs Vale only with a .vale.ini, _vale.ini, vale.ini, .vale or _vale inside it, or a config the rule names inside it"
        ),
    })
}

#[test]
fn config_stop_cases() {
    let not_named = |root: &str, style: &str| {
        Some(Stop::NotAvailable {
            why: format!("no BasedOnStyles in {root} names the style `{style}`: cctao never turns on a Vale style the repository did not choose"),
        })
    };
    let rows = [
        ("named style", config(&["Vale", "Other"], &["/repo/.vale.ini"]), "Other.Rule", None),
        ("config in a subfolder", config(&["Vale"], &["/repo/docs/.vale.ini"]), "Vale.Spelling", None),
        ("config above the top", config(&["Vale"], &["/.vale.ini"]), "Vale.Spelling", outside("/.vale.ini")),
        ("a second config outside", config(&["Vale"], &["/repo/.vale.ini", "/home/u/.vale.ini"]), "Vale.Spelling", outside("/home/u/.vale.ini")),
        ("a sibling sharing the prefix", config(&["Vale"], &["/repo2/.vale.ini"]), "Vale.Spelling", outside("/repo2/.vale.ini")),
        ("a .. climbing out", config(&["Vale"], &["/repo/../other/.vale.ini"]), "Vale.Spelling", outside("/repo/../other/.vale.ini")),
        ("outside before style", config(&[], &["/other/.vale.ini"]), "Vale.Spelling", outside("/other/.vale.ini")),
        ("style not named", config(&["Vale"], &["/repo/.vale.ini"]), "Other.Rule", not_named("/repo/.vale.ini", "Other")),
        ("no style at all", config(&[], &["/repo/.vale.ini"]), "Vale.Spelling", not_named("/repo/.vale.ini", "Vale")),
        ("style only a prefix", config(&["Valerie"], &["/repo/.vale.ini"]), "Vale.Spelling", not_named("/repo/.vale.ini", "Vale")),
    ];
    for (label, config, check, expected) in rows {
        assert_eq!(config_stop(&config, check, Path::new("/repo")), expected, "{label}");
    }
}

#[cfg(windows)]
#[test]
fn config_stop_windows_cases() {
    let rows = [
        ("plain config under a verbatim top", config(&["Vale"], &[r"C:\repo\.vale.ini"]), None),
        ("plain config outside a verbatim top", config(&["Vale"], &[r"C:\other\.vale.ini"]), outside(r"C:\other\.vale.ini")),
    ];
    for (label, config, expected) in rows {
        assert_eq!(config_stop(&config, "Vale.Spelling", Path::new(r"\\?\C:\repo")), expected, "{label}");
    }
}
