//! Unit tests for write_case_files.rs, with a recording writer: no disk. Each row shows the writes made and the why.

use super::write_case_files::write_case_files;
use super::BaselineWorld;
use rules::CaseFile;
use std::cell::RefCell;
use std::io;
use std::path::Path;

#[test]
fn write_case_files_rows() {
    let rows: [(&str, &[&str], Result<(), &str>, &[&str]); 4] = [
        ("each file inside the folder, in order", &["a.py", "./src/b.py"], Ok(()), &["/case/a.py", "/case/src/b.py"]),
        (
            "a name above the folder is refused, nothing written for it or after it",
            &["a.py", "../x.py", "c.py"],
            Err("case file \"../x.py\" is not a relative path inside the case folder"),
            &["/case/a.py"],
        ),
        (
            "an absolute name is refused",
            &["/abs.py"],
            Err("case file \"/abs.py\" is not a relative path inside the case folder"),
            &[],
        ),
        (
            "a failed write stops the rest",
            &["full.py", "c.py"],
            Err("cannot write full.py: disk full"),
            &["/case/full.py"],
        ),
    ];
    for (name, names, want, want_writes) in rows {
        let writes = RefCell::new(Vec::new());
        let write_file = |path: &Path, text: &str| {
            assert_eq!(text, "x = 1\n", "{name}");
            writes.borrow_mut().push(path.display().to_string());
            if path.ends_with("full.py") {
                return Err(io::Error::other("disk full"));
            }
            Ok(())
        };
        let world = BaselineWorld {
            default_sets: &[],
            rules_of: &|_| unreachable!("rules_of"),
            make_folder: &|| unreachable!("make_folder"),
            write_file: &write_file,
            commit_all: &|_| unreachable!("commit_all"),
            added_since: &|_, _| unreachable!("added_since"),
            remove_folder: &|_| unreachable!("remove_folder"),
            run_rules: &|_, _, _, _| unreachable!("run_rules"),
        };
        let files: Vec<CaseFile> = names.iter().map(|n| CaseFile { path: n.to_string(), text: "x = 1\n" }).collect();
        let got = write_case_files(Path::new("/case"), &files, &world);
        assert_eq!(got, want.map_err(str::to_string), "{name}");
        assert_eq!(*writes.borrow(), want_writes.to_vec(), "{name}");
    }
}
