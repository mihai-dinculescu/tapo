use std::ops::RangeInclusive;

use chrono::{DateTime, NaiveDate, Utc};
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use tapo::requests::{
    BRIGHTNESS_RANGE, COLOR_TEMPERATURE_RANGE, Color, HUE_RANGE, SATURATION_RANGE,
};
use tapo::responses::DetectionResult;

/// The Tapo devices found on the local network by the most recent discovery.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DevicesList {
    /// When the discovery that produced this list started.
    pub discovered_at: DateTime<Utc>,
    /// Supported devices found on the network.
    pub devices: Vec<Device>,
    /// Devices that are currently unsupported.
    pub unsupported: Vec<UnsupportedDevice>,
    /// Errors encountered during discovery.
    pub errors: Vec<DiscoveryError>,
    /// Why the most recent background discovery failed, if it did. The devices are then
    /// the ones from the last successful discovery, at `discovered_at`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_error: Option<String>,
}

/// A capability that can be set on a device with `control_device`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum SetCapability {
    /// Set the device brightness (1-100).
    Brightness,
    /// Set the device color using a preset name.
    Color,
    /// Set the device color using a hue (0-360) and saturation (1-100).
    HueSaturation,
    /// Set the device color temperature (2500-6500).
    ColorTemperature,
    /// Turn the device on or off.
    OnOff,
}

/// A capability that can be read from a device: `Snapshot` with `take_snapshot`, the
/// others with `get_device_state`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, IntoStaticStr)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum GetCapability {
    /// Read the detections stored on a camera's SD card, with the events
    /// detected, for a window of up to 24 hours (C120, C210, C220, C225,
    /// C325WB, C520WS, TC40, TC70).
    Detections,
    /// Read the device's current state.
    DeviceInfo,
    /// Read the energy used per hour, in watt-hours (P110, P110M, P115, P304M, P316M).
    EnergyDataHourly,
    /// Read the energy used per day, in watt-hours (P110, P110M, P115, P304M, P316M).
    EnergyDataDaily,
    /// Read the energy used per month, in watt-hours (P110, P110M, P115, P304M, P316M).
    EnergyDataMonthly,
    /// Read the live voltage, current and power, and the energy used since the
    /// device last restarted (P110, P110M, P115, P304M, P316M).
    EnergyMeterData,
    /// Read the power draw every 5 minutes, in watts (P110, P110M, P115, P304M, P316M).
    PowerDataEvery5Minutes,
    /// Read the power draw every hour, in watts (P110, P110M, P115, P304M, P316M).
    PowerDataHourly,
    /// Capture a still JPEG snapshot via the `take_snapshot` tool.
    Snapshot,
    /// Read the last 24 hours of temperature and humidity records (T310, T315) at 15 minute intervals.
    TemperatureHumidityRecords,
    /// Read paginated trigger logs from a hub child sensor (S200, T100, T110, T300).
    TriggerLogs,
}

impl GetCapability {
    /// The capability's name, as `list_devices` lists it.
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

/// A supported Tapo device found on the local network.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Device {
    /// Unique device identifier.
    pub id: String,
    /// User-assigned device name.
    pub name: String,
    /// Device model (e.g. "L530", "P110").
    pub model: String,
    /// Device IP address on the local network.
    pub ip: String,
    /// Capabilities that can be set on this device.
    pub set_capabilities: Vec<SetCapability>,
    /// Capabilities that can be read from this device.
    pub get_capabilities: Vec<GetCapability>,
    /// Child devices (e.g. individual plugs on a power strip).
    pub children: Vec<ChildDevice>,
}

/// A device attached to a parent device, such as a child plug on a power strip or a device paired with a hub.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ChildDevice {
    /// Unique child device identifier.
    pub id: String,
    /// User-assigned child device name.
    pub name: String,
    /// Child device model.
    pub model: String,
    /// IP address of the parent device. Child devices have no IP of their own;
    /// they are reached through their parent's.
    pub ip: String,
    /// Capabilities that can be set on this child device.
    pub set_capabilities: Vec<SetCapability>,
    /// Capabilities that can be read from this child device.
    pub get_capabilities: Vec<GetCapability>,
}

/// A Tapo device found on the local network that this server can't control.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct UnsupportedDevice {
    /// Device IP address on the local network.
    pub ip: String,
    /// Device model string.
    pub model: String,
}

/// An error from discovering a device or reading its child devices.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DiscoveryError {
    /// IP address of the device that encountered the error.
    pub ip: String,
    /// Error description.
    pub message: String,
}

/// The detections stored on a camera's SD card within the window that was read.
#[derive(Debug, Serialize)]
pub struct DetectionsResult {
    /// The detections, oldest first. Each has its `start_time`, its `end_time`,
    /// the event the camera filed it under (`trigger`) and every event detected
    /// in that span (`events`).
    pub detections: Vec<DetectionResult>,
}

/// A still JPEG snapshot taken from a Tapo camera.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SnapshotResult {
    /// Unique identifier of the camera the snapshot was taken from.
    pub device_id: String,
    /// When the snapshot was received from the camera.
    pub captured_at: DateTime<Utc>,
    /// Size of the image in bytes.
    pub size_bytes: usize,
    /// Link to the image, valid until `expires_at`. Share it with the user or
    /// fetch it to save the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// When `url` stops working.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Parameters for `check_device`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckDeviceParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`. For a child device, its `ip`, which is the parent's.
    pub ip: String,
}

/// A capability to set on a device, with the value to set it to.
#[derive(Debug, Clone, Deserialize, JsonSchema, IntoStaticStr)]
#[serde(tag = "type")]
pub enum SetCapabilityRequest {
    /// Set the device brightness. Also turns the device on if it's off.
    Brightness {
        /// Brightness level.
        #[schemars(range(min = *BRIGHTNESS_RANGE.start(), max = *BRIGHTNESS_RANGE.end()))]
        value: u8,
    },
    /// Set the device color using a preset name. Also turns the device on if it's off.
    Color {
        /// Preset color name.
        value: Color,
    },
    /// Set the device color using a hue and saturation. Also turns the device on if it's off.
    HueSaturation {
        /// Hue, in degrees.
        #[schemars(range(min = *HUE_RANGE.start(), max = *HUE_RANGE.end()))]
        hue: u16,
        /// Saturation level.
        #[schemars(range(min = *SATURATION_RANGE.start(), max = *SATURATION_RANGE.end()))]
        saturation: u8,
    },
    /// Set the device color temperature. Also turns the device on if it's off.
    ColorTemperature {
        /// Color temperature, in kelvin.
        #[schemars(range(min = *COLOR_TEMPERATURE_RANGE.start(), max = *COLOR_TEMPERATURE_RANGE.end()))]
        value: u16,
    },
    /// Turn the device on or off.
    OnOff {
        /// `true` to turn on, `false` to turn off.
        value: bool,
    },
}

impl SetCapabilityRequest {
    /// The capability's name, without its value.
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

/// A capability to read from a device, with its parameters.
#[derive(Debug, Clone, Deserialize, JsonSchema, IntoStaticStr)]
#[cfg_attr(test, derive(strum::VariantNames))]
#[serde(tag = "type")]
pub enum GetCapabilityRequest {
    /// Read the detections stored on a camera's SD card between
    /// `start_date_time` and `end_date_time`. Each comes with its start, its
    /// end, the event the camera filed it under (`trigger`) and every event
    /// detected in that span (`events`), such as `Person` and `Motion`. Fails
    /// with `STORAGE_NOT_EXIST` when the camera has no SD card.
    Detections {
        /// The start of the window, in UTC, in the years 2000 to 9999.
        start_date_time: DateTime<Utc>,
        /// The end of the window, in UTC, at most 24 hours after
        /// `start_date_time`.
        end_date_time: DateTime<Utc>,
    },
    /// Read the device's current state (on/off, brightness, etc.).
    DeviceInfo,
    /// Read the energy used per hour, in watt-hours, for whole days, from an
    /// energy-monitoring plug (P110, P110M, P115) or power strip child plug
    /// (P304M, P316M). Dates are in the plug's time zone. Hours that have not
    /// started yet are left out.
    EnergyDataHourly {
        /// The first day to read, in the years 2000 to 9999.
        start_date: NaiveDate,
        /// The number of days to read, including `start_date`.
        #[schemars(range(min = *ENERGY_HOURLY_DAYS.start(), max = *ENERGY_HOURLY_DAYS.end()))]
        days: u8,
    },
    /// Read the energy used per day, in watt-hours, for a quarter of a year,
    /// from an energy-monitoring plug (P110, P110M, P115) or power strip
    /// child plug (P304M, P316M). Dates are in the plug's time zone. Days that
    /// have not started yet are left out.
    EnergyDataDaily {
        /// The year the quarter is in.
        #[schemars(range(min = *DATA_YEARS.start(), max = *DATA_YEARS.end()))]
        year: i32,
        /// The quarter: 1 for January to March, 2 for April to June,
        /// 3 for July to September, 4 for October to December.
        #[schemars(range(min = *QUARTERS.start(), max = *QUARTERS.end()))]
        quarter: u8,
    },
    /// Read the energy used per month, in watt-hours, for a year, from an
    /// energy-monitoring plug (P110, P110M, P115) or power strip child plug
    /// (P304M, P316M). Dates are in the plug's time zone. Months that have not
    /// started yet are left out.
    EnergyDataMonthly {
        /// The year to read.
        #[schemars(range(min = *DATA_YEARS.start(), max = *DATA_YEARS.end()))]
        year: i32,
    },
    /// Read the live voltage in millivolts, current in milliamperes and active
    /// power in milliwatts, and the energy used since the device last
    /// restarted, as one total given in both watt-hours and milliwatt-hours
    /// (not to be added together), from an energy-monitoring plug
    /// (P110, P110M, P115) or power strip child plug (P304M, P316M).
    EnergyMeterData,
    /// Read the power draw every 5 minutes, in watts, from an
    /// energy-monitoring plug (P110, P110M, P115) or power strip child plug
    /// (P304M, P316M).
    PowerDataEvery5Minutes {
        /// When to start reading, in UTC, in the years 2000 to 9999. Rounded up
        /// to the next 5 minute mark.
        start_date_time: DateTime<Utc>,
        /// The number of hours to read.
        #[schemars(range(min = *POWER_5_MINUTE_HOURS.start(), max = *POWER_5_MINUTE_HOURS.end()))]
        hours: u8,
    },
    /// Read the power draw every hour, in watts, from an energy-monitoring
    /// plug (P110, P110M, P115) or power strip child plug (P304M, P316M).
    PowerDataHourly {
        /// When to start reading, in UTC, in the years 2000 to 9999. Rounded up
        /// to the next hour.
        start_date_time: DateTime<Utc>,
        /// The number of days to read.
        #[schemars(range(min = *POWER_HOURLY_DAYS.start(), max = *POWER_HOURLY_DAYS.end()))]
        days: u8,
    },
    /// Read the last 24 hours of temperature and humidity records (T310, T315)
    /// at 15 minute intervals.
    TemperatureHumidityRecords,
    /// Read paginated trigger logs from a hub child sensor
    /// (S200, T100, T110, T300). Logs are returned newest-first.
    TriggerLogs {
        /// The maximum number of log items to return.
        #[schemars(range(min = 1))]
        page_size: u64,
        /// The log item `id` from which to start returning results in reverse
        /// chronological order (newest first). Use `0` to get the most recent
        /// `page_size` logs.
        start_id: u64,
    },
}

impl GetCapabilityRequest {
    /// The capability's name, without its parameters.
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

/// The years energy, power and detection data can be read for. Tapo devices
/// postdate 2000, and the bounds keep every date's timestamp positive and finite.
pub const DATA_YEARS: RangeInclusive<i32> = 2000..=9999;
/// The longest window `GetCapabilityRequest::Detections` can read, in hours.
/// The camera rejects a longer one.
pub const DETECTIONS_WINDOW_HOURS: i64 = 24;
/// The number of days `GetCapabilityRequest::EnergyDataHourly` can read.
pub const ENERGY_HOURLY_DAYS: RangeInclusive<u8> = 1..=8;
/// The quarters of a year.
pub const QUARTERS: RangeInclusive<u8> = 1..=4;
/// The number of hours `GetCapabilityRequest::PowerDataEvery5Minutes` can read.
pub const POWER_5_MINUTE_HOURS: RangeInclusive<u8> = 1..=12;
/// The number of days `GetCapabilityRequest::PowerDataHourly` can read.
pub const POWER_HOURLY_DAYS: RangeInclusive<u8> = 1..=6;
/// The number of set capabilities one `control_device` call can apply.
pub const CONTROL_CAPABILITIES: RangeInclusive<usize> = 1..=12;

/// Parameters for `control_device`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlDeviceParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`. For a child device, its `ip`, which is the parent's.
    pub ip: String,
    /// The set capabilities to apply.
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    #[schemars(length(min = *CONTROL_CAPABILITIES.start(), max = *CONTROL_CAPABILITIES.end()))]
    pub capabilities: Vec<SetCapabilityRequest>,
}

/// Parameters for `get_device_state`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDeviceStateParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`. For a child device, its `ip`, which is the parent's.
    pub ip: String,
    /// The get capability to read.
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    pub capability: GetCapabilityRequest,
}

/// Parameters for `take_snapshot`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TakeSnapshotParams {
    /// Device ID from `list_devices`. Must identify a camera.
    pub id: String,
    /// Device IP address from `list_devices`.
    pub ip: String,
}

/// Deserializes a value that may have been stringified by the MCP client.
///
/// Some clients (e.g. Claude Code) use an XML-to-JSON pipeline that doesn't
/// coerce types based on the tool's `inputSchema`, causing nested objects to
/// arrive as JSON strings. This handles both proper JSON objects
/// (e.g. `{"type": "Brightness", "value": 100}`) and stringified JSON
/// (e.g. the string `'{"type": "Brightness", "value": 100}'`).
///
/// See: https://github.com/anthropics/claude-code/issues/32524
fn deserialize_from_stringified_json<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match &value {
        serde_json::Value::String(s) => serde_json::from_str(s).map_err(de::Error::custom),
        _ => serde_json::from_value(value).map_err(de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use serde_json::json;
    use strum::{IntoEnumIterator as _, VariantNames as _};

    use super::*;

    /// `name()` must stay equal to the serde `type` tag that clients send.
    #[test]
    fn capability_names_match_serde_tags() {
        let set_requests = [
            SetCapabilityRequest::Brightness { value: 50 },
            SetCapabilityRequest::Color {
                value: Color::Daylight,
            },
            SetCapabilityRequest::HueSaturation {
                hue: 120,
                saturation: 75,
            },
            SetCapabilityRequest::ColorTemperature { value: 3000 },
            SetCapabilityRequest::OnOff { value: true },
        ];
        for request in set_requests {
            let json = match &request {
                SetCapabilityRequest::Brightness { value } => {
                    json!({ "type": request.name(), "value": value })
                }
                SetCapabilityRequest::Color { value } => {
                    json!({ "type": request.name(), "value": value })
                }
                SetCapabilityRequest::HueSaturation { hue, saturation } => {
                    json!({ "type": request.name(), "hue": hue, "saturation": saturation })
                }
                SetCapabilityRequest::ColorTemperature { value } => {
                    json!({ "type": request.name(), "value": value })
                }
                SetCapabilityRequest::OnOff { value } => {
                    json!({ "type": request.name(), "value": value })
                }
            };
            let parsed: SetCapabilityRequest = serde_json::from_value(json).unwrap();
            assert_eq!(discriminant(&parsed), discriminant(&request));
        }

        let get_requests = [
            GetCapabilityRequest::Detections {
                start_date_time: DateTime::UNIX_EPOCH,
                end_date_time: DateTime::UNIX_EPOCH,
            },
            GetCapabilityRequest::DeviceInfo,
            GetCapabilityRequest::EnergyDataHourly {
                start_date: NaiveDate::MIN,
                days: 1,
            },
            GetCapabilityRequest::EnergyDataDaily {
                year: 2026,
                quarter: 4,
            },
            GetCapabilityRequest::EnergyDataMonthly { year: 2026 },
            GetCapabilityRequest::EnergyMeterData,
            GetCapabilityRequest::PowerDataEvery5Minutes {
                start_date_time: DateTime::UNIX_EPOCH,
                hours: 1,
            },
            GetCapabilityRequest::PowerDataHourly {
                start_date_time: DateTime::UNIX_EPOCH,
                days: 1,
            },
            GetCapabilityRequest::TemperatureHumidityRecords,
            GetCapabilityRequest::TriggerLogs {
                page_size: 10,
                start_id: 0,
            },
        ];
        for request in get_requests {
            let mut value = match &request {
                GetCapabilityRequest::Detections { .. } => {
                    json!({ "start_date_time": "2026-10-17T00:00:00Z", "end_date_time": "2026-10-17T01:00:00Z" })
                }
                GetCapabilityRequest::EnergyDataHourly { .. } => {
                    json!({ "start_date": "2026-10-17", "days": 1 })
                }
                GetCapabilityRequest::EnergyDataDaily { .. } => {
                    json!({ "year": 2026, "quarter": 4 })
                }
                GetCapabilityRequest::EnergyDataMonthly { .. } => json!({ "year": 2026 }),
                GetCapabilityRequest::PowerDataEvery5Minutes { .. } => {
                    json!({ "start_date_time": "2026-10-17T00:00:00Z", "hours": 1 })
                }
                GetCapabilityRequest::PowerDataHourly { .. } => {
                    json!({ "start_date_time": "2026-10-17T00:00:00Z", "days": 1 })
                }
                GetCapabilityRequest::TriggerLogs { .. } => {
                    json!({ "page_size": 10, "start_id": 0 })
                }
                GetCapabilityRequest::DeviceInfo
                | GetCapabilityRequest::EnergyMeterData
                | GetCapabilityRequest::TemperatureHumidityRecords => json!({}),
            };
            value["type"] = json!(request.name());
            let parsed: GetCapabilityRequest = serde_json::from_value(value).unwrap();
            assert_eq!(discriminant(&parsed), discriminant(&request));
        }
    }

    /// Every capability that `list_devices` lists, except `Snapshot`, must be a
    /// `type` that `get_device_state` accepts, and the other way around.
    #[test]
    fn listed_capabilities_match_request_types() {
        let mut listed: Vec<String> = GetCapability::iter()
            .filter(|capability| !matches!(capability, GetCapability::Snapshot))
            .map(|capability| serde_json::to_value(capability).unwrap())
            .map(|value| value.as_str().unwrap().to_string())
            .collect();
        let mut requested = GetCapabilityRequest::VARIANTS.to_vec();
        listed.sort_unstable();
        requested.sort_unstable();

        assert_eq!(listed, requested);
    }
}
