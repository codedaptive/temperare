//! `ruff_cmd`: the ruff run for one rule over a folder.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::path::Path;

/// `ruff check --no-cache --output-format json --select <check>`, then `--config <config>` when given, then `.`,
/// run in `folder`. `--no-cache` leaves no `.ruff_cache/` in the checked folder (settled S-21). `config` is passed as
/// written (relative to `folder`); it is never read or copied here.
pub fn ruff_cmd(check: &str, config: Option<&str>, folder: &Path) -> Cmd {
    let mut args = vec!["check", "--no-cache", "--output-format", "json", "--select", check];
    if let Some(config) = config {
        args.extend(["--config", config]);
    }
    args.push(".");
    Cmd {
        args: args.into_iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program(Engine::Ruff))
    }
}
