//! Done 9 (markdownlint-cli2): a rule naming the engine reports a known violation, in a
//! file the test made, with its rule, its place and its fix text, and the report lists the engine's version; with the
//! engine off the PATH, the same rule reports "not available" (exit 3). One rule, `engine = "markdownlint"` (the
//! rule file's name for markdownlint-cli2), `check = "MD013"` (line length, 80 by default; the repository has no
//! markdownlint config); example.md has a 100-character line 3, clean.md none over 80. cctao runs with PATH the folders
//! that give it markdownlint-cli2 and node (`cctao_path_for`) then /usr/bin:/bin: markdownlint-cli2 is a
//! `#!/usr/bin/env node` script, and the entry needs both markdownlint-cli2 and node, so cctao looks for each on that
//! PATH. Only when markdownlint-cli2 is not on the PATH `cctao` gets (`on_cctao_path`, which
//! runs nothing) is the reported case NOT RUN, with `cctao`'s reason, never PASS; `cctao` must then report the
//! rule not available.

use serde_json::json;

use crate::absent_engine_reason::absent_engine_reason;
use crate::check::Check;
use crate::not_available_pairs::not_available_pairs;
use crate::probed_version::probed_version;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;
use crate::cctao_path_for::cctao_path_for;

const RULE: &str = "md-line-length";
const FIX: &str = "Wrap the line at 80 characters.";

const RULES: &str = r#"version = 1

[[rule]]
id = "md-line-length"
engine = "markdownlint"
check = "MD013"
severity = "error"
fix = "Wrap the line at 80 characters."
"#;

const CLEAN: &str = "# Clean\n\nA clean line.\n";

// git is in /usr/bin; markdownlint-cli2 is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// A heading, a blank line, then line 3: nineteen `word ` and `words`, 100 characters.
fn example() -> String {
    format!("# Title\n\n{}words\n", "word ".repeat(19))
}

/// A repository holding the rule file, example.md and clean.md, committed.
fn markdown_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("example.md", &example());
    repo.write("clean.md", CLEAN);
    repo.commit("markdown files");
    repo
}

#[test]
fn a_markdownlint_violation_is_reported_with_its_rule_place_fix_and_version() {
    let mut check = Check::new(
        "a_markdownlint_violation_is_reported_with_its_rule_place_fix_and_version",
        "rule md-line-length (markdownlint, MD013), no markdownlint config; example.md line 3 is 100 characters, \
         clean.md has no line over 80; PATH is markdownlint-cli2's and node's folders then \
         /usr/bin:/bin",
    );
    let repo = markdown_repo();
    let args = ["check", "--json"];
    let path = cctao_path_for(&["markdownlint-cli2", "node"]);
    let a = cctao_with_path(&repo, &args, Some(&path));
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["markdownlint-cli2"], RULE) {
        check.not_run(why);
        return;
    }
    assert_eq!(a.code, 1, "a quality error is the refusal: {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, in example.md; clean.md passes: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "markdownlint");
    assert_eq!(f[0]["path"], "example.md");
    assert_eq!(f[0]["line"], 3);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("markdownlint-cli2", &["--help"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "markdownlint", "version": version }]), "{:#}", a.json);
}

#[test]
fn markdownlint_off_path_is_not_available() {
    let mut check = Check::new(
        "markdownlint_off_path_is_not_available",
        "rule md-line-length (markdownlint, MD013) over the same files; cctao runs with PATH /usr/bin:/bin, which holds \
         no markdownlint-cli2",
    );
    let repo = markdown_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    assert_eq!(not_available_pairs(&a), vec![(RULE.to_string(), "markdownlint".to_string())], "{:#}", a.json);
}
