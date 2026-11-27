/// Runs one job and returns how many items it handled.
pub fn run(items: &[u32]) -> usize {
    let mut handled = 0;
    for item in items {
        // An item of zero is empty and is skipped.
        if *item > 0 {
            handled += 1;
        }
    }
    handled
}
