//! Integration tests that exercise the real binary end to end.
//!
//! These cover the same behaviour as the unit tests in `src/main.rs`, but go
//! through the actual command-line interface (argument parsing, printing,
//! exit status), catching bugs that unit tests alone would miss.

use std::process::Command;

/// Runs the compiled `hello-world` binary with the given arguments and
/// returns its full stdout. Panics if the binary fails to execute or exits
/// with a non-zero status.
fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_hello-world"))
        .args(args)
        .output()
        .expect("failed to execute hello-world binary");
    assert!(output.status.success(), "binary exited with {:?}", output.status);
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

/// Arguments beyond the first are ignored.
#[test]
fn extra_arguments_are_ignored() {
    assert_eq!(run(&["Rust", "is", "great"]), "Hello, Rust!\n");
}
