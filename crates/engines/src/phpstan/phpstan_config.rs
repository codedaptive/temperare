//! `phpstan_config`: the wrapper config cctao hands PHPStan, around the repository's own.

use std::path::Path;

/// The text of the wrapper config: it includes `config` (absolute, single-quoted), puts PHPStan's `tmpDir` in a
/// `cache` folder beside the wrapper (PHPStan resolves a relative `tmpDir` against the file that sets it), and empties
/// `ignoreErrors` and `excludePaths` (`!` replaces a value rather than merging into it). The first drops the config's
/// own entries and those of any baseline neon it includes; the second keeps the config's exclusions from dropping a
/// file cctao passes. Both because an exception's home is the rule file (S-67 (1)). PHPStan 2.3.7 takes `excludePaths`
/// as a map holding `analyse`, `analyseAndScan` or both; it turns a plain list into `analyseAndScan`, but not one
/// given with `!`, so `excludePaths!: []` is refused. The wrapper sets both lists empty. Relative paths inside
/// `config` still resolve against `config`'s own folder. Err, naming the path, when `config` is not UTF-8 or holds `'`
/// (it would end the quoted string), `%` (Nette expands `%name%` as a parameter) or a line break.
pub(crate) fn phpstan_config(config: &Path) -> Result<String, String> {
    let shown = config.display();
    let Some(path) = config.to_str() else {
        return Err(format!("the phpstan config path {shown} is not UTF-8, so cctao cannot write it into a config"));
    };
    if path.contains(['\'', '%', '\n', '\r']) {
        return Err(format!("the phpstan config path {shown} holds ', % or a line break, so cctao cannot write it into a config"));
    }
    Ok(format!(
        "includes:\n    - '{path}'\nparameters:\n    tmpDir: cache\n    ignoreErrors!: []\n    \
         excludePaths!: {{analyse: [], analyseAndScan: []}}\n"
    ))
}
