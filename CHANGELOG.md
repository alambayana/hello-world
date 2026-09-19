# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-09-19

### Added

- First release of the `hello-world` CLI: greets a name given as one or more
  command-line arguments (joined with a single space); `Hello, world!` with
  no arguments, `Hello, there!` when only empty arguments remain, and the
  same when any argument is not valid UTF-8
- 23 tests: unit tests (including a Unicode verbatim property table with
  byte-length checks) and end-to-end integration tests driving the real
  binary
- Continuous integration on Linux, macOS, and Windows (`cargo fmt`,
  `cargo clippy`, `cargo test`, rustdoc — all with warnings denied)
- Documentation: README, rustdoc, and `docs/design.md` (design decisions
  and a contract-to-test map)
- Dual MIT / Apache-2.0 licensing
- Release pipeline with prebuilt binaries for Linux, macOS, and Windows
