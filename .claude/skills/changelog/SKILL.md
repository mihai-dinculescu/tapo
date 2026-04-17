---
name: changelog
description: Update CHANGELOG.md when the public API of the tapo (Rust), tapo-py (Python), or tapo-mcp (MCP) crates changes
---

# Changelog

Update `CHANGELOG.md` when the public API of the Rust or Python libraries or the MCP server is changed. The file follows [Keep a Changelog](https://keepachangelog.com/) with separate `[Rust Unreleased]`, `[Python Unreleased]`, and `[MCP Unreleased]` sections.

## Style

- Use the appropriate section: `### Added`, `### Changed`, `### Fixed`, `### Removed`
- Entries use the format: `` - `Type`: verb phrase. `` (e.g. `` - `HubHandler`: added `device_reboot` method. ``)
- Start with the type/handler name in backticks, followed by a colon and a verb (added, changed, corrected, removed)
- When a change affects both Rust and Python, add an entry to both unreleased sections
- When a change affects the MCP server (`tapo-mcp/`), add an entry to the `[MCP Unreleased]` section
