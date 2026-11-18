//! The install plan, decided before anything is written and with no I/O of its own: which harnesses to wire, what
//! to place and link, the picker's dialogue on given streams, the paths the install record cannot hold, and the
//! install record's text and removal order.

mod choose_targets;
mod parse_record;
mod pick_harnesses;
mod plan_install;
mod record_text;
mod unfit_for_record;
mod uninstall_order;
mod unrecordable_path;

#[cfg(test)]
mod choose_targets_tests;
#[cfg(test)]
mod parse_record_tests;
#[cfg(test)]
mod pick_harnesses_tests;
#[cfg(test)]
mod plan_install_tests;
#[cfg(test)]
mod record_text_tests;
#[cfg(test)]
mod unfit_for_record_tests;
#[cfg(test)]
mod uninstall_order_tests;
#[cfg(test)]
mod unrecordable_path_tests;

pub(crate) use choose_targets::choose_targets;
pub(crate) use parse_record::parse_record;
pub(crate) use pick_harnesses::pick_harnesses;
pub(crate) use plan_install::plan_install;
pub(crate) use record_text::record_text;
pub(crate) use uninstall_order::uninstall_order;
pub(crate) use unrecordable_path::unrecordable_path;
