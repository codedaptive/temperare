//! ZUT for ast_grep_cmd.rs: the program, arguments and working folder of the scan command.

use std::path::{Path, PathBuf};

use super::ast_grep_cmd;

#[test]
fn the_command_scans_with_the_rule_file_in_stream_json_from_the_folder() {
    let cmd = ast_grep_cmd(Path::new("/tmp/tm-ast-grep-1-0.yml"), Path::new("/w/repo"));
    assert_eq!(cmd.program, "ast-grep");
    assert_eq!(cmd.args, ["scan", "--rule", "/tmp/tm-ast-grep-1-0.yml", "--json=stream"]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/repo")));
}

#[test]
fn the_command_sets_nothing_else() {
    let cmd = ast_grep_cmd(Path::new("r.yml"), Path::new("/w"));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}
