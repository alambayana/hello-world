# Design notes

The program is ~70 lines of logic. This document explains *why* each
behaviour is the way it is — the decisions that aren't visible from the
code alone. The **behaviour contract** below is normative; every row is
pinned by at least one test (see the [test map](#test-map) at the end).

## Behaviour contract

| Input (after the program name) | Output | Exit code |
|---|---|---|
| *(nothing)* | `Hello, world!` | 0 |
| one or more args, all valid UTF-8, ≥1 non-empty | `Hello, <args joined by a single space>!` | 0 |
| args given, all empty strings | `Hello, there!` | 0 |
| args given, any of them not valid UTF-8 (Unix only) | `Hello, there!` | 0 |

Non-normative guarantees:

- The name is embedded **verbatim** as bytes. No escaping, no Unicode
  normalization, no truncation, no length limit.
- stdout is always valid UTF-8 ending in a single newline.
- The program never fails: it always exits 0.

## Design choices

### 1. Joining multiple arguments (`echo`-style)

The most common way to greet a full name is to forget the quotes. Joining
all arguments with a space means `hello-world Rust is great` and
`hello-world "Rust is great"` are identical, so the common mistake
isn't an error. We considered *rejecting* extra arguments (the strict-CLI
convention, right for tools where extra args usually signal a real mistake
like `git push`), but for a greeting there's no meaningful difference
between "one argument containing a space" and "two arguments" — the space
is just a word separator either way.

The join uses a **single** space between arguments, so the output is
deterministic and never accumulates extra whitespace.

### 2. "world" vs "there": the input was given or not

The two fallbacks encode a deliberate distinction:

- `Hello, world!` — *nothing was typed at all*. The canonical default.
- `Hello, there!` — *something was typed, but nothing usable was left*
  (only empty strings, or a failed batch — see §4). The user tried to give
  a name; greeting "world" would pretend they didn't.

In the code this maps cleanly: `name_from_args` returns `None` only when
the argument list is empty, and `Some("")` for "attempted but empty" — so
both "attempted" cases share the `there` arm of `greeting()` with no
special-casing.

### 3. Empty string ≠ whitespace

`""` is empty; `" "` (a space) is a *real* name — an odd one, but the
program doesn't get to decide what counts as a real name. So
`hello-world " "` prints `Hello,  !`. This keeps the rule trivially
stated: *zero-length arguments are dropped, everything else is kept*.

### 4. UTF-8 failure is all-or-nothing, and it's not an error

On Unix, command-line arguments are byte strings and may not be valid
UTF-8. When one argument fails validation:

- **The whole batch is discarded**, not just the broken argument. A
  partially broken name (`hello-world Ru\uFFst <garble>`) would be
  half-greeted, which could mask a corrupted or maliciously crafted input.
  All-or-nothing is easier to reason about and to document.
- **The result is `there`, not `world`** — see §2: input was attempted.
- **Exit code stays 0.** The program's job is to greet, it greeted, and a
  hostile/accidental byte sequence in an argument isn't a failure *of the
  program*. We deliberately do not print an error to stderr: this is a
  friendly hello, not a validator.

### 5. `args_os()`, not `args()`

Recent Rust versions (the toolchain this was developed on: 1.98) make
`std::env::args()` **panic** when handed a non-UTF-8 argument, instead of
skipping it. We want graceful `there`-behaviour, so `main` reads
`args_os()` and each argument is converted losslessly with
`OsString::into_string()` — which is also what makes the failure in §4
observable at all.

### 6. Verbatim is a feature

The program performs no Unicode normalization (NFC/NFD), no trimming, no
case folding. `e` + U+0301 (combining acute) and `é` (precomposed) produce
different output, because they are different input. A greeting tool should
never silently rewrite what someone typed; "pretty printing" belongs to
the display layer, not to data pass-through.

### 7. Testability by construction

`main()` is three lines; all decisions live in two pure functions
(`name_from_args`, `greeting`) that take plain data and return plain data.
Consequences:

- **Unit tests** exercise the contract directly, no process spawning, no
  environment setup. `name_from_args` takes `&[OsString]`, which lets
  tests construct invalid-UTF-8 input on Unix — something you can't do
  through `std::env` from inside the same process.
- **Integration tests** (`tests/cli.rs`) run the *real binary* via
  `env!("CARGO_BIN_EXE_hello-world")` and compare exact stdout bytes,
  covering argument passing, the OS boundary, exit status, and UTF-8
  validity of the output.
- **Property-style testing**: one unit test feeds ~19 edge-case names
  (accented scripts, CJK, RTL, ZWJ/skin-tone emoji, combining marks,
  zero-width and no-break spaces, newlines, format-string metacharacters,
  whitespace-only, 10k chars) through `greeting` and asserts both exact
  output and **byte length** (`7 + name.len() + 1`) — proving the verbatim
  and no-truncation guarantees at the byte level.
- Behaviour regressions have been caught by this suite, not by users:
  the `args()` panic (§5) was found before the fix, by the non-UTF-8
  integration test failing.

### 8. Deliberately not done

- No `--help`/`-h` flag, no `clap`. Argument handling is ~5 lines; a
  parsing framework would be 95% of the codebase.
- No `--name`-style options. Positional words are the whole interface.
- No exit-code distinction between the fallbacks. "It always worked" is a
  feature for a greeting.
- No internationalization of the greeting itself. The *name* is
  international; the greeting is English.

## Test map

| Contract row / guarantee | Pinned by |
|---|---|
| nothing typed → `world` | `no_argument_greets_world` (int), `no_args_yields_none` + `greets_world_when_no_name_is_given` (unit) |
| joined single spaces | `extra_arguments_are_joined_with_spaces`, `quoted_and_unquoted_multi_word_names_match` (int), `args_are_joined_with_single_spaces` (unit) |
| empty args dropped; only-empties → `there` | `empty_arguments_are_ignored_among_valid_ones`, `only_empty_arguments_greet_there`, `single_empty_argument_greets_there` (int), `empty_args_are_ignored_among_others`, `only_empty_args_yield_an_empty_name`, `greets_there_when_the_name_is_empty` (unit) |
| whitespace-only is a real name | `whitespace_only_arg_is_a_real_name` (unit), verbatim table (unit) |
| invalid UTF-8 → `there`, exit 0, all-or-nothing | `invalid_utf8_argument_greets_there`, `mixed_invalid_and_valid_arguments_greet_there` (int), `any_invalid_utf8_arg_fails_the_batch_to_empty_name` (unit) |
| verbatim / no truncation | `name_is_embedded_verbatim_for_a_variety_of_inputs` (unit, byte-length check), `unicode_argument_is_greeted_exactly`, `long_argument_is_not_truncated` (int) |

## Known limitation: the integration tests are Unix-only in CI

On GitHub's `windows-latest` runners (observed 2026-09-19, `rustc 1.98.1`
stable, `x86_64-pc-windows-msvc`), any test running on a **libtest worker
thread** that calls `std::process::Command` dies with
`STATUS_STACK_OVERFLOW` (0xc00000fd) — even for a test that passes zero
arguments and even though:

* the same binary run directly (or by `cargo run`) works on that runner;
* a plain `std::thread::spawn` calling `Command` works, with or without
  `RUST_MIN_STACK`;
* every unit test passes on Windows (they never spawn a process);
* the failure is independent of stack size (`RUST_MIN_STACK` up to 32 MiB
  was set and confirmed in effect), build profile (debug and release both
  fail), test runner (`cargo test` and `cargo nextest` — fresh process per
  test — both fail), execution mode (serial and parallel), and capture
  mode (`--no-capture` included);
* the project's argument handling contains no recursion; the crashing
  tests exercise the simplest possible inputs.

Conclusion: a harness/environment bug in the toolchain's test harness on
that platform, not a project bug. `tests/cli.rs` is therefore gated
`#![cfg(unix)]` so the Windows CI job runs the unit tests (which pin the
entire parsing contract), plus `cargo fmt`, `cargo clippy --all-targets`
(which compiles and lints *every* target on all platforms — this caught a
real Windows-only unused-import bug before any test ran), and rustdoc.
The integration tests should be re-enabled for Windows by removing the
gate once the toolchain is fixed upstream. The full investigation trail
is in the git history of `.github/workflows/ci.yml` and this repository's
commit messages (2026-09-19).
