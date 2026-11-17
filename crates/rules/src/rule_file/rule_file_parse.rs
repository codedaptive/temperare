//! `RuleFile::parse`: parse and check rule-file text.

use super::RuleFile;
use super::check_keys::check_keys;
use super::check_security_owner::check_security_owner;
use super::check_version::check_version;
use super::problem_at::problem_at;
use super::read_exceptions::read_exceptions;
use super::read_kits::read_kits;
use super::read_rules::read_rules;
use super::string_value::string_value;
use crate::Problem;
use std::path::Path;
use toml::de::DeTable;

const TOP_KEYS: [&str; 5] = ["version", "owner", "kits", "rule", "exception"];

impl RuleFile {
    /// Parses `text` as the rule file in `folder`. A TOML syntax error is one Problem at its position; otherwise
    /// every check runs and Err holds all problems sorted by (line, col).
    pub fn parse(text: &str, folder: &Path) -> Result<RuleFile, Vec<Problem>> {
        let doc = DeTable::parse(text).map_err(|e| vec![syntax_problem(text, &e)])?;
        let root = doc.get_ref();
        let mut out = Vec::new();
        check_keys(text, root, &TOP_KEYS, "the top level", &mut out);
        check_version(text, root, &mut out);
        // A blank owner counts as none: an owner is never blank.
        let owner = string_value(text, root, "owner", &mut out)
            .map(|(owner, _)| owner)
            .filter(|owner| !owner.trim().is_empty());
        check_security_owner(text, root, owner.as_deref(), &mut out);
        let kits = read_kits(text, root, &mut out);
        let (rules, rule_ids) = read_rules(text, root, &kits, &mut out);
        let exceptions = read_exceptions(text, root, &rule_ids, &mut out);
        if out.is_empty() {
            return Ok(RuleFile { folder: folder.to_path_buf(), kits, owner, rules, exceptions });
        }
        out.sort_by_key(|p| (p.line, p.col));
        Err(out)
    }
}

/// The Problem for a TOML syntax error, at its span (1:1 when toml gives none).
fn syntax_problem(text: &str, e: &toml::de::Error) -> Problem {
    problem_at(
        text,
        e.span().map_or(0, |span| span.start),
        format!("TOML syntax error: {}", e.message().trim()),
        "Correct the TOML syntax at this position.",
    )
}
