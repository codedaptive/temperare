//! Cart checks.

/// One line of a cart: how many, and the price of one in cents.
pub struct Line {
    pub qty: u32,
    pub price: u32,
}

/// A shopping cart.
pub struct Cart {
    pub lines: Vec<Line>,
}

impl Cart {
    /// The index of the first line of more than one item costing over `limit` cents, if any.
    pub fn first_bulk_over(&self, limit: u32) -> Option<usize> {
        for (index, line) in self.lines.iter().enumerate() {
            if line.qty > 1 {
                if let Some(cost) = line.qty.checked_mul(line.price) {
                    if cost > limit {
                        return Some(index);
                    }
                }
            }
        }
        None
    }
}
