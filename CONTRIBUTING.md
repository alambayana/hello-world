# Contributing

Thank you for your interest in contributing! This project follows a small,
explicit set of standards shared across the **alambayana** repositories
(see [docs/REPO_STANDARDS.md](docs/REPO_STANDARDS.md)). Reading this page
before opening a pull request saves everyone time.

## Getting set up

Requirements: a recent stable Rust toolchain (install via
[rustup](https://rustup.rs/)).

```sh
git clone https://github.com/alambayana/hello-world.git
cd hello-world
cargo test    # the full suite
```

## The verification bar (what CI enforces)

Every change must pass all of the following **locally** before it is pushed:

| Check      | Command                                            |
|------------|----------------------------------------------------|
| Formatting | `cargo fmt --all -- --check`                       |
| Lints      | `cargo clippy --all-targets -- -D warnings`        |
| Tests      | `cargo test`                                       |
| Docs       | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`   |

CI runs these on Linux, macOS, and Windows, and `main` is protected: a
merge is blocked until all three OS checks are green.

## Working on a change

1. Branch off `main`: `git checkout -b fix/<short-description>`
2. Make the change. **If it changes behaviour**, update four things
   together — the code, its doc comments, `docs/design.md` (the behaviour
   contract and its contract→test map), and the test suite. New behaviour
   must be pinned by a new test before it is considered done.
3. Record user-visible changes under `[Unreleased]` in `CHANGELOG.md`.
4. Open a pull request using the PR template. Keep PRs small and
   single-purpose.

### Commit style

- Imperative, short subject (e.g. `Join CLI arguments into one greeting`).
- Explain *why* in the body; a commit should make sense on its own.
- One logical change per commit; squash away the noise on merge.

## Releases (maintainers)

Releases follow [Semantic Versioning](https://semver.org/):

1. Move the `[Unreleased]` `CHANGELOG.md` section to a versioned section.
2. Bump `version` in `Cargo.toml`.
3. Commit and push, then create and push an annotated tag:
   `git tag -a v0.2.0 -m "Release v0.2.0" && git push origin v0.2.0`
4. The `Release` workflow creates the GitHub release and attaches
   prebuilt binaries for Linux, macOS, and Windows automatically.

## AI-assisted contributions (required)

If AI assistance was used in a contribution, declare it in the pull
request. This is a brand standard for all `alambayana` repositories:
disclosure is required, and **"no AI used" is a valid, explicit answer**.

### Categories

Check every category in which AI assistance was used:

| Category | Covers |
|---|---|
| **Implementation** | Source code: features, bug fixes, refactors, generated code |
| **Testing** | Test cases, test strategy, property tables, debugging failures |
| **Documentation** | README, design docs, doc comments, changelog |
| **Review & debugging** | Code review, bug hunting, CI/performance work, architecture discussion |

### What to report

- **Model(s)**: one canonical identifier per model —
  `Family-Size (quantization)` — for example
  `Qwen3.8-27B (Unsloth UD-Q6_K_XL quantization)`. Name every model
  involved, including models used only for review.
- **Nature of assistance**: what the model(s) actually did — drafted,
  generated, reviewed, debugged — and over which files or functions.

### Where

- **Pull request** — the required, per-contribution declaration (see the
  PR template). The PR is the unit of review, so disclosure travels
  with it.
- **README** — a project-level "AI assistance" summary, maintained by
  the maintainer and refreshed at each release, describing the current
  state of the codebase.
- **Not** per commit (history stays clean), **not** in package metadata
  such as `Cargo.toml` (disclosure is an audit concern, not a build
  concern), and **not** on issues (requests, not code contributions).

## Code of conduct

Interactions in this project — issues, pull requests, discussions — are
covered by the [Code of Conduct](CODE_OF_CONDUCT.md).
