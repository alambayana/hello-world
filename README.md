# hello-world

A tiny command-line greeting tool in Rust. It says hello to whatever name you
give it — and behaves exactly the same way for every weird name you can
think of.

## Usage

```text
hello-world                    →  Hello, world!
hello-world Devajyoti          →  Hello, Devajyoti!
hello-world Devajyoti Sarkar   →  Hello, Devajyoti Sarkar!
hello-world ""                 →  Hello, there!
```

- Multiple arguments are joined with a single space, so unquoted names work
  just like quoted ones: `hello-world Devajyoti Sarkar` and
  `hello-world "Devajyoti Sarkar"` produce identical output.
- Empty-string arguments are ignored; if nothing usable remains, you get
  `Hello, there!`.
- The name is echoed **verbatim** — no escaping, normalization, or
  truncation. Unicode is welcome: `hello-world José 🦀 世界` →
  `Hello, José 🦀 世界!`.

## Building & running

Requires a recent Rust toolchain (edition 2024):

```sh
cargo run                  # Hello, world!
cargo run -- Devajyoti     # Hello, Devajyoti!
cargo run -- Rust is great # Hello, Rust is great!
```

## Testing

The full behaviour contract is pinned by 23 tests (12 unit + 11
end-to-end integration tests that run the real binary):

```sh
cargo test
```

## Project layout

```text
src/main.rs    # ~70 lines: greeting(), name_from_args(), main() + unit tests
tests/cli.rs   # integration tests that drive the real binary
docs/design.md # why each behaviour is the way it is
```

## Further reading

Curious about *why* the program says `there` for broken input, or why
`args_os` instead of `args`? See [docs/design.md](docs/design.md).
