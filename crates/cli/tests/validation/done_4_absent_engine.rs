//! Done 4: an absent engine. "A rule whose linter is not installed, and a rule that needs Tessera with no Tessera, each
//! report 'not available'." Not available is never clean (mission 5): its own exit code, 3 (S-15).

use crate::check::Check;
use crate::env::tm_env;
use crate::repo::Repo;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;
use crate::tm::tm_with_path;

const RULES: &str = r#"version = 1

[[rule]]
id = "tessera-erosion"
engine = "tessera"
check = "erosion"
severity = "warning"
fix = "Split the file along its seams."

[[rule]]
id = "swift-force-cast"
engine = "swiftlint"
check = "force_cast"
severity = "error"
fix = "Use `as?` and handle nil."
"#;

// git is in /usr/bin; no linter and no `ts` is in either folder.
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

#[test]
fn an_absent_linter_and_absent_tessera_each_report_not_available() {
    let mut check = Check::new(
        "an_absent_linter_and_absent_tessera_each_report_not_available",
        "rules tessera-erosion (tessera) then swift-force-cast (swiftlint); tm runs with PATH /usr/bin:/bin, which \
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
            (String::from("swift-force-cast"), String::from("swiftlint")),
            (String::from("tessera-erosion"), String::from("tessera")),
        ],
        "both rules, each with its engine: {:#}",
        a.json
    );
    // `tm_with_path` keeps only the parsed JSON, so the printed bytes come from a second run in the same environment.
    let home = TempHome::new();
    let out = run_clean(env!("CARGO_BIN_EXE_temperare"), &args, repo.path(), tm_env(Some(PATH_WITHOUT_ENGINES), home.path()));
    let printed = String::from_utf8(out.stdout).unwrap();
    assert!(
        // The rule file names tessera's rule first, against the alphabetical order of both engine names and rule ids,
        // so a list sorted by either fails.
        printed.contains(r#""engines":[{"engine":"tessera","version":null},{"engine":"swiftlint","version":null}]"#),
        "each engine the rule file names, in rule-file order, with no version: {printed}"
    );
}
