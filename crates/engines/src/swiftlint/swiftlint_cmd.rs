//! `swiftlint_cmd`: the swiftlint run for one rule over a folder.

use crate::program::program;
use procio::Cmd;
use rules::Engine;
use std::path::Path;

/// `swiftlint lint --reporter json --quiet --only-rule <check>`, then `--config <config>` when given, run in
/// `folder` so swiftlint lints that folder. `config` is passed as written (relative to `folder`); it is never read
/// or copied here.
pub fn swiftlint_cmd(check: &str, config: Option<&str>, folder: &Path) -> Cmd {
    let mut args = vec!["lint", "--reporter", "json", "--quiet", "--only-rule", check];
    if let Some(config) = config {
        args.extend(["--config", config]);
    }
    Cmd {
        args: args.into_iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new(program(Engine::SwiftLint))
    }
}
