//! Unit tests for resolved_baseline.rs.

use super::resolved_baseline::ResolvedBaseline;
use std::collections::BTreeSet;

#[test]
fn the_default_takes_nothing() {
    let resolved = ResolvedBaseline::default();
    assert_eq!(resolved.taken, Vec::<String>::new());
    assert_eq!(resolved.rules.len(), 0);
    assert_eq!(resolved.not_applicable.len(), 0);
    assert_eq!(resolved.retired, Vec::<String>::new());
    assert_eq!(resolved.ids, BTreeSet::new());
    assert_eq!(resolved.security_at, None);
}
