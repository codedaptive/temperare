//! `ruff_cmd`: the ruff run for one rule over a folder.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::path::Path;

/// `ruff check --no-cache --output-format json --select=<check>`, then `--config=<config>` when given, then `.`,
/// run in `folder`. `--no-cache` leaves no `.ruff_cache/` in the checked folder. `config` is passed as
/// written (relative to `folder`); it is never read or copied here. Each rule value is joined to its option with `=`
/// so a value beginning with `-` is read as the value, never as an option (ruff answers `--select=--fix` with
/// "Unknown rule selector").
pub fn ruff_cmd(check: &str, config: Option<&str>, folder: &Path) -> Cmd {
    let mut args = vec![
        "check".to_string(),
        "--no-cache".to_string(),
        "--output-format".to_string(),
        "json".to_string(),
        format!("--select={check}"),
    ];
    args.extend(config.map(|config| format!("--config={config}")));
    args.push(".".to_string());
    Cmd {
        args: args.into_iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program(Engine::Ruff))
    }
}
