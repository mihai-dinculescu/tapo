---
name: release-openclaw-skill
description: Release a new version of the OpenClaw skill in `tapo-mcp/openclaw-skill/` by bumping its version, committing, and publishing via `npx clawhub`
---

# Release OpenClaw skill

Release a new version of the OpenClaw skill in `tapo-mcp/openclaw-skill/` to ClawHub.

## Steps

1. **Determine the new version.** Read:
   - The current skill version from the `version:` field in `tapo-mcp/openclaw-skill/SKILL.md` frontmatter.
   - The current `tapo-mcp` server version from `tapo-mcp/Cargo.toml`.

   Suggest matching the `tapo-mcp` version. Ask the user to confirm or supply a different version. ClawHub validates version uniqueness on publish (not monotonic ordering) — any semver string that hasn't been published before for this skill is accepted.

2. **Bump the version.** If the new version differs from the existing `version:` in `tapo-mcp/openclaw-skill/SKILL.md`, update the frontmatter.

3. **Determine the changelog summary.** Look at recent commits affecting the skill directory:

   ```bash
   git log --oneline -10 -- tapo-mcp/openclaw-skill/
   ```

   Propose a one-sentence summary of what's new in this release and ask the user to confirm or refine. This text is passed to `--changelog` at publish time and stored on ClawHub per version.

4. **Commit pending changes.** If the version bump (or any other prep) left uncommitted changes, run `/commit` with message `chore(tapo-mcp): release openclaw skill vX.X.X`. Skip this step if there is nothing to commit.

5. **Publish.** Ask the user for confirmation, then:

   ```bash
   npx clawhub publish tapo-mcp/openclaw-skill \
     --slug tapo \
     --name "Tapo" \
     --version X.X.X \
     --tags "tapo,smart-home,iot,mcporter" \
     --changelog "<summary from step 3>"
   ```
