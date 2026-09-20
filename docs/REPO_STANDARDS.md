# alambayana repository standards

This document is the brand manual for the `alambayana` repositories: the
shared file set, conventions, and workflow that every repository in the
family follows. `hello-world` is the reference implementation — copy from
it for every new project.

The goal is *uniformity and engagement*: anyone who opens any
`alambayana` repository should find the same structure, the same
standards, and the same professional finish.

## The standard file set

| File | Purpose | Customize per repo |
|---|---|---|
| `README.md` | Usage, examples, badges, project layout, link to design notes | Everything |
| `CONTRIBUTING.md` | Setup, verification bar, PR and commit conventions, release process | Repo URL, toolchain, test count |
| `CODE_OF_CONDUCT.md` | Contributor Covenant v2.1 — **use verbatim** | Contact line only |
| `SECURITY.md` | Supported versions, private reporting channel, response timeline | Version table, scope notes |
| `CHANGELOG.md` | Keep a Changelog format, `[Unreleased]` at the top | The entries |
| `LICENSE-MIT` + `LICENSE-APACHE-2.0` | Dual license — **use verbatim** | Copyright holder line in `LICENSE-MIT` |
| `.github/workflows/ci.yml` | fmt, clippy, test, rustdoc on 3 OSes, warnings denied | Job names (must match branch-protection check names) |
| `.github/workflows/release.yml` | Tag-triggered release + prebuilt binaries for 3 OSes | Binary name(s) in the upload step |
| `.github/ISSUE_TEMPLATE/` | Bug form (with platform dropdown + contract checkbox), feature form, config | Contact link in `config.yml` |
| `.github/PULL_REQUEST_TEMPLATE.md` | Change type + checklist mirroring the CI gates | Toolchain-specific commands |
| `docs/design.md` | Behaviour contract, design decisions, contract→test map | Everything |
| `.gitignore` | Build artifacts + editor noise | Per language |

## Metadata conventions

- **Repository name**: lowercase, hyphen-separated, short (`hello-world`).
- **Description** (GitHub "About"): one sentence — what it does, plus the
  quality story ("documented contract, N tests, prebuilt release
  binaries").
- **Topics**: include the language, the category (`cli`, `library`,
  `api`, …), and two or three content-specific topics.
- **License**: always dual `MIT OR Apache-2.0` (`license = "MIT OR
  Apache-2.0"` in `Cargo.toml`), so downstream users pick their preference.
- **Versioning**: start at `0.1.0`, follow [Semantic
  Versioning](https://semver.org/), and keep `Cargo.toml` version == tag
  `vX.Y.Z` == release title in lockstep.

## Identity and de-identification

Legal metadata carries the real name and email; everything visible in
day-to-day use carries the public handle:

- **Real name + email** — exactly two places: `authors` in `Cargo.toml`
  and the copyright line in `LICENSE-MIT`. (Same two places in
  `CODE_OF_CONDUCT.md`/`SECURITY.md` contact lines, which are
  functionally legal channels.)
- **Public handle `Alambayana` / `alambayana`** — README examples,
  docs, issue/PR templates, social media, and anything else that gets
  copied into screenshots, search engines, and social posts.

Rationale: legal attribution must be real; public-surface indexing
should be minimal and consistent.

## CI, releases, and protection

- **CI** (`ci.yml`): stable toolchain + `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, and
  rustdoc with `-D warnings`, run on `ubuntu-latest`, `macos-latest`,
  and `windows-latest` with `fail-fast: false`. Use step-level `env:`
  blocks (never inline `VAR=value cmd`) so the scripts work under
  PowerShell.
- **Releases** (`release.yml`): on `v*` tag push, create the release
  (idempotently) and attach release-mode binaries named with the actual
  `rustc` host triple so platform assets never collide.
- **Branch protection** on `main`: require all three CI check names to
  pass; enable "Require pull requests before merging"; consider
  "Apply these rules to administrators" once the setup is trusted.

## Bootstrapping a new repository (checklist)

1. `cargo new <name>`; set `main` as the default branch; push.
2. Copy the standard file set from `hello-world`; customize per the
   table above (URLs, names, toolchain, examples).
3. Fill in the GitHub "About": description + topics per the conventions.
4. Run the local verification bar; push the first commit.
5. Configure branch protection (same check names as the CI jobs).
6. Cut `v0.1.0`: changelog section, `Cargo.toml` bump, annotated tag,
   push — the release pipeline does the rest.
7. Re-check: CI badge green, release has all three binaries,
   "Security" and "Code of conduct" appear in the sidebar.

## Promoting to a GitHub template repository

When the brand matures (and a dedicated `alambayana/repo-template`
repo holds only the standard files, without `hello-world`'s code),
enable **Settings → General → Template repository** on it. Every new
project can then be generated from it in one click, and the badge row,
templates, workflows, and docs arrive pre-installed. `hello-world`
stays the *reference implementation* that the template is audited
against.
