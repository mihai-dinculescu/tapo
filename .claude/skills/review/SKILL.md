# Code Review Checklist

Run all checks, fix any issues found, then present a summary table.

## Rust (`tapo/`)

- `cargo check` — fix errors
- `cargo clippy` — fix warnings
- `cargo fmt` — fix formatting
- `cargo test -p tapo` — fix failures
- No `unwrap()` in non-test code without a safety comment
- No unnecessary clones
- No deeply nested `use` (max one level of `{}` nesting)

## Python (`tapo-py/`)

- Update `.pyi` stubs if Python-exposed Rust types changed
- `cd tapo-py && uv run mypy .` — fix type errors
- `cd tapo-py && uv run black .` — fix formatting
