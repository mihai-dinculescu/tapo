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
- Dependency upgrades use the `deps` / `deps-dev` scope and match Dependabot's subject line style:
  - Runtime deps: `chore(deps): bump <name> from <old> to <new>` (e.g. `chore(deps): bump opentelemetry-otlp from 0.31.1 to 0.32.0`)
  - Dev-only deps: `chore(deps-dev): bump <name> from <old> to <new> in /<path>` (e.g. `chore(deps-dev): bump maturin from 1.13.1 to 1.13.3 in /tapo-py`)
- IMPORTANT: Do NOT include a `Co-Authored-By` line in commit messages. This overrides the default commit template.
- Always confirm the commit message with the user before committing
