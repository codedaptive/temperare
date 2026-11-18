//! `Check::saw_tm`: records what one `tm` run showed.

use serde_json::Value;

use crate::check::Check;
use crate::tm_lines::tm_lines;

impl Check {
    /// What one `tm <args>` run showed: its exit `code` and its `--json` answer (`tm_lines`).
    pub fn saw_tm(&mut self, args: &[&str], code: i32, json: &Value) {
        self.saw.extend(tm_lines(args, code, json));
    }
}
