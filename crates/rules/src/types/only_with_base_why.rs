//! `ONLY_WITH_BASE_WHY`: the why for a base-only rule checked with no base.

/// The why reported for a rule with `only_with_base = true` when the check is given no base: such a rule reports only
/// what a change adds, so without a base it is "not available" rather than clean.
pub const ONLY_WITH_BASE_WHY: &str = "reports only what a change adds; run with --base";
