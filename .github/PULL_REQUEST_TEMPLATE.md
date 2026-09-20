## Summary

<!-- What does this PR change, and why? One or two sentences. -->

## Type of change

- [ ] Bug fix
- [ ] New behaviour / feature
- [ ] Documentation
- [ ] CI / tooling
- [ ] Other

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] If behaviour changed: the crate doc comment, `docs/design.md`, and the
      test map were updated together, and the new behaviour is pinned by a test
- [ ] If user-visible: `CHANGELOG.md` has an entry under `[Unreleased]`

## AI assistance (required)

Check every category in which AI assistance was used in this change.
"No AI used" is a valid, explicit answer — in that case leave nothing
checked and write `No AI used` in the model field.

- [ ] Implementation (source code)
- [ ] Testing (test cases / strategy)
- [ ] Documentation (README, design docs, comments)
- [ ] Review & debugging (review, debugging, refactoring, CI)

| Field | Value |
|---|---|
| Model(s) | Canonical identifier(s), e.g. `Qwen3.8-27B` (Unsloth `UD-Q6_K_XL` quantization) — or `No AI used` |
| Nature of assistance | What the model(s) did (drafted / generated / reviewed / debugged) and over which files or functions |

<!-- If you changed the behaviour contract, the contract → test map in
     docs/design.md is the source of truth: every row must still be covered. -->
