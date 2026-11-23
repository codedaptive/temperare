//! Unit tests for clang_tidy_cmd.rs: the argument lists built for an entry with the clang-tidy entry's field values, with and without a config. Nothing runs.

use super::{clang_tidy_cmd, parse_clang_tidy};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const ENTRY: Entry = Entry {
    name: "clang-tidy",
    engine: Engine::ClangTidy,
    program: "clang-tidy",
    needs: &["clang-tidy"],
    never: None,
    args: &["--quiet", "--warnings-as-errors=-*"],
    pick: Pick::Arg("--checks=-*,"),
    params: Params::Args(&[]),
    config: Some("--config-file="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: |_| None,
    unknown: |_, _| None,
    read: parse_clang_tidy,
    run: run_plain,
};

#[test]
fn clang_tidy_cmd_cases() {
    let files = [PathBuf::from("a.c"), PathBuf::from("-dash/b.cpp")];
    let rows: [(&str, Option<&str>, &[&str]); 2] = [
        ("no config", None, &["--quiet", "--warnings-as-errors=-*", "--checks=-*,bugprone-branch-clone", "--config={}", "/w/a.c", "/w/-dash/b.cpp"]),
        ("a config", Some("tidy.yml"), &["--quiet", "--warnings-as-errors=-*", "--checks=-*,bugprone-branch-clone", "--config-file=tidy.yml", "/w/a.c", "/w/-dash/b.cpp"]),
    ];
    for (label, config, expected) in rows {
        let cmd = clang_tidy_cmd(&ENTRY, "bugprone-branch-clone", config, &files, Path::new("/w"));
        assert_eq!(cmd.program, OsString::from("clang-tidy"), "{label}");
        assert_eq!(cmd.cwd, Some(PathBuf::from("/w")), "{label}");
        assert_eq!(cmd.args, expected.iter().map(OsString::from).collect::<Vec<_>>(), "{label}");
    }
}
