//! ZUT for mypy_cmd.rs: the argument lists built for `MYPY`, with and without a config. Nothing runs.

use super::{mypy_cmd, MYPY};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn mypy_cmd_cases() {
    let files = [PathBuf::from("a.py"), PathBuf::from("-x.py"), PathBuf::from("pkg/b.pyi")];
    let rows: [(&str, Option<&str>, &[&str]); 2] = [
        ("no config", None, &["--output=json", "--cache-dir=/dev/null", "--enable-error-code=arg-type", "--config-file=", "--", "a.py", "-x.py", "pkg/b.pyi"]),
        ("a config", Some("/w/mypy.ini"), &["--output=json", "--cache-dir=/dev/null", "--enable-error-code=arg-type", "--config-file=/w/mypy.ini", "--", "a.py", "-x.py", "pkg/b.pyi"]),
    ];
    for (label, config, expected) in rows {
        let cmd = mypy_cmd(&MYPY, "arg-type", config.map(Path::new), &files, Path::new("/w"));
        assert_eq!(cmd.program, OsString::from("mypy"), "{label}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w")), "{label}");
        assert_eq!(cmd.args, expected.iter().map(OsString::from).collect::<Vec<_>>(), "{label}");
    }
}
