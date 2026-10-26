# Releases and alpha compatibility

## Published prerelease

**[0.1.0-alpha.4 on crates.io](https://crates.io/crates/edit-session/0.1.0-alpha.4)**
was published on **27 October 2026 (Europe/Kyiv; 26 October UTC)**. Its public API,
behavior, Rust 1.88 minimum and zero runtime dependencies are unchanged from alpha.3.
A fresh consumer downloaded it from the registry and checked commit, no-op and
stale-context behavior. The archive hash matches the upload and release CI artifact.
[Evidence](evidence/release-alpha4.json) · [Installation](../README.md#use-from-another-project).
The previous alpha.3 GitHub prerelease remains unchanged.

## Alpha compatibility

- Alpha releases are experimental. Public types, methods, trait bounds and behavior
  may change between releases, even within `0.1.0`. Alpha.3 matching alpha.2 does not
  promise compatibility with every future alpha.
- Pin a full Git revision (or an exact crate version once published) and keep the
  application's lockfile. Before upgrading,
  read the [changelog](../CHANGELOG.md) and run the host's draft, stale-context,
  no-op and command-retry tests. Library tests do not replace those integration tests.
- Breaking changes, including a raised minimum Rust version, must be called out
  in the changelog with migration guidance. The current minimum is Rust **1.88**.
- Existing release tags and assets must not be replaced; fixes need a new version.

If an upgrade changes a result, [report the old and new revisions with a small test](https://github.com/PerchAnton/edit-session/issues/new?template=bug_report.yml).
A stable 1.0 needs a supported compatibility contract and experience maintaining
real integrations.

## Preparing a future release

Update the version and changelog for the new candidate, then run:

```sh
cargo test --locked
cargo run --locked --example retry_command
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo package --locked --list
cargo publish --dry-run --locked --registry crates-io
cargo package --locked
```

Run these from a clean checkout. `cargo package` verifies the extracted crate builds.
It also leaves an archive at the documented path; `cargo publish --dry-run` may keep
its intermediate archive elsewhere, depending on the Cargo version.
The archive includes sources, licenses, tests, examples and their documentation. CI
checks the source and example on Rust 1.88 and stable. To validate packaged tests too,
unpack the new `target/package/edit-session-<version>.crate` into a temporary directory
and run `cargo test --locked` and the example from that directory.

A GitHub prerelease can attach the checked `.crate` archive and SHA-256 checksum to
an exact green commit. It is a source archive, not a globally installed executable.
Consumers can continue using a fixed Git revision while the API is experimental.
Do not present an alpha release as the latest stable release.

## Registry publication

### Published version

`0.1.0-alpha.4` is **published on crates.io**. The API, behavior, Rust 1.88 minimum
and runtime dependencies are unchanged. Package metadata/docs change, so the candidate
uses a new version; alpha.3 tags and archives stay untouched. `publish = ["crates-io"]`
limits the registry target. `PACKAGE.md` is the registry landing page and uses absolute
links so it works outside GitHub.

The registry lists `PerchAnton` as the owner. This version cannot be overwritten;
prepare a new version for any future package change.

### First upload: completed

Alpha.4 was uploaded with the owner's scoped bootstrap token using Cargo 1.88.
The procedure below is retained for reference; do not upload this version again.

Crates.io currently requires an API token for a crate's **first** publication. Trusted
Publishing can only be configured after the crate exists. There is no token in this
repository and the OIDC workflow cannot bootstrap a new crate.

1. Sign in to [crates.io](https://crates.io/) with the intended GitHub owner account
   and verify the email in Account Settings.
2. Once publication is approved, create a short-lived API token restricted to
   publishing the new `edit-session` crate. Enter it through `cargo login --registry
   crates-io` at its private prompt; do not paste it into a chat, command argument or log.
3. Require green CI on the exact release commit and tag it `v0.1.0-alpha.4`. From a
   clean checkout of that tag, inspect the manifest/version, then run:

   ```sh
   cargo publish --dry-run --locked --registry crates-io
   cargo publish --locked --registry crates-io
   ```

4. Verify the registry version, owner, license, Rust minimum and README. Compare its
   downloaded `.crate` SHA-256 with the local archive from the actual upload. Revoke
   the bootstrap token in crates.io and run `cargo logout --registry crates-io`.

The second command uploads permanently; it is deliberately outside the default
rehearsal. Do not run it merely to test credentials or reserve a name.

### Subsequent releases: Trusted Publishing

In the published crate's **Settings → Trusted Publishing**, add:

| Field | Value |
| --- | --- |
| Repository owner | `PerchAnton` |
| Repository name | `edit-session` |
| Workflow filename | `publish.yml` |
| Environment | `crates-io` |

Create the `crates-io` GitHub environment, require owner approval and allow release
tags (`v*`). These repository files do not configure the account or environment.
The workflow uses the Rust project's OIDC action; no permanent CI token is needed.

Run **Prepare or publish to crates.io** from `main` with `publish=false` (default)
to rehearse: Rust 1.88 tests/Clippy, `cargo publish --dry-run`, packaged tests and the
retry example, plus a saved `.crate` and SHA-256 checksum. Pushes and PRs never upload.
Ordinary CI also checks stable Rust and performs a registry dry-run.

For a later approved version, require green CI and a matching version tag whose
commit is contained in `main`. Run the workflow from that tag with `publish=true`.
It rechecks the package before requesting the short-lived publish token. Do not use
the workflow to re-upload alpha.4 after its manual first publication.

### Verify installation after publication

Use a fresh consumer project and the exact prerelease dependency:

```toml
[dependencies]
edit-session = "=0.1.0-alpha.4"
```

Run `cargo check`, confirm `Cargo.lock` names the crates.io source (not a path/Git
override), and check the [API documentation](https://docs.rs/edit-session/0.1.0-alpha.4/).
Only then mark the README/changelog as published. Never
overwrite a published version; fixes require a new version.

Reference: [Cargo's publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html).
See also [crates.io Trusted Publishing](https://crates.io/docs/trusted-publishing).
