//! ZUT for left_out.rs: the stock target list of real rubocop (list_stock.txt: Rakefile, a.rb, legacy/d.rb,
//! lib/b.rb; see fixtures/fixtures.txt) against the files a check run inspected, within a rule's paths.

use super::left_out;
use rules::{RelPath, RuleFile};
use std::path::Path;

const LIST: &str = include_str!("fixtures/list_stock.txt");

#[test]
fn left_out_names_the_first_stock_file_in_scope_the_check_run_missed() {
    let all = ["Rakefile", "a.rb", "legacy/d.rb", "lib/b.rb"];
    let cases: [(&str, &str, &[&str], Option<&str>); 6] = [
        ("all inspected", "", &all, None),
        ("vendor dropped by the stock list too", "", &["Rakefile", "a.rb", "legacy/d.rb", "lib/b.rb", "vendor/c.rb"], None),
        ("one left out", "", &["Rakefile", "a.rb", "lib/b.rb"], Some("legacy/d.rb")),
        ("the first of two left out", "", &["a.rb", "lib/b.rb"], Some("Rakefile")),
        ("left out outside the rule's paths", "paths = [\"lib/**\"]\n", &["lib/b.rb"], None),
        ("left out inside the rule's paths", "paths = [\"lib/**\", \"legacy/**\"]\n", &["lib/b.rb"], Some("legacy/d.rb")),
    ];
    let mut failed = Vec::new();
    for (name, paths, inspected, want) in cases {
        let result = std::panic::catch_unwind(|| {
            let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"rubocop\"\ncheck = \"Lint/UselessAssignment\"\nfix = \"f\"\n{paths}");
            let file = RuleFile::parse(&text, Path::new("/w/rb")).unwrap();
            let inspected: Vec<RelPath> = inspected.iter().map(|path| RelPath::new(path)).collect();
            assert_eq!(left_out(LIST, &inspected, &file.rules[0]).as_deref(), want, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
