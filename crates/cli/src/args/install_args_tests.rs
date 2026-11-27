//! Unit test for install_args.rs: the default is a plain install: place, no target named, HOME, ask.

use super::InstallArgs;

#[test]
fn default_sets_no_flag() {
    let d = InstallArgs::default();
    assert!(!d.no_place);
    assert_eq!(d.targets, None);
    assert!(!d.local);
    assert!(!d.yes);
}
