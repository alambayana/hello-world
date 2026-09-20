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

<!-- If you changed the behaviour contract, the contract → test map in
     docs/design.md is the source of truth: every row must still be covered. -->
