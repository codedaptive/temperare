//! Done 4: an absent engine. "A rule whose linter is not installed, and a rule that needs Tessera with no Tessera, each
//! report 'not available'." Not available is never clean: its own exit code, 3.

use crate::check::Check;
use crate::repo::Repo;
use crate::tm::tm_with_path;

const RULES: &str = r#"version = 1

[[rule]]
id = "swift-force-cast"
engine = "swiftlint"
check = "force_cast"
severity = "error"
fix = "Use `as?` and handle nil."

[[rule]]
id = "file-erosion"
engine = "tessera"
check = "erosion"
severity = "warning"
fix = "Split the file along its seams."
"#;

// git is in /usr/bin; no linter and no `ts` is in either folder.
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

#[test]
fn an_absent_linter_and_absent_tessera_each_report_not_available() {
    let mut check = Check::new(
        "an_absent_linter_and_absent_tessera_each_report_not_available",
        "rules swift-force-cast (swiftlint) and file-erosion (tessera); tm runs with PATH /usr/bin:/bin, which \
         holds neither swiftlint nor ts",
    );
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("Sources/App.swift", "let x = y as! Int\n");
    repo.commit("a swift file");
    let args = ["check", "--json"];
    let a = tm_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    let mut missing: Vec<(String, String)> = a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| (n["rule"].as_str().unwrap_or("").into(), n["engine"].as_str().unwrap_or("").into()))
        .collect();
    missing.sort();
    assert_eq!(
        missing,
        vec![
            (String::from("file-erosion"), String::from("tessera")),
            (String::from("swift-force-cast"), String::from("swiftlint")),
        ],
        "both rules, each with its engine: {:#}",
        a.json
    );
}
