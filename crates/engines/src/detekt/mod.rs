//! The detekt adapter: its entry (`DETEKT`) and the flow it runs for one rule; cctao's list of detekt 1.23.8's
//! built-in rules (`detekt_rules.txt`, read by `detekt_rule_list`), the refusal of a check with a malformed name, the
//! not-listed stop for a well-formed name that is not in the list, the rules a check picks, and cctao's own config layer
//! that makes each picked rule run on every file detekt is given, whatever the repository's config says; the listing
//! of the Kotlin sources in a repository (tracked, and untracked but not ignored; never a symlink) and its no-file
//! outcome before availability (settled by `no_file_listed`, in `engine_hits_with`'s step 1); detekt's arguments, its
//! output reshaped around the SARIF report file, detekt's words for a rule it lacks, and the reader of the report; and
//! the rules that need type resolution
//! (`detekt_typed_rules.txt`, read by `typed_rule`), which cctao cannot run, with the why a check naming one is not
//! available.

mod detekt_args;
mod detekt_entry;
mod detekt_layer;
mod detekt_list_version;
mod detekt_output;
mod detekt_refusal;
mod detekt_rule_list;
mod detekt_unlisted;
mod kotlin_files;
mod parse_detekt;
mod picked_rules;
mod run_detekt;
mod typed_rule;
mod typed_why;
mod unknown_rule;

#[cfg(test)]
mod detekt_args_tests;
#[cfg(test)]
mod detekt_entry_tests;
#[cfg(test)]
mod detekt_layer_tests;
#[cfg(test)]
mod detekt_list_version_tests;
#[cfg(test)]
mod detekt_output_tests;
#[cfg(test)]
mod detekt_refusal_tests;
#[cfg(test)]
mod detekt_rule_list_tests;
#[cfg(test)]
mod detekt_unlisted_tests;
#[cfg(test)]
mod parse_detekt_tests;
#[cfg(test)]
mod picked_rules_tests;
#[cfg(test)]
mod typed_rule_tests;
#[cfg(test)]
mod typed_why_tests;
#[cfg(test)]
mod unknown_rule_tests;

pub(crate) use detekt_args::detekt_args;
pub use detekt_entry::DETEKT;
pub(crate) use detekt_layer::detekt_layer;
pub(crate) use detekt_list_version::DETEKT_LIST_VERSION;
pub(crate) use detekt_output::detekt_output;
pub(crate) use detekt_refusal::detekt_refusal;
pub(crate) use detekt_rule_list::detekt_rule_list;
pub(crate) use detekt_unlisted::detekt_unlisted;
pub(crate) use kotlin_files::kotlin_files;
pub use parse_detekt::parse_detekt;
pub(crate) use picked_rules::picked_rules;
pub(crate) use run_detekt::run_detekt;
pub(crate) use typed_rule::typed_rule;
pub(crate) use typed_why::typed_why;
pub(crate) use unknown_rule::unknown_rule;
