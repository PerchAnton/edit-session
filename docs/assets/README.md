# README overview asset

`overview.svg` is an editable, self-contained vector graphic. It uses system fonts,
explicit light-background colors, a title and a description, with no scripts, external
fonts, images or trackers. The repository README repeats its measurements as accessible
text with links; the image alone is not the evidence source.

Snapshot: **26 October 2026**. The before/after PdfCraft checks were recorded on
25 October. Sources:

- [Cargo.toml](../../Cargo.toml): zero runtime dependencies.
- [Recovery example](../../examples/retry_command.rs): 8 host-model tests; not consumer UI tests.
- [Lifecycle tests](../../tests/lifecycle.rs) and [doctest](../../src/lib.rs): 16 + 1.
  The clone-count test covers 999 proposal/check pairs, one construction clone,
  one additional Reset clone, and no finish clone. It does not count caller work.
- [Adoption analysis](../ADOPTION.md): two consumer regressions, exact baseline/fix
  revisions, and the production UI diff of 267 added / 50 removed lines.

To update the graphic, first reproduce or obtain new evidence. Then edit the SVG text
and proportional bar widths, its snapshot date, and the README table together. Keep
the consumer approval status and measurement limitations visible. Render the SVG and
check text fit at the GitHub README width. The CI badge is live; these counts are not.

`commit-boundary.svg` is an API responsibility diagram, not a new measurement.
Its source is `EditSession::check`, `EditSession::finish` and `Change` in
[src/lib.rs](../../src/lib.rs). It shows the host/library/host sequence and the
consuming finish boundary, including command-error recovery. The README gives
the same information in a text table. Existing measured counts are unchanged.

## Release landing graphic

`release.svg` is a dated release summary and a three-step workflow diagram. Its
version, publication date, exact compressed file size and CI references come from
the [release snapshot](../evidence/release-alpha3.json). KiB means bytes / 1024,
rounded to one decimal; this is download size, not runtime memory or a comparison
between projects. The README repeats the scoped counts and links to the real assets.

The workflow describes the implemented example/CLI; it is not a performance chart.
For interaction-check, native-test translation is manual and a prefix is not an
automatically minimized or rerun reproduction. For edit-session, recovery belongs
to the host example and requires atomic rejection and serialized freshness checks.
Existing comparative figures remain available in the README's expandable evidence
section. CI badges are live; release facts are snapshots. Do not update an old release's
size or date using an unreleased package. Inspect the SVG at README width after edits.
