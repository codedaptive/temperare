//! `config_why`: what one gitleaks config file means for the guard: why cctao reads no findings under it, or the files it
//! extends.

use super::allowlist_key;
use toml::{Table, Value};

/// `text`, the config file `name`, read as TOML:
/// - it does not parse: Err saying so. The parser's message is not quoted, since it can echo the config's lines;
/// - it has an `allowlist_key`: Err naming that key path;
/// - else Ok with every string value whose key path is `extend` then `path` (gitleaks' extend file), in table order,
///   each key split on '.' and matched in any ASCII case as gitleaks reads them. Every case variant's path is kept
///   (`[Extend]` and `[extend]` may both appear, and which one gitleaks keeps is its own business), so a collision can
///   only make the guard read more. Empty when there is none.
pub(crate) fn config_why(name: &str, text: &str) -> Result<Vec<String>, String> {
    let table: Table = text.parse().map_err(|_| {
        format!(
            "gitleaks config `{name}` does not parse as TOML, so cctao cannot tell whether it declares an allowlist and \
             reads no findings under it"
        )
    })?;
    if let Some(path) = allowlist_key(&table) {
        return Err(format!(
            "gitleaks config `{name}` declares an allowlist (`{path}`); the rule file is the only home for an \
             exception, so cctao reads no findings under it"
        ));
    }
    Ok(strings_at(&table, &["extend", "path"]))
}

/// Every string value in `table` whose key segments, through nested tables, are `want` in any ASCII case.
fn strings_at(table: &Table, want: &[&str]) -> Vec<String> {
    table
        .iter()
        .flat_map(|(key, value)| {
            let segments: Vec<&str> = key.split('.').collect();
            let rest = want.get(segments.len()..).filter(|_| {
                segments.iter().zip(want).all(|(segment, name)| segment.eq_ignore_ascii_case(name))
            });
            match (rest, value) {
                (Some([]), Value::String(text)) => vec![text.clone()],
                (Some(rest @ [_, ..]), Value::Table(inner)) => strings_at(inner, rest),
                _ => Vec::new(),
            }
        })
        .collect()
}
