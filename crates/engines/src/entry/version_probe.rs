//! `VersionProbe`: how an engine's version is read.

use procio::Output;

/// `<entry program> <args>` run in the rule file's folder, and `read` taking the version from its output
/// (`version_word` for an engine that prints `<name> <version>` or the bare version).
#[derive(Debug, Clone, Copy)]
pub struct VersionProbe {
    /// The arguments after the entry's program (`["--version"]`).
    pub args: &'static [&'static str],
    /// The version from the probe's output; `None` when it gives none.
    pub read: fn(&Output) -> Option<String>,
}
