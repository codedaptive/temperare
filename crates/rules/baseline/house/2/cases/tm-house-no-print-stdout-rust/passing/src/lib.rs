//! The total of an order.

/// The sum of `prices`, in cents.
pub fn total(prices: &[u64]) -> u64 {
    prices.iter().sum()
}
