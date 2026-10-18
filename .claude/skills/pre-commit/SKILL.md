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
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo fmt --all`
- `cargo test --all-features`
- `cargo clean --doc && RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features`
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
- The device family words in `tapo-mcp/src/server.rs` `with_instructions(...)` and the OpenClaw `description:` are checked by `/verify-device-support`, which the Rust and Python checks below run when a model or category changes

### Non-exhaustive enum checks

Run these checks if a change adds a variant to a `#[non_exhaustive]` enum in `tapo/src` (`DeviceType`, `DiscoveryResult`, `ChildDeviceHubResult`, `ChildDeviceHubIrResult`, `RecordingType`, …). Fix all issues found.

`#[non_exhaustive]` lets end users of `tapo` skip new variants with a `_ =>` arm. `tapo-py`, `tapo-mcp` and the examples in `tapo/examples` don't get that choice: they must support every variant `tapo` has. Their matches end in a `_ => unreachable!()` arm only to compile, so the compiler no longer flags a missing variant there. The grep below lists those enums and their `unreachable!()` sites. Other `_ =>` arms (the capability matches in `tapo-mcp`) don't show up in it and are covered by the **MCP capabilities** check:

```bash
grep -rn -A2 '#\[non_exhaustive\]' tapo/src/api/discovery tapo/src/responses | grep 'pub enum'
grep -rn 'unreachable!()' tapo-py/src tapo-mcp/src tapo/examples
```

For each new variant:

- **No `unreachable!()` hits**: every `match` that ends in `_ => unreachable!()` names the new variant. A variant without a capability is still named, in the arm that rejects it (for example, a device without on/off support in the `WrongDeviceType` arm of `control_device`)
- **Python mirrors**: enums that mirror the Rust one get the variant too, with all their per-variant methods: `PyDiscoveryResult` (`tapo-py/src/api/discovery/discovery_result.rs`), `PyRecordingType` and the hub child list macros. Add it to the `.pyi` stubs, including those for enums exposed directly, such as `DeviceType`
- **MCP capabilities**: `tapo-mcp/src/requests/get_devices.rs` and `check_device.rs` and the tools under `tapo-mcp/src/tools/` give the new variant every capability it supports, and list its children if it has any

### Documentation checks

Run the following checks if there are changes in the `tapo/` or `tapo-py/` directories. Fix all issues found.

- Run `/verify-device-support` when a device model or category was added or removed, a handler's public method list changed, or a method's `#[cfg(feature = "debug")]` gating changed. It checks `SUPPORTED_DEVICES.md`, the "Tested with" sentences, the builders, the handler and device info docs, the Python stubs, the examples and `tapo-mcp` against the code. Fix every finding it reports
- Verify that `TROUBLESHOOTING.md` still matches the library when the change touches protocols, authentication, discovery, error types or messages, or logging. Read the whole file, not only the sections the change seems to concern:
  - **Correct**: every error message, log line, URL path, port, protocol name, and example file it quotes is what the code produces now. Grep each quoted string against `tapo/src/` and `tapo-py/` and fix the ones that drifted
  - **Relevant**: every section describes a failure a user of the current library can still hit. Rewrite or remove sections, symptoms, and solutions about behavior the change removed. Describe only current behavior, with no notes about what older versions did
  - **Complete**: if the change adds or rewords a failure that users must resolve on their side (a new error variant, a message that tells them to change a setting, a new protocol or firmware limitation), add it to the matching section or write a new one in the existing Symptoms / Cause / Solutions layout. Skip this when the error message already states the cause and the fix, so the guide would only repeat it: an entry has to add something the message cannot carry, such as several steps to try in order, a cause that is not obvious, or symptoms that only show in the logs

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

### Integration test checks

Run these checks if anything under `tapo/tests/` was added or changed, including new untracked files. Fix all issues found.

The simulator stands in for a real device, so a test only proves something if the simulator behaves like the device and not like the library. Everything it knows comes from a fixture or from what real devices were seen to do.

#### Fixtures

Read every added or changed fixture in full.

- **Only device answers**: every key is the discovery response, a method `result`, a `login` answer keyed by `sub_method`, or an error response the device gave. Trimming the per-login fields the simulator computes is fine; adding a flag, setting, or hint that no device sends is not
- **Folder and name**: a camera or camera hub lives in `fixtures/smartcam/`, everything else in `fixtures/smart/`. The file name follows the convention in `tapo/tests/simulator/fixture.rs` and names the protocol the device takes. The simulator reads both, so a wrong folder or protocol changes how it behaves
- **No personal data**: only the device's behavior may survive the capture. Check every value that could identify a person, home, network, account, or single device:
  - Ids, tokens, and keys (`device_id`, `owner`, `hw_id`, `fw_id`, `oem_id`, `dev_id`, …) are obvious placeholders: hex counting patterns such as `0123456789ABCDEF…` or `FEDCBA98…`, all zeros, or `8000…0110` ending in the model number
  - MAC addresses are `AA-BB-CC-DD-EE-FF` or end in a short model suffix such as `-35`, in every spelling the device uses (`AABBCCDDEEFF` in the `discover` login answer)
  - IP addresses are `192.168.1.100`
  - Names are made up, like "Simulated Plug" or "Simulated Camera". Decode `nickname`, `ssid`, and every other base64 string to check; camera names such as `device_name` are plain text
  - `latitude` and `longitude` are `0`, or the London placeholder `515074` / `-1278` with `region` `Europe/London`
  - No email addresses, usernames, passwords, cloud tokens, user ids, room names, schedules, or other free text

  Leave values that only describe the device: model, versions, component lists, signal strength, settings, and usage or energy counters.

#### Simulator

- The simulator must not use the library it tests. The only allowed hits of this grep are in `assertions.rs`, which checks what the library returned:

  ```bash
  grep -rnE '\btapo::|use tapo\b' tapo/tests/simulator/
  ```

- The simulator must not copy the library's rules either. Read every changed function that decides device behavior (protocol, password transform, port, error) and check that it follows the fixture's data, folder, or name, or what a real device did, not the branches in `tapo/src/`. A comment that justifies the simulator with what "the library" does is a sign of this; rewrite it to say what the device does

## Code Review

After fixing all issues found in the checks, review the code changes for correctness, readability, and maintainability and propose improvements.
Summarize the findings according to severity.
