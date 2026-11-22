//! `Finding::new`: a finding from its rule, place and message.

use super::{Engine, Finding, Kind, RelPath, Severity};

impl Finding {
    /// A finding with `owner` None; the caller sets the pub `owner` field when the rule file names one.
    ///
    /// `line` takes a number or an `Option<u32>` and is stored as given.
    ///
    /// Engine, kind and severity come as one tuple because they come together from the rule, and it keeps the
    /// constructor at seven arguments, the most clippy's `too_many_arguments` allows.
    pub fn new(
        rule: impl Into<String>,
        (engine, kind, severity): (Engine, Kind, Severity),
        path: RelPath,
        line: impl Into<Option<u32>>,
        col: Option<u32>,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Finding {
        Finding {
            rule: rule.into(),
            engine,
            kind,
            severity,
            owner: None,
            path,
            line: line.into(),
            col,
            what: what.into(),
            fix: fix.into(),
        }
    }
}
