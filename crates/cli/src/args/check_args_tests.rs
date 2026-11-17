//! ZUT for check_args.rs: the default is no flags and no paths.

use super::CheckArgs;

#[test]
fn default_has_nothing_set() {
    let d = CheckArgs::default();
    assert_eq!(d.rules, None);
    assert_eq!(d.base, None);
    assert_eq!(d.kind, None);
    assert!(!d.json);
    assert!(d.paths.is_empty());
}
