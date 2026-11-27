//! `Drop for Check`: writes the case's part of the report when its test ends.

use std::fs;

use crate::check::Check;

impl Drop for Check {
    /// Writes `### <case>`, the setup, what the case saw and its Result to the part file: FAIL when the test is
    /// panicking, else NOT RUN when `not_run` was called, else PASS. Writes nothing when there is no part file.
    fn drop(&mut self) {
        let Some(part) = &self.part else { return };
        let panicked = std::thread::panicking();
        let result = match (panicked, &self.not_run) {
            (true, _) => "FAIL".to_string(),
            (false, Some(why)) => format!("NOT RUN ({why})"),
            (false, None) => "PASS".to_string(),
        };
        let saw = if self.saw.is_empty() {
            "Saw: nothing; the case ended before cctao answered.".to_string()
        } else {
            format!("Saw:\n\n```text\n{}\n```", self.saw.join("\n"))
        };
        let mut text = format!("### {}\n\nSet up: {}\n\n{saw}\n\n", self.case, self.setup);
        if panicked {
            text.push_str("The test panicked; its output on the terminal says why.\n\n");
        }
        text.push_str(&format!("Result: {result}\n"));
        // A panic here, while the test is already panicking, would abort the whole run, so a write error is only
        // printed; scripts/accept.sh then shows this case NOT RUN, with no result recorded.
        if let Err(e) = fs::write(part, text) {
            eprintln!("accept: could not write {}: {e}", part.display());
        }
    }
}
