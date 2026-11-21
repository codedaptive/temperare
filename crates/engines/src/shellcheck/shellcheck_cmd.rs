//! `shellcheck_cmd`: the shellcheck run for one rule, as a command.

use procio::Cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// `shellcheck -f json1 --enable=all <config> -- <files>` run in `cwd`. `<config>` is `--rcfile=<config>` as one
/// argument when the rule gives one, else `--norc`: the rule file is the only home for a rule and its exceptions, so a
/// `disable=` in a `.shellcheckrc` tm was not given must not silence a rule. `SHELLCHECK_OPTS` is removed from the
/// environment, since shellcheck prepends it to its arguments and an `--exclude` or `--include` there could silence
/// the rule too. The rule's check is not passed: on shellcheck 0.11.0 `--include` hides the optional checks (they
/// need `--enable`) and the SC1xxx parse failures of a script it could not parse, so every check runs and
/// `parse_shellcheck` keeps the rule's. `--` ends the options, so a file name beginning with `-` is read as a file;
/// a file named exactly `-` is passed as `./-`, because shellcheck reads a bare `-` as standard input even after
/// `--`, and that input is empty here.
pub fn shellcheck_cmd(config: Option<&str>, files: &[PathBuf], cwd: &Path) -> Cmd {
    let mut args: Vec<OsString> = ["-f", "json1", "--enable=all"].into_iter().map(OsString::from).collect();
    args.push(config.map_or_else(|| OsString::from("--norc"), |config| OsString::from(format!("--rcfile={config}"))));
    args.push(OsString::from("--"));
    args.extend(files.iter().map(|file| if file == Path::new("-") { OsString::from("./-") } else { file.into() }));
    Cmd {
        args,
        cwd: Some(cwd.to_path_buf()),
        env_remove: vec![OsString::from("SHELLCHECK_OPTS")],
        ..Cmd::new("shellcheck")
    }
}
