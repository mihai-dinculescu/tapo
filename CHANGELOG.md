# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com][keepachangelog].

## [Unreleased]

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

[unreleased]: https://github.com/mihai-dinculescu/tapo
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
