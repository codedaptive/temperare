//! `RuleFile::parse`: parse and check rule-file text, with the baseline sets it takes.

use super::RuleFile;
use super::baseline_set::BaselineSet;
use super::check_keys::check_keys;
use super::check_security_owner::check_security_owner;
use super::check_version::check_version;
use super::embedded_sets::EMBEDDED_SETS;
use super::read_baseline::read_baseline;
use super::read_exceptions::read_exceptions;
use super::read_kits::read_kits;
use super::read_overrides::read_overrides;
use super::read_rules::read_rules;
use super::resolve_baseline::resolve_baseline;
use super::string_value::string_value;
use super::syntax_problem::syntax_problem;
use crate::Problem;
use std::path::Path;
use toml::de::DeTable;

const TOP_KEYS: [&str; 7] = ["version", "owner", "kits", "baseline", "override", "rule", "exception"];

impl RuleFile {
    /// Parses `text` as the rule file in `folder`, taking its `baseline` sets from the sets built into `tm`. A TOML
    /// syntax error is one Problem at its position; otherwise every check runs and Err holds all problems sorted by
    /// (line, col). `rules` holds the taken sets' rules that apply, with the `[[override]]`s applied, then the file's
    /// own; a set rule turned off is left out, and one missing a required param is in `not_applicable_rules`
    /// instead. An exception may name any rule of the taken sets or of the file.
    pub fn parse(text: &str, folder: &Path) -> Result<RuleFile, Vec<Problem>> {
        parse_with_sets(text, folder, EMBEDDED_SETS)
    }
}

/// `RuleFile::parse` with `sets` as the baseline sets a `baseline` entry may take.
pub(crate) fn parse_with_sets(text: &str, folder: &Path, sets: &[BaselineSet]) -> Result<RuleFile, Vec<Problem>> {
    let doc = DeTable::parse(text).map_err(|e| vec![syntax_problem(text, &e)])?;
    let root = doc.get_ref();
    let mut out = Vec::new();
    check_keys(text, root, &TOP_KEYS, "the top level", &mut out);
    check_version(text, root, &mut out);
    // A blank owner counts as none: an owner is never blank.
    let owner = string_value(text, root, "owner", &mut out)
        .map(|(owner, _)| owner)
        .filter(|owner| !owner.trim().is_empty());
    let kits = read_kits(text, root, &mut out);
    let taken = read_baseline(text, root, sets, &mut out);
    let overrides = read_overrides(text, root, &mut out);
    let (own_rules, mut rule_ids) = read_rules(text, root, &kits, false, &mut out);
    let resolved = resolve_baseline(text, &taken, &overrides, &mut out);
    check_security_owner(text, root, owner.as_deref(), resolved.security_at, &mut out);
    rule_ids.extend(resolved.ids);
    let exceptions = read_exceptions(text, root, &rule_ids, &mut out);
    if out.is_empty() {
        let mut rules = resolved.rules;
        rules.extend(own_rules);
        return Ok(RuleFile {
            folder: folder.to_path_buf(),
            kits,
            owner,
            rules,
            exceptions,
            baseline: resolved.taken,
            retired: resolved.retired,
            not_applicable_rules: resolved.not_applicable,
        });
    }
    out.sort_by_key(|p| (p.line, p.col));
    Err(out)
}
