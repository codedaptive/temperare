use super::package_subject;
use std::path::Path;

macro_rules! cases {
    ($( $name:ident => ($package:expr, $root:expr, $top:expr, $expected:expr) ),+ $(,)?) => {
        $(#[test] fn $name() {
            assert_eq!(package_subject(Path::new($package), Path::new($root), Path::new($top)), $expected);
        })+
    };
}

cases! {
    periphery_tidy_root => ("/repo/App", "/repo/App", "/repo", "the package"),
    periphery_tidy_nested => ("/repo/Packages/Inner", "/repo/App", "/repo", "the local package Packages/Inner"),
    periphery_tidy_top => ("/repo", "/repo/App", "/repo", "the local package ."),
    periphery_tidy_root_is_top => ("/repo", "/repo", "/repo", "the package"),
}
