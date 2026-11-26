//! `temperare install` and `uninstall` (S-32): the harnesses, the machine as input (`InstallHost`), the install
//! paths, the plan and its record, the skill text; the pure plan in plan/, the disk steps in disk/.

mod disk;
mod harness;
mod harness_all;
mod harness_folder;
mod harness_from_name;
mod harness_name;
mod harness_skill_file;
mod install_host;
mod install_host_real;
mod install_paths;
mod install_paths_under;
mod install_plan;
mod package_prefixes;
mod plan;
mod record_entry;
mod skill_text;

#[cfg(test)]
mod harness_all_tests;
#[cfg(test)]
mod harness_folder_tests;
#[cfg(test)]
mod harness_from_name_tests;
#[cfg(test)]
mod harness_name_tests;
#[cfg(test)]
mod harness_skill_file_tests;
// Unix only: install is unix-only for now (tm is a symlink, S-32), and these fixtures are unix absolute paths.
#[cfg(all(test, unix))]
mod install_host_real_tests;
#[cfg(test)]
mod install_paths_under_tests;
#[cfg(test)]
mod skill_text_tests;

pub use harness::Harness;
pub use install_host::InstallHost;
pub use install_paths::InstallPaths;
pub use install_plan::InstallPlan;
pub use package_prefixes::PACKAGE_PREFIXES;
pub use record_entry::RecordEntry;
pub use skill_text::SKILL_TEXT;

pub(crate) use disk::{
    apply_install, apply_uninstall, detect_harnesses, make_dirs, read_record_text, tm_owner, write_through_temp,
};
pub(crate) use plan::{
    choose_targets, parse_record, pick_harnesses, plan_install, record_text, uninstall_order, unrecordable_path,
};
