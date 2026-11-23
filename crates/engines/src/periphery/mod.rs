//! The Periphery adapter: its entry (`PERIPHERY`) and the flow it runs for one rule, the refusal of a check that is not
//! a result hint, the reader of its JSON results, the commands of its runs, the walk over the manifests of the package
//! and the local packages it reaches, the remote dependencies and binary targets that make a package not available
//! and the local dependencies the walk follows, how a why names a package the walk read, the package a rule runs on,
//! and the scratch folder each run builds in.

mod answered;
mod build_index_cmd;
mod describe_package_cmd;
mod dump_package_cmd;
mod local_dependencies;
mod manifest_walk;
mod package_root;
mod package_subject;
mod parse_periphery;
mod periphery_cmd;
mod periphery_entry;
mod periphery_refusal;
mod remote_dependencies;
mod run_periphery;
mod scratch_dir;
mod scratch_dir_drop;

#[cfg(test)]
mod build_index_cmd_tests;
#[cfg(test)]
mod describe_package_cmd_tests;
#[cfg(test)]
mod dump_package_cmd_tests;
#[cfg(test)]
mod local_dependencies_tests;
#[cfg(test)]
mod manifest_walk_tests;
#[cfg(test)]
mod package_subject_tests;
#[cfg(test)]
mod parse_periphery_tests;
#[cfg(test)]
mod periphery_cmd_tests;
#[cfg(test)]
mod periphery_entry_tests;
#[cfg(test)]
mod periphery_refusal_tests;
#[cfg(test)]
mod remote_dependencies_tests;

pub(crate) use answered::answered;
pub use build_index_cmd::build_index_cmd;
pub use describe_package_cmd::describe_package_cmd;
pub use dump_package_cmd::dump_package_cmd;
pub use local_dependencies::local_dependencies;
pub(crate) use manifest_walk::manifest_walk;
pub(crate) use package_root::package_root;
pub(crate) use package_subject::package_subject;
pub use parse_periphery::parse_periphery;
pub use periphery_cmd::periphery_cmd;
pub use periphery_entry::PERIPHERY;
pub(crate) use periphery_refusal::periphery_refusal;
pub use remote_dependencies::remote_dependencies;
pub(crate) use run_periphery::run_periphery;
pub(crate) use scratch_dir::ScratchDir;
