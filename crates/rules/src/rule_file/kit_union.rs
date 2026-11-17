//! `kit_union`: the paths of the kits a rule names under any key that names kits.

use super::problem_at::problem_at;
use crate::{Kit, Problem};

/// The union of the paths of the kits of `kits` named in `names` (each name with its byte offset), read from the
/// rule-file key `key`; None when no kit is named, since an empty list would match nothing. A name not under
/// `[kits]` is a Problem at its string, whose fix names `key`.
pub(crate) fn kit_union(
    text: &str,
    names: &[(String, usize)],
    kits: &[Kit],
    key: &str,
    out: &mut Vec<Problem>,
) -> Option<Vec<String>> {
    if names.is_empty() {
        return None;
    }
    let mut union = Vec::new();
    for (name, at) in names {
        match kits.iter().find(|kit| kit.name == *name) {
            Some(kit) => union.extend(kit.paths.iter().cloned()),
            None => out.push(problem_at(
                text,
                *at,
                format!("kit \"{name}\" is not defined under [kits]"),
                &format!("Define [kits.{name}] with its `paths`, or remove \"{name}\" from `{key}`."),
            )),
        }
    }
    Some(union)
}
