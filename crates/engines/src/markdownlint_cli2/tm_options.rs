//! `tm_options`: the options module tm hands markdownlint-cli2 with `--config`, its file name and its text.

/// The module's file name. markdownlint-cli2 0.24.2 reads a `--config` file whose name ends with
/// `.markdownlint-cli2.cjs` as an options module; tm's name ends so.
const FILE: &str = "tm.markdownlint-cli2.cjs";

/// The module's text: inline `<!-- markdownlint-* -->` comments off (`noInlineConfig`, S-67 (1)), no fixing, no banner
/// or progress lines on stdout, and one output formatter that prints the results as one JSON array. markdownlint-cli2
/// ships no JSON formatter, so the formatter is a function in place of a formatter module; `outputFormatters` holds
/// `[formatter, options]` pairs, so the function sits inside two brackets (measured: in one, nothing is printed).
const TEXT: &str = "module.exports = {
  noInlineConfig: true,
  fix: false,
  noBanner: true,
  noProgress: true,
  outputFormatters: [[ (o) => { o.logMessage(JSON.stringify(o.results)); } ]],
};
";

/// `(file name, text)` of the options module the flow writes in a temporary folder and passes as `--config`.
pub(crate) fn tm_options() -> (&'static str, &'static str) {
    (FILE, TEXT)
}
