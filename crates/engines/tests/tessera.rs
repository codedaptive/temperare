use engines::{availability, Availability};
use rules::Engine;

#[test]
fn tessera_is_not_available_regardless_of_ts_on_path() {
    assert_eq!(availability(Engine::Tessera), Availability::NotAvailable {
        why: "Tessera's slop records are not defined yet".into(),
    });
}
