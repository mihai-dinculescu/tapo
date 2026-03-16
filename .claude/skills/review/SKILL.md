# Code Review

Run all checks, fix any issues found, then present a summary table.

## Checks

### Rust checks

Run the following checks if there are changes in the `tapo/`, `tapo-py/`, or `tapo-mcp/` directories. Fix all issues found. Run independent checks (`cargo check`, `cargo clippy`, `cargo fmt`, `cargo test`) in parallel.
Unless otherwise specified, run checks at the workspace level (no `-p` flags) with `--all-features` to ensure cross-crate issues are caught.

- `cargo check --all-features`
- `cargo clippy --all-features`
- `cargo fmt --all`
- `cargo test --all-features`
- `cargo clean --doc && RUSTDOCFLAGS="-D warnings" cargo doc -p tapo --no-deps --all-features`
- No `unwrap()` in non-test code without a `// safe:` comment
- No `unsafe` in non-test code without a `// SAFETY:` comment
- No unnecessary clones
- No deeply nested `use` (max one level of `{}` nesting)

### Python checks

Run the following checks if there are changes in the `tapo/` or `tapo-py/` directories. Fix all issues found.

- Update `.pyi` stubs if Python-exposed Rust types changed
- Verify new `#[pyclass]` types are imported and registered in `tapo-py/src/lib.rs`
- Verify Python examples in `tapo-py/examples/` are updated to match corresponding Rust examples in `tapo/examples/`
- `cd tapo-py` and activate the virtual environment
- `uv run mypy .` — fix all type errors
- `uv run black .` — fix all formatting issues

### MCP checks

Run the following checks if there are changes in the `tapo-mcp/` directory. Fix all issues found.

- Verify that **all** `#[derive(JsonSchema)]` types have `schemars` annotations — including tool input params, response types, enums, and their fields/variants. Check for descriptions and range constraints where applicable.
- Verify that `README.md` reflects any MCP API changes
- Verify that the OpenClaw skill (`openclaw-skill/`) reflects any MCP API changes

## Code Review

After fixing all issues found in the checks, review the code changes for correctness, readability, and maintainability and propose improvements.
Summarize the findings according to severity.
