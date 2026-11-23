//! `Oid` and its methods, one impl per file.

mod oid;
mod oid_as_str;
mod oid_display;
mod oid_parse;

pub use oid::Oid;

#[cfg(test)]
mod oid_as_str_tests;
#[cfg(test)]
mod oid_display_tests;
#[cfg(test)]
mod oid_parse_tests;
#[cfg(test)]
mod oid_tests;
