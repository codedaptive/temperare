//! `Pick`: how a rule's `check` reaches its engine.

/// How `entry_cmd` passes the check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pick {
    /// One argument, the flag and the check joined: `{flag}{check}`. The flag carries its own `=` (`"--select="`),
    /// so a check beginning with `-` is read as the flag's value, never as an option.
    Arg(&'static str),
    /// Two arguments: the option, then the path of cctao's filled copy of the check file (ast-grep's `"--rule"`).
    File(&'static str),
    /// The check is not passed.
    None,
}
