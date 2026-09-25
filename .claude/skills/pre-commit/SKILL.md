---
name: pre-commit
description: Run Rust/Python/MCP checks, fix issues, then summarize findings before committing
---

# Pre-Commit

Run all checks, fix any issues found, then present a summary table.

## Checks

### Rust checks

Run the following checks if there are changes in the `tapo/`, `tapo-py/`, or `tapo-mcp/` directories. Fix all issues found. Run independent checks (`cargo check`, `cargo clippy`, `cargo fmt`, `cargo test`) in parallel.
Unless otherwise specified, run checks at the workspace level (no `-p` flags) with `--all-features` to ensure cross-crate issues are caught.
Link-free checks (`check`, `clippy`) take `--workspace` so they cover `tapo-py` too; `cargo test` must NOT — it links test binaries and examples, which fails with `tapo-py`'s `pyo3/extension-module`, so it relies on `default-members` excluding `tapo-py`.

- `cargo check --workspace --all-features`
- `cargo clippy --workspace --all-features`
- `cargo fmt --all`
- `cargo test --all-features`
- `cargo clean --doc && RUSTDOCFLAGS="-D warnings" cargo doc -p tapo --no-deps --all-features`
- No `unwrap()` in non-test code without a `// safe:` comment
- No `unsafe` in non-test code without a `// SAFETY:` comment
- No unnecessary clones
- No deeply nested `use` (max one level of `{}` nesting)
- If a protocol puts a session token or other secret into a request URL in a new place, update `redact_session_token` in `tapo/src/error.rs` and its tests, because reqwest prints the full URL in its errors
- Module layout, for every module added or touched by the change:
  - A module with submodules is declared by `module_name.rs` next to the `module_name/` folder, never by `module_name/mod.rs`
  - `module_name.rs` holds only its `mod` declarations and `use`/`pub use` re-exports (plus `//!` docs and attributes such as `#[cfg(...)]` on them). Move any types, functions, impls, constants, or tests into a submodule file

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
- Verify that `tapo-mcp/README.md` reflects any MCP API changes (tools, resources, capabilities, env vars, auth)
- Verify that the OpenClaw skill reflects any MCP API changes. The skill spans three files — check each:
  - `tapo-mcp/openclaw-skill/SKILL.md` — frontmatter (`description`, `version`, `requires`), Setup, Tools section with example `npx mcporter call` invocations
  - `tapo-mcp/openclaw-skill/references/setup.md` — verification table (tool, description, parameters)
  - `tapo-mcp/openclaw-skill/references/tapo-mcp-setup.md` — Tools table, Resources table, Configuration env vars, Authentication, Deployment (kept in sync with `tapo-mcp/README.md`)
- When device-support categories change (e.g. adding a new family like the H100 hub), verify the device-type enumeration is in sync across both surfaces that list it:
  - `tapo-mcp/src/server.rs` `with_instructions(...)` (e.g. `"plugs, lights, power strips, hubs and their child sensors, cameras"`)
  - `tapo-mcp/openclaw-skill/SKILL.md` frontmatter `description:` (e.g. `(lights, plugs, power strips, hubs and sensors, cameras)`)

### Documentation checks

Run the following checks if there are changes in the `tapo/` or `tapo-py/` directories. Fix all issues found.

- Verify that `SUPPORTED_DEVICES.md` is up to date: add, remove, or regroup rows/columns when a handler's public method list changed, a device model was added/removed, or a method's `#[cfg(feature = "debug")]` gating changed

### Decompiled app checks

Run these checks on every change, whatever directories it touches. Fix all issues found.

The decompiled Tapo Android app is a private reverse-engineering aid that lives outside the repo, and its names are obfuscated per app release. Nothing committed may point at it.

- Scan the files the change touches, including new untracked ones, for references to the decompiled sources, and for app class names written the Java way (`Type.CONSTANT`, `Type.method()`) or right after "app's":

  ```bash
  { git diff --name-only --diff-filter=d HEAD -- . ':!.claude/skills/pre-commit/SKILL.md'; git ls-files -o --exclude-standard -- . ':!.claude/skills/pre-commit/SKILL.md'; } | xargs grep -nEi '\.(java|kt|smali)\b|decompil|base\.apk|apkmirror|jadx|com[./]tplink'
  { git diff --name-only --diff-filter=d HEAD -- . ':!.claude/skills/pre-commit/SKILL.md'; git ls-files -o --exclude-standard -- . ':!.claude/skills/pre-commit/SKILL.md'; } | xargs grep -nE '`[A-Z][A-Za-z0-9]*\.([A-Z_]+|[a-z][A-Za-z0-9]*\(\))`|app(.s)?[^`]{0,30}`[A-Z][a-z]+[A-Z][A-Za-z]*`'
  ```

  The second pattern works one line at a time, so it misses a class name that wraps onto the line after "app". Then read every comment in the touched files that mentions the app, not only the lines the diff changed, for what the patterns miss: two/three-character package or class names (`ab1`, `xy2`, `zz9/c1`), app-internal class and method names (`KeyMixer.d()`, `StreamGatewayImpl`), and absolute paths into the decompiled tree.
- Rewrite every hit to state what the app does, not where that was read. Keep wire-level names (JSON methods and fields, HTTP headers, request and response shapes as they travel); drop file, class, and line-number citations.
  - Not OK: ``//! The Tapo app (`ab1/c.java`, `xy2/b.java`) derives the keys from the `Key-Exchange` header``
  - OK: ``//! The Tapo app derives the keys from the `Key-Exchange` header``
- The one intentional mention is the "Reverse Engineering the Tapo API" section in `CONTRIBUTING.md`, which describes the workflow without naming a file. Leave it as is.

## Code Review

After fixing all issues found in the checks, review the code changes for correctness, readability, and maintainability and propose improvements.
Summarize the findings according to severity.
