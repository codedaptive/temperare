//! Discounts.

/// `price` in cents less `percent` per cent.
pub fn discounted(price: u64, percent: u64) -> u64 {
    let off = dbg!(price * percent / 100);
    price - off
}
