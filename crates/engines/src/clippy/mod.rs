//! The clippy adapter: finds the Cargo root and the workspace root, builds the `clippy-driver -W help`,
//! `cargo locate-project` and `cargo clippy` commands, reads their output and a rule's `allow` params, tells a lint
//! group and an unlisted name from one lint, and runs one clippy rule. The toolchain's lints and lint groups come from
//! the toolchain itself (`-W help` in the Cargo root), never from a list kept here, so a group a newer toolchain adds
//! is still refused and a lint it adds is still taken.

mod cargo_root;
mod clippy_allows;
mod clippy_cmd;
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
mod clippy_allows_tests;
#[cfg(test)]
mod clippy_cmd_tests;
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
pub(crate) use clippy_allows::clippy_allows;
pub use clippy_cmd::clippy_cmd;
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
