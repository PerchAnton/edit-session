# Release preparation

The next candidate is **v0.1.0-alpha.3**. The public library API is the same as alpha.2;
the new content is the tested recovery example, documentation and package verification.
The crate has not been published to crates.io; `publish = false` remains explicit.

```sh
cargo test --locked
cargo run --locked --example retry_command
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo package --locked --list
cargo package --locked
```

Run these from a clean checkout. `cargo package` verifies the extracted crate builds.
The archive includes sources, licenses, tests, examples and their documentation. CI
checks the source and example on Rust 1.88 and stable. To validate packaged tests too,
unpack `target/package/edit-session-0.1.0-alpha.3.crate` into a temporary directory and
run `cargo test --locked` and the example from that directory.

A GitHub prerelease can attach the checked `.crate` archive and SHA-256 checksum to
an exact green commit. It is a source archive, not a globally installed executable.
Consumers can continue using a fixed Git revision while the API is experimental.
Do not present an alpha release as the latest stable release.

Crates.io publication is a separate decision: confirm name/account ownership, approve
the registry target, change `publish = false` to the intended registry, then run
`cargo publish --dry-run` before an authenticated upload. No registry upload or publish
credentials are configured by this change. A stable 1.0 needs a supported compatibility
contract and experience maintaining real integrations.

Reference: [Cargo's publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html).
