use super::parse_periphery;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const SCAN: &[u8] = include_bytes!("fixtures/scan.json");
const ROOT: &str = "/private/var/folders/zc/zl4w__wx2xdfmd5lps9dp2000000gn/T/periphery-scan-fixture.W5ECgzZMGi/package";

#[test]
fn parse_periphery_cases() {
    let hit = |line, col, what: &str| Hit {
        path: RelPath::new("Sources/Probe/main.swift"), line: Some(line), col: Some(col), what: what.into(),
    };
    let unused = hit(1, 6, "function.free unusedHelper(): unused");
    let protocol = hit(2, 10, "protocol Greeter: redundantProtocol");
    let conformance = hit(3, 15, "protocol Greeter: redundantConformance");
    let property = hit(4, 9, "var.instance assigned: assignOnlyProperty");
    let all = vec![unused.clone(), protocol.clone(), conformance.clone(), property.clone()];

    let mut modified: serde_json::Value = serde_json::from_slice(SCAN).unwrap();
    modified[0]["location"] = "/outside/main.swift:1:6".into();
    let outside = serde_json::to_vec(&modified).unwrap();
    modified[0]["location"] = "Sources/Probe/main.swift:1:6".into();
    let relative = serde_json::to_vec(&modified).unwrap();
    modified[0]["location"] = "not-a-location".into();
    let bad_location = serde_json::to_vec(&modified).unwrap();

    let rows: [(&str, &[u8], &str, Option<Vec<Hit>>); 9] = [
        ("all", SCAN, "all", Some(all.clone())),
        ("unused", SCAN, "unused", Some(vec![unused])),
        ("assign-only property", SCAN, "assignOnlyProperty", Some(vec![property.clone()])),
        ("no matching hint", SCAN, "superfluousIgnoreCommand", Some(vec![])),
        ("outside folder is skipped", &outside, "all", Some(vec![protocol, conformance, property])),
        ("relative location", &relative, "all", Some(all)),
        ("bad location", &bad_location, "all", None),
        ("not JSON", b"not JSON", "all", None),
        ("not an array", b"{}", "all", None),
    ];
    for (label, stdout, check, expected) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: vec![] };
        let result = parse_periphery(&out, check, Path::new(ROOT), Path::new(ROOT));
        match expected {
            Some(hits) => assert_eq!(result.unwrap(), hits, "{label}"),
            None => assert!(result.is_err(), "{label}: {result:?}"),
        }
    }
}
