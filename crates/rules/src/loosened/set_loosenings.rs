//! `set_loosenings`: the baseline sets a change dropped or took at an older version.

use super::{Loosening, LooseningKind};
use crate::RuleFile;

/// For each of the base's `baseline` entries ("name@version", split at the last '@'), in base order: SetDropped when
/// now takes no set of that name; SetOlder when now takes it at a lower version, both versions read as u32. Entries
/// with no '@' give nothing; a version that does not read as u32 is never SetOlder.
pub(super) fn set_loosenings(base: &RuleFile, now: &RuleFile) -> Vec<Loosening> {
    let taken: Vec<(&str, &str)> = now.baseline.iter().filter_map(|entry| entry.rsplit_once('@')).collect();
    let mut found = Vec::new();
    for (name, version) in base.baseline.iter().filter_map(|entry| entry.rsplit_once('@')) {
        let about = name.to_string();
        match taken.iter().find(|(later, _)| *later == name) {
            None => found.push(Loosening {
                kind: LooseningKind::SetDropped,
                about,
                before: Some(version.to_string()),
                after: None,
            }),
            Some((_, later)) if older(version, later) => found.push(Loosening {
                kind: LooseningKind::SetOlder,
                about,
                before: Some(version.to_string()),
                after: Some(later.to_string()),
            }),
            Some(_) => {}
        }
    }
    found
}

fn older(base: &str, now: &str) -> bool {
    matches!((base.parse::<u32>(), now.parse::<u32>()), (Ok(base), Ok(now)) if now < base)
}
