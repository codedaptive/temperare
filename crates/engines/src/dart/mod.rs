//! The dart adapter: its entry (`DART`) and the flow it runs for one rule, the refusal of a check that is not one
//! lower-case code, its no-file outcome before availability, the listing of the Dart sources in a repository
//! (tracked, and untracked but not ignored; never a symlink), the package root a rule runs in, the not-available case
//! of dependencies never fetched, the reader of its machine output, the analysis options (the files that govern a
//! folder, their includes, and package URIs made native Windows paths by `windows_path`), read with a YAML parser
//! (`yaml_entries`) for the suppressions in them and whether they enable a lint, and the probe run that tells a lint
//! from a diagnostic and an unknown code.

mod check_kind;
mod dart_entry;
mod dart_files;
mod dart_refusal;
mod include_targets;
mod is_dart_source;
mod kind_of;
mod lint_enabled;
mod missing_packages;
mod options_files;
mod options_suppression;
mod package_path;
mod parse_dart;
mod probe_dir;
mod probe_dir_drop;
mod probe_files;
mod pubspec_root;
mod run_dart;
mod windows_path;

#[cfg(test)]
mod dart_entry_tests;
#[cfg(test)]
mod dart_refusal_tests;
#[cfg(test)]
mod include_targets_tests;
#[cfg(test)]
mod is_dart_source_tests;
#[cfg(test)]
mod kind_of_tests;
#[cfg(test)]
mod lint_enabled_tests;
#[cfg(test)]
mod missing_packages_tests;
#[cfg(test)]
mod options_suppression_tests;
#[cfg(test)]
mod package_path_tests;
#[cfg(test)]
mod parse_dart_tests;
#[cfg(test)]
mod probe_files_tests;
#[cfg(test)]
mod windows_path_tests;

pub(crate) use check_kind::CheckKind;
pub use dart_entry::DART;
pub(crate) use dart_files::dart_files;
pub(crate) use dart_refusal::dart_refusal;
pub(crate) use include_targets::include_targets;
pub(crate) use is_dart_source::is_dart_source;
pub(crate) use kind_of::kind_of;
pub(crate) use lint_enabled::lint_enabled;
pub(crate) use missing_packages::missing_packages;
pub(crate) use options_files::options_files;
pub(crate) use options_suppression::options_suppression;
pub(crate) use package_path::package_path;
pub(crate) use parse_dart::parse_dart;
pub(crate) use probe_dir::ProbeDir;
pub(crate) use probe_files::probe_files;
pub(crate) use pubspec_root::pubspec_root;
pub(crate) use run_dart::run_dart;
pub(crate) use windows_path::windows_path;
