//! `now_secs`: the real clock, for the worlds' `now` seams.

/// Whole seconds since the Unix epoch; 0 for a clock set before it.
pub(crate) fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |since| since.as_secs())
}
