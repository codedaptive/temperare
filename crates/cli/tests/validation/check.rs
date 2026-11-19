//! `Check`: one acceptance case's part of the `make accept` report (S-31, SPEC-accept WO-orc-accept-1). A check
//! records what it set up and what `temperare` showed it, before it asserts (`Check::new`, `saw_tm`, `not_run`). When it is
//! dropped it writes its part into the folder `scripts/accept.sh` names in ACCEPT_PARTS, ending `Result: PASS`,
//! `Result: FAIL` when the test panicked (its output on the terminal says why), or `Result: NOT RUN (<why>)`.
//! Without ACCEPT_PARTS (the validation run after a publish) it writes nothing and starts nothing.

use std::path::PathBuf;

pub struct Check {
    /// The file this case's part is written to; None when ACCEPT_PARTS is not set.
    pub(crate) part: Option<PathBuf>,
    /// The case's name, as checks.txt lists it.
    pub(crate) case: &'static str,
    /// What the case set up, in one line.
    pub(crate) setup: String,
    /// What the case saw, line by line.
    pub(crate) saw: Vec<String>,
    /// Why the case could not run, once `not_run` says so.
    pub(crate) not_run: Option<String>,
}
