//! ZUT for report_engines.rs: an empty engine list gives no entries, reading nothing from the engines crate. The cases
//! that read engines' real entries are in integration/report_engines_tests.rs.

use super::report_engines::report_engines;
use engines::Availability;
use rules::Engine;

fn found(_: Engine) -> Availability {
    Availability::Available
}

#[test]
fn no_engines_give_no_entries() {
    assert_eq!(report_engines(vec![], &found), vec![]);
}
