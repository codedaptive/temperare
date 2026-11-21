//! The price of an order line.

/// The price of `quantity` items at `unit_cents` each, in cents.
pub fn price(quantity: u64, unit_cents: u64) -> u64 {
    quantity * unit_cents
}
