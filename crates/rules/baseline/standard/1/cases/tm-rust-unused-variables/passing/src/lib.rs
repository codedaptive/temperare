//! Greetings.

/// A greeting for each name, one per line.
pub fn greet_all(names: &[&str]) -> String {
    names.iter().map(|name| format!("Hello, {name}!\n")).collect()
}
