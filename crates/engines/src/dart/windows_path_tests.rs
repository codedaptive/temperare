//! ZUT for windows_path.rs: paths with a drive in URL and Windows form made native, `.` and `..` resolved, and the
//! paths it returns unchanged (no drive, UNC, drive-relative).

use super::windows_path;

#[test]
fn windows_path_cases() {
    let rows = [
        ("file drive", "/C:/x", r"C:\x"),
        ("lowercase decoded space", "/c:/a b", r"c:\a b"),
        ("drive root slash", "/C:/", "C:\\"),
        ("drive root", "/C:", "C:\\"),
        ("mixed separators", r"/C:/x\lib/a.yaml", r"C:\x\lib\a.yaml"),
        ("verbatim relative root", r"\\?\C:\w\pkg\.dart_tool\../lib/options.yaml", r"\\?\C:\w\pkg\lib\options.yaml"),
        ("dot and empty segments", "/C:/x//./lib/a.yaml", r"C:\x\lib\a.yaml"),
        ("above root", "/C:/../x", r"C:\x"),
        ("plain drive", "C:/x/../y", r"C:\y"),
        ("backslash drive", r"C:\x\..\y", r"C:\y"),
        ("no drive", r"/x\lib/a.yaml", r"/x\lib/a.yaml"),
        ("verbatim UNC", r"\\?\UNC\srv\share\a\..\b", r"\\?\UNC\srv\share\a\..\b"),
        ("UNC", r"\\srv\share", r"\\srv\share"),
        ("drive relative", "/C:x", "/C:x"),
        ("nonletter drive", "/1:/x", "/1:/x"),
    ];
    let mut failed = Vec::new();
    for (name, input, want) in rows {
        let got = windows_path(input);
        if got != want {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
