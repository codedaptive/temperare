//! `case_rel_path`: how a baseline case file's name is read, the one rule the writer and the judge share.

/// Case file `name` as a relative path inside its case folder. The name is read as text, as rules'
/// `check_case_file_name` reads it, with `/` and `\` both separating its parts. An empty name, a name that starts
/// with a separator (a root), starts with a drive (an ASCII letter then `:`), or has a part that is exactly `..` is
/// refused: the why says it is not a relative path inside the case folder. rules refuses the names that could leave
/// the folder (a separator start, a drive, a `..` part) when it reads a set; this is a second guard. Otherwise the
/// name's non-empty parts other than `.`, joined by `/`. Pure: it never canonicalizes and never touches the disk.
pub(crate) fn case_rel_path(name: &str) -> Result<String, String> {
    let drive = matches!(name.as_bytes(), [letter, b':', ..] if letter.is_ascii_alphabetic());
    let parts = name.split(['/', '\\']);
    if name.is_empty() || drive || name.starts_with(['/', '\\']) || parts.clone().any(|part| part == "..") {
        return Err(format!("case file \"{name}\" is not a relative path inside the case folder"));
    }
    Ok(parts.filter(|part| !part.is_empty() && *part != ".").collect::<Vec<_>>().join("/"))
}
