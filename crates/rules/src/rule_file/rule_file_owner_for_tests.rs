//! ZUT for rule_file_owner_for.rs. Each RuleFile is built by `RuleFile::parse` on in-memory text; the expected owners
//! are the spec's rule: the first kit in file order whose paths hold the file, else the top-level owner.

use crate::{RelPath, RuleFile};
use std::path::Path;

fn owner_of(text: &str, file: &str) -> Option<String> {
    let rules = RuleFile::parse(text, Path::new(".")).expect("valid");
    rules.owner_for(&RelPath::new(file)).map(str::to_string)
}

/// `zeta` comes before `alpha` in the file and both hold `shared/`; `bare` has no owner; the top level owns the rest.
const TEXT: &str = "version = 1
owner = \"repo\"
[kits.zeta]
paths = [\"zeta/**\", \"shared/**\"]
owner = \"zeta-zone\"
[kits.alpha]
paths = [\"alpha/**\", \"shared/**\"]
owner = \"alpha-zone\"
[kits.bare]
paths = [\"bare/**\", \"late/**\"]
[kits.late]
paths = [\"late/**\"]
owner = \"late-zone\"
";

#[test]
fn a_file_in_an_owned_kit_has_the_kit_owner() {
    assert_eq!(owner_of(TEXT, "alpha/src/a.rs").as_deref(), Some("alpha-zone"));
}

#[test]
fn a_file_in_two_kits_has_the_owner_of_the_first_in_file_order() {
    assert_eq!(owner_of(TEXT, "shared/x.rs").as_deref(), Some("zeta-zone"));
}

#[test]
fn a_file_in_no_kit_has_the_top_level_owner() {
    assert_eq!(owner_of(TEXT, "elsewhere/x.rs").as_deref(), Some("repo"));
}

#[test]
fn a_file_in_a_kit_with_no_owner_has_the_top_level_owner() {
    assert_eq!(owner_of(TEXT, "bare/x.rs").as_deref(), Some("repo"));
}

#[test]
fn the_first_holding_kit_without_an_owner_gives_the_top_level_owner_over_a_later_kits() {
    assert_eq!(owner_of(TEXT, "late/x.rs").as_deref(), Some("repo"));
}

#[test]
fn no_owner_anywhere_is_none() {
    assert_eq!(owner_of("version = 1\n[kits.core]\npaths = [\"c/**\"]\n", "c/x.rs"), None);
    assert_eq!(owner_of("version = 1\n", "c/x.rs"), None);
}

#[test]
fn a_blank_kit_owner_gives_the_top_level_owner() {
    let text = "version = 1\nowner = \"repo\"\n[kits.core]\npaths = [\"c/**\"]\nowner = \"  \"\n";
    assert_eq!(owner_of(text, "c/x.rs").as_deref(), Some("repo"));
}

#[test]
fn owners_are_kept_as_written() {
    let text = "version = 1\nowner = \" repo \"\n[kits.core]\npaths = [\"c/**\"]\nowner = \" core \"\n";
    assert_eq!(owner_of(text, "c/x.rs").as_deref(), Some(" core "));
    assert_eq!(owner_of(text, "d/x.rs").as_deref(), Some(" repo "));
}
