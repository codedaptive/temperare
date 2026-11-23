//! ZUT for md_rules.rs against markdownlint 0.42.2's rule list: all 53 rows, names and tags, in order, written out by
//! hand as markdownlint's own `rules.mjs` lists them.

use super::MD_RULES;

const EXPECTED: [(&[&str], &[&str]); 53] = [
    (&["MD001", "heading-increment"], &["headings"]),
    (&["MD003", "heading-style"], &["headings"]),
    (&["MD004", "ul-style"], &["bullet", "ul"]),
    (&["MD005", "list-indent"], &["bullet", "ul", "indentation"]),
    (&["MD007", "ul-indent"], &["bullet", "ul", "indentation"]),
    (&["MD009", "no-trailing-spaces"], &["whitespace"]),
    (&["MD010", "no-hard-tabs"], &["whitespace", "hard_tab"]),
    (&["MD011", "no-reversed-links"], &["links"]),
    (&["MD012", "no-multiple-blanks"], &["whitespace", "blank_lines"]),
    (&["MD013", "line-length"], &["line_length"]),
    (&["MD014", "commands-show-output"], &["code"]),
    (&["MD018", "no-missing-space-atx"], &["headings", "atx", "spaces"]),
    (&["MD019", "no-multiple-space-atx"], &["headings", "atx", "spaces"]),
    (&["MD020", "no-missing-space-closed-atx"], &["headings", "atx_closed", "spaces"]),
    (&["MD021", "no-multiple-space-closed-atx"], &["headings", "atx_closed", "spaces"]),
    (&["MD022", "blanks-around-headings"], &["headings", "blank_lines"]),
    (&["MD023", "heading-start-left"], &["headings", "spaces"]),
    (&["MD024", "no-duplicate-heading"], &["headings"]),
    (&["MD025", "single-title", "single-h1"], &["headings"]),
    (&["MD026", "no-trailing-punctuation"], &["headings"]),
    (&["MD027", "no-multiple-space-blockquote"], &["blockquote", "whitespace", "indentation"]),
    (&["MD028", "no-blanks-blockquote"], &["blockquote", "whitespace"]),
    (&["MD029", "ol-prefix"], &["ol"]),
    (&["MD030", "list-marker-space"], &["ol", "ul", "whitespace"]),
    (&["MD031", "blanks-around-fences"], &["code", "blank_lines"]),
    (&["MD032", "blanks-around-lists"], &["bullet", "ul", "ol", "blank_lines"]),
    (&["MD033", "no-inline-html"], &["html"]),
    (&["MD034", "no-bare-urls"], &["links", "url"]),
    (&["MD035", "hr-style"], &["hr"]),
    (&["MD036", "no-emphasis-as-heading"], &["headings", "emphasis"]),
    (&["MD037", "no-space-in-emphasis"], &["whitespace", "emphasis"]),
    (&["MD038", "no-space-in-code"], &["whitespace", "code"]),
    (&["MD039", "no-space-in-links"], &["whitespace", "links"]),
    (&["MD040", "fenced-code-language"], &["code", "language"]),
    (&["MD041", "first-line-heading", "first-line-h1"], &["headings"]),
    (&["MD042", "no-empty-links"], &["links"]),
    (&["MD043", "required-headings"], &["headings"]),
    (&["MD044", "proper-names"], &["spelling"]),
    (&["MD045", "no-alt-text"], &["accessibility", "images"]),
    (&["MD046", "code-block-style"], &["code"]),
    (&["MD047", "single-trailing-newline"], &["blank_lines"]),
    (&["MD048", "code-fence-style"], &["code"]),
    (&["MD049", "emphasis-style"], &["emphasis"]),
    (&["MD050", "strong-style"], &["emphasis"]),
    (&["MD051", "link-fragments"], &["links"]),
    (&["MD052", "reference-links-images"], &["images", "links"]),
    (&["MD053", "link-image-reference-definitions"], &["images", "links"]),
    (&["MD054", "link-image-style"], &["images", "links"]),
    (&["MD055", "table-pipe-style"], &["table"]),
    (&["MD056", "table-column-count"], &["table"]),
    (&["MD058", "blanks-around-tables"], &["table"]),
    (&["MD059", "descriptive-link-text"], &["accessibility", "links"]),
    (&["MD060", "table-column-style"], &["table"]),
];

#[test]
fn md_rules_holds_markdownlint_0_42_2s_rules() {
    assert_eq!(MD_RULES.len(), EXPECTED.len());
    for (rule, (names, tags)) in MD_RULES.iter().zip(EXPECTED) {
        assert_eq!((rule.names, rule.tags), (names, tags), "{}", names[0]);
    }
}
