//! `pmd_refusal`: why tm will not run a PMD check, if it will not.

/// The reason `check` is refused, naming the first comma part refused; None when every part is
/// `category/java/<category>.xml`, the category ASCII lowercase letters, or that plus `/<Rule>`, the rule ASCII letters
/// and digits starting with an uppercase letter. The category is not checked against PMD's list: a category this PMD
/// does not have is its "Cannot load ruleset" (`unknown_ruleset`), so the rule is not available, as for a rule it does
/// not have. Anything else PMD would load is a ruleset file or URL: the rule file is the only home for rules and their
/// exceptions, which a ruleset file can carry, and a URL would reach the network.
pub(crate) fn pmd_refusal(check: &str) -> Option<String> {
    let part = check.split(',').find(|part| !is_reference(part))?;
    let why = if part.is_empty() {
        "an empty part is not a PMD rule reference".to_string()
    } else if part.starts_with("category/") && !part.starts_with("category/java/") {
        format!("`{part}` is not Java, the only language tm runs PMD on")
    } else {
        format!(
            "`{part}` is not a PMD Java category or rule reference (category/java/<category>.xml[/<Rule>]); a ruleset \
             file or URL is refused, since the rule file is the only home for rules and their exceptions"
        )
    };
    Some(format!("pmd check `{check}`: {why}"))
}

/// Whether `part` is `category/java/<category>.xml` or that plus `/<Rule>`.
fn is_reference(part: &str) -> bool {
    let Some(rest) = part.strip_prefix("category/java/") else {
        return false;
    };
    let (file, rule) = rest.split_once('/').map_or((rest, None), |(file, rule)| (file, Some(rule)));
    let category = file.strip_suffix(".xml").is_some_and(|name| !name.is_empty() && name.bytes().all(|b| b.is_ascii_lowercase()));
    category && rule.is_none_or(is_rule_name)
}

/// Whether `name` is ASCII letters and digits starting with an uppercase letter.
fn is_rule_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase()) && name.chars().all(|c| c.is_ascii_alphanumeric())
}
