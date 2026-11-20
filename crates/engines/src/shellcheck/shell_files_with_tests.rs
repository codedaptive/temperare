//! ZUT for shell_files_with.rs with a fake host and a fake head reader: git's tracked listing read into the scripts
//! kept, which paths are read, the command, git's failure. Nothing runs and no file is read.
//!
//! ls_files_tracked.bin: stdout of `git ls-files -z -t --cached --deleted` (git 2.56.0) in a throwaway repository
//! tracking .gitignore, a.sh, d.sh (deleted from disk), notes (`plain text`) and run (first line
//! `#!/usr/bin/env bash`), with an untracked u.sh that the tracked listing leaves out, exit 0.

use super::shell_files_with;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/no/such/engines-zut-folder";

/// What one run gives: the result, the commands run, the paths the head reader was asked for, `raw_chars` after.
struct Ran {
    got: Result<Vec<PathBuf>, Stop>,
    cmds: Vec<Cmd>,
    asked: Vec<PathBuf>,
    raw_chars: Option<usize>,
}

/// `shell_files_with` with git answering `answer` and the head reader giving `heads[name]` for `FOLDER/name`.
fn run(answer: Output, heads: &[(&str, &[u8])]) -> Ran {
    let cmds = RefCell::new(Vec::new());
    let host_run = |cmd: &Cmd| -> Result<Output, ProcError> {
        cmds.borrow_mut().push(cmd.clone());
        Ok(answer.clone())
    };
    let heads: HashMap<PathBuf, Vec<u8>> = heads.iter().map(|(name, bytes)| (Path::new(FOLDER).join(name), bytes.to_vec())).collect();
    let asked = RefCell::new(Vec::new());
    let head = |path: &Path| -> Option<Vec<u8>> {
        asked.borrow_mut().push(path.to_path_buf());
        heads.get(path).cloned()
    };
    let raw_chars = Cell::new(None);
    let got = shell_files_with(Path::new(FOLDER), &Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars }, &head);
    Ran { got, cmds: cmds.into_inner(), asked: asked.into_inner(), raw_chars: raw_chars.get() }
}

fn answer(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn tracked() -> Output {
    answer(0, include_bytes!("fixtures/ls_files_tracked.bin"), b"")
}

fn paths(names: &[&str]) -> Vec<PathBuf> {
    names.iter().map(PathBuf::from).collect()
}

const GITIGNORE: (&str, &[u8]) = (".gitignore", b"i.sh\n");
const NOTES: (&str, &[u8]) = ("notes", b"plain text\n");

#[test]
fn a_script_by_extension_and_one_by_shebang_are_kept_and_a_deleted_one_is_not() {
    let ran = run(tracked(), &[GITIGNORE, NOTES, ("run", b"#!/usr/bin/env bash\necho run\n")]);
    assert_eq!(ran.got, Ok(paths(&["a.sh", "run"])));
    assert_eq!(ran.raw_chars, None);
}

#[test]
fn only_the_extensionless_paths_are_read() {
    let ran = run(tracked(), &[GITIGNORE, NOTES, ("run", b"#!/usr/bin/env bash\necho run\n")]);
    let folder = Path::new(FOLDER);
    assert_eq!(ran.asked, vec![folder.join(".gitignore"), folder.join("notes"), folder.join("run")]);
}

#[test]
fn a_file_that_cannot_be_read_is_left_out() {
    let ran = run(tracked(), &[GITIGNORE, NOTES]);
    assert_eq!(ran.got, Ok(paths(&["a.sh"])));
}

#[test]
fn a_shebang_of_another_shell_is_left_out() {
    let ran = run(tracked(), &[GITIGNORE, NOTES, ("run", b"#!/bin/zsh\n")]);
    assert_eq!(ran.got, Ok(paths(&["a.sh"])));
}

#[test]
fn git_lists_the_tracked_files_in_the_folder() {
    let ran = run(answer(0, b"", b""), &[]);
    assert_eq!(ran.cmds.len(), 1);
    assert_eq!(ran.cmds[0].program, OsString::from("git"));
    assert_eq!(ran.cmds[0].args, ["ls-files", "-z", "-t", "--cached", "--deleted"].map(OsString::from).to_vec());
    assert_eq!(ran.cmds[0].cwd, Some(PathBuf::from(FOLDER)));
}

#[test]
fn a_failed_git_is_failed_with_its_exit_code_and_stderr() {
    let ran = run(answer(128, b"", b"fatal: not a git repository\n"), &[]);
    assert_eq!(ran.got, Err(Stop::Failed { code: Some(128), stderr: "fatal: not a git repository\n".to_string() }));
    assert_eq!(ran.raw_chars, None);
}

#[test]
fn a_failed_git_with_no_stderr_says_so() {
    let ran = run(answer(1, b"", b""), &[]);
    assert_eq!(ran.got, Err(Stop::Failed { code: Some(1), stderr: "git ls-files printed nothing on stderr".to_string() }));
}

#[test]
fn no_files_gives_none() {
    let ran = run(answer(0, b"", b""), &[]);
    assert_eq!(ran.got, Ok(vec![]));
}

#[test]
fn only_the_shell_extensions_are_kept() {
    let stdout = b"H a.bash\0H b.ksh\0H c.dash\0H d.shx\0H e.py\0H sub/f.sh\0";
    let ran = run(answer(0, stdout, b""), &[]);
    assert_eq!(ran.got, Ok(paths(&["a.bash", "b.ksh", "c.dash", "sub/f.sh"])));
    assert_eq!(ran.asked, Vec::<PathBuf>::new());
}

#[test]
fn a_file_named_sh_with_no_shebang_is_left_out() {
    let ran = run(answer(0, b"H sh\0", b""), &[("sh", b"echo hi\n")]);
    assert_eq!(ran.got, Ok(vec![]));
}

#[cfg(unix)]
#[test]
fn a_name_that_is_not_utf8_reaches_the_result_byte_for_byte() {
    use std::os::unix::ffi::OsStrExt;
    let ran = run(answer(0, b"H caf\xe9.sh\0", b""), &[]);
    assert_eq!(ran.got, Ok(vec![PathBuf::from(std::ffi::OsStr::from_bytes(b"caf\xe9.sh"))]));
}

#[test]
fn untracked_and_other_tags_are_not_kept() {
    let ran = run(answer(0, b"? u.sh\0S skip.sh\0M merge.sh\0C changed.sh\0K killed.sh\0", b""), &[]);
    assert_eq!(ran.got, Ok(vec![]));
}
