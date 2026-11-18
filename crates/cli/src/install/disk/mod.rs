//! The install steps that touch the disk: finding harnesses and another tm, making folders, reading the install record,
//! writing a file through a temporary file, placing and removing.
//! None has a ZUT (each reads or writes the disk); the zone integration target `tests/install` covers them.

mod apply_install;
mod apply_uninstall;
mod detect_harnesses;
mod make_dirs;
mod read_record_text;
mod tm_owner;
mod write_through_temp;

pub(crate) use apply_install::apply_install;
pub(crate) use apply_uninstall::apply_uninstall;
pub(crate) use detect_harnesses::detect_harnesses;
pub(crate) use make_dirs::make_dirs;
pub(crate) use read_record_text::read_record_text;
pub(crate) use tm_owner::tm_owner;
pub(crate) use write_through_temp::write_through_temp;
