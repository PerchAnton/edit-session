# README overview asset

`overview.svg` is an editable, self-contained vector graphic. It uses system fonts,
explicit light-background colors, a title and a description, with no scripts, external
fonts, images or trackers. The repository README repeats its measurements as accessible
text with links; the image alone is not the evidence source.

Snapshot: **26 October 2026**. The before/after PdfCraft checks were recorded on
25 October. Sources:

- [Cargo.toml](../../Cargo.toml): zero runtime dependencies.
- [Lifecycle tests](../../tests/lifecycle.rs) and [doctest](../../src/lib.rs): 16 + 1.
  The clone-count test covers 999 proposal/check pairs, one construction clone,
  one additional Reset clone, and no finish clone. It does not count caller work.
- [Adoption analysis](../ADOPTION.md): two consumer regressions, exact baseline/fix
  revisions, and the production UI diff of 267 added / 50 removed lines.

To update the graphic, first reproduce or obtain new evidence. Then edit the SVG text
and proportional bar widths, its snapshot date, and the README table together. Keep
the consumer approval status and measurement limitations visible. Render the SVG and
check text fit at the GitHub README width. The CI badge is live; these counts are not.
