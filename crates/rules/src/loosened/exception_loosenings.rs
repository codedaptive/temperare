//! `exception_loosenings`: the exceptions a change added or widened.

use super::{Loosening, LooseningKind};
use crate::{Date, Exception, RuleFile};

/// At most one loosening for each of now's exceptions, in file order. A base exception covers one with the same rule
/// and path when its `what` is None or the same, so narrowing an exception by adding a `what` gives no ExceptionAdded and
/// widening one by dropping it gives ExceptionAdded, as does an exception no base exception covers. A covered one is
/// judged against the base's reach: no end if any covering exception has no `expires`, else the latest `expires`
/// among them. Against a dated reach, no `expires` now gives ExpiryDropped and a later one ExpiryLater; an earlier or
/// equal date, or an undated reach, gives nothing. Reason and approval are not compared.
pub(super) fn exception_loosenings(base: &RuleFile, now: &RuleFile) -> Vec<Loosening> {
    now.exceptions
        .iter()
        .filter_map(|later| {
            let coverers: Vec<&Exception> =
                base.exceptions.iter().filter(|earlier| covers_exception(earlier, later)).collect();
            if coverers.is_empty() {
                return Some(Loosening {
                    kind: LooseningKind::ExceptionAdded,
                    about: later.rule.clone(),
                    before: None,
                    after: Some(match &later.what {
                        Some(what) => format!("{} ({what})", later.path.as_str()),
                        None => later.path.as_str().to_string(),
                    }),
                });
            }
            expiry_loosening(&coverers, later)
        })
        .collect()
}

fn covers_exception(earlier: &Exception, later: &Exception) -> bool {
    earlier.rule == later.rule && earlier.path == later.path && (earlier.what.is_none() || earlier.what == later.what)
}

/// The loosening of `later`'s `expires` against the reach of its non-empty `coverers`; None when the reach has no end
/// (a coverer with no `expires`) or `later` ends no later than it.
fn expiry_loosening(coverers: &[&Exception], later: &Exception) -> Option<Loosening> {
    let reach = coverers.iter().map(|earlier| earlier.expires).collect::<Option<Vec<Date>>>()?.into_iter().max()?;
    let (kind, after) = match later.expires {
        None => (LooseningKind::ExpiryDropped, None),
        Some(day) if day > reach => (LooseningKind::ExpiryLater, Some(day.to_string())),
        Some(_) => return None,
    };
    Some(Loosening { kind, about: later.rule.clone(), before: Some(reach.to_string()), after })
}
