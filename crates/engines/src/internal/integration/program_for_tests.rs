//! ZIT for program_for.rs: ESLint's executable local `node_modules/.bin/eslint` in a real temporary folder, else
//! the entry's program.

use crate::entry::Entry;
use crate::eslint::ESLINT;
use crate::program_for::program_for;
use crate::stylelint::STYLELINT;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-program-for-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

#[test]
fn program_for_prefers_a_local_eslint() {
    // (row, entry, `.bin` name, symlink flag and file mode if made, whether the folder is given, the program
    // expected, which is under the folder when the last column is true)
    let cases: [(&str, &Entry, Option<(&str, bool, u32)>, bool, &str, bool); 6] = [
        ("eslint local file", &ESLINT, Some(("eslint", false, 0o755)), true, "node_modules/.bin/eslint", true),
        ("eslint local symlink", &ESLINT, Some(("eslint", true, 0o755)), true, "node_modules/.bin/eslint", true),
        ("eslint no local file", &ESLINT, None, true, "eslint", false),
        ("eslint no folder", &ESLINT, Some(("eslint", false, 0o755)), false, "eslint", false),
        ("stylelint local file", &STYLELINT, Some(("stylelint", false, 0o755)), true, "stylelint", false),
        ("eslint local not executable", &ESLINT, Some(("eslint", false, 0o644)), true, "eslint", false),
    ];
    for (row, entry, made, give_folder, expected, under_folder) in cases {
        let folder = TempFolder::new(&row.replace(' ', "-"));
        let root = &folder.0;
        if let Some((name, linked, mode)) = made {
            fs::create_dir_all(root.join("node_modules/.bin")).unwrap();
            let bin = root.join("node_modules/.bin").join(name);
            if linked {
                fs::write(root.join("node_modules/real-bin"), "#!/bin/sh\n").unwrap();
                fs::set_permissions(root.join("node_modules/real-bin"), fs::Permissions::from_mode(mode)).unwrap();
                symlink(root.join("node_modules/real-bin"), &bin).unwrap();
            } else {
                fs::write(&bin, "#!/bin/sh\n").unwrap();
                fs::set_permissions(&bin, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
        let got = program_for(entry, give_folder.then_some(root.as_path()));
        let want = if under_folder { root.join(expected).into_os_string() } else { OsString::from(expected) };
        assert_eq!(got, want, "{row}");
    }
}
