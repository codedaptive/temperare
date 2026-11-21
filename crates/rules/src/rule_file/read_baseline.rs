//! `read_baseline`: the baseline sets a rule file takes with its top-level `baseline` key.

use super::baseline_set::BaselineSet;
use super::problem_at::problem_at;
use super::string_list::string_list;
use crate::Problem;
use toml::de::DeTable;

/// The sets of `sets` that `baseline = ["name@version", ...]` takes, in file order, each with the byte offset of its
/// entry string; absent is none. An entry that is malformed, names no set or version in `sets`, names a set an
/// earlier entry already took, or names a set that is not ready is a Problem at the entry and is left out.
pub(crate) fn read_baseline<'a>(
    text: &str,
    root: &DeTable,
    sets: &'a [BaselineSet],
    out: &mut Vec<Problem>,
) -> Vec<(&'a BaselineSet, usize)> {
    let mut taken: Vec<(&'a BaselineSet, usize)> = Vec::new();
    for (entry, at) in string_list(text, root, "baseline", out) {
        match take(&entry, sets, &taken) {
            Ok(set) => taken.push((set, at)),
            Err((what, fix)) => out.push(problem_at(text, at, what, &fix)),
        }
    }
    taken
}

/// The set `entry` names, or the Problem's what and fix. The checks run in a fixed order (malformed, unknown set,
/// unknown version, taken twice, not ready) so an entry gets the first thing wrong with it.
fn take<'a>(
    entry: &str,
    sets: &'a [BaselineSet],
    taken: &[(&BaselineSet, usize)],
) -> Result<&'a BaselineSet, (String, String)> {
    let Some((name, version)) = split_entry(entry) else {
        return Err((
            format!("malformed baseline entry \"{entry}\""),
            "Write the entry as a set name, '@' and a whole-number version, e.g. \"standard@1\".".to_string(),
        ));
    };
    let mut named: Vec<&BaselineSet> = sets.iter().filter(|set| set.name == name).collect();
    if named.is_empty() {
        let mut names: Vec<&str> = sets.iter().map(|set| set.name).collect();
        names.sort_unstable();
        names.dedup();
        return Err((format!("unknown baseline set \"{name}\""), format!("Use one of: {}.", names.join(", "))));
    }
    named.sort_by_key(|set| set.version);
    let Some(set) = named.iter().find(|set| set.version == version) else {
        let versions: Vec<String> = named.iter().map(|set| format!("{name}@{}", set.version)).collect();
        return Err((
            format!("unknown version {version} of baseline set \"{name}\""),
            format!("Use one of: {}.", versions.join(", ")),
        ));
    };
    if taken.iter().any(|(earlier, _)| earlier.name == name) {
        return Err((format!("baseline set \"{name}\" is taken twice"), "Take each set once, at one version.".to_string()));
    }
    if !set.ready {
        let id = format!("{}@{}", set.name, set.version);
        return Err((
            format!("{id} is not ready in this build: its rules are not yet proven"),
            format!("remove {id} from baseline"),
        ));
    }
    Ok(set)
}

/// The name and version of `name@version`: the name not empty, the version only ASCII digits and within u32. The
/// split is at the first '@', so a second '@' lands in the version and makes the entry malformed.
fn split_entry(entry: &str) -> Option<(&str, u32)> {
    let (name, version) = entry.split_once('@')?;
    if name.is_empty() || version.is_empty() || !version.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((name, version.parse().ok()?))
}
