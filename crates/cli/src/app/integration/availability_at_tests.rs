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
    let unresolved = availability_at(Engine::Eslint, &missing);
    let names_folder_and_error = match &unresolved {
        Availability::NotAvailable { why } => {
            why.contains(&missing.display().to_string()) && why.contains("No such file or directory")
        }
        Availability::Available => false,
    };
    assert!(
        names_folder_and_error,
        "a missing folder gives NotAvailable naming {} and the io error: got {unresolved:?}",
        missing.display()
    );
}
