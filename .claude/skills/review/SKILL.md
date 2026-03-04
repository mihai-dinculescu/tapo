# Code Review Checklist

## Rust (`tapo/`, `tapo-py/`)

- Run `cargo check` and fix any errors
- Run `cargo clippy` and address any warnings
- Run `cargo fmt` to ensure proper formatting
- Run `cargo test` to ensure all tests pass
- No `unwrap()` in non-test code (if you need to use `unwrap()`, add a comment explaining why it's safe to do so)
- No unnecessary clones
- `use` statements should not nest more than one level deep (e.g. `use axum::extract::{Path, State};` is fine, but `use axum::{extract::{Path, State}, http::StatusCode};` is not — split it into `use axum::extract::{Path, State};` and `use axum::http::StatusCode;`)

## Python bindings (`tapo-py/`)

- If Rust types exposed to Python changed, update corresponding `.pyi` stubs
- Run `uv run black .` in `tapo-py/` to format Python code

## Summary

Present a summary of all findings.
