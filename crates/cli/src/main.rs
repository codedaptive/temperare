//! temperare: the binary; tm is its short name (a symlink). A thin shell: it reads the command line and the
//! current folder, and the work is in the library's `run`, whose answer is the exit code.

fn main() {
    let args: Vec<String> = std::env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()).collect();
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => {
            eprintln!("tm: cannot read the current directory: {e}\nnext: run tm from a folder that exists");
            std::process::exit(2);
        }
    };
    // The locks are dropped before exit, which runs no destructors; exit itself flushes stdout.
    let code = cli::run(&args, &cwd, &mut std::io::stdout().lock(), &mut std::io::stderr().lock());
    std::process::exit(i32::from(code));
}
