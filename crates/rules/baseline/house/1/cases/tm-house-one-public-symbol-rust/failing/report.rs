/// A report: the lines it prints.
pub struct Report {
    lines: Vec<String>,
}

/// The report as one text, a line each.
pub fn render(report: &Report) -> String {
    report.lines.join("\n")
}
