//! ZUT for vale_wrapper.rs: the exact wrapper text, with and without StylesPath and Vocab; a value with a line break
//! or a StylesPath that is not UTF-8 (on unix only) is Err naming it.

use super::{vale_wrapper, ValeConfig};
use std::path::PathBuf;

#[test]
fn vale_wrapper_cases() {
    let config = |paths: Vec<PathBuf>, vocab: &[&str]| ValeConfig {
        styles: vec!["Vale".to_string(), "Other".to_string()],
        vocab: vocab.iter().map(|name| name.to_string()).collect(),
        paths,
        files: vec![PathBuf::from("/repo/.vale.ini")],
    };
    let styles = || vec![PathBuf::from("/repo/styles")];
    let rows: [(&str, ValeConfig, &str, Result<&str, &str>); 6] = [
        (
            "StylesPath and Vocab",
            config(styles(), &["Proj", "Base"]),
            "Vale",
            Ok("StylesPath = /repo/styles\nMinAlertLevel = suggestion\nVocab = Proj, Base\n\n[*]\nBasedOnStyles = Vale\n"),
        ),
        (
            "the first StylesPath only",
            config(vec![PathBuf::from("/repo/styles"), PathBuf::from("/home/u/styles")], &[]),
            "Other",
            Ok("StylesPath = /repo/styles\nMinAlertLevel = suggestion\n\n[*]\nBasedOnStyles = Other\n"),
        ),
        ("neither", config(vec![], &[]), "Vale", Ok("MinAlertLevel = suggestion\n\n[*]\nBasedOnStyles = Vale\n")),
        (
            "a Vocab name with a line break",
            config(styles(), &["Proj\nBlockIgnores = x"]),
            "Vale",
            Err("the Vale config value \"Proj\\nBlockIgnores = x\" holds a line break, so cctao cannot write it into a config"),
        ),
        (
            "a StylesPath with a carriage return",
            config(vec![PathBuf::from("/repo/sty\rles")], &[]),
            "Vale",
            Err("the Vale config value \"/repo/sty\\rles\" holds a line break, so cctao cannot write it into a config"),
        ),
        (
            "a style with a line break",
            config(styles(), &[]),
            "Vale\n",
            Err("the Vale config value \"Vale\\n\" holds a line break, so cctao cannot write it into a config"),
        ),
    ];
    for (label, config, style, expected) in rows {
        let expected = expected.map(str::to_string).map_err(str::to_string);
        assert_eq!(vale_wrapper(&config, style), expected, "{label}");
    }
}

// Unix only: the non-UTF-8 StylesPath is built from raw bytes with OsStrExt::from_bytes, which only unix has.
#[cfg(unix)]
#[test]
fn vale_wrapper_refuses_a_styles_path_not_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let config = ValeConfig {
        styles: vec!["Vale".to_string(), "Other".to_string()],
        vocab: Vec::new(),
        paths: vec![PathBuf::from(OsStr::from_bytes(b"/repo/st\xffyles"))],
        files: vec![PathBuf::from("/repo/.vale.ini")],
    };
    let expected = Err("the Vale StylesPath /repo/st\u{fffd}yles is not UTF-8, so cctao cannot write it into a config".to_string());
    assert_eq!(vale_wrapper(&config, "Vale"), expected, "a StylesPath not UTF-8");
}
