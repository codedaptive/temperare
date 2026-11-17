//! The adapters' shared value types: an engine's availability, one hit, the parse and param errors, and why a rule
//! stops without findings.

mod availability;
mod hit;
mod param_error;
mod parse_error;
mod stop;

#[cfg(test)]
mod availability_tests;
#[cfg(test)]
mod hit_tests;
#[cfg(test)]
mod param_error_tests;
#[cfg(test)]
mod parse_error_tests;
#[cfg(test)]
mod stop_tests;

pub use availability::Availability;
pub use hit::Hit;
pub use param_error::ParamError;
pub use parse_error::ParseError;
pub(crate) use stop::Stop;
