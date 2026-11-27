//! Unit test for linted_files.rs: real ESLint 10.15.0 output (fixtures/*.json; folders in fixtures/fixtures.txt) read into
//! the files it linted. Each capture keeps its folder under `/private/tmp/tm-eslint-fixtures`, which the rows pass as
//! `folder`; no folder is read. The not-JSON row and the row with a file outside the folder are hand-built.

use super::linted_files;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const NV: &str = "/private/tmp/tm-eslint-fixtures/nv";
const NF_SRC: &str = "/private/tmp/tm-eslint-fixtures/nf/src";

#[test]
fn linted_files_lists_the_files_in_the_folder_or_none() {
    let rows: [(&str, &[u8], &str, Option<&[&str]>); 4] = [
        ("two files linted", include_bytes!("fixtures/no_var_and_unused.json"), NV, Some(&["eslint.config.mjs", "example.js"])),
        ("no file linted", include_bytes!("fixtures/no_files.json"), NF_SRC, Some(&[])),
        ("not JSON", b"Oops", NV, None),
        (
            "a file outside the folder",
            br#"[{"filePath":"/private/tmp/tm-eslint-fixtures/nv/example.js"},{"filePath":"/private/tmp/elsewhere/a.js"}]"#,
            NV,
            Some(&["example.js"]),
        ),
    ];
    for (name, stdout, folder, want) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: Vec::new() };
        let want = want.map(|files| files.iter().map(|file| RelPath::new(file)).collect::<Vec<_>>());
        assert_eq!(linted_files(&out, Path::new(folder)), want, "{name}");
    }
}
