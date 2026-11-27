//! `Reach`: how a threshold's setting reaches its engine.

/// Where `threshold_run` puts a threshold's `setting(n)` for the check run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
    /// One argument `{flag}{setting}`; the flag carries its own `=` (ruff: `"--config="`).
    Arg(&'static str),
    /// The setting is written to a temporary file `cctao-<entry name>-<pid>-<n>.<ext>`, passed as one argument
    /// `{option}{path}`. `own` is the file name of the engine's own config in the rule file's folder
    /// (".swiftlint.yml"): when the rule gives no config and that file is there, it is passed first as
    /// `{option}{folder/own}`, since an explicit option makes the engine skip it.
    File { option: &'static str, ext: &'static str, own: Option<&'static str> },
    /// The setting is written as `file` inside a new temporary folder, whose path is set in the environment
    /// variable `env` for the run (clippy: `CLIPPY_CONF_DIR`, `clippy.toml`).
    Dir { env: &'static str, file: &'static str },
}
