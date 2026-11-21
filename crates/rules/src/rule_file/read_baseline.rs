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
        return Err((format!("unknown baseline set \"{name}\""), ready_sets_fix(sets)));
    }
    named.sort_by_key(|set| set.version);
    let Some(set) = named.iter().find(|set| set.version == version) else {
        return Err((format!("unknown version {version} of baseline set \"{name}\""), ready_versions_fix(name, &named)));
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

/// The fix for an unknown version of `name`: its ready versions as `name@version`, in the order of `named` (sorted by
/// version), since an unready one would be refused in turn; when none is, the entry can only be removed.
fn ready_versions_fix(name: &str, named: &[&BaselineSet]) -> String {
    let versions: Vec<String> =
        named.iter().filter(|set| set.ready).map(|set| format!("{name}@{}", set.version)).collect();
    if versions.is_empty() {
        return format!("No version of baseline set \"{name}\" is ready in this build; remove the entry.");
    }
    format!("Use one of: {}.", versions.join(", "))
}

/// The fix for an unknown set: every ready set as `name@version`, sorted by name then by version as a number, since
/// only a ready set can be taken and an entry must name a version (a bare name is malformed); when none is ready, the
/// entry can only be removed.
fn ready_sets_fix(sets: &[BaselineSet]) -> String {
    let mut ready: Vec<&BaselineSet> = sets.iter().filter(|set| set.ready).collect();
    if ready.is_empty() {
        return "No baseline set is ready in this build; remove the entry.".to_string();
    }
    ready.sort_unstable_by_key(|set| (set.name, set.version));
    let ids: Vec<String> = ready.iter().map(|set| format!("{}@{}", set.name, set.version)).collect();
    format!("Use one of: {}.", ids.join(", "))
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
