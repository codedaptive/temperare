//! Unit test for include_targets.rs: the top-level `include:` scalar and its items read, block or flow, escapes decoded; an
//! `include` anywhere else not one; text YAML cannot read an error naming its line.

use super::include_targets;

#[test]
fn include_targets_cases() {
    let rows: [(&str, &str, Result<&[&str], usize>); 17] = [
        ("no include", "linter:\n  rules:\n    - a\n", Ok(&[])),
        ("scalar", "include: package:lints/recommended.yaml\n", Ok(&["package:lints/recommended.yaml"])),
        ("quoted scalar, comment", "include: 'base.yaml' # shared\n", Ok(&["base.yaml"])),
        ("indented list", "include:\n  - a.yaml\n\n  - \"b.yaml\"\nlinter:\n", Ok(&["a.yaml", "b.yaml"])),
        ("column-0 list", "include:\n- a.yaml\n", Ok(&["a.yaml"])),
        ("byte-order mark", "\u{feff}include: a.yaml\n", Ok(&["a.yaml"])),
        ("item named include", "include:\n  - include.yaml\n", Ok(&["include.yaml"])),
        ("quoted key", "\"include\": a.yaml\n", Ok(&["a.yaml"])),
        ("nested key", "analyzer:\n  include: a.yaml\n", Ok(&[])),
        ("other key", "includes: a.yaml\n", Ok(&[])),
        ("no target", "include:\nlinter:\n", Ok(&[])),
        ("escaped key (measured)", "\"\\x69nclude\": ign.yaml\n", Ok(&["ign.yaml"])),
        ("flow document", "{\"inc\\x6cude\": a.yaml}\n", Ok(&["a.yaml"])),
        ("flow list", "include: [a.yaml]\n", Ok(&["a.yaml"])),
        ("scalar on the next line", "include:\n  a.yaml\n", Ok(&["a.yaml"])),
        ("quote left open", "include: 'a.yaml\n", Err(1)),
        ("flow after a list", "include:\n  - a.yaml\nanalyzer: {include: b.yaml}\n", Ok(&["a.yaml"])),
    ];
    let mut failed = Vec::new();
    for (name, text, want) in rows {
        let got = include_targets(text);
        let ok = match (&got, want) {
            (Ok(targets), Ok(want)) => targets == want,
            (Err(why), Err(number)) => why.starts_with(&format!("line {number}: ")),
            _ => false,
        };
        if !ok {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
