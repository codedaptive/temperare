use std::ffi::{OsStr, OsString};

use crate::names_to_try::names_to_try;

#[test]
fn names_to_try_cases() {
    let cases: &[(&str, &str, Option<&str>, &[&str])] = &[
        ("no extension rules", "tool", None, &["tool"]),
        ("colon without extension rules", "C:tool", None, &["C:tool"]),
        ("PATHEXT order", "tool", Some(".CMD;.EXE"), &["tool.CMD", "tool.EXE"]),
        ("lowercase extension", "git.exe", Some(".COM;.EXE"), &["git.exe"]),
        ("mixed-case extension", "GIT.Exe", Some(".COM;.EXE"), &["GIT.Exe"]),
        ("empty items", "tool", Some(";;.EXE;"), &["tool.EXE"]),
        ("dotless and lone dot", "tool", Some("EXE;.;.BAT"), &["tool.BAT"]),
        (
            "empty PATHEXT",
            "tool",
            Some(""),
            &["tool.COM", "tool.EXE", "tool.BAT", "tool.CMD"],
        ),
        (
            "all items skipped",
            "tool",
            Some("EXE;BAT"),
            &["tool.COM", "tool.EXE", "tool.BAT", "tool.CMD"],
        ),
        ("colon with extension rules", "C:tool", Some(".EXE"), &[]),
    ];

    for (case, name, pathext, expected) in cases {
        let actual = names_to_try(OsStr::new(name), pathext.map(OsStr::new));
        let expected: Vec<OsString> = expected.iter().map(|s| OsString::from(*s)).collect();
        assert_eq!(actual, expected, "{case}");
    }
}
