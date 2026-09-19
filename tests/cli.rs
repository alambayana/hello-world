//! Integration tests that exercise the real binary end to end.
//!
//! These cover the same behaviour as the unit tests in `src/main.rs`, but go
//! through the actual command-line interface (argument parsing, printing,
//! exit status, and the raw UTF-8 bytes on stdout).
//!
//! **Unix-only.** On GitHub's `windows-latest` runners, rustc 1.98.1's test
//! harness overflows the stack (`STATUS_STACK_OVERFLOW`, 0xc00000fd) when a
//! libtest worker thread calls `std::process::Command` — regardless of stack
//! size (`RUST_MIN_STACK` up to 32 MiB was tried), build profile (debug and
//! release), runner (`cargo test` and `cargo nextest`, i.e. fresh process
//! per test), serial or parallel execution, or capture mode. Every
//! non-harness path passes on the same runners: the binary run directly, a
//! plain `std::thread::spawn` calling `Command`, and all unit tests. It is
//! therefore a harness/environment bug, not a project bug. Re-enable for
//! Windows by deleting the `cfg` below once the toolchain is fixed.
#![cfg(unix)]

use std::ffi::OsString;

/// Runs the compiled `hello-world` binary with the given arguments and
/// returns its full stdout. Panics if the binary fails to execute or exits
/// with a non-zero status.
fn run(args: &[&str]) -> String {
    let os_args: Vec<OsString> = args.iter().map(OsString::from).collect();
    run_os(&os_args)
}

/// Like `run`, but accepts `OsString` arguments so tests can pass input that
/// is not valid UTF-8 (possible only on Unix, the platform these tests
/// target).
fn run_os(args: &[OsString]) -> String {
    use std::process::Command;

    let output = Command::new(env!("CARGO_BIN_EXE_hello-world"))
        .args(args)
        .output()
        .expect("failed to execute hello-world binary");
    assert!(
        output.status.success(),
        "binary exited with {:?}",
        output.status
    );
    String::from_utf8(output.stdout).expect("stdout was not valid UTF-8")
}

/// With no arguments the program prints the default greeting.
#[test]
fn no_argument_greets_world() {
    assert_eq!(run(&[]), "Hello, world!\n");
}

/// The first argument is greeted by name.
#[test]
fn first_argument_is_greeted() {
    assert_eq!(run(&["Rust"]), "Hello, Rust!\n");
}

/// Multiple arguments are joined with a single space.
#[test]
fn extra_arguments_are_joined_with_spaces() {
    assert_eq!(run(&["Rust", "is", "great"]), "Hello, Rust is great!\n");
}

/// Unquoted multi-word input produces the same output as quoted input —
/// the whole point of the join behaviour.
#[test]
fn quoted_and_unquoted_multi_word_names_match() {
    let quoted = run(&["Rust is great"]);
    let unquoted = run(&["Rust", "is", "great"]);
    assert_eq!(quoted, unquoted);
    assert_eq!(quoted, "Hello, Rust is great!\n");
}

/// A lone empty argument yields the "there" greeting.
#[test]
fn single_empty_argument_greets_there() {
    assert_eq!(run(&[""]), "Hello, there!\n");
}

/// Empty-string arguments are ignored wherever they appear, and the rest
/// are joined normally.
#[test]
fn empty_arguments_are_ignored_among_valid_ones() {
    assert_eq!(
        run(&["", "Rust", "", "is", "great", ""]),
        "Hello, Rust is great!\n"
    );
}

/// Only empty arguments → "there", regardless of how many there are.
#[test]
fn only_empty_arguments_greet_there() {
    assert_eq!(run(&["", "", ""]), "Hello, there!\n");
}

/// Any non-UTF-8 argument fails the whole batch, even if other arguments
/// are perfectly valid: a partially broken name is never half-greeted, so
/// the program says "Hello, there!".
#[test]
fn mixed_invalid_and_valid_arguments_greet_there() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let invalid = OsString::from(OsStr::from_bytes(&[0xFF, 0xFE, 0x41]));
    assert_eq!(
        run_os(&[invalid, OsString::from("Rust")]),
        "Hello, there!\n"
    );
}

/// Mixed scripts, accents, and emoji survive the full trip through the real
/// process boundary as exact UTF-8 bytes (the `from_utf8` check inside
/// `run_os` additionally proves stdout is valid UTF-8).
#[test]
fn unicode_argument_is_greeted_exactly() {
    assert_eq!(run(&["José 🦀 世界"]), "Hello, José 🦀 世界!\n");
}

/// A long argument is echoed in full with no truncation.
#[test]
fn long_argument_is_not_truncated() {
    let name = "x".repeat(10_000);
    let expected = format!("Hello, {name}!\n");
    assert_eq!(run(&[&name]), expected);
}

/// A non-UTF-8 argument leaves nothing usable to greet, so the program says
/// "Hello, there!" and exits successfully — the user did try to give a name.
#[test]
fn invalid_utf8_argument_greets_there() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let invalid = OsString::from(OsStr::from_bytes(&[0xFF, 0xFE, 0x41]));
    assert_eq!(run_os(&[invalid]), "Hello, there!\n");
}
