//! ZUT for md_rules_from.rs: the release named is the markdownlint library markdownlint-cli2 0.24.2 bundles, as its
//! `--help` banner prints it (fixtures/fixtures.txt).

use super::MD_RULES_FROM;

#[test]
fn md_rules_from_names_markdownlint_0_42_2() {
    assert_eq!(MD_RULES_FROM, "markdownlint 0.42.2");
}
