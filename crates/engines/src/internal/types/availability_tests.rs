//! Unit test for availability.rs: plain data; equality tells the two cases and different reasons apart.

use super::Availability;

#[test]
fn not_available_differs_from_available_and_by_its_why() {
    let absent = Availability::NotAvailable { why: "ruff not found on PATH".to_string() };
    assert_ne!(absent, Availability::Available);
    assert_ne!(absent, Availability::NotAvailable { why: "ast-grep not found on PATH".to_string() });
    assert_eq!(absent.clone(), Availability::NotAvailable { why: "ruff not found on PATH".to_string() });
}
