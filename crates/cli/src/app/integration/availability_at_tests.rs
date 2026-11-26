//! ZIT and RIT for availability_at.rs on the real disk, with no PATH lookup: Tessera answers from its entry before any
//! lookup, and a folder that does not exist fails at canonicalize before engines is asked.

use super::super::availability_at::availability_at;
use engines::Availability;
use rules::Engine;
use std::path::Path;

#[test]
fn availability_at_answers_without_a_path_lookup() {
    let missing = std::env::temp_dir().join(format!("tm-availability-at-missing-{}", std::process::id()));
    assert_eq!(
        availability_at(Engine::Tessera, Path::new(env!("CARGO_MANIFEST_DIR"))),
        Availability::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() },
        "Tessera on an existing folder"
    );
    // The io error's text is the OS's own (it differs between unix and Windows), so it is taken from the same call
    // on the same missing folder.
    let io_error = std::fs::canonicalize(&missing).unwrap_err();
    assert_eq!(
        availability_at(Engine::Eslint, &missing),
        Availability::NotAvailable {
            why: format!("cannot resolve the rule file's folder {}: {io_error}", missing.display())
        },
        "a missing folder gives NotAvailable naming the folder and the io error"
    );
}
