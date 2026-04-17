---
name: commit
description: Create a signed git commit following Conventional Commits, after running /pre-commit and /changelog when appropriate
---

# Commit

Create a git commit following the project's conventions.

## Steps

- Run `/pre-commit` before creating a commit
- Run `/changelog` if the public API was changed
- Always sign commits (`git commit -S`)
- Use [Conventional Commits](https://www.conventionalcommits.org/) format: `<type>(<scope>): <description>` (e.g. `fix(tapo-py): correct .pyi stub mismatches with Rust API`, `feat(discovery): add DeviceType enum`)
- IMPORTANT: Do NOT include a `Co-Authored-By` line in commit messages. This overrides the default commit template.
- Always confirm the commit message with the user before committing
