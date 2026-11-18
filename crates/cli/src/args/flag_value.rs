//! `flag_value`: the value that follows a flag, shared by the `tm rules`, `tm check` and `tm install` parsers.

use super::UsageError;

/// The argument after `flag` in `rest`. It is missing when `flag` is last or the next argument starts with `-`
/// (that is another flag, not a value); the error's fix shows the form, `Write <flag> <form>.`
pub(super) fn flag_value<'a>(
    flag: &str,
    form: &str,
    rest: &mut impl Iterator<Item = &'a String>,
) -> Result<String, UsageError> {
    match rest.next() {
        Some(value) if !value.starts_with('-') => Ok(value.clone()),
        _ => Err(UsageError {
            what: format!("{flag} needs a value"),
            fix: format!("Write {flag} {form}."),
        }),
    }
}
