//! ZUT for check_off_why.rs, for MD013 (`line-length`, tag `line_length`): the first `default` sets the baseline, then
//! the last key naming the rule by name, alias or tag, in any case, decides. A row's expected key is the one the why
//! must name (with the config and the rule), None when the rule stays on.

use super::{check_off_why, MdRule};

#[test]
fn check_off_why_cases() {
    let md013 = MdRule { names: &["MD013", "line-length"], tags: &["line_length"] };
    let rows: [(&str, &[(&str, bool)], Option<&str>); 13] = [
        ("no keys", &[], None),
        ("id off", &[("MD013", false)], Some("MD013")),
        ("id in lower case off", &[("md013", false)], Some("md013")),
        ("alias off", &[("line-length", false)], Some("line-length")),
        ("tag off, upper case", &[("LINE_LENGTH", false)], Some("LINE_LENGTH")),
        ("default off", &[("default", false)], Some("default")),
        ("default off, id on", &[("default", false), ("MD013", true)], None),
        ("later default is no baseline", &[("MD013", false), ("default", true)], Some("MD013")),
        ("first default decides", &[("Default", false), ("default", true)], Some("Default")),
        ("last key wins on", &[("MD013", false), ("line-length", true)], None),
        ("last key wins off", &[("MD013", true), ("line_length", false)], Some("line_length")),
        ("another rule off", &[("MD009", false)], None),
        ("another rule's tag off", &[("headings", false)], None),
    ];
    for (label, settings, key) in rows {
        let settings: Vec<(String, bool)> = settings.iter().map(|(key, on)| (key.to_string(), *on)).collect();
        let why = check_off_why(&md013, "docs/.markdownlint.json", &settings);
        match key {
            None => assert_eq!(why, None, "{label}"),
            Some(key) => assert!(
                why.as_ref().is_some_and(|why| {
                    why.starts_with("docs/.markdownlint.json ") && why.contains(" MD013 ") && why.contains(&format!("`{key}`"))
                }),
                "{label}: {why:?}"
            ),
        }
    }
}
