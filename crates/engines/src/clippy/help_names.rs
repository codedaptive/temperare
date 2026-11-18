//! `help_names`: the names one kind of section lists in `clippy-driver -W help` output.

/// Where the reader is in the help text.
enum At {
    /// Outside a section being read.
    Other,
    /// After a section title starting `title`, before its `----` row.
    Title,
    /// In the section's table, after the `----` row.
    Rows,
}

/// The names, in order, from every section of `help` whose title line starts with `title` (rustc's and clippy's),
/// each with `-` read as `_`. A row's name is its first word; a section's table starts after its `----` row and ends
/// at a blank line. Sections with another title are never read, so `"Lint groups"` takes only the groups and
/// `"Lint checks"` only the lints.
pub(crate) fn help_names(help: &str, title: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut at = At::Other;
    for line in help.lines() {
        at = match at {
            _ if line.starts_with(title) => At::Title,
            At::Title if line.trim_start().starts_with("----") => At::Rows,
            At::Rows => match line.split_whitespace().next() {
                Some(name) => {
                    names.push(name.replace('-', "_"));
                    At::Rows
                }
                None => At::Other,
            },
            other => other,
        };
    }
    names
}
