/// The reading of a meter in whole litres.
pub fn litres(millilitres: u64) -> u64 {
    // A part litre is not billed.
    // Rounds down, so a customer never pays for water not yet used.
    millilitres / 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_down() {
        assert_eq!(litres(1999), 1, "a part litre is dropped");
    }
}
