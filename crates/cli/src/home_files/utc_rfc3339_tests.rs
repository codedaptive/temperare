//! ZUT for utc_rfc3339: Unix seconds to UTC RFC 3339 text, each expected date worked out by hand.

use super::utc_rfc3339;

#[test]
fn utc_rfc3339_writes_the_utc_second() {
    let rows: [(&str, u64, &str); 5] = [
        ("epoch", 0, "1970-01-01T00:00:00Z"),
        ("non-midnight", 1_795_680_000, "2026-11-26T08:00:00Z"),
        ("leap day", 951_782_400, "2000-02-29T00:00:00Z"),
        ("year-end second", 1_704_067_199, "2023-12-31T23:59:59Z"),
        ("century without a leap day", 4_107_542_400, "2100-03-01T00:00:00Z"),
    ];
    for (case, unix_secs, expected) in rows {
        assert_eq!(utc_rfc3339(unix_secs), expected, "{case}");
    }
}
