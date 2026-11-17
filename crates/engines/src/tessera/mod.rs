//! The tessera adapter: builds the `ts slop --json` command, which no rule runs.
//! It has no parser: Tessera's slop records are not defined yet, so `hits_tessera` makes every tessera rule not
//! available and `ts` is never run (settled S-20).

mod hits_tessera;
mod tessera_cmd;

#[cfg(test)]
mod hits_tessera_tests;
#[cfg(test)]
mod tessera_cmd_tests;

pub(crate) use hits_tessera::hits_tessera;
pub use tessera_cmd::tessera_cmd;
