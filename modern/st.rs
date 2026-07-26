//! `st` / `tree` — smart-tree (github.com/8b-is/smart-tree) as a rustybox
//! `extras` applet.
//!
//! smart-tree is a large, AI-oriented tool: LLM inference (candle), a ratatui
//! TUI, an axum web dashboard, an MCP server, tree-sitter parsers for a dozen
//! languages. Linking that into rustybox would defeat rustybox's whole reason
//! to exist inside entheai's worker jail — *one small, audited, static
//! multicall binary that shrinks the executable surface*.
//!
//! So, unlike the in-process `modern-*` backends (which link a library and
//! never subprocess), this `extras` applet deliberately **delegates** to a
//! standalone `st` binary — exactly how entheai already execs `git` and
//! `rustybox` itself. The heavy tool stays a separate, separately-audited
//! executable; rustybox just knows the name and forwards to it. That deviation
//! from the "no subprocessing" rule is the price of keeping the core lean, and
//! it is confined to the opt-in `extras` layer (never the sandbox core).
//!
//! Build with `--features extras-st` (or `--features extras`). The delegated
//! binary is `st` on `PATH`, or `$RUSTYBOX_ST` when set — use the override if
//! rustybox is itself installed under the name `st`, to avoid self-exec.

use std::process::Command;

const NOT_FOUND: &str = "\
st: smart-tree ('st') was not found on PATH.
    rustybox's `st`/`tree` applet delegates to the standalone smart-tree binary.
    install:  cargo install smart-tree
          or: curl -sSL https://raw.githubusercontent.com/8b-is/smart-tree/main/scripts/install.sh | bash
    if it is installed under another name/path, set $RUSTYBOX_ST to point at it.
";

/// Delegate to the real smart-tree binary, forwarding every operand unchanged,
/// and return its exit status. Both `st` and `tree` route here (`tree` is a
/// familiar alias — smart-tree *is* the tree).
pub fn run(argv: &[&str]) -> i32 {
    let bin = std::env::var("RUSTYBOX_ST").unwrap_or_else(|_| "st".to_string());
    match Command::new(&bin).args(argv).status() {
        Ok(status) => status.code().unwrap_or_else(|| signal_code(&status)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprint!("{NOT_FOUND}");
            127
        }
        Err(e) => {
            eprintln!("st: could not run '{bin}': {e}");
            126
        }
    }
}

/// Applet-table entrypoint: run and exit with the delegated status.
pub fn run_and_exit(argv: &[&str]) -> ! {
    std::process::exit(run(argv));
}

#[cfg(unix)]
fn signal_code(status: &std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    // Match the shell convention: 128 + signal number for a signal-killed child.
    128 + status.signal().unwrap_or(0)
}

#[cfg(not(unix))]
fn signal_code(_status: &std::process::ExitStatus) -> i32 {
    1
}
