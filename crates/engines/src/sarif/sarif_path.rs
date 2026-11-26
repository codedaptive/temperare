//! `sarif_path`: a SARIF artifact URI, as the path it names.

use crate::windows_path::windows_path;

/// The path `uri` names: after `file://` when it starts with that, otherwise the URI itself (a path relative to where
/// the engine ran), with each `%XX` read as the byte it encodes and the bytes read as lossy UTF-8. A `%` not followed
/// by two hex digits stays as it is. `windows` is the platform as data (the caller passes `cfg!(windows)`): when true,
/// the decoded path goes through `windows_path`, so `file:///C:/x` (or `file:///C%3A/x`, decoded first) gives `C:\x`.
/// A path with no drive comes back as decoded on both platforms: a relative URI, and the four-slash
/// `file:////server/share` URI, which stays `//server/share`.
pub(crate) fn sarif_path(uri: &str, windows: bool) -> String {
    let encoded = uri.strip_prefix("file://").unwrap_or(uri).as_bytes();
    let mut bytes = Vec::with_capacity(encoded.len());
    let mut at = 0;
    while at < encoded.len() {
        match (encoded[at], hex_byte(&encoded[at + 1..])) {
            (b'%', Some(byte)) => {
                bytes.push(byte);
                at += 3;
            }
            (other, _) => {
                bytes.push(other);
                at += 1;
            }
        }
    }
    let path = String::from_utf8_lossy(&bytes);
    if windows { windows_path(&path) } else { path.into_owned() }
}

/// The byte two hex digits at the start of `digits` encode; None when it does not start with two.
fn hex_byte(digits: &[u8]) -> Option<u8> {
    let [high, low] = [hex_digit(*digits.first()?)?, hex_digit(*digits.get(1)?)?];
    Some(high * 16 + low)
}

/// The value of one ASCII hex digit; None for any other byte.
fn hex_digit(byte: u8) -> Option<u8> {
    char::from(byte).to_digit(16).and_then(|value| u8::try_from(value).ok())
}
