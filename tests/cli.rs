//! Integration tests that exercise the real binary end to end.
//!
//! These cover the same behaviour as the unit tests in `src/main.rs`, but go
//! through the actual command-line interface (argument parsing, printing,
//! exit status, and the raw UTF-8 bytes on stdout).

use std::ffi::OsString;
use std::process::Command;

/// Runs the compiled `hello-world` binary with the given arguments and
/// returns its full stdout. Panics if the binary fails to execute or exits
/// with a non-zero status.
fn run(args: &[&str]) -> String {
    let os_args: Vec<OsString> = args.iter().map(OsString::from).collect();
    run_os(&os_args)
}

/// Like `run`, but accepts `OsString` arguments so tests can pass input that
/// is not valid UTF-8 (only possible on Unix).
#[cfg(unix)]
fn run_os(args: &[OsString]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_hello-world"))
        .args(args)
        .output()
        .expect("failed to execute hello-world binary");
    assert!(output.status.success(), "binary exited with {:?}", output.status);
    String::from_utf8(output.stdout).expect("stdout was not valid UTF-8")
}

/// On non-Unix platforms there is no way to express non-UTF-8 arguments, so
/// fall back to the plain helper.
#[cfg(not(unix))]
fn run_os(args: &[OsString]) -> String {
    let lossy: Vec<String> = args.iter().map(|a| a.to_string_lossy().into_owned()).collect();
    let refs: Vec<&str> = lossy.iter().map(String::as_str).collect();
    run(&refs)
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
    let quoted = run(&["Devajyoti Sarkar"]);
    let unquoted = run(&["Devajyoti", "Sarkar"]);
    assert_eq!(quoted, unquoted);
    assert_eq!(quoted, "Hello, Devajyoti Sarkar!\n");
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
/// are perfectly valid: the program behaves as if no arguments were given.
#[test]
#[cfg(unix)]
fn mixed_invalid_and_valid_arguments_fall_back_to_default() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let invalid = OsString::from(OsStr::from_bytes(&[0xFF, 0xFE, 0x41]));
    assert_eq!(run_os(&[invalid, OsString::from("Rust")]), "Hello, world!\n");
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

/// A non-UTF-8 first argument (Unix-only input) is silently skipped by
/// `std::env::args()`, so the program falls back to the default greeting
/// and still exits successfully.
#[test]
#[cfg(unix)]
fn invalid_utf8_argument_falls_back_to_default() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let invalid = OsString::from(OsStr::from_bytes(&[0xFF, 0xFE, 0x41]));
    assert_eq!(run_os(&[invalid]), "Hello, world!\n");
}
