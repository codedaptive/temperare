//! `run_cargo_audit`: cargo-audit's flow for one rule. It reads the disk and runs the engine, so its test is the zone
//! integration test tests/cargo_audit.rs and it has no unit-test sibling.

use super::{advisory_db, cargo_audit_cmd, known_advisory};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `CARGO_AUDIT`. The central `refused` has
/// already settled the rule's refusals, and `engine_hits_with` a folder whose Cargo.lock the rule does not reach
/// (`lock_listed`). Each step can stop the rule:
/// 1. the advisory database is cargo-audit's default (`advisory_db`, from CARGO_HOME, else HOME, which are only
///    read); neither set is `NotAvailable`;
/// 2. a database with no `crates` folder is `NotAvailable`, naming the command that fetches it: tm never fetches it;
/// 3. an advisory id the database does not hold (`known_advisory`) is `UnknownCheck`: cargo-audit would report
///    nothing for it, which is not clean. A check that is a kind needs no lookup;
/// 4. the check run of `cargo_audit_cmd` in `folder`, through `check_run` so it is counted;
/// 5. its report read by `entry.read`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_cargo_audit(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let Some(db) = advisory_db(std::env::var_os("CARGO_HOME").as_deref(), std::env::var_os("HOME").as_deref()) else {
        return Err(Stop::NotAvailable { why: "no advisory database: neither CARGO_HOME nor HOME is set".into() });
    };
    if !db.join("crates").is_dir() {
        return Err(Stop::NotAvailable {
            why: format!(
                "no advisory database at {}: fetch it with `cargo audit update-db --db {}` (tm never fetches it)",
                db.display(),
                db.display()
            ),
        });
    }
    // `refused` has let through only the kinds and `RUSTSEC-` ids, so the prefix marks an id.
    if rule.check.starts_with("RUSTSEC-") && !known_advisory(&db, &rule.check) {
        return Err(Stop::UnknownCheck {
            said: format!("{} is not in the advisory database at {}", rule.check, db.display()),
        });
    }
    let out = check_run(&cargo_audit_cmd(&db, folder), host)?;
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}
