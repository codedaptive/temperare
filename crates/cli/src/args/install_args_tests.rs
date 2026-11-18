//! ZUT for install_args.rs: the default is a plain install: place, link, no target named, HOME, ask.

use super::InstallArgs;

#[test]
fn default_sets_no_flag() {
    let d = InstallArgs::default();
    assert!(!d.no_place);
    assert!(!d.short_name);
    assert_eq!(d.targets, None);
    assert!(!d.local);
    assert!(!d.yes);
}
