//! Unit test for sqlfluff_cmd.rs: the argument lists and environment built for `SQLFLUFF`, with and without a config.
//! Nothing runs.

use super::{sqlfluff_cmd, SQLFLUFF};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn sqlfluff_cmd_cases() {
    let files = [PathBuf::from("q.sql"), PathBuf::from("-x.sql"), PathBuf::from("db/m.sql.j2")];
    let lead = [
        "lint", "--format", "json", "--nocolor", "--disable-progress-bar", "--disable-noqa", "--disregard-sqlfluffignores",
        "--exclude-rules=", "--ignore=", "--rules=LT01",
    ];
    let rows: [(&str, Option<&str>, &[&str]); 2] = [
        ("no config", None, &["--", "q.sql", "-x.sql", "db/m.sql.j2"]),
        ("a config", Some("cfg/extra.cfg"), &["--config=cfg/extra.cfg", "--", "q.sql", "-x.sql", "db/m.sql.j2"]),
    ];
    for (label, config, rest) in rows {
        let cmd = sqlfluff_cmd(&SQLFLUFF, "LT01", config, &files, Path::new("/w/db"), Path::new("/w"));
        assert_eq!(cmd.program, OsString::from("sqlfluff"), "{label}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w/db")), "{label}");
        let expected: Vec<OsString> = lead.iter().chain(rest.iter()).map(OsString::from).collect();
        assert_eq!(cmd.args, expected, "{label}");
        assert_eq!(cmd.env, [(OsString::from("HOME"), OsString::from("/w"))], "{label}");
        assert_eq!(cmd.env_remove, ["XDG_CONFIG_HOME", "PYTHONPATH", "PYTHONUSERBASE"].map(OsString::from), "{label}");
    }
}
