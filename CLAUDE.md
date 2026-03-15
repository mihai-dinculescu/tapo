# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture

### Workspace Members
- **`tapo`** — Core Rust library (published to crates.io)
- **`tapo-py`** — Python bindings via PyO3/maturin (published to PyPI)
- **`tapo-mcp`** — MCP server exposing Tapo devices as AI tools/resources

## Cross-Language Bindings

When modifying Rust code that has Python bindings (tapo-py), always check if corresponding Python type stubs (.pyi files) need updating. The `debug` feature in `tapo` is **user-facing public API** — `tapo-py` enables it (`features = ["python", "debug"]`), so all `debug`-gated types are also exposed to Python. Treat changes behind `cfg(feature = "debug")` as public API changes requiring changelog entries for both Rust and Python.

### Exposing Rust Types to Python
- **Simple value/result types**: Annotate directly in the `tapo` crate with `#[cfg_attr(feature = "python", pyo3::prelude::pyclass(from_py_object, ...))]` and `#[cfg(feature = "python")] crate::impl_to_dict!(TypeName);`. Do NOT create a redundant wrapper struct in `tapo-py`.
- **`serde_json::Value` → Python**: Use `crate::python::serde_object_to_py_dict`. Do NOT write custom conversion functions — pyo3's `serde` feature does not auto-implement `IntoPyObject` for `Value`.

### Python Stub Conventions
- `Ext` classes in `.pyi` stubs (e.g. `ToDictExt`, `DebugExt`, `OnOffExt`) must use `typing.Protocol` since they describe capabilities of PyO3 classes without real Python-level inheritance.

## Problem Solving

When a first fix attempt fails, step back and investigate the root cause before trying another surface-level fix. Explain the diagnosis before proposing the next approach.

## Git Commits

Always run `/commit` when committing. It contains the project's commit conventions.
