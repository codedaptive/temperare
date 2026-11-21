//! `TestRatio`: a git rule's three params, read.

use rules::Scope;

/// What a git rule allows: at most `max_ratio` lines of test per line of code, judged only once a change adds at
/// least `floor` lines of test; `test_paths` says which files are tests.
#[derive(Debug)]
pub struct TestRatio {
    pub max_ratio: f64,
    pub floor: u32,
    pub test_paths: Scope,
}
