//! The tessera adapter: the tessera entry (`ts slop --json`), which no rule runs.
//! It has no reader: Tessera's slop records are not defined yet, so the entry is never available, `hits_tessera` makes
//! every tessera rule not available and `ts` is never run.

mod hits_tessera;
mod tessera_entry;

#[cfg(test)]
mod hits_tessera_tests;
#[cfg(test)]
mod tessera_entry_tests;

pub(crate) use hits_tessera::hits_tessera;
pub use tessera_entry::TESSERA;
