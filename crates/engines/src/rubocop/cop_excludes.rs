//! `cop_excludes`: one cop's `Exclude` list in a `rubocop --show-cops` dump.

/// In `show`, the text `rubocop --show-cops=<cops>` prints, the entries of `cop`'s `Exclude` list, as printed (each
/// one quoted, an absolute path or glob). A cop's block starts at the line that is exactly `<cop>:` and runs to the
/// next blank line; within it the entries are the `  - ` lines that follow the line `  Exclude:`, up to the first line
/// that is not one (other keys, such as SupportedStyles, print `  - ` lists of their own). Empty when `cop` has no
/// block or its block has no `Exclude`.
pub(crate) fn cop_excludes(show: &str, cop: &str) -> Vec<String> {
    let header = format!("{cop}:");
    show.lines()
        .skip_while(|line| *line != header)
        .take_while(|line| !line.is_empty())
        .skip_while(|line| *line != "  Exclude:")
        .skip(1)
        .map_while(|line| line.strip_prefix("  - "))
        .map(str::to_string)
        .collect()
}
