# hello-world

[![CI](https://github.com/alambayana/hello-world/actions/workflows/ci.yml/badge.svg)](https://github.com/alambayana/hello-world/actions/workflows/ci.yml)

A tiny command-line greeting tool in Rust. It says hello to whatever name you
give it — and behaves exactly the same way for every weird name you can
think of.

## Usage

```text
hello-world                    →  Hello, world!
hello-world Alambayana         →  Hello, Alambayana!
hello-world Rust is great      →  Hello, Rust is great!
hello-world ""                 →  Hello, there!
```

- Multiple arguments are joined with a single space, so unquoted names work
  just like quoted ones: `hello-world Rust is great` and
  `hello-world "Rust is great"` produce identical output.
- Empty-string arguments are ignored; if nothing usable remains, you get
  `Hello, there!`.
- The name is echoed **verbatim** — no escaping, normalization, or
  truncation. Unicode is welcome: `hello-world José 🦀 世界` →
  `Hello, José 🦀 世界!`.

## Building & running

Requires a recent Rust toolchain (edition 2024):

```sh
cargo run                  # Hello, world!
cargo run -- Alambayana    # Hello, Alambayana!
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

## AI assistance

This project was developed with **extensive AI assistance**: the
implementation, test suite, documentation, and CI/release pipeline were
drafted and refined in collaboration with an AI coding agent
(`Qwen3.8-27B`, Unsloth `UD-Q6_K_XL` quantization), with human direction,
review, and sign-off at every step.

All contributions — including the original — follow the AI-assistance
declaration standard in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Dual-licensed under either of:

- MIT License (see [LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 (see [LICENSE-APACHE-2.0](LICENSE-APACHE-2.0))

at your option.

## Further reading

Curious about *why* the program says `there` for broken input, or why
`args_os` instead of `args`? See [docs/design.md](docs/design.md).
