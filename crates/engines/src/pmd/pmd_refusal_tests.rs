//! ZUT for pmd_refusal.rs: which checks are refused and the reason given, written out from the contract.

use super::pmd_refusal;

const NOT_A_REFERENCE: &str = "is not a PMD Java category or rule reference (category/java/<category>.xml[/<Rule>]); a ruleset file or URL is refused, since the rule file is the only home for rules and their exceptions";

#[test]
fn pmd_refusal_refuses_all_but_java_category_and_rule_references() {
    let rows: [(&str, &str, Option<String>); 13] = [
        ("a rule", "category/java/bestpractices.xml/UnusedPrivateField", None),
        ("a whole category", "category/java/errorprone.xml", None),
        ("a comma list", "category/java/design.xml/GodClass,category/java/security.xml", None),
        ("a category PMD lacks", "category/java/nosuch.xml/Foo", None),
        ("a rule name with digits", "category/java/codestyle.xml/Rule2", None),
        ("empty", "", Some("pmd check ``: an empty part is not a PMD rule reference".into())),
        ("a trailing comma", "category/java/design.xml,", Some("pmd check `category/java/design.xml,`: an empty part is not a PMD rule reference".into())),
        ("another language", "category/apex/design.xml", Some("pmd check `category/apex/design.xml`: `category/apex/design.xml` is not Java, the only language tm runs PMD on".into())),
        ("a ruleset file", "ruleset.xml", Some(format!("pmd check `ruleset.xml`: `ruleset.xml` {NOT_A_REFERENCE}"))),
        ("a URL", "https://example.com/r.xml", Some(format!("pmd check `https://example.com/r.xml`: `https://example.com/r.xml` {NOT_A_REFERENCE}"))),
        ("a lowercase rule", "category/java/design.xml/godClass", Some(format!("pmd check `category/java/design.xml/godClass`: `category/java/design.xml/godClass` {NOT_A_REFERENCE}"))),
        ("a path below a rule", "category/java/design.xml/GodClass/X", Some(format!("pmd check `category/java/design.xml/GodClass/X`: `category/java/design.xml/GodClass/X` {NOT_A_REFERENCE}"))),
        ("not .xml", "category/java/design", Some(format!("pmd check `category/java/design`: `category/java/design` {NOT_A_REFERENCE}"))),
    ];
    for (name, check, want) in rows {
        assert_eq!(pmd_refusal(check), want, "{name}");
    }
}
