//! `typed_why`: why a detekt check cannot run, when it picks a rule that needs type resolution.

use super::{picked_rules, typed_rule};

/// The why when any rule `picked_rules(check)` picks (a rule part's rule, or any rule of a set part) is one that
/// `typed_rule` says needs type resolution, naming every such rule, comma-separated, in list order; None when none
/// is. Such a rule needs a classpath cctao does not give detekt. Under `--debug` detekt says it ran each of them without
/// type resolution; without `--debug`, Deprecation gave exit 0 and no results on code that calls a deprecated
/// function (fixtures/commands.txt, "Type resolution probes"). So a check that ran without it could read clean for a
/// rule that never ran, and not available is never clean. Only the sets with no such rule (`comments`,
/// `empty-blocks`) run as sets.
pub(crate) fn typed_why(check: &str) -> Option<String> {
    let typed: Vec<&str> = picked_rules(check).into_iter().map(|(_, rule)| rule).filter(|rule| typed_rule(rule)).collect();
    (!typed.is_empty()).then(|| {
        format!(
            "detekt needs type resolution for {}, which needs a classpath that cctao does not give detekt, so they would report nothing",
            typed.join(", ")
        )
    })
}
