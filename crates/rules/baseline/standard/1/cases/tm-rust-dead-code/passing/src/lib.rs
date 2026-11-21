//! Order totals.

/// The order's total in cents.
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

/// The order's dearest item in cents, 0 for an empty order.
pub fn largest(prices: &[u32]) -> u32 {
    prices.iter().copied().max().unwrap_or(0)
}
