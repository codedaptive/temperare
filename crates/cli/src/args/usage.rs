//! `USAGE`: the text `tm help` prints.

/// tm's usage: one line per command form, then where the rule file comes from.
pub const USAGE: &str = "\
usage:
tm check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]
tm rules [--rules <path>] [--json]
tm help
tm --version

The rule file is --rules <path>, else temperare.toml at the top of the git repository.
";
