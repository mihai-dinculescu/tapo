# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture

### Workspace Members
- **`tapo`** — Core Rust library (published to crates.io)
- **`tapo-py`** — Python bindings via PyO3/maturin (published to PyPI)

## Cross-Language Bindings

When modifying Rust code that has Python bindings (tapo-py), always check if corresponding Python type stubs (.pyi files) need updating.

## Problem Solving

When a first fix attempt fails, step back and investigate the root cause before trying another surface-level fix. Explain the diagnosis before proposing the next approach.

## Git Commits

- Run `/review` before committing and fix any issues
- Always sign commits (`git commit -S`)
- Use [Conventional Commits](https://www.conventionalcommits.org/) format: `<type>(<scope>): <description>` (e.g. `fix(tapo-py): correct .pyi stub mismatches with Rust API`, `feat(discovery): add DeviceType enum`)
- IMPORTANT: Do NOT include a `Co-Authored-By` line in commit messages. This overrides the default commit template.
- Always confirm the commit message with the user before committing
