//! Unit test for cop_excludes.rs: Exclude lists read from real `--show-cops` dumps (see fixtures/fixtures.txt), whose
//! absolute /private/tmp paths are kept as rubocop printed them.

use super::cop_excludes;

const REPO: &str = include_str!("fixtures/show_cops_repo.txt");
const STOCK: &str = include_str!("fixtures/show_cops_stock.txt");

#[test]
fn cop_excludes_reads_one_cops_exclude_entries_as_printed() {
    let cases: [(&str, &str, &str, Vec<&str>); 8] = [
        ("todo exclude", REPO, "Lint/UselessAssignment", vec!["\"/private/tmp/rubocop-fx.VqLSFl/repo/a.rb\""]),
        ("config exclude", REPO, "Style/StringLiterals", vec!["\"/private/tmp/rubocop-fx.VqLSFl/repo/legacy/**/*\""]),
        (
            "stock exclude in the repo dump",
            REPO,
            "Style/Documentation",
            vec!["\"/private/tmp/rubocop-fx.VqLSFl/repo/spec/**/*\"", "\"/private/tmp/rubocop-fx.VqLSFl/repo/test/**/*\""],
        ),
        (
            "stock exclude",
            STOCK,
            "Style/Documentation",
            vec!["\"/private/tmp/rubocop-fx.VqLSFl/repo/spec/**/*\"", "\"/private/tmp/rubocop-fx.VqLSFl/repo/test/**/*\""],
        ),
        ("no exclude", STOCK, "Lint/UselessAssignment", vec![]),
        ("another key's list is not an exclude", STOCK, "Style/StringLiterals", vec![]),
        ("no block", REPO, "Style/NoSuchCop", vec![]),
        ("a name the header only ends with", REPO, "Documentation", vec![]),
    ];
    let mut failed = Vec::new();
    for (name, show, cop, want) in cases {
        let result = std::panic::catch_unwind(|| assert_eq!(cop_excludes(show, cop), want, "{name}"));
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
