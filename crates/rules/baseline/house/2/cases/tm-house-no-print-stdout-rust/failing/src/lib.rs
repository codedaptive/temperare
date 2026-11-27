//! The total of an order.

/// The sum of `prices`, in cents.
pub fn total(prices: &[u64]) -> u64 {
    let sum = prices.iter().sum();
    println!("order total: {sum}");
    sum
}
