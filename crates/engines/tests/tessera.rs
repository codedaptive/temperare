use std::path::Path;

use engines::{availability_in, Availability};
use rules::Engine;

#[test]
fn tessera_is_not_available_regardless_of_ts_on_path() {
    assert_eq!(availability_in(Engine::Tessera, Path::new(env!("CARGO_MANIFEST_DIR"))), Availability::NotAvailable {
        why: "Tessera's slop records are not defined yet".into(),
    });
}
