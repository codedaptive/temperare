//! The clippy adapter: the clippy entry (its `cargo clippy` command and its `allow` param), the flow that finds the
//! Cargo root and the workspace root, the `clippy-driver -W help` and `cargo locate-project` commands and their
//! readers, the reader of clippy's output, and what tells a lint group and an unlisted name from one lint. The
//! toolchain's lints and lint groups come from the toolchain itself (`-W help` in the Cargo root), never from a list
//! kept here, so a group a newer toolchain adds is still refused and a lint it adds is still taken.

mod cargo_root;
mod clippy_entry;
mod help_names;
mod hits_clippy;
mod known_lint;
mod lint_group;
mod lint_groups_cmd;
mod locate_workspace_cmd;
mod parse_clippy;
mod parse_lint_groups;
mod parse_lints;
mod parse_locate_workspace;

#[cfg(test)]
mod clippy_entry_tests;
#[cfg(test)]
mod help_names_tests;
#[cfg(test)]
mod hits_clippy_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod known_lint_tests;
#[cfg(test)]
mod lint_group_tests;
#[cfg(test)]
mod lint_groups_cmd_tests;
#[cfg(test)]
mod locate_workspace_cmd_tests;
#[cfg(test)]
mod parse_clippy_tests;
#[cfg(test)]
mod parse_lint_groups_tests;
#[cfg(test)]
mod parse_lints_tests;
#[cfg(test)]
mod parse_locate_workspace_tests;

pub use cargo_root::cargo_root;
pub use clippy_entry::CLIPPY;
pub(crate) use help_names::help_names;
pub(crate) use hits_clippy::hits_clippy;
pub(crate) use known_lint::known_lint;
pub(crate) use lint_group::lint_group;
pub(crate) use lint_groups_cmd::lint_groups_cmd;
pub use locate_workspace_cmd::locate_workspace_cmd;
pub use parse_clippy::parse_clippy;
pub(crate) use parse_lint_groups::parse_lint_groups;
pub(crate) use parse_lints::parse_lints;
pub use parse_locate_workspace::parse_locate_workspace;
