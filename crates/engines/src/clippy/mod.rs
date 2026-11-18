//! The clippy adapter: finds the Cargo root and the workspace root, builds the `cargo clippy` and
//! `cargo locate-project` commands, reads their output and a rule's `allow` params, and runs one clippy rule.

mod cargo_root;
mod clippy_allows;
mod clippy_cmd;
mod hits_clippy;
mod locate_workspace_cmd;
mod parse_clippy;
mod parse_locate_workspace;

#[cfg(test)]
mod cargo_root_tests;
#[cfg(test)]
mod clippy_allows_tests;
#[cfg(test)]
mod clippy_cmd_tests;
#[cfg(test)]
mod hits_clippy_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod locate_workspace_cmd_tests;
#[cfg(test)]
mod parse_clippy_tests;
#[cfg(test)]
mod parse_locate_workspace_tests;

pub use cargo_root::cargo_root;
pub(crate) use clippy_allows::clippy_allows;
pub use clippy_cmd::clippy_cmd;
pub(crate) use hits_clippy::hits_clippy;
pub use locate_workspace_cmd::locate_workspace_cmd;
pub use parse_clippy::parse_clippy;
pub use parse_locate_workspace::parse_locate_workspace;
