//! `VersionProbe`: how an engine's version is read.

use procio::{Cmd, Output};

/// The program `program_for` gives for the rule file's folder (ESLint's executable local `eslint` when
/// there is one, else the entry's program) with `args`, run in that folder, with `wrap` applied to the command when
/// there is one, and `read` taking the version from its output (`version_word` for an engine that prints
/// `<name> <version>` or the bare version).
#[derive(Debug, Clone, Copy)]
pub struct VersionProbe {
    /// The arguments after the entry's program (`["--version"]`).
    pub args: &'static [&'static str],
    /// The version from the probe's output; `None` when it gives none.
    pub read: fn(&Output) -> Option<String>,
    /// The probe command given the same environment the entry's run gives its check run; `None`: the probe runs
    /// with cctao's environment as is.
    pub wrap: Option<fn(Cmd) -> Cmd>,
}
