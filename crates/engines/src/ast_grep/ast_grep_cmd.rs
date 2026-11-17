//! `ast_grep_cmd`: the `ast-grep scan` command for one rule file over one folder.

use std::path::Path;

use procio::Cmd;
use rules::Engine;

use crate::program::program;

/// `ast-grep scan --rule <rule_file> --json=stream`, run in `folder`: every match of the rule file's rules under
/// `folder`, one JSON object per line.
pub fn ast_grep_cmd(rule_file: &Path, folder: &Path) -> Cmd {
    Cmd {
        args: vec!["scan".into(), "--rule".into(), rule_file.into(), "--json=stream".into()],
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program(Engine::AstGrep))
    }
}
