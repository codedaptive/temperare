//! `Check::saw_cctao`: records what one `cctao` run showed.

use serde_json::Value;

use crate::check::Check;
use crate::cctao_lines::cctao_lines;

impl Check {
    /// What one `cctao <args>` run showed: its exit `code` and its `--json` answer (`cctao_lines`).
    pub fn saw_cctao(&mut self, args: &[&str], code: i32, json: &Value) {
        self.saw.extend(cctao_lines(args, code, json));
    }
}
