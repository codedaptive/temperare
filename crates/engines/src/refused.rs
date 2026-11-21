//! `refused`: the `Failed` a rule gets before anything runs, when tm refuses its check or its params.

use crate::ast_grep::split_count_params;
use crate::entry::{param_args, param_why, threshold_of};
use crate::entry_for::entry_for;
use crate::types::Stop;
use rules::{Engine, Rule};
use std::path::Path;

/// `Failed` with no exit code when the entry of `rule`'s engine (`entry_for`; an engine with none refuses nothing)
/// refuses it, in this order: a check `entry.refuse` refuses, with the refusal (clippy's is a `rustdoc::` lint,
/// through `CLIPPY.refuse`); an ast-grep rule with no embedded YAML whose check is an absolute path, since a check
/// names a file inside the rule file's folder; params `param_args` cannot use, with `param_why`'s text; an ast-grep
/// rule's count params (`count`, `shared_setup`) `split_count_params` refuses, with its text; a threshold
/// `threshold_of` refuses, with `param_why`'s text. `None` when none of these refuses it. A refusal settles a rule
/// first, whatever its files and whether its engine is available, since no run could make such a rule clean, so its
/// flow does not reach these checks for the refusal; a flow reads the params again only for their values. ast-grep's
/// count params are refused here; `hits_ast_grep` only splits them off, which this has already checked. Only its
/// `{{name}}` params are left to its flow, since they are checked by filling the rule YAML, which it reads from the
/// rule file's folder.
/// Pure: it reads no file and runs nothing.
pub(crate) fn refused(rule: &Rule) -> Option<Stop> {
    let entry = entry_for(rule.engine)?;
    let failed = |stderr| Some(Stop::Failed { code: None, stderr });
    if let Some(why) = (entry.refuse)(&rule.check).or_else(|| absolute_rule_yaml(rule)) {
        return failed(why);
    }
    if let Err(err) = param_args(entry, &rule.params) {
        return failed(param_why(entry.name, err));
    }
    if let Some(why) = count_params_refusal(rule) {
        return failed(why);
    }
    match threshold_of(entry, &rule.check, &rule.params) {
        Err(err) => failed(param_why(entry.name, err)),
        Ok(_) => None,
    }
}

/// The refusal of an ast-grep rule whose check, naming its rule YAML, is an absolute path; a rule whose YAML is
/// embedded reads no file, so its check is not a path.
fn absolute_rule_yaml(rule: &Rule) -> Option<String> {
    (rule.engine == Engine::AstGrep && rule.embedded_yaml().is_none() && Path::new(&rule.check).is_absolute()).then(|| {
        format!(
            "ast-grep rule {} is an absolute path: a check names a file inside the rule file's folder",
            rule.check
        )
    })
}

/// The refusal of an ast-grep rule's count params, `split_count_params`'s Err text; other engines take no count params.
fn count_params_refusal(rule: &Rule) -> Option<String> {
    (rule.engine == Engine::AstGrep).then(|| split_count_params(&rule.params, &rule.check).err()).flatten()
}
