//! Unit test for detekt_layer.rs: the exact layer for one rule, and rules sitting under their own sets for a mixed check.

use super::detekt_layer;

#[test]
fn detekt_layer_activates_and_unfilters_each_picked_rule_under_its_set() {
    let magic_number = "config:
  validation: true
  excludes: ''
output-reports:
  active: true
style:
  active: true
  excludes: []
  includes: []
  MagicNumber:
    active: true
    excludes: []
    includes: []
    ignoreAnnotated: []
";
    assert_eq!(detekt_layer("MagicNumber"), magic_number);

    let mixed = detekt_layer("empty-blocks,MagicNumber");
    for part in [
        "empty-blocks:\n  active: true\n  excludes: []\n  includes: []\n  EmptyCatchBlock:\n    active: true\n",
        "  EmptyWhenBlock:\n    active: true\n    excludes: []\n    includes: []\n    ignoreAnnotated: []\n  EmptyWhileBlock:\n",
        "  EmptyWhileBlock:\n    active: true\n    excludes: []\n    includes: []\n    ignoreAnnotated: []\nstyle:\n  active: true\n",
        "style:\n  active: true\n  excludes: []\n  includes: []\n  MagicNumber:\n    active: true\n",
    ] {
        assert!(mixed.contains(part), "{part}\nin\n{mixed}");
    }
    assert_eq!(mixed.matches("style:\n").count(), 1, "{mixed}");
    assert_eq!(mixed.matches("empty-blocks:\n").count(), 1, "{mixed}");
    assert_eq!(mixed.matches("    ignoreAnnotated: []\n").count(), 16, "{mixed}");
}
