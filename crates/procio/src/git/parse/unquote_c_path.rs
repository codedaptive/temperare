//! `unquote_c_path`: the bytes of a path git printed C-quoted.

/// The bytes of a path git printed C-quoted (core.quotePath), or `None` when `text` is not one.
///
/// The text starts and ends with `"`. Inside, `\a \b \t \n \v \f \r \" \\` and three-digit octal `\ooo` each
/// stand for one byte; every other byte stands for itself. A text not starting with `"` is `None`: the caller
/// reads unquoted paths itself. No closing quote, bytes after it, an unknown escape or a bad octal is `None`.
pub(crate) fn unquote_c_path(text: &[u8]) -> Option<Vec<u8>> {
    let mut rest = text.strip_prefix(b"\"")?;
    let mut out = Vec::new();
    loop {
        match rest {
            [] => return None,
            [b'"', after @ ..] => return after.is_empty().then_some(out),
            [b'\\', after @ ..] => {
                let (byte, tail) = unescape(after)?;
                out.push(byte);
                rest = tail;
            }
            [byte, after @ ..] => {
                out.push(*byte);
                rest = after;
            }
        }
    }
}

/// The byte one escape stands for and the text after it; `text` starts just after the backslash.
fn unescape(text: &[u8]) -> Option<(u8, &[u8])> {
    let (&first, after) = text.split_first()?;
    let byte = match first {
        b'a' => 0x07,
        b'b' => 0x08,
        b't' => b'\t',
        b'n' => b'\n',
        b'v' => 0x0B,
        b'f' => 0x0C,
        b'r' => b'\r',
        b'"' => b'"',
        b'\\' => b'\\',
        _ => return octal(text),
    };
    Some((byte, after))
}

/// A three-digit octal byte. The first digit stops at 3 because `\377` is the largest byte.
fn octal(text: &[u8]) -> Option<(u8, &[u8])> {
    match text {
        [high @ b'0'..=b'3', mid @ b'0'..=b'7', low @ b'0'..=b'7', after @ ..] => {
            Some((((high - b'0') << 6) | ((mid - b'0') << 3) | (low - b'0'), after))
        }
        _ => None,
    }
}
