//! `parse_hunk_header`: the counts and new start of a unified-diff hunk header.

/// `(old_count, new_start, new_count)` from a hunk header `@@ -a[,b] +c[,d] @@[ section text]`, or `None` when
/// the line is not one.
///
/// An omitted count is 1, as in git's unified diff; `+c,0` (a hunk that only removes lines) gives new_count 0.
pub(crate) fn parse_hunk_header(line: &[u8]) -> Option<(u32, u32, u32)> {
    let rest = line.strip_prefix(b"@@ -")?;
    let (old, rest) = split_at_space(rest)?;
    let (new, tail) = split_at_space(rest.strip_prefix(b"+")?)?;
    if tail != b"@@" && !tail.starts_with(b"@@ ") {
        return None;
    }
    let (_, old_count) = range(old)?;
    let (new_start, new_count) = range(new)?;
    Some((old_count, new_start, new_count))
}

/// The bytes before the first space and those after it.
fn split_at_space(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let at = bytes.iter().position(|byte| *byte == b' ')?;
    Some((&bytes[..at], &bytes[at + 1..]))
}

/// `start[,count]` as `(start, count)`, the count 1 when omitted.
fn range(bytes: &[u8]) -> Option<(u32, u32)> {
    match bytes.iter().position(|byte| *byte == b',') {
        Some(at) => Some((number(&bytes[..at])?, number(&bytes[at + 1..])?)),
        None => Some((number(bytes)?, 1)),
    }
}

/// A decimal number of ASCII digits only: `str::parse` alone would also take a sign.
fn number(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    std::str::from_utf8(bytes).ok()?.parse().ok()
}
