//! Done 3: a house rule. "'No stored Bool on a persisted entity' is stated in a rule file that lists the kit's entity
//! types. A stored Bool added to a listed type is reported in a Swift file and in a Rust file. A computed accessor, and
//! a Bool on a type that is not listed, pass." ast-grep does the reading; the entity types reach its rule through
//! `params` (SPEC-engines, params into ast-grep).

use crate::repo::Repo;
use crate::tm::tm;

const RULES: &str = r#"version = 1

[[rule]]
id = "no-stored-bool"
engine = "ast-grep"
check = "rules/no-stored-bool.yml"
paths = ["**/*.swift", "**/*.rs"]
severity = "error"
fix = "Store a bitmap field; expose the Bool as a computed accessor."
params = { entity_types = ["Order", "Invoice"] }
"#;

const AST_GREP_RULE: &str = r#"id: no-stored-bool-rust
language: rust
message: stored Bool on a persisted entity
severity: error
rule:
  kind: field_declaration
  has:
    field: type
    regex: ^bool$
  inside:
    kind: struct_item
    stopBy: end
    has:
      field: name
      regex: ^({{entity_types}})$
---
id: no-stored-bool-swift
language: swift
message: stored Bool on a persisted entity
severity: error
rule:
  kind: property_declaration
  has:
    kind: type_annotation
    regex: Bool
  not:
    has:
      kind: computed_property
  inside:
    kind: class_declaration
    stopBy: end
    has:
      kind: type_identifier
      regex: ^({{entity_types}})$
"#;

// Line 3 of each is the stored Bool on a listed type; the rest must pass.
const SWIFT: &str = "final class Order {\n    var id: Int = 0\n    var isArchived: Bool = false\n    \
                     var isOpen: Bool { id > 0 }\n}\n\nstruct Settings {\n    var darkMode: Bool = false\n}\n";
const RUST: &str = "pub struct Order {\n    pub id: u64,\n    pub is_paid: bool,\n}\n\nimpl Order {\n    \
                    pub fn is_open(&self) -> bool {\n        !self.is_paid\n    }\n}\n\npub struct Settings {\n    \
                    pub dark_mode: bool,\n}\n";

#[test]
fn a_stored_bool_on_a_listed_type_is_reported_in_swift_and_rust() {
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("rules/no-stored-bool.yml", AST_GREP_RULE);
    repo.write("Sources/Model.swift", SWIFT);
    repo.write("src/model.rs", RUST);
    repo.commit("entities");
    let a = tm(&repo, &["check", "--json"]);
    assert_eq!(a.code, 1, "the house rule refuses: {:#}\n{}", a.json, a.stderr);
    assert_eq!(
        a.places(),
        vec![(String::from("Sources/Model.swift"), 3), (String::from("src/model.rs"), 3)],
        "the two stored Bools on Order only; the computed accessor and Settings pass: {:#}",
        a.json
    );
    for f in a.findings() {
        assert_eq!(f["rule"], "no-stored-bool");
        assert_eq!(f["fix"], "Store a bitmap field; expose the Bool as a computed accessor.");
    }
}
