//! `detekt_refusal`: why cctao cannot read a detekt check as names, if it cannot.

/// The reason `check` is refused; None when every part of it, split on `,`, is a well-formed name: an ASCII letter,
/// then only ASCII letters, digits or `-` (`MagicNumber`, `empty-blocks`). The first part that is not (an empty part,
/// `detekt.style.MagicNumber`, `Magic Number`) is named in the reason. A well-formed name that is not in cctao's
/// list (`ALL`, `Style`) passes here; `detekt_unlisted` makes it not available.
pub(crate) fn detekt_refusal(check: &str) -> Option<String> {
    let bad = check.split(',').find(|part| !well_formed(part))?;
    let named = if bad.is_empty() { "an empty name".to_string() } else { format!("`{bad}`") };
    Some(format!(
        "detekt check has {named}, which is not a well-formed rule or rule-set name: cctao picks detekt's rules and rule sets by name, comma-separated, each an ASCII letter followed by ASCII letters, digits or `-`, such as `MagicNumber` or `empty-blocks`"
    ))
}

fn well_formed(part: &str) -> bool {
    let mut bytes = part.bytes();
    bytes.next().is_some_and(|first| first.is_ascii_alphabetic())
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'-')
}
