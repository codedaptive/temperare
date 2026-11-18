//! Done 8: named only inside (mission must match 10). "A rule that a command is run only inside one kit reports a use
//! of it in another kit and passes a use inside." One ast-grep rule matching the command, with `outside_kits` naming the
//! kit it belongs to (SPEC-rules WO-rules-8, S-25). The same call sits in both kits; only the one outside is reported,
//! and its owner is the kit that holds the file (S-9).

use crate::check::Check;
use crate::repo::Repo;
use crate::tm::tm;

const RULES: &str = r#"version = 1
owner = "repo"

[kits.app]
paths = ["app/**"]
owner = "app"

[kits.deploy]
paths = ["deploy/**"]
owner = "deploy"

[[rule]]
id = "git-only-in-deploy"
engine = "ast-grep"
check = "rules/git-only-in-deploy.yml"
paths = ["**/*.rs"]
outside_kits = ["deploy"]
severity = "error"
fix = "Run git through the deploy kit; only deploy starts git."
"#;

const AST_GREP_RULE: &str = r#"id: git-only-in-deploy
language: rust
message: runs git outside the deploy kit
severity: error
rule:
  pattern: Command::new("git")
"#;

// Line 4 of each starts git with the same call, Command::new("git"); only the argument differs.
const DEPLOY: &str = "use std::process::Command;\n\npub fn push() {\n    \
                      let _ = Command::new(\"git\").arg(\"push\").status();\n}\n";
const APP: &str = "use std::process::Command;\n\npub fn sync() {\n    \
                   let _ = Command::new(\"git\").arg(\"pull\").status();\n}\n";

#[test]
fn a_command_named_only_inside_one_kit_is_reported_in_another_and_passes_inside() {
    let mut check = Check::new(
        "a_command_named_only_inside_one_kit_is_reported_in_another_and_passes_inside",
        "kits app and deploy; rule git-only-in-deploy (ast-grep, outside_kits deploy); line 4 of deploy/src/push.rs \
         and of app/src/sync.rs each call Command::new(\"git\")",
    );
    let repo = Repo::new();
    repo.write("temperare.toml", RULES);
    repo.write("rules/git-only-in-deploy.yml", AST_GREP_RULE);
    repo.write("deploy/src/push.rs", DEPLOY);
    repo.write("app/src/sync.rs", APP);
    repo.commit("two kits");

    let args = ["check", "--json"];
    let a = tm(&repo, &args);
    check.saw_tm(&args, a.code, &a.json);
    assert_eq!(a.code, 1, "the use outside the kit refuses: {:#}\n{}", a.json, a.stderr);
    assert_eq!(
        a.places(),
        vec![(String::from("app/src/sync.rs"), 4)],
        "only the use in app; the same use inside deploy passes: {:#}",
        a.json
    );
    let f = &a.findings()[0];
    assert_eq!(f["rule"], "git-only-in-deploy");
    assert_eq!(f["engine"], "ast-grep");
    assert_eq!(f["owner"], "app", "the owner is the kit holding the file");
    assert_eq!(f["fix"], "Run git through the deploy kit; only deploy starts git.");
}
