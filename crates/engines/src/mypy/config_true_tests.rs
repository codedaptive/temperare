//! ZUT for config_true.rs: each true word in mixed case, the false words, other words and empty text.

use super::config_true;

#[test]
fn config_true_reads_configparser_truth_words() {
    let rows: [(&str, bool); 11] = [
        ("1", true),
        ("Yes", true),
        ("tRUE", true),
        ("ON", true),
        ("0", false),
        ("no", false),
        ("False", false),
        ("off", false),
        ("maybe", false),
        ("y", false),
        ("", false),
    ];
    let failed: Vec<String> =
        rows.into_iter().filter(|(value, want)| config_true(value) != *want).map(|(value, _)| format!("{value:?}")).collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}
