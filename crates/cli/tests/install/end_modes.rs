//! Installed binary, skill and record modes are independent of umask. Child processes isolate the process-wide umask;
//! their install on a temporary home makes this a zone integration test.

use crate::support::{detected, skill, Fixture};
use cli::InstallArgs;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn installed_modes_do_not_depend_on_umask() {
    if let Ok(mask) = std::env::var("CCTAO_END_MODES_CHILD") {
        let f = Fixture::new();
        let probe = f.root.join("umask-probe");
        fs::write(&probe, b"probe").unwrap();
        let mask_value = u32::from_str_radix(&mask, 8).unwrap();
        assert_eq!(
            fs::metadata(&probe).unwrap().permissions().mode() & 0o777,
            0o666 & !mask_value,
            "child umask {mask} is not in force"
        );
        detected(&f);
        let (status, out, err) = f.install(&InstallArgs { yes: true, ..Default::default() });
        assert_eq!(status, 0, "install failed under umask {mask}: {out} {err}");
        for (path, expected) in [
            (f.host.home.join(".local/bin/cctao"), 0o755),
            (skill(&f.host.home, ".claude"), 0o644),
            (f.host.home.join(".local/state/cctao/installed"), 0o644),
        ] {
            assert_eq!(
                fs::symlink_metadata(&path).unwrap().permissions().mode() & 0o7777,
                expected,
                "{} under umask {mask}",
                path.display()
            );
        }
        return;
    }

    for mask in ["002", "077"] {
        let child = procio::run(&procio::Cmd {
            args: vec![
                "-c".into(),
                "umask \"$1\" && shift && exec \"$0\" \"$@\"".into(),
                std::env::current_exe().unwrap().into_os_string(),
                mask.into(),
                "--exact".into(),
                "end_modes::installed_modes_do_not_depend_on_umask".into(),
                "--nocapture".into(),
            ],
            env: vec![("CCTAO_END_MODES_CHILD".into(), mask.into())],
            ..procio::Cmd::new("sh")
        }).unwrap();
        let stdout = String::from_utf8_lossy(&child.stdout);
        let stderr = String::from_utf8_lossy(&child.stderr);
        assert_eq!(child.code, Some(0), "child under umask {mask} failed:\nstdout: {stdout}\nstderr: {stderr}");
        assert!(
            stdout.contains("1 passed"),
            "child under umask {mask} ran no test:\nstdout: {stdout}\nstderr: {stderr}"
        );
    }
}
