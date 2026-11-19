//! The binary's temporary file is owner-only (0o600) while copied, then 0o755 once complete, before rename.

use crate::support::{Fixture, EXE_BYTES};
use cli::InstallArgs;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

#[test]
fn temp_mode_is_private_until_the_full_executable_is_installed() {
    let f = Fixture::new();
    fs::remove_file(&f.host.exe).unwrap();
    let fifo_result = procio::run(&procio::Cmd {
        args: vec![f.host.exe.as_os_str().into()],
        ..procio::Cmd::new("mkfifo")
    }).unwrap();
    assert_eq!(fifo_result.code, Some(0), "mkfifo failed: {}", String::from_utf8_lossy(&fifo_result.stderr));
    let bin = f.host.home.join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    let first = vec![0x5a; 8 * 1024 * 1024];
    let mut payload = first.clone();
    payload.extend_from_slice(EXE_BYTES);
    let connection = Arc::new(AtomicU8::new(0));
    let writer_connection = Arc::clone(&connection);
    let fifo = f.host.exe.clone();
    let writer_bin = bin.clone();
    let writer = thread::spawn(move || {
        let mut source = OpenOptions::new().write(true).open(fifo).unwrap();
        // If install returned before the writer opened, hold this end until the cleanup reader opens.
        if writer_connection.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            while writer_connection.load(Ordering::SeqCst) != 3 {
                thread::yield_now();
            }
            return None;
        }
        // A write larger than the FIFO buffer returns only after install has read from the FIFO in io::copy, which
        // runs after the temp is created and set to 0o600, and before EOF can allow its rename. If no entry is
        // present, dropping the writer lets install finish and the test fail.
        let _ = source.write_all(&first);
        let temp = fs::read_dir(&writer_bin).unwrap().next()?.unwrap().path();
        let mode = fs::symlink_metadata(&temp).unwrap().permissions().mode() & 0o7777;
        let seen = (temp.file_name().unwrap().to_string_lossy().into_owned(), mode);
        let _ = source.write_all(EXE_BYTES);
        Some(seen)
    });

    let (status, out, err) = f.install(&InstallArgs::default());
    if connection.compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        connection.store(3, Ordering::SeqCst);
        let reader = OpenOptions::new().read(true).open(&f.host.exe).unwrap();
        drop(reader);
    }
    let seen = writer.join().unwrap();
    assert_eq!(status, 0, "install failed: {out} {err}");
    let (name, mode) = seen.expect("the binary temporary file was never observed");
    assert!(name.starts_with(".temperare.") && name.ends_with(".installing"), "unexpected entry {name}");
    assert_eq!(mode, 0o600, "temporary binary mode was {mode:#05o}, expected 0o600");
    let binary = bin.join("temperare");
    assert!(fs::read(&binary).unwrap() == payload, "the installed binary is not the whole payload");
    assert_eq!(fs::symlink_metadata(binary).unwrap().permissions().mode() & 0o7777, 0o755);
}
