//! The git entry (`GIT`): counts the lines a change adds against the base, as test or code, and reports a change that
//! adds too much test for its code. Nothing is parsed. Also its flow, the check refusal and the params' reader; the
//! change itself is read by the crate's `change_of`.

mod count_change;
mod counts;
mod git_entry;
mod git_refusal;
mod ratio_hit;
mod read_test_ratio;
mod run_git;
mod run_git_with;
mod test_ratio;

#[cfg(test)]
mod count_change_tests;
#[cfg(test)]
mod git_entry_tests;
#[cfg(test)]
mod git_refusal_tests;
#[cfg(test)]
mod ratio_hit_tests;
#[cfg(test)]
mod read_test_ratio_tests;
#[cfg(test)]
mod run_git_with_tests;

pub(crate) use count_change::count_change;
pub use counts::Counts;
pub use git_entry::GIT;
pub(crate) use git_refusal::git_refusal;
pub(crate) use ratio_hit::ratio_hit;
pub(crate) use read_test_ratio::read_test_ratio;
pub(crate) use run_git::run_git;
pub(crate) use run_git_with::run_git_with;
pub use test_ratio::TestRatio;
