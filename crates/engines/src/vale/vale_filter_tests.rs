//! Unit test for vale_filter.rs: the exact argument, as fixtures/fixtures.txt records the runs it was measured with.

use super::vale_filter;

#[test]
fn vale_filter_cases() {
    let rows = [
        (
            "Vale.Spelling",
            "--filter=.Name == \"Vale.Spelling\" || (none(Rules, {.Name == \"Vale.Spelling\"}) && int(\"unknown\") == 0)",
        ),
        (
            "write-good.Passive",
            "--filter=.Name == \"write-good.Passive\" || (none(Rules, {.Name == \"write-good.Passive\"}) && int(\"unknown\") == 0)",
        ),
    ];
    for (check, expected) in rows {
        assert_eq!(vale_filter(check), expected, "{check}");
    }
}
