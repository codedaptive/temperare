//! The shared value types: engine, kind, severity, relative path, finding, rule outcome and rule-file problem.

mod engine;
mod engine_from_name;
mod engine_name;
mod engine_needs_graph;
mod finding;
mod finding_line;
mod finding_new;
mod kind;
mod kind_from_name;
mod problem;
mod rel_path;
mod rel_path_as_str;
mod rel_path_new;
mod rule_outcome;
mod severity;
mod severity_from_name;

#[cfg(test)]
mod engine_from_name_tests;
#[cfg(test)]
mod engine_name_tests;
#[cfg(test)]
mod engine_needs_graph_tests;
#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod finding_line_tests;
#[cfg(test)]
mod finding_new_tests;
#[cfg(test)]
mod finding_tests;
#[cfg(test)]
mod kind_from_name_tests;
#[cfg(test)]
mod kind_tests;
#[cfg(test)]
mod problem_tests;
#[cfg(test)]
mod rel_path_as_str_tests;
#[cfg(test)]
mod rel_path_new_tests;
#[cfg(test)]
mod rel_path_tests;
#[cfg(test)]
mod rule_outcome_tests;
#[cfg(test)]
mod severity_from_name_tests;
#[cfg(test)]
mod severity_tests;

pub use engine::Engine;
pub use finding::Finding;
pub use kind::Kind;
pub use problem::Problem;
pub use rel_path::RelPath;
pub use rule_outcome::RuleOutcome;
pub use severity::Severity;
