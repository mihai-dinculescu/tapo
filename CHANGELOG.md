# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased]

## [v0.4.0] - 2023-02-25

### Added

- `get_energy_data` is now available for the *P110* devices. (thanks to @kuhschnappel)

### Changed

- `EnergyUsageResult`'s `past24h`, `past7d`, `past30d` and `past1y` fields are now deprecated. `get_energy_data` should be used instead. (thanks to @felixhauptmann)

## [v0.3.1] - 2023-02-19

### Added
- `examples/tapo_generic_device_toggle.rs` demonstrates how `device_info` can be used to assess the current status of a generic device and toggle it.

### Changed
- `on_time` is now optional for the `L510` and `L530` devices because the v2 hardware no longer returns it.

## [v0.3.0] - 2022-11-20

### Added
- The `set` API allows multiple properties to be set in a single request for the *L510* and *L530* devices.

### Changed

- `tapo::Color` has been moved to `tapo::requests::Color`.
- `GenericDeviceInfoResult::on_time` has been changed from `u64` to `Option<u64>` because some devices (like *L930*) do not provide this field.
- All response structs have been moved under `tapo::responses`.
- The docs have been improved.

## [v0.2.1] - 2022-08-07

### Changed

- `latitude` and `longitude` in `GenericDeviceInfoResult`, `L510DeviceInfoResult`, `L530DeviceInfoResult` and `PlugDeviceInfoResult` are now signed integers to accommodate for incoming responses with negative numbers. (thanks to @JPablomr)

## [v0.2.0] - 2022-06-13

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
[v0.4.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.4.0
[v0.3.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.3.1
[v0.3.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.3.0
[v0.2.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.2.0
[v0.1.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.1.0
[keepachangelog]: https://keepachangelog.com
