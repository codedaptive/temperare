//! `parse_user_config`: the savings switch read from the text of `~/.config/temperare/config.toml`.

/// `Ok` with the value of `savings_line` when it is a bool, `Ok(true)` when the text has no keys; `Err(why)` when the
/// text is not TOML (the first line of the parser's message), when a key other than `savings_line` is present
/// (``unknown key `<key>` ``, the first in sorted order, escaped as Rust's `escape_debug` does), or when
/// `savings_line` is not a bool. `why` is always one line: it never holds a newline.
pub(crate) fn parse_user_config(text: &str) -> Result<bool, String> {
    // The parser's message spans several lines (a source snippet); the caller prints `why` on one stderr line.
    let table = text
        .parse::<toml::Table>()
        .map_err(|error| error.to_string().lines().next().unwrap_or_default().to_string())?;
    // `min` rather than the first key: the table's iteration order depends on toml's `preserve_order` feature.
    if let Some(key) = table.keys().filter(|key| *key != "savings_line").min() {
        // A quoted TOML key may hold a newline (`"a\nb"`); escaping keeps `why` on one line.
        return Err(format!("unknown key `{}`", key.escape_debug()));
    }
    match table.get("savings_line") {
        None => Ok(true),
        Some(toml::Value::Boolean(on)) => Ok(*on),
        Some(_) => Err("savings_line must be true or false".to_string()),
    }
}
