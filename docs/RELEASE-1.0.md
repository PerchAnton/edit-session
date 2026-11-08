# 1.0.0 preparation

**Unreleased candidate.** The registry release remains 0.1.0. This change defines
the [1.x commitment](COMPATIBILITY.md), rather than adding another editing abstraction.
`src/lib.rs` is unchanged from 0.1.0; Rust 1.88, no_std and zero dependencies remain.

## Upgrade

After publication, change the dependency pin to `=1.0.0`, retain the lockfile and
run the host's Cancel, Reset, stale target/revision, no-op and rejected-command
regressions. No stored-data migration is needed. A host's source approval policy
still applies to a registry package, organisation-owned source or vendored copy.
See [the source choices and local-helper comparison](source-policy.md).

## Release gates

- [ ] Review the 1.x contract and candidate diff; confirm no runtime changes.
- [ ] Pass MSRV and stable CI, formatting, Clippy, lifecycle/consumer/example tests.
- [ ] Run the same consumer contract against the published 0.1.0 and extracted
      1.0.0 crate. Preserve exhaustive error matching and minimal trait bounds.
- [ ] Review the archive inventory, licenses and exact hash, including packaged tests.
- [ ] Review and merge the candidate; require successful CI on main.
- [ ] Obtain release authorization, create a new v1.0.0 tag, then use the existing
      crates-io workflow and environment approval. Never reuse v0.1.0.
- [ ] Compare registry archive bytes/hash; run a clean registry consumer before
      marking README, release notes or installation instructions as published.

Local results and PR CI are preparation evidence, not proof of publication.
Do not imply that PhotoCraft uses a crates.io dependency: its accepted integration
uses an internal source module. Existing integrations are not upgraded by this release.

## Announcement draft (only after publication)

edit-session 1.0 defines a compatible Rust API for draft validation and one-time
commit decisions. An executable Undo/Reset example shows why restoring a value
does not make an old draft current. The host still owns revisions, commands and
history. Compare the crate with a correct local helper before adding a dependency.
