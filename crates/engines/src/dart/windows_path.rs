//! `windows_path`: a path with a drive, in URL form or Windows form, as the native Windows path, `.` and `..` resolved.

/// `path` as a native Windows path when it starts at a drive: `/X:` (a file URL's path, X an ASCII letter, then `/`,
/// `\` or the end), `X:\` or `X:/`, or the verbatim disk `\\?\X:\`, whose `\\?\` prefix is kept. The segments after
/// the root, the verbatim form's included, are split on `/` and `\`, empty and `.` segments dropped, each `..`
/// dropping the segment before it but never the root (as dart's `Uri.resolve` resolves them, since Windows does not
/// resolve `..` under `\\?\`), and joined with `\`; the root alone gives `X:\` (`\\?\X:\` for the verbatim form). The
/// drive letter keeps its case. Limits: a path with no drive is returned unchanged, and so is UNC (`\\srv\share`,
/// `\\?\UNC\...`), which this does not handle (package_path answers None for a `file://server/share` rootUri), and a
/// drive-relative `X:x`. Pure.
pub(crate) fn windows_path(path: &str) -> String {
    let Some((root, rest)) = drive_root(path) else {
        return path.to_string();
    };
    let mut segments = Vec::new();
    for segment in rest.split(['/', '\\']) {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            _ => segments.push(segment),
        }
    }
    format!("{root}{}", segments.join("\\"))
}

/// The native root `path` starts at (`X:\` or `\\?\X:\`) and the text after its drive; None when it has no drive.
fn drive_root(path: &str) -> Option<(String, &str)> {
    if let Some(rest) = path.strip_prefix(r"\\?\") {
        let (drive, after) = split_drive(rest, false)?;
        return after.starts_with('\\').then(|| (format!(r"\\?\{drive}\"), after));
    }
    let (drive, after) = match path.strip_prefix('/') {
        Some(rest) => split_drive(rest, true)?,
        None => split_drive(path, false)?,
    };
    Some((format!(r"{drive}\"), after))
}

/// `text` split after its drive `X:` when a `/` or `\` follows it, or the end when `bare` allows that.
fn split_drive(text: &str, bare: bool) -> Option<(&str, &str)> {
    let bytes = text.as_bytes();
    let separated = match bytes.get(2) {
        Some(b'/' | b'\\') => true,
        None => bare,
        Some(_) => false,
    };
    (bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && separated).then(|| text.split_at(2))
}
