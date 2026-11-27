use super::Report;

/// The report as one text, a line each.
pub fn render(report: &Report) -> String {
    report.lines.iter().map(|line| trimmed(line)).collect::<Vec<_>>().join("\n")
}

fn trimmed(line: &str) -> &str {
    line.trim_end()
}
