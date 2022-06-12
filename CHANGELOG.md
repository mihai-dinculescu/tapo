# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased]

### Added

- Generic Device example.

### Changed

- `get_device_usage` has been moved to the base implementation so that all devices have access to it.
- `Color` now implements `serde::Serialize` and `serde::Deserialize`.

### Removed

- `TapoDeviceExt` is no longer has `Default` and `serde::Serialize` as supersets.

## [v0.1.0] - 2022-06-07

### Initial Release of Tapo

[unreleased]: https://github.com/mihai-dinculescu/tapo
[v0.1.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.1.0
[keepachangelog]: https://keepachangelog.com
