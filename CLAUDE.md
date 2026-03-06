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

Always run `/commit` when committing. It contains the project's commit conventions.
