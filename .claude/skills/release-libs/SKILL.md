---
name: release-libs
description: Release a new version of the tapo (Rust) and tapo-py (Python) crates together, sharing the same version number
---

# Release tapo & tapo-py

Release a new version of the `tapo` (Rust) and `tapo-py` (Python) crates. Both libraries are released simultaneously and share the same version number.

## Steps

1. **Determine the new version.** Read the current version from `tapo/Cargo.toml`. Ask the user what the new version should be if not provided.

2. **Bump the versions.** Update the `version` field in both:
   - `tapo/Cargo.toml`
   - `tapo-py/pyproject.toml`

3. **Update changelog.** In `CHANGELOG.md`:
   - Replace the heading `## [Python Unreleased][Unreleased]` with `## [Python vX.X.X][vX.X.X] - YYYY-MM-DD` (use the current date).
   - Replace the heading `## [Rust Unreleased][Unreleased]` with `## [Rust vX.X.X][vX.X.X] - YYYY-MM-DD` (use the current date).
   - Insert new empty `## [Rust Unreleased][Unreleased]` and `## [Python Unreleased][Unreleased]` sections (in that order) immediately above the newly versioned Rust section.
   - Add a link reference `[vX.X.X]: https://github.com/mihai-dinculescu/tapo/tree/vX.X.X` to the link definitions at the bottom of the file (right after the `[Unreleased]` link).

4. **Commit.** Run `/commit` with message `chore(tapo): release vX.X.X`.

5. **Tag.** Create an annotated git tag:

   ```bash
   git tag -a vX.X.X -m "vX.X.X"
   ```

6. **Push.** Ask the user for confirmation, then:

   ```bash
   git push && git push origin vX.X.X
   ```

7. **Create GitHub release.** Ask the user for confirmation. Use the content under the `## [Rust vX.X.X][vX.X.X]` and `## [Python vX.X.X][vX.X.X]` headings from `CHANGELOG.md` as the release body, followed by a full changelog comparison link. The previous version (vP.P.P) is the tag before this release:

   ```bash
   gh release create vX.X.X --title "Tapo vX.X.X" --notes "$(cat <<'EOF'
   ## Rust

   <rust changelog content here>

   ## Python

   <python changelog content here>

   **Full Changelog**: https://github.com/mihai-dinculescu/tapo/compare/vP.P.P...vX.X.X
   EOF
   )"
   ```
