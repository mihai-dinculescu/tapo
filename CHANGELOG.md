# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased][Unreleased]

### Rust

### Python

## [v0.8.1][v0.8.1] - 2025-02-10

### Rust

#### Added

- Added functionality for controlling the alarm on the H100 hub via the `play_alarm` and `stop_alarm` methods in the `H100Handler`. Additionally, `get_supported_ringtone_list` is available to retrieve the list of supported ringtones for debugging purposes. (thanks to @kay)
- Added the ability to retrieve the color configuration (`hue`, `saturation`, `color_temperature`) for the `Color` enum values through the `get_color_config` method. (thanks to @WhySoBad)

#### Changed

- The internal implementation of `H100Handler`'s `get_child_device_list` has been updated to fetch all pages, not just the first one.
- `H100Handler`'s `get_child_device_list_json` now includes a `start_index` parameter to fetch child devices starting from a specific index.

#### Fixed

- Resolved an issue that caused the passthrough protocol test to incorrectly indicate support when it was not actually supported. (thanks to @WhySoBad)

### Python

#### Added

- Added functionality for controlling the alarm on the H100 hub via the `play_alarm` and `stop_alarm` methods in the `H100Handler`. Additionally, `get_supported_ringtone_list` is available to retrieve the list of supported ringtones for debugging purposes.
- Added the ability to retrieve the color configuration (`hue`, `saturation`, `color_temperature`) for the `Color` enum values through the `get_color_config` method. (thanks to @WhySoBad)

#### Changed

- The internal implementation of `H100Handler`'s `get_child_device_list` has been updated to fetch all pages, not just the first one.
- `H100Handler`'s `get_child_device_list_json` now includes a `start_index` parameter to fetch child devices starting from a specific index.

#### Fixed

- Resolved an issue that caused the passthrough protocol test to incorrectly indicate support when it was not actually supported. (thanks to @WhySoBad)

## [v0.8.0][v0.8.0] - 2024-12-07

This marks the first unified release of the Rust and Python libraries. Moving forward, both libraries will be released simultaneously and will share the same version number.

### Rust

#### Added

- Added an example for the L900 light strips.

#### Changed

- `LightingEffect`'s `fadeoff` field has been renamed to `fade_off`, and its `with_fadeoff` method has been renamed to `with_fade_off`.
- `LightingEffect`'s `new_with_random_id` function has been removed. The `new` function now creates a `LightingEffect` instance with a random ID by default.

### Python

#### Added

- Added support for the L900 light strips.
- Added support for the L920 and L930 light strips.
- Added support for Python 3.13.

## [Python v0.7.0][py-v0.7.0] - 2024-11-07

### Added

- Added support for the KE100 thermostatic radiator valve (TRV).

## [Rust v0.7.17][v0.7.17] - 2024-10-23

### Added

- Added support for the P304 power strip.

### Changed

- The `openssl` dependency has been replaced with native Rust alternatives to expand cross-compilation options, such as for Android, and to decrease build times (thanks to @rbock44).
- `PlugPowerStripHandler` has been renamed to `PowerStripPlugHandler` to be consistent with the rest of the library.
- `PlugPowerStripResult` has been renamed to `PowerStripPlugResult` to be consistent with the rest of the library.
- The `UsageByPeriodResult` fields `today`, `past7`, and `past30` have been updated to `Option<u64>` to handle cases where the API returns negative values, which will be represented as `None`.

### Fixed

- Updated all comments referencing Watts to confirm the correct units are specified.

## [Python v0.6.0][py-v0.6.0] - 2024-10-23

### Added

- Added support for the P300 and P304 power strips.
- Python logs can now capture entries from the underlying Rust library.

### Changed

- The `openssl` dependency has been replaced with native Rust alternatives to expand cross-compilation options, such as for Android, and to decrease build times (thanks to @rbock44).
- The `UsageByPeriodResult` fields `today`, `past7`, and `past30` have been updated to `Optional[int]` to handle cases where the API returns negative values, which will be represented as `null`.

### Fixed

- Updated all comments referencing Watts to confirm the correct units are specified.

## [Rust v0.7.16][v0.7.16] - 2024-09-27

### Added

- Added support for the L535 light bulbs.

### Fixed

- Fixed an issue that prevented the color from being set properly for the L535 light bulbs.

## [Python v0.5.1][py-v0.5.1] - 2024-09-27

### Added

- Added support for the L535 light bulbs.

### Fixed

- Fixed an issue that prevented the color from being set properly for the L535 light bulbs.

## [Rust v0.7.15][v0.7.15] - 2024-09-18

### Added

- The `LowBattery` variant has been added to the `S200BLog` enum.

### Changed

- The `t310` and `t315` methods of `HubHandler` can now create `T31XHandler` handlers for either of the two device types.
- The child device handlers for the H100 hub and the P300 power strip have been redesigned to eliminate the use of lifetimes, to facilitate FFI integrations.
- The comments of `start_timestamp` and `end_timestamp` fields in `EnergyDataResult` have been updated to better describe their purpose.
- `S200BRotationParams`'s field `degrees` has been renamed to `rotation_degrees`.

### Fixed

- Fixed an issue with the `Color` presets that triggered a validation error when attempting to set the `color` to `DarkRed`.

### Removed

- The deprecated `past24h`, `past7d`, `past30d` and `past1y` fields have been removed from `EnergyUsageResult`. This data is now available exclusively through `get_energy_data`'s `EnergyDataResult` response.

## [Python v0.5.0][py-v0.5.0] - 2024-09-18

### Added

- Added full support for the S200B switches through the `S200BHandler` handler.
- Added full support for the T100 sensors through the `T100Handler` handler.
- Added full support for the T110 sensors through the `T110Handler` handler.
- Added full support for the T300 sensors through the `T300Handler` handler.
- Added full support for the T310 and T315 sensors through the `T31XHandler` handler.

### Changed

- The comments of `start_timestamp` and `end_timestamp` fields in `EnergyDataResult` have been updated to better describe their purpose.

### Fixed

- Fixed an issue with the `Color` presets that triggered a validation error when attempting to set the `color` to `DarkRed`.

## [Rust v0.7.14][v0.7.14] - 2024-08-31

### Changed

- `DeviceInfoPlugEnergyMonitoringResult` has been added to support the P110 and P115 devices, which have different responses compared to the P100 and P105 devices.

### Fixed

- `DeviceInfoPlugResult` has been updated to correctly support the P100 and P105 devices.

## [Python v0.4.0][py-v0.4.0] - 2024-08-31

### Changed

- `DeviceInfoPlugEnergyMonitoringResult` has been added to support the P110 and P115 devices, which have different responses compared to the P100 and P105 devices.

### Fixed

- Resolved an issue that led to unrecoverable process hangs when a device request timed out.
- The concurrency of device handlers has been significantly enhanced by replacing all `Mutex` instances with `RwLock`.
- `DeviceInfoPlugResult` has been updated to correctly support the P100 and P105 devices.

## [Rust v0.7.13][v0.7.13] - 2024-08-26

### Changed

- To align with the latest API updates, the `overheated` field for plugs has been replaced by three enums: `overcurrent_status`, `overheat_status`, and `power_protection_status` (thanks to @padenot).

## [Python v0.3.2][py-v0.3.2] - 2024-08-26

### Changed

- To align with the latest API updates, the `overheated` field for plugs has been replaced by three enums: `overcurrent_status`, `overheat_status`, and `power_protection_status`.

## [Rust v0.7.12][v0.7.12] - 2024-06-27

### Changed

- H100's create child device handler methods now take a `HubDevice` `enum` instead of a `String` and are now `async` to allow for more flexibility. This enables the caller to find child devices by either device ID or nickname.
- `PlugIdentifier` has been renamed to `Plug`.
- `Plug::ByDeviceId` now verifies that the provided device ID is found and returns an `Error::DeviceNotFound` error when it's not.
- `HubDevice` variants now take a `String` instead of a `&str` to allow for more flexibility.
- `Plug` variants now take a `String` instead of a `&str` to allow for more flexibility.

### Fixed

- `ColorLightSetDeviceInfoParams` `hue` field validation has been changed from `between 1 and 360` to `between 0 and 360` to match the device's expected range.
- Fixed an issue where the `EnergyDataResult's `start_timestamp` and `end_timestamp` did not correctly adjust for timezone offsets.
- The `chrono` dependency has been updated to `0.4.34` to fix the minimum version requirement.

### Removed

- The `overheated` property has been removed from `DeviceInfoGenericResult` because it's not present in the response of all devices.

## [Python v0.3.1][py-v0.3.1] - 2024-06-27

### Fixed

- `ColorLightSetDeviceInfoParams` `hue` field validation has been changed from `between 1 and 360` to `between 0 and 360` to match the device's expected range.
- Fixed an issue where the `EnergyDataResult's `start_timestamp` and `end_timestamp` did not correctly adjust for timezone offsets.
- All handlers are now correctly exported and can be imported from the `tapo` module.

### Removed

- The `overheated` property has been removed from `DeviceInfoGenericResult` because it's not present in the response of all devices.

## [Rust v0.7.11][v0.7.11] - 2024-05-04

### Added

- Added support for the P300 power strip (thanks to @Michal-Szczepaniak).
- `RgbLightStripHandler` and `DeviceInfoRgbLightStripResult` have been added to support the L900 devices separately from the L530 and L630 devices.

### Changed

- `ChildDeviceResult` has been renamed to `ChildDeviceHubResult` to facilitate adding support for other devices with children.
- `ColorLightStripHandler` has been renamed to `RgbicLightStripHandler` to better reflect its purpose.
- `DeviceInfoColorLightStripResult` has been renamed to `DeviceInfoRgbicLightStripResult` to better reflect its purpose.

## [Python v0.3.0][py-v0.3.0] - 2024-05-04

### Added

- Added partial support for the H100 hub and its child devices. Currently, only the `get_device_info` function is supported for the child devices through the hub's `get_child_device_list` method.

### Changed

- A large number of types have been reorganized to me more in line with the Rust library. This includes moving many of them under the `requests` and `responses` sub modules.

### Removed

- `l900` has been removed from the `ApiClient` until proper support is added.

## [Rust v0.7.10][v0.7.10] - 2024-04-05

### Changed

- The implementation of `ApiClient::new` has been improved to allow for the return of `ApiClient` instead of `Result<ApiClient, Error>`.
- The default timeout for all requests has been reduced to 30 seconds from 300 seconds.
- `ApiClient::with_timeout` has been added to allow for the setting of a custom timeout for all requests (thanks to @skoky).

## [Python v0.2.1][py-v0.2.1] - 2024-04-05

### Changed

- The default timeout for all requests has been reduced to 30 seconds from 300 seconds.
- The `timeout_s` optional parameter has been added to the `ApiClient` constructor to allow for the setting of a custom timeout for all requests (thanks to @skoky).

## [Rust v0.7.9][v0.7.9] - 2024-01-27

### Changed

- The `send()` method of the `.set()` API now takes a reference to the device handler in order to allow for better ergonomics.

### Fixed

- The device info response for the L510, L520, and L610 devices has been fixed.

## [Python v0.2.0][py-v0.2.0] - 2024-01-27

### Added

- Added support for the L530, L630, and L900 color light bulbs.

### Fixed

- Fixed a misconfiguration that was preventing the sdist package from working properly.
- The device info response for the L510, L520, and L610 devices has been fixed.

## [Rust v0.7.8][v0.7.8] - 2024-01-22

### Added

- Added the `device_reset` method to all plugs and lights.

### Fixed

- The device info response for the L510, L520, and L610 devices has been fixed to have the `re_power_type` field as optional.

## [Python v0.1.5][py-v0.1.5] - 2024-01-22

### Added

- Added the `device_reset` method to all plugs and lights.

### Fixed

- The device info response for the L510, L520, and L610 devices has been fixed to have the `re_power_type` field as optional.

## [Rust v0.7.7][v0.7.7] - 2024-01-13

### Changed

- The `anyhow::anyhow!("Local hash does not match server hash")` error has been replaced with the more specific `tapo::TapoResponseError::InvalidCredentials` error.

### Fixed

- The `default_states` field that's part of the device info response has been changed for the L510, L520, and L610 devices to match the actual response from the device.
- A handful of edge cases around the Klap Protocol that were causing panics have been fixed to return `tapo::TapoResponseError::SessionTimeout` or `tapo::TapoResponseError::InvalidResponse` errors instead.

## [Python v0.1.4][py-v0.1.4] - 2024-01-13

### Changed

- The "Local hash does not match server hash" error has been replaced with the more specific `tapo::TapoResponseError::InvalidCredentials` error.

### Fixed

- The `default_states` field that's part of the device info response has been changed for the L510, L520, and L610 devices to match the actual response from the device.
- A handful of edge cases around the Klap Protocol that were causing panics have been fixed to return `tapo::TapoResponseError::SessionTimeout` or `tapo::TapoResponseError::InvalidResponse` errors instead.

## [Rust v0.7.6][v0.7.6] - 2023-11-25

### Added

- Added support for the KE100 thermostatic radiator valve (TRV) devices (thanks to @pwoerndle).

### Fixed

- Fixed an issue that was preventing the `nickname` field from being decoded in the `get_device_info` results of child devices of the H100 hub.

## [Rust v0.7.5][v0.7.5] - 2023-11-05

### Added

- Added support for the T300 water sensor.
- Added a dedicated handler for the L520 devices.

## [Python v0.1.3][py-v0.1.3] - 2023-11-04

### Added

- Added support for the L510, L520 and L610 light bulbs.

### Changed

- The minimum required version of Python has been changed to 3.8, up from 3.7.

### Fixed

- Fixed an issue that was preventing `get_device_info_json` from working on the plug devices.

## [Python v0.1.2][py-v0.1.2] - 2023-10-19

### Added

- Added support for generic devices.
- Added `get_device_info_json` to all currently supported devices.

## [Python v0.1.1][py-v0.1.1] - 2023-10-01

This is the first version of the Python wrapper library.
It supports the plug devices P100, P105, P110 and P115.

## [v0.7.4] - 2023-09-15

### Fixed

- Fixed the minimum version of the chrono dependency by setting it to 0.4.25.

### Changes

- `DeviceUsageResult` has been split into `DeviceUsageResult` and `DeviceUsageEnergyMonitoringResult`. The former is now returned for the P100 and P105 devices while the latter is returned for all the other devices that support energy monitoring.
- `EnergyMonitoringPlugHandler` has been renamed to `PlugEnergyMonitoringHandler`.
- All `___DeviceInfoResult` structs have been renamed to `DeviceInfo___Result`.
- All `___DefaultState` structs have been renamed to `Default___State`.

### Removed

- `get_device_usage` has been removed from the `GenericDeviceHandler` because it is not supported by all devices.

## [v0.7.3] - 2023-09-14

### Added

- Added support for the newly introduced KLAP protocol, which is required to interact with the latest firmware version of multiple devices.

### Changed

- All uses of `time` have been replaced with `chrono`:
  - `EnergyDataInterval`'s `time::OffsetDateTime` and `time::Date` fields have been replaced with `chrono::NaiveDate`.
  - `EnergyUsageResult::local_time` field is now `chrono::NaiveDateTime` instead of `time::OffsetDateTime`.
  - `EnergyDataResult::local_time` field is now `chrono::NaiveDateTime` instead of `time::OffsetDateTime`.
  - `TemperatureHumidityRecords`'s and `TemperatureHumidityRecord` `datetime` fields are now `chrono::DateTime<chrono::Utc>` instead of `time::OffsetDateTime`.
- `EnergyDataInterval::Hourly::start_datetime` and `EnergyDataInterval::Hourly::end_datetime` have been renamed to `start_date` and `end_date` because the time component is not required.
- The `login` function on all handlers has been renamed to `refresh_session` to better reflect its purpose and it now takes and returns a `&mut self` instead of `self`.
- `L510DeviceInfoResult` has been renamed to `LightDeviceInfoResult` to better reflect its purpose when used for L510 and L610 devices.
- `L530DeviceInfoResult` has been renamed to `ColorLightDeviceInfoResult` to better reflect its purpose when used for L530, L630 and L900 devices.
- `L930DeviceInfoResult` has been renamed to `ColorLightStripDeviceInfoResult` to better reflect its purpose when used for L920 and L930 devices.
- The `default_states` field of `LightDeviceInfoResult`, `ColorLightDeviceInfoResult`, `ColorLightStripDeviceInfoResult` and `PlugDeviceInfoResult` is now a struct instead of an enum.

## [v0.7.2] - 2023-08-21

### Added

- Added `get_current_power` to the `P110` and `P115` plugs. (thanks to @Michal-Szczepaniak)

## [v0.7.1] - 2023-05-30

### Added

- Added `get_temperature_humidity_records` to the `T310` and `T315` sensors.

### Changed

- The creation of device handlers has been simplified.

```rust
// old
let device = ApiClient::new(ip_address, tapo_username, tapo_password)?
    .l530()
    .login()
    .await?;

// new
let device = ApiClient::new(tapo_username, tapo_password)?
    .l530(ip_address)
    .await?;
```

- The creation of child device handlers has been reworked so that they can be created without requiring a call to `get_child_device_list` when the child Device ID is known.
- `ApiClient` now implements `Clone` to allow for a cheaper duplication of the client.

### Removed

- The `L510` and `L610` devices no longer expose the `set()` API because changing multiple properties simultaneously does not make sense for these devices.

## [v0.7.0] - 2023-05-26

### Added

- Added initial support for the H100 device, the S200B switch and the T100, T110, T310, T315 sensors. The child devices currently support `get_device_info` and `get_trigger_logs`.
- All responses now derive `serde::Serialize` to allow for more straightforward consumer serialisation. (thanks to @ClementNerma)
- `ApiClient` has been marked as both `Send` and `Sync` to allow for sharing between threads. (thanks to @ClementNerma)

### Changed

- `GenericDeviceInfoResult`'s `device_on` property has been made optional to accommodate devices that do not provide this field.

## [v0.6.0] - 2023-05-08

### Added
- Added support for the L920 and L930 light strips. The highlight is the `tapo::ColorLightStripHandler::set_lighting_effect` function, which supports all the effects that the Tapo app contains alongside user-defined effects.
- Added support for the L900 light strips.
- Each supported device now has it's own handler creator.

### Changed
- `set_*` functions like `tapo::requests::ColorLightSetDeviceInfoParams::set_brightness` now return `Self` instead of `Result<Self, Error>` to allow for better ergonomics. The validations will now run when `tapo::requests::ColorLightSetDeviceInfoParams::send` is called.
- `tapo::requests::L510SetDeviceInfoParams` has been renamed to `tapo::requests::LightSetDeviceInfoParams` to better reflect its purpose when used for L510, L610, and L900 devices.
- `tapo::requests::L530SetDeviceInfoParams` has been renamed to `tapo::requests::ColorLightSetDeviceInfoParams` to better reflect its purpose when used for L530, L630, L920 and L930 devices.
- `tapo::P100Handler` has been renamed to `tapo::PlugHandler`.
- `tapo::P110Handler` has been renamed to `tapo::EnergyMonitoringPlugHandler`.
- `tapo::L510Handler` has been renamed to `tapo::LightHandler`.
- `tapo::L530Handler` has been renamed to `tapo::ColorLightHandler`.
- `tapo::L930Handler` has been renamed to `tapo::ColorLightStripHandler`.

## [v0.5.0] - 2023-04-16

### Changed

- The creation of an API Client for a specific device is now done through handler methods on the `ApiClient` struct. This allows for a more ergonomic API. (thanks to [Octocrab](https://github.com/XAMPPRocky/octocrab) for inspirations)

```rust
// old
let device = ApiClient::<L530>::new(ip_address, tapo_username, tapo_password, true).await?;

// new
let device = ApiClient::new(ip_address, tapo_username, tapo_password)?
    .l530()
    .login()
    .await?;
```

- `ApiClient::new` parameters are now `impl Into<String>` instead of `String` to allow for more flexibility.
- Error handling has been reworked. All functions that could error now return a `Result<..., tapo::Error>`.

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

[Unreleased]: https://github.com/mihai-dinculescu/tapo
[v0.8.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.8.1
[v0.8.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.8.0
[py-v0.7.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.7.0
[v0.7.17]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.17
[py-v0.6.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.6.0
[v0.7.16]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.16
[py-v0.5.1]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.5.1
[v0.7.15]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.15
[py-v0.5.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.5.0
[v0.7.14]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.14
[py-v0.4.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.4.0
[v0.7.13]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.13
[py-v0.3.2]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.3.2
[v0.7.12]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.12
[py-v0.3.1]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.3.1
[v0.7.11]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.11
[py-v0.3.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.3.0
[v0.7.10]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.10
[py-v0.2.1]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.2.1
[v0.7.9]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.9
[py-v0.2.0]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.2.0
[v0.7.8]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.8
[py-v0.1.5]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.1.5
[v0.7.7]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.7
[py-v0.1.4]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.1.4
[v0.7.6]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.6
[v0.7.5]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.5
[py-v0.1.3]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.1.3
[py-v0.1.2]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.1.2
[py-v0.1.1]: https://github.com/mihai-dinculescu/tapo/tree/py-v0.1.1
[v0.7.4]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.4
[v0.7.3]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.3
[v0.7.2]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.2
[v0.7.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.1
[v0.7.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.7.0
[v0.6.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.6.0
[v0.5.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.5.0
[v0.4.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.4.0
[v0.3.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.3.1
[v0.3.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.3.0
[v0.2.1]: https://github.com/mihai-dinculescu/tapo/tree/v0.2.1
[v0.2.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.2.0
[v0.1.0]: https://github.com/mihai-dinculescu/tapo/tree/v0.1.0
[keepachangelog]: https://keepachangelog.com
