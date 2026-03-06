# Code Review Checklist

Run all checks, fix any issues found, then present a summary table.

## Rust (`tapo/`)

- `cargo check` — fix all errors
- `cargo clippy` — fix all warnings
- `cargo fmt` — fix all formatting issues
- `cargo test -p tapo` — fix all test failures
- `cargo clean --doc && RUSTDOCFLAGS="-D warnings" cargo doc -p tapo --no-deps --all-features` — fix all documentation issues
- No `unwrap()` in non-test code without a safety comment
- No unnecessary clones
- No deeply nested `use` (max one level of `{}` nesting)

## Python (`tapo-py/`)

- Update `.pyi` stubs if Python-exposed Rust types changed
- `cd tapo-py` and activate the virtual environment
- `uv run mypy .` — fix all type errors
- `uv run black .` — fix all formatting issues
