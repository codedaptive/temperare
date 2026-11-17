//! parse: pure readers of the bytes git prints. They start no process, so every one is tested on captured output.

mod parse_added_lines;
mod parse_hunk_header;
mod parse_nul_list;
mod path_from_bytes;
mod unquote_c_path;

pub(crate) use parse_added_lines::parse_added_lines;
pub(crate) use parse_hunk_header::parse_hunk_header;
pub(crate) use parse_nul_list::parse_nul_list;
pub(crate) use path_from_bytes::path_from_bytes;
pub(crate) use unquote_c_path::unquote_c_path;

#[cfg(test)]
mod parse_added_lines_tests;
#[cfg(test)]
mod parse_hunk_header_tests;
#[cfg(test)]
mod parse_nul_list_tests;
#[cfg(test)]
mod path_from_bytes_tests;
#[cfg(test)]
mod unquote_c_path_tests;
