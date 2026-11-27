//! ZUT for run_baseline_test.rs, with closure worlds: no disk, no engine. The test sets list their rules out of sort
//! order (tm-zeta before tm-alpha) and the table's default sets name standard@1 before house@1, so a sort would show
//! in the output. Each row measures the whole printed `out` and `err` as literal strings, the sets read, and counts
//! the case folders made and the rule runs. Which sets cctao itself carries and marks ready is rules' data: the RITs in
//! tests/world/baseline_world_real.rs read it.

use super::run_baseline_test::run_baseline_test;
use super::BaselineWorld;
use crate::BaselineArgs;
use engines::{CheckedRules, NotAvailableCause, RuleRun};
use rules::{
    CaseFile, Engine, ExpectedFinding, Finding, Kind, Problem, RelPath, Rule, RuleCases, RuleFile, RuleOutcome,
    Severity,
};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const FAILING_TEXT: &str = "bad\n";

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

/// A params table from its TOML text.
fn table(text: &str) -> toml::Table {
    toml::from_str(text).unwrap()
}

/// The test sets: standard@1 (tm-zeta, tm-alpha), house@1 (tm-mid), bare@1 (tm-bare, which carries no cases and
/// requires entity_types, so it is broken, not not applicable), change@1 (tm-change, only_with_base, expected at
/// a.py:2), aside@1 (tm-zeta, then tm-x, which carries cases and requires entity_types, a param neither case gives, so
/// each case is not applicable and not run), params@1 (tm-p, requiring kit: its params kit = "set-default" and
/// other = "kept", its failing case's kit = "failing-kit", its passing case's kit = "passing-kit"); empty@1 cannot be
/// read and carries no problem; any other set is unknown, its first problem at line 2, column 12 of rules' own text,
/// which cctao must not print, then a second one.
fn test_set(set: &str) -> Result<Vec<Rule>, Vec<Problem>> {
    match set {
        "standard@1" => Ok(vec![with_cases("tm-zeta", Engine::Ruff), with_cases("tm-alpha", Engine::AstGrep)]),
        "house@1" => Ok(vec![with_cases("tm-mid", Engine::SwiftLint)]),
        "bare@1" => {
            let mut rule = Rule::new("tm-bare", Engine::Ruff, "F401");
            rule.required_params = vec!["entity_types".to_string()];
            Ok(vec![rule])
        }
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
            rule.required_params = vec!["entity_types".to_string()];
            Ok(vec![with_cases("tm-zeta", Engine::Ruff), rule])
        }
        "params@1" => {
            let mut rule = with_cases("tm-p", Engine::Ruff);
            rule.required_params = vec!["kit".to_string()];
            rule.params = table("kit = \"set-default\"\nother = \"kept\"");
            rule.failing_params = table("kit = \"failing-kit\"");
            rule.passing_params = table("kit = \"passing-kit\"");
            Ok(vec![rule])
        }
        "empty@1" => Err(Vec::new()),
        _ => Err(vec![
            Problem {
                line: 2,
                col: 12,
                what: "unknown baseline set \"nosuch\"".to_string(),
                fix: "the test world's fix for nosuch".to_string(),
            },
            Problem { line: 3, col: 1, what: "second".to_string(), fix: "not printed".to_string() },
        ]),
    }
}

/// Every rule holds: the failing case gives its expected finding, the passing case none.
fn all_hold(rule: &Rule, text: &str) -> RuleOutcome {
    let findings = if text == FAILING_TEXT {
        let place = (Engine::Ruff, Kind::Quality, Severity::Error);
        vec![Finding::new(&rule.id, place, RelPath::new("a.py"), 1, None, "w", "f")]
    } else {
        Vec::new()
    };
    RuleOutcome::Findings(findings)
}

/// The failing case gives a finding on line 1 and one on line 2; the passing case none. Only line 2 is added since
/// the base (the world's `added_since`), so a change rule holds only when line 1 is dropped.
fn on_lines_1_and_2(rule: &Rule, text: &str) -> RuleOutcome {
    let place = (Engine::Ruff, Kind::Quality, Severity::Error);
    let lines: &[u32] = if text == FAILING_TEXT { &[1, 2] } else { &[] };
    RuleOutcome::Findings(
        lines.iter().map(|line| Finding::new(&rule.id, place, RelPath::new("a.py"), *line, None, "w", "f")).collect(),
    )
}

/// tm-mid's engine is not available; the others hold.
fn mid_not_available(rule: &Rule, text: &str) -> RuleOutcome {
    if rule.id != "tm-mid" {
        return all_hold(rule, text);
    }
    let why = "swiftlint not found on PATH".to_string();
    RuleOutcome::NotAvailable { rule: rule.id.clone(), engine: Engine::SwiftLint, why }
}

/// tm-alpha gives no finding on its failing case; tm-mid's engine is not available; tm-zeta holds.
fn alpha_broken_mid_not_available(rule: &Rule, text: &str) -> RuleOutcome {
    if rule.id == "tm-alpha" {
        return RuleOutcome::Findings(Vec::new());
    }
    mid_not_available(rule, text)
}

/// Holds only when the run's params are exactly the case's own over the rule's: kit = "failing-kit" on the failing
/// case, kit = "passing-kit" on the passing one, other = "kept" on both. Any other params give a finding at
/// `<kit>.py`, so the printed why names the kit the run got.
fn by_case_params(rule: &Rule, text: &str) -> RuleOutcome {
    let want = if text == FAILING_TEXT {
        "kit = \"failing-kit\"\nother = \"kept\""
    } else {
        "kit = \"passing-kit\"\nother = \"kept\""
    };
    if rule.params == table(want) {
        return all_hold(rule, text);
    }
    let kit = rule.params.get("kit").and_then(toml::Value::as_str).unwrap_or("no-kit");
    let place = (Engine::Ruff, Kind::Quality, Severity::Error);
    let path = RelPath::new(&format!("{kit}.py"));
    RuleOutcome::Findings(vec![Finding::new(&rule.id, place, path, 1, None, "w", "f")])
}

/// Runs `cctao baseline test` with `args` on the test sets, `default_sets` the world's (name@version, ready), each case
/// run giving what `outcome` says for the rule it runs and its file's text; (exit, out, err, the sets read, (the case
/// folders made, the rules run)).
fn run(
    args: BaselineArgs,
    default_sets: &[(&str, bool)],
    outcome: Outcome,
) -> (u8, String, String, Vec<String>, (usize, usize)) {
    let default_sets: Vec<(String, bool)> = default_sets.iter().map(|(set, ready)| (set.to_string(), *ready)).collect();
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
    let run_rules = |rf: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
        *ran.borrow_mut() += 1;
        let rule = &rf.rules[0];
        let outcome = outcome(rule, &written.borrow());
        let absent = matches!(outcome, RuleOutcome::NotAvailable { .. });
        let run = RuleRun::new(rule.id.clone(), rule.engine, outcome, None);
        // The only not-available outcome these rows give is an absent swiftlint: engines gives it EngineNotOnPath.
        let run = if absent { run.with_cause(NotAvailableCause::EngineNotOnPath) } else { run };
        CheckedRules { runs: vec![run], engines: vec![] }
    };
    let world = BaselineWorld {
        default_sets: &default_sets,
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

/// What a case run gives for the rule it runs (its params included) and its file's text.
type Outcome = fn(&Rule, &str) -> RuleOutcome;

/// (name, args, default sets, outcome, exit, out, err, sets read, (case folders made, rules run))
type Row = (&'static str, BaselineArgs, Defaults, Outcome, u8, &'static str, &'static str, Sets, (usize, usize));

/// A world's default sets, as ("name@version", ready).
type Defaults = &'static [(&'static str, bool)];

/// A list of sets, as "name@version".
type Sets = &'static [&'static str];

#[test]
fn run_baseline_test_rows() {
    let text = BaselineArgs::default();
    let json = BaselineArgs { json: true, ..BaselineArgs::default() };
    let set = |set: &str| BaselineArgs { set: Some(set.to_string()), json: false };
    // A test world's default_sets, not cctao's: cctao's own are rules' data, read by the RITs in
    // tests/world/baseline_world_real.rs.
    let defaults: Defaults = &[("standard@1", true), ("house@1", true)];
    let house_not_ready: Defaults = &[("standard@1", true), ("house@1", false)];
    let unknown = "cctao: unknown baseline set \"nosuch\"\nnext: the test world's fix for nosuch\n";
    let rows: [Row; 17] = [
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
             next: install swiftlint, then run cctao baseline test again\n",
            "",
            &["standard@1", "house@1"],
            (6, 6),
        ),
        (
            "a set that cannot be read: every set read first, nothing run",
            text.clone(),
            &[("standard@1", true), ("nosuch@1", true)],
            all_hold,
            2,
            "",
            unknown,
            &["standard@1", "nosuch@1"],
            (0, 0),
        ),
        (
            "a set that cannot be read, as JSON",
            BaselineArgs { set: Some("nosuch@1".to_string()), json: true },
            defaults,
            all_hold,
            2,
            "{\"error\":\"unknown baseline set \\\"nosuch\\\"\",\"exit\":2,\"next\":\"the test world's fix for nosuch\"}\n",
            unknown,
            &["nosuch@1"],
            (0, 0),
        ),
        ("--set of an unknown set", set("nosuch@1"), defaults, all_hold, 2, "", unknown, &["nosuch@1"], (0, 0)),
        (
            "a set that cannot be read with no problem",
            set("empty@1"),
            defaults,
            all_hold,
            2,
            "",
            "cctao: cannot read baseline set \"empty@1\"\nnext: Run cctao help to see the commands.\n",
            &["empty@1"],
            (0, 0),
        ),
        (
            "the default runs the ready sets only, in order",
            text.clone(),
            &[("house@1", true), ("bare@1", false), ("standard@1", true)],
            all_hold,
            0,
            "house@1 tm-mid (swiftlint): holds\n\
             standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): holds\n\
             3 rules: 3 hold, 0 broken, 0 not available, 0 not applicable\n\
             next: nothing to do\n",
            "",
            &["house@1", "standard@1"],
            (6, 6),
        ),
        (
            "--set of a not-ready set runs it",
            set("house@1"),
            house_not_ready,
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
            "each case runs with its own params over the rule's",
            set("params@1"),
            defaults,
            by_case_params,
            0,
            "params@1 tm-p (ruff): holds\n\
             1 rules: 1 hold, 0 broken, 0 not available, 0 not applicable\n\
             next: nothing to do\n",
            "",
            &["params@1"],
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
            "a rule whose cases lack a required param is not applicable and is not run",
            set("aside@1"),
            defaults,
            all_hold,
            3,
            "aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: the case gives no value for required params: entity_types\n  \
             passing: not applicable: the case gives no value for required params: entity_types\n\
             2 rules: 1 hold, 0 broken, 0 not available, 1 not applicable\n\
             next: each not-applicable rule above says why it was not run\n",
            "",
            &["aside@1"],
            (2, 2),
        ),
        (
            "a rule whose cases lack a required param, as JSON",
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
                r#""why":"the case gives no value for required params: entity_types"},"#,
                r#""passing":{"result":"not_applicable","#,
                r#""why":"the case gives no value for required params: entity_types"},"#,
                r#""result":"not_applicable","rule":"tm-x","set":"aside@1"}]}"#,
                "\n"
            ),
            "",
            &["aside@1"],
            (2, 2),
        ),
        (
            "broken beside a not-applicable rule: broken wins",
            text.clone(),
            &[("standard@1", true), ("aside@1", true)],
            alpha_broken_mid_not_available,
            1,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (ast-grep): broken\n  \
             failing: broken: gave no finding; expected tm-alpha at a.py:1\n\
             aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: the case gives no value for required params: entity_types\n  \
             passing: not applicable: the case gives no value for required params: entity_types\n\
             4 rules: 2 hold, 1 broken, 0 not available, 1 not applicable\n\
             next: fix each broken rule or its cases above\n",
            "",
            &["standard@1", "aside@1"],
            (6, 6),
        ),
        (
            "not available beside a not-applicable rule: exit 3 with both next parts",
            text.clone(),
            &[("house@1", true), ("aside@1", true)],
            mid_not_available,
            3,
            "house@1 tm-mid (swiftlint): not available\n  \
             failing: not available: swiftlint not found on PATH\n  \
             passing: not available: swiftlint not found on PATH\n\
             aside@1 tm-zeta (ruff): holds\n\
             aside@1 tm-x (ruff): not applicable\n  \
             failing: not applicable: the case gives no value for required params: entity_types\n  \
             passing: not applicable: the case gives no value for required params: entity_types\n\
             3 rules: 1 hold, 0 broken, 1 not available, 1 not applicable\n\
             next: install swiftlint, then run cctao baseline test again; \
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
