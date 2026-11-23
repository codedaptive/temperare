//! ZUT for ignored_listed.rs: an `ignored` key (any ASCII case, any YAML style, through an alias or a `!!seq` tag)
//! holding an item or a scalar stops the rule naming the file and its line; a null or empty `ignored`, `override` and
//! other keys hide nothing; text tm cannot read (a duplicate key and a `<<` merge among it) stops the rule naming its
//! line.

use super::ignored_listed;

const NAME: &str = "/r/.hadolint.yaml";

/// What a row expects: nothing hidden, `ignored` hiding findings at a line, or text tm cannot read at a line, with a
/// word from yaml_entries' why.
enum Want {
    Clean,
    Hides(usize),
    Unreadable(usize, &'static str),
}

#[test]
fn ignored_listed_cases() {
    use Want::{Clean, Hides, Unreadable};
    let rows: [(&str, &str, Want); 18] = [
        ("flow", "ignored: [DL3006]\n", Hides(1)),
        ("block", "failure-threshold: info\nignored:\n  - DL3006\n", Hides(3)),
        ("quoted key", "\"ignored\": [DL3006]\n", Hides(1)),
        ("quoted item", "ignored:\n  - 'DL3006'\n", Hides(2)),
        ("`Ignored:` key", "Ignored: [DL3006]\n", Hides(1)),
        ("alias", "codes: &c [DL3006]\nignored: *c\n", Hides(2)),
        ("`!!seq` tag", "ignored: !!seq [DL3006]\n", Hides(1)),
        ("a scalar", "ignored: DL3006\n", Hides(1)),
        ("null", "ignored:\n", Clean),
        ("`~`", "ignored: ~\n", Clean),
        ("`[]`", "ignored: []\n", Clean),
        ("override only", "override:\n  error:\n    - DL3006\n", Clean),
        ("`ignored` under override", "override:\n  ignored:\n    - DL3006\n", Clean),
        ("`{}`", "{}\n", Clean),
        ("other keys", "trustedRegistries:\n  - docker.io\nstrict-labels: true\n", Clean),
        ("unreadable YAML", "a:\n    b: 1\n  c: 2\n", Unreadable(3, "did not find expected key")),
        ("duplicate key", "ignored: []\nignored: []\n", Unreadable(2, "duplicate mapping key `ignored`")),
        ("`<<` merge", "defaults: &d\n  ignored: []\n<<: *d\n", Unreadable(3, "a merge key")),
    ];
    let mut failed = Vec::new();
    for (label, text, want) in rows {
        let got = ignored_listed(NAME, text);
        let ok = match (&got, want) {
            (None, Clean) => true,
            (Some(why), Hides(line)) => {
                why == &format!(
                    "/r/.hadolint.yaml: line {line}: `ignored` hides hadolint findings outside the rule file; put \
                     exceptions in the rule file (S-67 (1))"
                )
            }
            (Some(why), Unreadable(line, what)) => {
                why.starts_with(&format!("/r/.hadolint.yaml: line {line}: tm cannot read it: "))
                    && why.contains(what)
                    && why.ends_with("; put exceptions in the rule file")
            }
            _ => false,
        };
        if !ok {
            eprintln!("{label}: got {got:?}");
            failed.push(label);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
