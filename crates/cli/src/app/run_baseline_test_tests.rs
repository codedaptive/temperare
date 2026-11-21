//! ZUT for run_baseline_test.rs, with closure worlds: no disk, no engine. The test sets list their rules out of sort
//! order (tm-zeta before tm-alpha) and the table's default sets name standard@1 before house@1, so a sort would show
//! in the output. Each row measures the whole printed `out` and `err` as literal strings, and counts the case folders
//! made and the rule runs. Two tests take the real world with a `rules_of` that returns no rules, so no other seam of
//! it is called, and read which sets tm tests with and without `--set`.

use super::run_baseline_test::run_baseline_test;
use super::BaselineWorld;
use crate::{BaselineArgs, UsageError};
use engines::{CheckedRules, RuleRun};
use rules::{
    CaseFile, Engine, ExpectedFinding, Finding, Kind, RelPath, Rule, RuleCases, RuleFile, RuleOutcome, Severity,
};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const FAILING_TEXT: &str = "bad\n";

/// rules' reason for tm-x, set aside because its required params are not given.
const ASIDE_WHY: &str =
    "required params not given: entity_types; give them in the params of an [[override]] for \"tm-x\"";

/// A rule whose failing case is `a.py` holding FAILING_TEXT, expected at a.py:1, and whose passing case is `a.py`
/// holding `good\n`.
fn with_cases(id: &str, engine: Engine) -> Rule {
    let mut rule = Rule::new(id, engine, "check");
    rule.embedded_cases = Some(RuleCases {
        failing: vec![CaseFile { path: "a.py".to_string(), text: FAILING_TEXT }],
        passing: vec![CaseFile { path: "a.py".to_string(), text: "good\n" }],
        expected: ExpectedFinding { rule_id: id.to_string(), path: "a.py".to_string(), line: 1 },
    });
    rule
}

/// The test sets: standard@1 (tm-zeta, tm-alpha), house@1 (tm-mid), bare@1 (tm-bare, which carries no cases),
/// change@1 (tm-change, only_with_base, expected at a.py:2), aside@1 (tm-zeta, then tm-x, which carries cases but rules
/// set aside with ASIDE_WHY); any other set is unknown.
fn test_set(set: &str) -> Result<Vec<Rule>, UsageError> {
    match set {
        "standard@1" => Ok(vec![with_cases("tm-zeta", Engine::Ruff), with_cases("tm-alpha", Engine::AstGrep)]),
        "house@1" => Ok(vec![with_cases("tm-mid", Engine::SwiftLint)]),
        "bare@1" => Ok(vec![Rule::new("tm-bare", Engine::Ruff, "F401")]),
        "change@1" => {
            let mut rule = with_cases("tm-change", Engine::Ruff);
            rule.only_with_base = true;
            if let Some(cases) = &mut rule.embedded_cases {
                cases.expected.line = 2;
            }
            Ok(vec![rule])
        }
        "aside@1" => {
            let mut rule = with_cases("tm-x", Engine::Ruff);
            rule.not_applicable = Some(ASIDE_WHY.to_string());
            Ok(vec![with_cases("tm-zeta", Engine::Ruff), rule])
        }
        _ => Err(UsageError {
            what: "unknown baseline set \"nosuch\"".to_string(),
            fix: "Use one of: house.".to_string(),
        }),
    }
}

/// Every rule holds: the failing case gives its expected finding, the passing case none.
fn all_hold(rule: &str, text: &str) -> RuleOutcome {
    let findings = if text == FAILING_TEXT {
        let place = (Engine::Ruff, Kind::Quality, Severity::Error);
        vec![Finding::new(rule, place, RelPath::new("a.py"), 1, None, "w", "f")]
    } else {
        Vec::new()
    };
    RuleOutcome::Findings(findings)
}

/// The failing case gives a finding on line 1 and one on line 2; the passing case none. Only line 2 is added since
/// the base (the world's `added_since`), so a change rule holds only when line 1 is dropped.
fn on_lines_1_and_2(rule: &str, text: &str) -> RuleOutcome {
    let place = (Engine::Ruff, Kind::Quality, Severity::Error);
    let lines: &[u32] = if text == FAILING_TEXT { &[1, 2] } else { &[] };
    RuleOutcome::Findings(
        lines.iter().map(|line| Finding::new(rule, place, RelPath::new("a.py"), *line, None, "w", "f")).collect(),
    )
}

/// tm-mid's engine is not available; the others hold.
fn mid_not_available(rule: &str, text: &str) -> RuleOutcome {
    if rule != "tm-mid" {
        return all_hold(rule, text);
    }
    let why = "swiftlint not found on PATH".to_string();
    RuleOutcome::NotAvailable { rule: rule.to_string(), engine: Engine::SwiftLint, why }
}

/// tm-alpha gives no finding on its failing case; tm-mid's engine is not available; tm-zeta holds.
fn alpha_broken_mid_not_available(rule: &str, text: &str) -> RuleOutcome {
    if rule == "tm-alpha" {
        return RuleOutcome::Findings(Vec::new());
    }
    mid_not_available(rule, text)
}

/// Runs `tm baseline test` with `args` on the test sets, `default_sets` the world's, each case run giving what
/// `outcome` says for its rule and its file's text; (exit, out, err, the sets read, (the case folders made, the rules
/// run)).
fn run(
    args: BaselineArgs,
    default_sets: &[&str],
    outcome: Outcome,
) -> (u8, String, String, Vec<String>, (usize, usize)) {
    let (read, made, written) = (RefCell::new(Vec::new()), RefCell::new(0), RefCell::new(String::new()));
    let ran = RefCell::new(0);
    let rules_of = |set: &str| {
        read.borrow_mut().push(set.to_string());
        test_set(set)
    };
    let make_folder = || {
        *made.borrow_mut() += 1;
        Ok(PathBuf::from("/case"))
    };
    let write_file = |_: &Path, text: &str| {
        *written.borrow_mut() = text.to_string();
        Ok(())
    };
    let run_rules = |rf: &RuleFile, _: &Path, _: &[RelPath]| {
        *ran.borrow_mut() += 1;
        let rule = &rf.rules[0];
        let run = RuleRun::new(rule.id.clone(), rule.engine, outcome(&rule.id, &written.borrow()), None);
        CheckedRules { runs: vec![run], engines: vec![] }
    };
    let world = BaselineWorld {
        default_sets,
        rules_of: &rules_of,
        make_folder: &make_folder,
        write_file: &write_file,
        commit_all: &|_| Ok("base".to_string()),
        added_since: &|_, _| Ok(BTreeMap::from([(PathBuf::from("a.py"), vec![2..3])])),
        remove_folder: &|_| Ok(()),
        run_rules: &run_rules,
    };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = run_baseline_test(&args, &world, &mut out, &mut err);
    let (read, calls) = (read.into_inner(), (made.into_inner(), ran.into_inner()));
    (exit, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap(), read, calls)
}

/// What a case run gives for its rule and its file's text.
type Outcome = fn(&str, &str) -> RuleOutcome;

/// (name, args, default sets, outcome, exit, out, err, sets read, (case folders made, rules run))
type Row = (&'static str, BaselineArgs, Sets, Outcome, u8, &'static str, &'static str, Sets, (usize, usize));

/// A list of sets, as "name@version".
type Sets = &'static [&'static str];

#[test]
fn run_baseline_test_rows() {
    let text = BaselineArgs::default();
    let json = BaselineArgs { json: true, ..BaselineArgs::default() };
    let set = |set: &str| BaselineArgs { set: Some(set.to_string()), json: false };
    // A test world's default_sets, not tm's: tm's own (the ready sets, standard@1 today) is pinned by
    // no_set_tests_the_ready_sets_only.
    let defaults: Sets = &["standard@1", "house@1"];
    let rows: [Row; 12] = [
        (
            "all hold",
            text.clone(),
            defaults,
            all_hold,
            0,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): holds\n\
             house@1 tm-mid (swiftlint): holds\n\
             3 rules: 3 hold, 0 broken, 0 not available, 0 not applicable\n\
             next: nothing to do\n",
            "",
            &["standard@1", "house@1"],
            (6, 6),
        ),
        (
            "one broken and one not available: broken wins",
            text.clone(),
            defaults,
            alpha_broken_mid_not_available,
            1,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): broken\n  \
             failing: broken: gave no finding; expected tm-alpha at a.py:1\n\
             house@1 tm-mid (swiftlint): not available\n  \
             failing: not available: swiftlint not found on PATH\n  \
             passing: not available: swiftlint not found on PATH\n\
             3 rules: 1 hold, 1 broken, 1 not available, 0 not applicable\n\
             next: fix each broken rule or its cases above\n",
            "",
            &["standard@1", "house@1"],
            (6, 6),
        ),
        (
            "one broken and one not available, as JSON",
            json.clone(),
            defaults,
            alpha_broken_mid_not_available,
            1,
            concat!(
                r#"{"counts":{"broken":1,"holds":1,"not_applicable":0,"not_available":1,"rules":3},"exit":1,"#,
                r#""next":"fix each broken rule or its cases above","rules":["#,
                r#"{"engine":"ruff","failing":{"result":"holds","why":null},"#,
                r#""passing":{"result":"holds","why":null},"result":"holds","rule":"tm-zeta","set":"standard@1"},"#,
                r#"{"engine":"ast-grep","#,
                r#""failing":{"result":"broken","why":"gave no finding; expected tm-alpha at a.py:1"},"#,
                r#""passing":{"result":"holds","why":null},"result":"broken","rule":"tm-alpha","set":"standard@1"},"#,
                r#"{"engine":"swiftlint","failing":{"result":"not_available","why":"swiftlint not found on PATH"},"#,
                r#""passing":{"result":"not_available","why":"swiftlint not found on PATH"},"#,
                r#""result":"not_available","rule":"tm-mid","set":"house@1"}]}"#,
                "\n"
            ),
            "",
            &["standard@1", "house@1"],
            (6, 6),
        ),
        (
            "one not available, none broken",
            text.clone(),
            defaults,
            mid_not_available,
            3,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): holds\n\
             house@1 tm-mid (swiftlint): not available\n  \
             failing: not available: swiftlint not found on PATH\n  \
             passing: not available: swiftlint not found on PATH\n\
             3 rules: 2 hold, 0 broken, 1 not available, 0 not applicable\n\
             next: install swiftlint, then run tm baseline test again\n",
            "",
            &["standard@1", "house@1"],
            (6, 6),
        ),
        (
            "a set that cannot be read: every set read first, nothing run",
            text.clone(),
            &["standard@1", "nosuch@1"],
            all_hold,
            2,
            "",
            "tm: unknown baseline set \"nosuch\"\nnext: Use one of: house.\n",
            &["standard@1", "nosuch@1"],
            (0, 0),
        ),
        (
            "a set that cannot be read, as JSON",
            BaselineArgs { set: Some("nosuch@1".to_string()), json: true },
            defaults,
            all_hold,
            2,
            "{\"error\":\"unknown baseline set \\\"nosuch\\\"\",\"exit\":2,\"next\":\"Use one of: house.\"}\n",
            "tm: unknown baseline set \"nosuch\"\nnext: Use one of: house.\n",
            &["nosuch@1"],
            (0, 0),
        ),
        (
            "an only_with_base rule keeps only the findings on added lines",
            set("change@1"),
            defaults,
            on_lines_1_and_2,
            0,
            "change@1 tm-change (ruff): holds\n\
             1 rules: 1 hold, 0 broken, 0 not available, 0 not applicable\n\
             next: nothing to do\n",
            "",
            &["change@1"],
            (2, 2),
        ),
        (
            "--set tests only that set",
            set("house@1"),
            defaults,
            all_hold,
            0,
            "house@1 tm-mid (swiftlint): holds\n\
             1 rules: 1 hold, 0 broken, 0 not available, 0 not applicable\n\
             next: nothing to do\n",
            "",
            &["house@1"],
            (2, 2),
        ),
        (
            "a set-aside rule is not applicable and is not run",
            set("aside@1"),
            defaults,
            all_hold,
            3,
            "aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n  \
             passing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n\
             2 rules: 1 hold, 0 broken, 0 not available, 1 not applicable\n\
             next: each not-applicable rule above says why it was not run\n",
            "",
            &["aside@1"],
            (2, 2),
        ),
        (
            "a set-aside rule, as JSON",
            BaselineArgs { set: Some("aside@1".to_string()), json: true },
            defaults,
            all_hold,
            3,
            concat!(
                r#"{"counts":{"broken":0,"holds":1,"not_applicable":1,"not_available":0,"rules":2},"exit":3,"#,
                r#""next":"each not-applicable rule above says why it was not run","rules":["#,
                r#"{"engine":"ruff","failing":{"result":"holds","why":null},"#,
                r#""passing":{"result":"holds","why":null},"result":"holds","rule":"tm-zeta","set":"aside@1"},"#,
                r#"{"engine":"ruff","failing":{"result":"not_applicable","#,
                r#""why":"required params not given: entity_types; "#,
                r#"give them in the params of an [[override]] for \"tm-x\""},"#,
                r#""passing":{"result":"not_applicable","#,
                r#""why":"required params not given: entity_types; "#,
                r#"give them in the params of an [[override]] for \"tm-x\""},"#,
                r#""result":"not_applicable","rule":"tm-x","set":"aside@1"}]}"#,
                "\n"
            ),
            "",
            &["aside@1"],
            (2, 2),
        ),
        (
            "broken beside a set-aside rule: broken wins",
            text.clone(),
            &["standard@1", "aside@1"],
            alpha_broken_mid_not_available,
            1,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): broken\n  \
             failing: broken: gave no finding; expected tm-alpha at a.py:1\n\
             aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n  \
             passing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n\
             4 rules: 2 hold, 1 broken, 0 not available, 1 not applicable\n\
             next: fix each broken rule or its cases above\n",
            "",
            &["standard@1", "aside@1"],
            (6, 6),
        ),
        (
            "not available beside a set-aside rule: exit 3 with both next parts",
            text.clone(),
            &["house@1", "aside@1"],
            mid_not_available,
            3,
            "house@1 tm-mid (swiftlint): not available\n  \
             failing: not available: swiftlint not found on PATH\n  \
             passing: not available: swiftlint not found on PATH\n\
             aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n  \
             passing: not applicable: required params not given: entity_types; \
             give them in the params of an [[override]] for \"tm-x\"\n\
             3 rules: 1 hold, 0 broken, 1 not available, 1 not applicable\n\
             next: install swiftlint, then run tm baseline test again; \
             each not-applicable rule above says why it was not run\n",
            "",
            &["house@1", "aside@1"],
            (4, 4),
        ),
    ];
    for (name, args, default_sets, outcome, exit, out, err, read, made) in rows {
        let got = run(args, default_sets, outcome);
        let read: Vec<String> = read.iter().map(|set| set.to_string()).collect();
        assert_eq!(got, (exit, out.to_string(), err.to_string(), read, made), "{name}");
    }
}

/// Runs `tm baseline test` with `set` on the real world whose `rules_of` returns no rules; (exit, the sets read).
fn sets_read_by_real(set: Option<&str>) -> (u8, Vec<String>) {
    let read = RefCell::new(Vec::new());
    let rules_of = |set: &str| {
        read.borrow_mut().push(set.to_string());
        Ok(Vec::new())
    };
    let world = BaselineWorld { rules_of: &rules_of, ..BaselineWorld::real() };
    let args = BaselineArgs { set: set.map(str::to_string), json: false };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = run_baseline_test(&args, &world, &mut out, &mut err);
    (exit, read.into_inner())
}

#[test]
fn no_set_tests_the_ready_sets_only() {
    assert_eq!(sets_read_by_real(None), (0, vec!["standard@1".to_string()]));
}

#[test]
fn set_house_still_runs_house() {
    assert_eq!(sets_read_by_real(Some("house@1")), (0, vec!["house@1".to_string()]));
}

#[test]
fn a_rule_with_no_cases_is_broken_and_runs_nothing() {
    let args = BaselineArgs { set: Some("bare@1".to_string()), json: false };
    let (exit, out, err, _, calls) = run(args, &[], all_hold);
    assert_eq!(exit, 1);
    assert_eq!(
        out,
        "bare@1 tm-bare (ruff): broken\n  \
         failing: broken: the set carries no cases for this rule\n  \
         passing: broken: the set carries no cases for this rule\n\
         1 rules: 0 hold, 1 broken, 0 not available, 0 not applicable\n\
         next: fix each broken rule or its cases above\n"
    );
    assert_eq!(err, "");
    assert_eq!(calls, (0, 0));
}
