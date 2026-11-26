//! `entry_for`: each engine's entry as an `Option`, for callers that take one.

use crate::entry::Entry;
use crate::entry_of::entry_of;
use rules::Engine;

/// `entry_of(engine)` wrapped in `Some`, so it is always `Some`. Kept for callers that take an `Option`, such as
/// cli's report.
pub fn entry_for(engine: Engine) -> Option<&'static Entry> {
    Some(entry_of(engine))
}
