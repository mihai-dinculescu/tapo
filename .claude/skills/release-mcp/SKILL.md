# Release tapo-mcp

Release a new version of the `tapo-mcp` crate.

## Steps

1. **Determine the new version.** Read the current version from `tapo-mcp/Cargo.toml`. Ask the user what the new version should be if not provided.

2. **Bump the version.** Update the `version` field in `tapo-mcp/Cargo.toml`.

3. **Update changelog.** In `CHANGELOG.md`:
   - Replace the heading `## [MCP Unreleased][Unreleased]` with `## [MCP vX.X.X][tapo-mcp-vX.X.X] - YYYY-MM-DD` (use the current date).
   - Insert a new empty `## [MCP Unreleased][Unreleased]` section immediately above the newly versioned section.
   - Add a link reference `[tapo-mcp-vX.X.X]: https://github.com/mihai-dinculescu/tapo/tree/tapo-mcp-vX.X.X` to the link definitions at the bottom of the file (right after the `[Unreleased]` link).

4. **Commit.** Run `/commit` with message `chore(tapo-mcp): release vX.X.X`.

5. **Tag.** Create an annotated git tag:

   ```bash
   git tag -a tapo-mcp-vX.X.X -m "tapo-mcp-vX.X.X"
   ```

6. **Push.** Ask the user for confirmation, then:

   ```bash
   git push && git push origin tapo-mcp-vX.X.X
   ```

7. **Create GitHub release.** Ask the user for confirmation. Use the content under the `## [MCP vX.X.X][tapo-mcp-vX.X.X]` heading from `CHANGELOG.md` as the release body, followed by a full changelog comparison link. The previous version (vP.P.P) is the tapo-mcp tag before this release:

   ```bash
   gh release create tapo-mcp-vX.X.X --title "Tapo MCP vX.X.X" --notes "$(cat <<'EOF'
   <changelog content here>

   **Full Changelog**: https://github.com/mihai-dinculescu/tapo/compare/tapo-mcp-vP.P.P...tapo-mcp-vX.X.X
   EOF
   )"
   ```
