//! Order totals.

/// The order's total in cents.
pub fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

fn largest(prices: &[u32]) -> u32 {
    prices.iter().copied().max().unwrap_or(0)
}
