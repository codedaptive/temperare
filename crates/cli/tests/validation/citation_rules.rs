//! house@2's rule file with every rule but its three citation rules turned off, shared by the house_2 citation cases.

// house@2's rules other than its three citation rules, each turned off so only those three are in play.
const OFF: [&str; 36] = [
    "tm-house-no-unwrap-rust",
    "tm-house-no-expect-rust",
    "tm-house-no-unsafe-rust",
    "tm-house-no-print-stdout-rust",
    "tm-house-no-todo-python",
    "tm-house-no-todo-swift",
    "tm-house-no-todo-rust",
    "tm-house-no-force-cast-swift",
    "tm-house-no-force-try-swift",
    "tm-house-no-force-unwrapping-swift",
    "tm-house-no-as-conversions-rust",
    "tm-house-no-assign-then-return-python",
    "tm-house-no-let-and-return-rust",
    "tm-house-arg-type-hints-python",
    "tm-house-return-type-hints-python",
    "tm-house-pathlib-python",
    "tm-house-one-public-symbol-rust",
    "tm-house-one-declaration-swift",
    "tm-house-file-name-swift",
    "tm-house-file-length-swift",
    "tm-house-no-stored-bool-swift",
    "tm-house-no-stored-bool-rust",
    "tm-house-bitmap-int64-swift",
    "tm-house-bitmap-int64-rust",
    "tm-house-leading-trailing-swift",
    "tm-house-display-text-literal-swift",
    "tm-house-swift-testing-swift",
    "tm-house-test-ratio",
    "tm-house-test-asserts-sentence-rust",
    "tm-house-test-asserts-sentence-python",
    "tm-house-test-asserts-sentence-swift",
    "tm-house-unit-test-fakes-own-kit-rust",
    "tm-house-unit-test-fakes-own-kit-python",
    "tm-house-test-setup-heavier-rust",
    "tm-house-test-setup-heavier-python",
    "tm-house-test-setup-heavier-swift",
];

/// The rule file: house@2 with every rule but the citation ones turned off.
pub(crate) fn citation_rules() -> String {
    OFF.iter().fold(String::from("version = 1\nbaseline = [\"house@2\"]\n"), |text, id| {
        text + &format!("\n[[override]]\nrule = \"{id}\"\noff = true\n")
    })
}
