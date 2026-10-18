use std::ops::RangeInclusive;

use chrono::{DateTime, NaiveDate, Utc};
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::de;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;
use tapo::requests::Color;

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
    /// Turn the device on or off.
    OnOff,
}

/// A capability that can be read from a device with `get_device_state`.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum GetCapability {
    /// Read the device's current state.
    DeviceInfo,
    /// Read the energy used per hour, day or month, in watt-hours (P110, P110M, P115, P304M, P316M).
    EnergyData,
    /// Read the power draw every 5 minutes or every hour, in watts (P110, P110M, P115, P304M, P316M).
    PowerData,
    /// Capture a still JPEG snapshot via the `take_snapshot` tool.
    Snapshot,
    /// Read the last 24 hours of temperature and humidity records (T310, T315) at 15 minute intervals.
    TemperatureHumidityRecords,
    /// Read paginated trigger logs from a hub child sensor (S200, T100, T110, T300).
    TriggerLogs,
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

/// A device attached to a parent device, such as an outlet on a power strip or a device paired with a hub.
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
        #[schemars(range(min = 1, max = 100))]
        value: u8,
    },
    /// Set the device color using a preset name. Also turns the device on if it's off.
    Color {
        /// Preset color name.
        value: Color,
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
#[serde(tag = "type")]
pub enum GetCapabilityRequest {
    /// Read the device's current state (on/off, brightness, etc.).
    DeviceInfo,
    /// Read the energy used per hour, day or month, in watt-hours, from an
    /// energy-monitoring plug (P110, P110M, P115) or power strip outlet
    /// (P304M, P316M).
    EnergyData {
        /// The period to read and how finely to split it.
        interval: EnergyDataIntervalRequest,
    },
    /// Read the power draw every 5 minutes or every hour, in watts, from an
    /// energy-monitoring plug (P110, P110M, P115) or power strip outlet
    /// (P304M, P316M).
    PowerData {
        /// The period to read and how finely to split it.
        interval: PowerDataIntervalRequest,
    },
    /// Not callable via `get_device_state`. Returns an error pointing to the
    /// dedicated `take_snapshot` tool. Listed here so the schema can give a
    /// helpful error rather than `unknown variant 'Snapshot'`.
    Snapshot,
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

/// The years energy and power data can be read for. Tapo plugs postdate 2000,
/// and the bounds keep every date's timestamp positive and finite.
pub const DATA_YEARS: RangeInclusive<i32> = 2000..=9999;
/// The number of days `EnergyDataIntervalRequest::Hourly` can read.
pub const ENERGY_HOURLY_DAYS: RangeInclusive<u8> = 1..=8;
/// The quarters of a year.
pub const QUARTERS: RangeInclusive<u8> = 1..=4;
/// The number of hours `PowerDataIntervalRequest::Every5Minutes` can read.
pub const POWER_5_MINUTE_HOURS: RangeInclusive<u8> = 1..=12;
/// The number of days `PowerDataIntervalRequest::Hourly` can read.
pub const POWER_HOURLY_DAYS: RangeInclusive<u8> = 1..=6;

/// The period to read energy data for. Dates are in the plug's time zone.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum EnergyDataIntervalRequest {
    /// One entry per hour, for whole days.
    Hourly {
        /// The first day to read, in the years 2000 to 9999.
        start_date: NaiveDate,
        /// The number of days to read, including `start_date`.
        #[schemars(range(min = *ENERGY_HOURLY_DAYS.start(), max = *ENERGY_HOURLY_DAYS.end()))]
        days: u8,
    },
    /// One entry per day, for a quarter of a year.
    Daily {
        /// The year the quarter is in.
        #[schemars(range(min = *DATA_YEARS.start(), max = *DATA_YEARS.end()))]
        year: i32,
        /// The quarter: 1 for January to March, 2 for April to June,
        /// 3 for July to September, 4 for October to December.
        #[schemars(range(min = *QUARTERS.start(), max = *QUARTERS.end()))]
        quarter: u8,
    },
    /// One entry per month, for a year.
    Monthly {
        /// The year to read.
        #[schemars(range(min = *DATA_YEARS.start(), max = *DATA_YEARS.end()))]
        year: i32,
    },
}

/// The period to read power data for.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum PowerDataIntervalRequest {
    /// One entry every 5 minutes.
    Every5Minutes {
        /// When to start reading, in UTC, in the years 2000 to 9999. Rounded up
        /// to the next 5 minute mark.
        start_date_time: DateTime<Utc>,
        /// The number of hours to read.
        #[schemars(range(min = *POWER_5_MINUTE_HOURS.start(), max = *POWER_5_MINUTE_HOURS.end()))]
        hours: u8,
    },
    /// One entry every hour.
    Hourly {
        /// When to start reading, in UTC, in the years 2000 to 9999. Rounded up
        /// to the next hour.
        start_date_time: DateTime<Utc>,
        /// The number of days to read.
        #[schemars(range(min = *POWER_HOURLY_DAYS.start(), max = *POWER_HOURLY_DAYS.end()))]
        days: u8,
    },
}

/// Parameters for `control_device`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControlDeviceParams {
    /// Device ID from `list_devices`.
    pub id: String,
    /// Device IP address from `list_devices`. For a child device, its `ip`, which is the parent's.
    pub ip: String,
    /// The set capabilities to apply.
    #[serde(deserialize_with = "deserialize_from_stringified_json")]
    #[schemars(length(min = 1))]
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

    use super::*;

    /// `name()` must stay equal to the serde `type` tag that clients send.
    #[test]
    fn capability_names_match_serde_tags() {
        let set_requests = [
            SetCapabilityRequest::Brightness { value: 50 },
            SetCapabilityRequest::Color {
                value: Color::Daylight,
            },
            SetCapabilityRequest::OnOff { value: true },
        ];
        for request in set_requests {
            let value = match &request {
                SetCapabilityRequest::Brightness { .. } => json!(50),
                SetCapabilityRequest::Color { .. } => json!("Daylight"),
                SetCapabilityRequest::OnOff { .. } => json!(true),
            };
            let parsed: SetCapabilityRequest =
                serde_json::from_value(json!({ "type": request.name(), "value": value })).unwrap();
            assert_eq!(discriminant(&parsed), discriminant(&request));
        }

        let get_requests = [
            GetCapabilityRequest::DeviceInfo,
            GetCapabilityRequest::EnergyData {
                interval: EnergyDataIntervalRequest::Monthly { year: 2026 },
            },
            GetCapabilityRequest::PowerData {
                interval: PowerDataIntervalRequest::Every5Minutes {
                    start_date_time: DateTime::UNIX_EPOCH,
                    hours: 1,
                },
            },
            GetCapabilityRequest::Snapshot,
            GetCapabilityRequest::TemperatureHumidityRecords,
            GetCapabilityRequest::TriggerLogs {
                page_size: 10,
                start_id: 0,
            },
        ];
        for request in get_requests {
            let interval = match &request {
                GetCapabilityRequest::EnergyData { .. } => {
                    json!({ "type": "Monthly", "year": 2026 })
                }
                GetCapabilityRequest::PowerData { .. } => json!({
                    "type": "Every5Minutes",
                    "start_date_time": "2026-10-17T00:00:00Z",
                    "hours": 1,
                }),
                _ => json!(null),
            };
            let parsed: GetCapabilityRequest = serde_json::from_value(json!({
                "type": request.name(),
                "interval": interval,
                "page_size": 10,
                "start_id": 0,
            }))
            .unwrap();
            assert_eq!(discriminant(&parsed), discriminant(&request));
        }
    }
}
