//! `not_available_pairs`: the rules `temperare` reports not available in a `--json` answer, as (rule, engine) pairs.

use crate::tm::Answer;

/// The (rule, engine) of each entry of `a`'s `not_available` list, in the order the answer gives them; an entry
/// missing a field gives "" for it. Panics, printing the JSON, when the answer holds no `not_available` list.
pub fn not_available_pairs(a: &Answer) -> Vec<(String, String)> {
    a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| {
            let field = |key: &str| n[key].as_str().unwrap_or("").to_string();
            (field("rule"), field("engine"))
        })
        .collect()
}
