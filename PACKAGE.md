# edit-session

This page documents **1.0.0**. Check the [release status](https://github.com/PerchAnton/edit-session/releases/tag/v1.0.0) for registry availability.

Version **1.0.0** preserves the runnable local-helper comparison and the dependency source guide. Read the [compatibility boundary](https://github.com/PerchAnton/edit-session/blob/main/docs/COMPATIBILITY.md) before upgrading.

A small, toolkit-independent Rust library for preview, validation and commit
lifecycles. **`no_std`, zero runtime dependencies, Rust 1.88+.** MIT OR Apache-2.0.

Use it when several controls edit one draft and the host should commit one final
change. The host owns validation, preview rendering, commands, undo and document
identity. This crate does not provide widgets or an undo stack.

## Installation

Add the release to your dependencies:

```toml
[dependencies]
edit-session = "=1.0.0"
```

The 1.x line preserves the documented API. Pin an exact version and keep `Cargo.lock` for
applications. Read the
[compatibility policy](https://github.com/PerchAnton/edit-session/blob/main/docs/COMPATIBILITY.md)
before upgrading. For other distribution options and current publication status,
see the [project README](https://github.com/PerchAnton/edit-session#use-from-another-project).

## Example

```rust
use edit_session::EditSession;

let context = ("document-7", "layer-2", 42_u64);
let live_opacity = 100_u8;
let mut draft = EditSession::new(live_opacity, context);
draft.propose::<&str>(Ok(75)).unwrap();
draft.propose::<&str>(Ok(50)).unwrap();

if let Some(change) = draft.finish(&live_opacity, &context).unwrap() {
    assert_eq!(change.after, 50);
    // Send change.after to the host's validated command once.
}
```

`finish` consumes the session. If the host command can reject a change and the UI
must retain the draft for retry, use the borrowed-preview/check pattern in the
[tested recovery example](https://github.com/PerchAnton/edit-session/blob/main/examples/retry_command.rs).
Check the live value and context again on every attempt.

- [API source and contracts](https://github.com/PerchAnton/edit-session/blob/main/src/lib.rs)
- [Runnable example: Undo restores a value but leaves an old draft stale](https://github.com/PerchAnton/edit-session/blob/main/docs/CONTEXT_WALKTHROUGH.md)
- [Changelog](https://github.com/PerchAnton/edit-session/blob/main/CHANGELOG.md)
- [Integration evidence and limits](https://github.com/PerchAnton/edit-session/blob/main/docs/ADOPTION.md)
- [Report a bug](https://github.com/PerchAnton/edit-session/issues/new?template=bug_report.yml)

The separate [interaction-check](https://github.com/PerchAnton/interaction-check)
runner is optional and is not a dependency of this crate.
