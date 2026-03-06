# Changelog

Update `CHANGELOG.md` when the public API of the Rust or Python libraries is changed. The file follows [Keep a Changelog](https://keepachangelog.com/) with separate `[Rust Unreleased]` and `[Python Unreleased]` sections.

## Style

- Use the appropriate section: `### Added`, `### Changed`, `### Fixed`, `### Removed`
- Entries use the format: `` - `Type`: verb phrase. `` (e.g. `` - `HubHandler`: added `device_reboot` method. ``)
- Start with the type/handler name in backticks, followed by a colon and a verb (added, changed, corrected, removed)
- When a change affects both Rust and Python, add an entry to both unreleased sections
