//! ZUT for unknown_ruleset.rs: PMD 7.29.0's real stderr for an unknown rule and an unknown category
//! (fixtures/, see fixtures.txt), picked out when it names a part of the check.

use super::unknown_ruleset;
use procio::Output;

const NO_RULE: &str = "[WARN] No rules found in ruleset category/java/bestpractices.xml/NoSuchRule";
const NO_CATEGORY: &str = "[ERROR] Cannot load ruleset category/java/nosuch.xml/Foo: Cannot resolve rule/ruleset reference 'category/java/nosuch.xml/Foo'.  Make sure the resource is a valid file or URL and is on the CLASSPATH. Use --debug (or a fine log level) to see the current classpath.";

#[test]
fn unknown_ruleset_is_pmd_words_about_a_part_of_the_check() {
    let rule: &[u8] = include_bytes!("fixtures/unknown_rule.stderr");
    let category: &[u8] = include_bytes!("fixtures/unknown_category.stderr");
    let rows: [(&str, &[u8], &str, Option<&str>); 7] = [
        ("an unknown rule", rule, "category/java/bestpractices.xml/NoSuchRule", Some(NO_RULE)),
        ("an unknown rule in a comma list", rule, "category/java/design.xml,category/java/bestpractices.xml/NoSuchRule", Some(NO_RULE)),
        ("an unknown category", category, "category/java/nosuch.xml/Foo", Some(NO_CATEGORY)),
        ("another rule", rule, "category/java/bestpractices.xml/UnusedPrivateField", None),
        ("a rule the named one only starts with", rule, "category/java/bestpractices.xml/NoSuch", None),
        ("another category", category, "category/java/nosuch.xml", None),
        ("a run with no such words", include_bytes!("fixtures/parse_error.stderr"), "category/java/bestpractices.xml/UnusedPrivateField", None),
    ];
    for (name, stderr, check, want) in rows {
        let out = Output { code: Some(0), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(unknown_ruleset(&out, check), want.map(str::to_string), "{name}");
    }
}
