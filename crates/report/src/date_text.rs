//! `date_text`: a calendar day as its `YYYY-MM-DD` text.

/// The `YYYY-MM-DD` text of the day, each part zero-padded (the year to 4 digits). `rules::Date` has no `Display`,
/// so the report writes the text from the same numbers it builds its `Date` from.
pub(crate) fn date_text(year: u16, month: u8, day: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}
