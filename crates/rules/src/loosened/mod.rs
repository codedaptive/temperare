//! What a change to a rule file loosened: the loosening, its kind, and the comparison of two rule files.

mod exception_loosenings;
mod loosened;
mod loosening;
mod loosening_kind;
mod rule_loosenings;
mod set_loosenings;

#[cfg(test)]
mod exception_loosenings_tests;
#[cfg(test)]
mod loosened_tests;
#[cfg(test)]
mod loosening_kind_tests;
#[cfg(test)]
mod loosening_tests;
#[cfg(test)]
mod rule_loosenings_tests;
#[cfg(test)]
mod set_loosenings_tests;

pub use loosened::loosened;
pub use loosening::Loosening;
pub use loosening_kind::LooseningKind;
